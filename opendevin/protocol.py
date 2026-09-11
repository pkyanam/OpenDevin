"""OpenDevin — protocol layer.

Implements the ConnectRPC `GetChatMessage` wire format that the real `devin`
CLI speaks against `server.codeium.com`, using a dependency-free protobuf
codec (`wire.py`) built from the official descriptors + live capture.

Verified against traffic captured from `devin 3000.10.21`
(see reports/ and analysis/devin_protocol.md in the repo root).
"""
from __future__ import annotations

import gzip
import json
import os
import struct
import threading
import uuid
from typing import Any, Dict, Iterator, List, Optional, Tuple

import requests

from .wire import (encode_get_chat_message_request,
                   parse_chat_message_response)

UPSTREAM = os.environ.get("DEVIN_UPSTREAM", "https://server.codeium.com")
CHAT_PATH = "/exa.api_server_pb.ApiServerService/GetChatMessage"

# Identity the real CLI presents (verified from captured traffic):
IDE_NAME = "devin-cli"
IDE_VERSION = "3000.10.21"
EXTENSION_NAME = "chisel"
EXTENSION_TYPE = "chisel"
LOCALE = "en"
OS = "darwin"

REQ_CASCADE = 5            # request_type (CASCADE)
PLANNER_DEFAULT = 1        # planner_mode
SRC_USER, SRC_SYSTEM, SRC_TOOL = 1, 2, 4


# ---------------------------------------------------------------------------
# Credentials
# ---------------------------------------------------------------------------

def load_api_key() -> str:
    env = os.environ.get("DEVIN_API_KEY")
    if env:
        return env
    for cred in (
        os.path.expanduser("~/.local/share/devin/credentials.toml"),
        os.path.expanduser("~/.config/devin/credentials.toml"),
    ):
        if not os.path.exists(cred):
            continue
        for line in open(cred):
            if line.startswith("windsurf_api_key"):
                key = line.split('"')[1].strip()
                if key:
                    return key
    raise RuntimeError(
        "No Devin credential found. Run `devin auth login` first, or set DEVIN_API_KEY."
    )


# ---------------------------------------------------------------------------
# Request construction (mirrors the real `devin` CLI request)
# ---------------------------------------------------------------------------

def _text_of(content) -> str:
    if isinstance(content, str):
        return content
    out = []
    for part in content or []:
        if part.get("type") == "text":
            out.append(part.get("text", ""))
    return "".join(out)


def _images_of(content) -> List[Dict[str, str]]:
    if isinstance(content, str) or not content:
        return []
    out = []
    for part in content or []:
        if part.get("type") == "image_url":
            url = part.get("image_url", {}).get("url", "")
            if url.startswith("data:"):
                mime, _, b64 = url[5:].partition(";base64,")
                out.append({"base64_data": b64, "mime_type": mime or "image/png"})
    return out


def _build_conversation(body: Dict[str, Any], cascade_id: str) -> List[Dict[str, Any]]:
    prompts = []
    for i, m in enumerate(body.get("messages", [])):
        role = m.get("role")
        mid = str(uuid.uuid5(uuid.NAMESPACE_URL, f"{cascade_id}\0{i}\0{role}"))
        if role in ("system", "developer"):
            continue  # folded into the top-level `prompt`
        if role == "user":
            prompts.append({"message_id": mid, "source": SRC_USER,
                            "prompt": _text_of(m.get("content")),
                            "images": _images_of(m.get("content"))})
        elif role == "assistant":
            tcs = [{"id": tc.get("id", ""), "name": tc.get("function", {}).get("name", ""),
                    "arguments_json": tc.get("function", {}).get("arguments", "")}
                   for tc in m.get("tool_calls") or []]
            prompts.append({"message_id": mid, "source": SRC_SYSTEM,
                            "prompt": _text_of(m.get("content")), "tool_calls": tcs})
        elif role == "tool":
            prompts.append({"message_id": mid, "source": SRC_TOOL,
                            "tool_call_id": m.get("tool_call_id", ""),
                            "prompt": _text_of(m.get("content"))})
    return prompts


def build_chat_request(body: Dict[str, Any], api_key: str) -> Tuple[bytes, str]:
    """Translate an OpenAI chat.completions body into a GetChatMessageRequest."""
    conv_seed = next((_text_of(m.get("content")) for m in body.get("messages", [])
                      if m.get("role") == "user"), "")
    cascade_id = str(uuid.uuid5(uuid.NAMESPACE_URL, "opendevin\0" + conv_seed))
    session_id = str(uuid.uuid4())
    request_id = uuid.uuid4().int & (2**63 - 1)

    system_parts = [_text_of(m.get("content")) for m in body.get("messages", [])
                    if m.get("role") in ("system", "developer")]

    tools = [{"name": t["function"]["name"],
              "description": t["function"].get("description", ""),
              "json_schema_string": json.dumps(t["function"].get("parameters") or {}),
              "strict": bool(t["function"].get("strict"))}
             for t in body.get("tools") or [] if t.get("type") == "function"]

    conf: Dict[str, Any] = {"num_completions": 1, "max_newlines": 400,
                            "top_k": 40, "top_p": 0.9}
    conf["max_tokens"] = int(body.get("max_completion_tokens") or body.get("max_tokens") or 64000)
    if body.get("temperature") is not None:
        conf["temperature"] = conf["first_temperature"] = float(body["temperature"])
    else:
        conf["temperature"] = conf["first_temperature"] = 1.0

    model = body.get("model", "swe-2-high")
    if "/" in model:
        model = model.rsplit("/", 1)[-1]

    req = {
        "metadata": {
            "ide_name": IDE_NAME,
            "ide_version": IDE_VERSION,
            "ide_type": EXTENSION_TYPE,
            "extension_name": EXTENSION_NAME,
            "extension_version": IDE_VERSION,
            "api_key": api_key,
            "locale": LOCALE,
            "os": OS,
            "session_id": session_id,
            "request_id": request_id,
        },
        "prompt": "\n\n".join(p for p in system_parts if p),
        "chat_message_prompts": _build_conversation(body, cascade_id),
        "chat_model_uid": model,
        "request_type": REQ_CASCADE,
        "planner_mode": PLANNER_DEFAULT,
        "cascade_id": cascade_id,
        "execution_id": str(uuid.uuid4()),
        "configuration": conf,
        "tools": tools,
    }
    return encode_get_chat_message_request(req), model


# ---------------------------------------------------------------------------
# ConnectRPC transport
# ---------------------------------------------------------------------------

def chat_stream(req: bytes, api_key: str) -> Iterator[Tuple[Any, Optional[str]]]:
    """POST GetChatMessage with Connect framing; yield (ChatMessageResponse, None)
    or (None, error_string). Retries on auth failures and connection errors."""
    last_err = None
    for attempt in range(3):
        gz = gzip.compress(req)
        frame = bytes([1]) + struct.pack(">I", len(gz)) + gz
        try:
            r = requests.post(UPSTREAM + CHAT_PATH, data=frame,
                              headers={
                                  "content-type": "application/connect+proto",
                                  "connect-protocol-version": "1",
                                  "connect-content-encoding": "gzip",
                                  "connect-accept-encoding": "gzip",
                                  "authorization": "Basic " + api_key + "-" + api_key,
                                  "user-agent": "connect-go/1.18.1 (go1.26.3)",
                              }, timeout=(15, 600), stream=True)
        except requests.exceptions.RequestException as e:
            last_err = f"connection error: {e}"
            continue
        if r.status_code == 200:
            break
        if r.status_code in (401, 403) and attempt < 2:
            continue
        yield None, f"upstream HTTP {r.status_code}: {r.text[:400]}"
        return
    else:
        yield None, last_err or "upstream unreachable"
        return
    buf = b""
    for chunk in r.iter_content(65536):
        buf += chunk
        while len(buf) >= 5:
            flag = buf[0]
            ln = struct.unpack(">I", buf[1:5])[0]
            if len(buf) < 5 + ln:
                break
            payload = buf[5:5 + ln]
            buf = buf[5 + ln:]
            if flag & 2:
                trailer = gzip.decompress(payload) if flag & 1 else payload
                try:
                    err = json.loads(trailer).get("error") or {}
                except Exception:
                    err = {}
                if err.get("message"):
                    yield None, f"{err.get('code', 'error')}: {err['message']}"
                continue
            raw = gzip.decompress(payload) if flag & 1 else payload
            yield parse_chat_message_response(raw), None