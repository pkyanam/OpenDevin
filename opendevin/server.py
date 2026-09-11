"""OpenDevin — OpenAI-compatible server.

Serves /v1/chat/completions (stream + non-stream) and /v1/models by
translating to Cognition's ConnectRPC GetChatMessage — the same wire format
the `devin` CLI itself uses.

Usage:
    opendevin serve [--port 8321]
or:
    python -m opendevin serve
"""
from __future__ import annotations

import json
import os
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from .protocol import build_chat_request, chat_stream, load_api_key

# Model catalog: loaded from the Devin CLI's own cached registry when present
# (~/.cache/devin/cli/model_configs_v5.*.bin), else this static list.
import glob
import json

_DEFAULT_MODELS = [
    "swe-2-max", "swe-2-high", "swe-2-medium",
    "swe-1-7", "swe-1-7-medium", "swe-1-7-lightning",
    "swe-1-6", "swe-1-6-fast", "swe-1-6-slow",
    "claude-opus-4-7-medium", "gpt-5-5-high",
]


def _load_catalog():
    try:
        here = os.path.dirname(os.path.abspath(__file__))
        # catalog lives at bridge/proto (next to the package dir)
        for cand in (os.path.join(here, "proto", "model_catalog.json"),
                     os.path.join(here, "..", "proto", "model_catalog.json")):
            if os.path.exists(cand):
                with open(cand) as f:
                    entries = json.load(f)
                uids = [e["uid"] for e in entries if e.get("uid")]
                if uids:
                    return uids
    except Exception:
        pass
    return list(_DEFAULT_MODELS)


MODELS = _load_catalog()

_api_key = None
_key_lock = threading.Lock()


def _get_key():
    global _api_key
    with _key_lock:
        if _api_key is None:
            _api_key = load_api_key()
        return _api_key


def _openai_chunk(model, delta=None, finish=None, usage=None):
    ch = {"index": 0}
    if delta is not None:
        ch["delta"] = delta
    if finish:
        ch["finish_reason"] = finish
    out = {"id": "chatcmpl-opendevin", "object": "chat.completion.chunk",
           "created": int(time.time()), "model": model, "choices": [ch]}
    if usage:
        out["usage"] = usage
    return out


def _sse(obj) -> bytes:
    return f"data: {json.dumps(obj)}\n\n".encode()


def run_chat(body, wfile):
    """Translate one OpenAI chat.completions call; returns (resp, usage, error)."""
    try:
        req, model = build_chat_request(body, _get_key())
    except Exception as e:
        return None, None, f"request build: {e}"
    stream = bool(body.get("stream"))
    w = wfile if stream else None
    headers_sent = False
    text, thinking = [], []
    tool_blocks, tool_order = {}, []
    usage, stop = {}, 0
    err = None
    for msg, e in chat_stream(req, _get_key()):
        if e:
            err = e
            break
        if not headers_sent and w:
            w.write(b"HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\n"
                    b"cache-control: no-cache\r\nconnection: close\r\n\r\n")
            w.flush()
            w.write(_sse(_openai_chunk(model, delta={"role": "assistant"})))
            w.flush()
            headers_sent = True
        if msg.delta_text:
            text.append(msg.delta_text)
            if w:
                w.write(_sse(_openai_chunk(model, delta={"content": msg.delta_text})))
                w.flush()
        if msg.delta_thinking:
            thinking.append(msg.delta_thinking)
            if w:
                w.write(_sse(_openai_chunk(model, delta={"reasoning_content": msg.delta_thinking})))
                w.flush()
        for tc in msg.tool_calls:
            tid = tc.id or (tool_order[-1] if tool_order else "")
            if not tid:
                continue
            if tid not in tool_blocks:
                tool_blocks[tid] = {"name": tc.name, "json": ""}
                tool_order.append(tid)
                if w:
                    idx = tool_order.index(tid)
                    w.write(_sse(_openai_chunk(model, delta={"tool_calls": [
                        {"index": idx, "id": tid, "type": "function",
                         "function": {"name": tc.name, "arguments": ""}}]})))
                    w.flush()
            if tc.name:
                tool_blocks[tid]["name"] = tc.name
            if tc.arguments_json:
                prev = tool_blocks[tid]["json"]
                acc = tc.arguments_json if tc.arguments_json.startswith(prev) else prev + tc.arguments_json
                delta = acc[len(prev):]
                tool_blocks[tid]["json"] = acc
                if w and delta:
                    idx = tool_order.index(tid)
                    w.write(_sse(_openai_chunk(model, delta={"tool_calls": [
                        {"index": idx, "function": {"arguments": delta}}]})))
                    w.flush()
        if msg.usage is not None and msg.usage.input_tokens:
            usage = {"prompt_tokens": int(msg.usage.input_tokens),
                     "completion_tokens": int(msg.usage.output_tokens),
                     "total_tokens": int(msg.usage.input_tokens + msg.usage.output_tokens)}
            if msg.usage.cache_read_tokens:
                usage["prompt_tokens_details"] = {"cached_tokens": int(msg.usage.cache_read_tokens)}
        stop = msg.stop_reason
    if err:
        if w:
            w.write(_sse({"error": {"message": err, "type": "upstream_error"}}))
            w.write(b"data: [DONE]\n\n")
            w.flush()
        return None, None, err
    finish = "tool_calls" if tool_blocks else "stop"
    if not tool_blocks and stop == 3:
        finish = "length"
    if w:
        final = _openai_chunk(model, delta={}, finish=finish,
                              usage=usage if (body.get("stream_options") or {}).get("include_usage") else None)
        w.write(_sse(final))
        w.write(b"data: [DONE]\n\n")
        w.flush()
        return None, None, None
    msg_out = {"role": "assistant", "content": "".join(text)}
    if thinking:
        msg_out["reasoning_content"] = "".join(thinking)
    if tool_blocks:
        msg_out["tool_calls"] = [
            {"id": tid, "type": "function",
             "function": {"name": b["name"], "arguments": b["json"]}}
            for tid, b in ((tid, tool_blocks[tid]) for tid in tool_order)]
        msg_out["content"] = msg_out["content"] or None
    resp = {"id": "chatcmpl-opendevin", "object": "chat.completion",
            "created": int(time.time()), "model": model,
            "choices": [{"index": 0, "message": msg_out, "finish_reason": finish}]}
    if usage:
        resp["usage"] = usage
    return resp, usage, None


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, fmt, *args):
        pass

    def _json(self, code, obj):
        data = json.dumps(obj).encode()
        self.send_response(code)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        path = self.path.split("?")[0]
        if path in ("/", "/chat", "/index.html"):
            from .ui import page
            data = page()
            self.send_response(200)
            self.send_header("content-type", "text/html; charset=utf-8")
            self.send_header("content-length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
        elif path in ("/v1/models", "/models"):
            self._json(200, {"object": "list", "data": [
                {"id": m, "object": "model", "created": 0, "owned_by": "devin"} for m in MODELS]})
        elif path in ("/healthz", "/health"):
            self._json(200, {"ok": True})
        else:
            self._json(404, {"error": {"message": "not found", "type": "invalid_request_error"}})

    def do_POST(self):
        path = self.path.split("?")[0]
        if path not in ("/v1/chat/completions", "/chat/completions"):
            self._json(404, {"error": {"message": "not found", "type": "invalid_request_error"}})
            return
        try:
            raw = self.rfile.read(int(self.headers.get("content-length", 0) or 0))
            body = json.loads(raw)
        except Exception as e:
            self._json(400, {"error": {"message": f"bad json: {e}", "type": "invalid_request_error"}})
            return
        if body.get("stream"):
            try:
                run_chat(body, self.wfile)
            except (BrokenPipeError, ConnectionResetError):
                pass
            return
        resp, _, err = run_chat(body, None)
        if err:
            self._json(502, {"error": {"message": err, "type": "upstream_error"}})
        else:
            self._json(200, resp)


def serve(port: int = 8321, host: str = "127.0.0.1"):
    print(f"OpenDevin listening on {host}:{port} (OpenAI-compatible /v1/chat/completions)")
    ThreadingHTTPServer((host, port), Handler).serve_forever()


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description="OpenDevin OpenAI-compatible bridge")
    ap.add_argument("--port", type=int, default=int(os.environ.get("OPENDEVIN_PORT", "8321")))
    ap.add_argument("--host", default=os.environ.get("OPENDEVIN_HOST", "127.0.0.1"))
    args = ap.parse_args()
    serve(args.port, args.host)