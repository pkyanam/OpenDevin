"""Minimal protobuf wire codec (pure Python, no protobuf dependency).

Implements just the subset of the Cognition `exa.*` schemas that OpenDevin
needs, with field numbers taken from the official descriptors shipped in the
`jeopi-catalog` npm package (see `proto/*.fdp` in this repo) and verified
against live traffic captured from `devin 3000.10.21`.

Only varint, length-delimited (string/bytes/message) and fixed64/fixed32
wire types are used by the messages we touch.
"""
from __future__ import annotations

import struct
from typing import Any, Dict, List, Optional, Tuple


def _varint(n: int) -> bytes:
    n &= 0xFFFFFFFFFFFFFFFF
    out = bytearray()
    while True:
        b = n & 0x7F
        n >>= 7
        if n:
            out.append(b | 0x80)
        else:
            out.append(b)
            return bytes(out)


def _tag(field: int, wire: int) -> bytes:
    return _varint((field << 3) | wire)


def _len(v: bytes) -> bytes:
    return _varint(len(v)) + v


def _field_str(field: int, s: str) -> bytes:
    return _tag(field, 2) + _len(s.encode("utf-8"))


def _field_bytes(field: int, b: bytes) -> bytes:
    return _tag(field, 2) + _len(b)


def _field_msg(field: int, m: bytes) -> bytes:
    return _field_bytes(field, m)


def _field_varint(field: int, n: int) -> bytes:
    return _tag(field, 0) + _varint(n)


def _field_f64(field: int, x: float) -> bytes:
    return _tag(field, 1) + struct.pack("<d", x)


def _field_f32(field: int, x: float) -> bytes:
    return _tag(field, 5) + struct.pack("<f", x)


def _field_bool(field: int, b: bool) -> bytes:
    return _field_varint(field, 1 if b else 0)


def _msg(fields: bytes) -> bytes:
    return fields


# ---------------------------------------------------------------------------
# Encode helpers for the messages we build
# ---------------------------------------------------------------------------

def encode_chat_tool_call(tc: Dict[str, str]) -> bytes:
    out = b""
    if tc.get("id"):
        out += _field_str(1, tc["id"])
    if tc.get("name"):
        out += _field_str(2, tc["name"])
    if tc.get("arguments_json"):
        out += _field_str(3, tc["arguments_json"])
    return _msg(out)


def encode_image(data: Dict[str, str]) -> bytes:
    out = b""
    if data.get("base64_data"):
        out += _field_str(1, data["base64_data"])
    if data.get("mime_type"):
        out += _field_str(2, data["mime_type"])
    return _msg(out)


def encode_chat_message_prompt(p: Dict[str, Any]) -> bytes:
    out = b""
    if p.get("message_id"):
        out += _field_str(1, p["message_id"])
    if p.get("source"):
        out += _field_varint(2, p["source"])
    if p.get("prompt"):
        out += _field_str(3, p["prompt"])
    for tc in p.get("tool_calls") or []:
        out += _field_msg(6, encode_chat_tool_call(tc))
    if p.get("tool_call_id"):
        out += _field_str(7, p["tool_call_id"])
    for img in p.get("images") or []:
        out += _field_msg(10, encode_image(img))
    if p.get("thinking"):
        out += _field_str(11, p["thinking"])
    if p.get("signature"):
        out += _field_str(12, p["signature"])
    if p.get("thinking_id"):
        out += _field_str(16, p["thinking_id"])
    if p.get("signature_type"):
        out += _field_str(18, p["signature_type"])
    if p.get("phase"):
        out += _field_str(19, p["phase"])
    return _msg(out)


def encode_completion_config(conf: Dict[str, Any]) -> bytes:
    out = b""
    if "num_completions" in conf:
        out += _field_varint(1, conf["num_completions"])
    if "max_tokens" in conf:
        out += _field_varint(2, conf["max_tokens"])
    if "max_newlines" in conf:
        out += _field_varint(3, conf["max_newlines"])
    if "temperature" in conf:
        out += _field_f64(5, conf["temperature"])
    if "top_k" in conf:
        out += _field_varint(7, conf["top_k"])
    if "top_p" in conf:
        out += _field_f64(8, conf["top_p"])
    if "first_temperature" in conf:
        out += _field_f64(9, conf["first_temperature"])
    if "fim_eot_prob_threshold" in conf:
        out += _field_f64(10, conf["fim_eot_prob_threshold"])
    return _msg(out)


def encode_tool_definition(t: Dict[str, Any]) -> bytes:
    out = b""
    if t.get("name"):
        out += _field_str(1, t["name"])
    if t.get("description"):
        out += _field_str(2, t["description"])
    if t.get("json_schema_string"):
        out += _field_str(3, t["json_schema_string"])
    if "strict" in t and t["strict"]:
        out += _field_bool(4, t["strict"])
    return _msg(out)


def encode_metadata(md: Dict[str, Any]) -> bytes:
    out = b""
    if md.get("ide_name"):
        out += _field_str(1, md["ide_name"])
    if md.get("extension_version"):
        out += _field_str(2, md["extension_version"])
    if md.get("api_key"):
        out += _field_str(3, md["api_key"])
    if md.get("locale"):
        out += _field_str(4, md["locale"])
    if md.get("os"):
        out += _field_str(5, md["os"])
    if md.get("ide_version"):
        out += _field_str(7, md["ide_version"])
    if md.get("request_id"):
        out += _field_varint(9, md["request_id"])
    if md.get("session_id"):
        out += _field_str(10, md["session_id"])
    if md.get("extension_name"):
        out += _field_str(12, md["extension_name"])
    if md.get("ide_type"):
        out += _field_str(28, md["ide_type"])
    if md.get("identity_digest"):
        out += _field_str(31, md["identity_digest"])
    return _msg(out)


def encode_get_chat_message_request(r: Dict[str, Any]) -> bytes:
    out = b""
    if r.get("metadata"):
        out += _field_msg(1, encode_metadata(r["metadata"]))
    if r.get("prompt"):
        out += _field_str(2, r["prompt"])
    for p in r.get("chat_message_prompts") or []:
        out += _field_msg(3, encode_chat_message_prompt(p))
    if r.get("request_type"):
        out += _field_varint(7, r["request_type"])
    if r.get("configuration"):
        out += _field_msg(8, encode_completion_config(r["configuration"]))
    for t in r.get("tools") or []:
        out += _field_msg(10, encode_tool_definition(t))
    if r.get("cascade_id"):
        out += _field_str(16, r["cascade_id"])
    if r.get("planner_mode"):
        out += _field_varint(20, r["planner_mode"])
    if r.get("chat_model_uid"):
        out += _field_str(21, r["chat_model_uid"])
    if r.get("execution_id"):
        out += _field_str(22, r["execution_id"])
    return _msg(out)


# ---------------------------------------------------------------------------
# Decode helpers for the response stream
# ---------------------------------------------------------------------------

def _read_varint(b: bytes, i: int) -> Tuple[int, int]:
    r = 0
    s = 0
    while True:
        x = b[i]
        i += 1
        r |= (x & 0x7F) << s
        if not x & 0x80:
            return r, i
        s += 7


def iter_fields(b: bytes):
    """Yield (field_number, wire_type, raw_value) for a protobuf message."""
    i = 0
    n = len(b)
    while i < n:
        tag, i = _read_varint(b, i)
        fno, wt = tag >> 3, tag & 7
        if wt == 0:
            v, i = _read_varint(b, i)
            yield fno, wt, v
        elif wt == 1:
            yield fno, wt, b[i:i + 8]
            i += 8
        elif wt == 2:
            ln, i = _read_varint(b, i)
            yield fno, wt, b[i:i + ln]
            i += ln
        elif wt == 5:
            yield fno, wt, b[i:i + 4]
            i += 4
        else:
            break


class ToolCall:
    __slots__ = ("id", "name", "arguments_json")

    def __init__(self):
        self.id = ""
        self.name = ""
        self.arguments_json = ""


class Usage:
    __slots__ = ("input_tokens", "output_tokens", "cache_read_tokens",
                 "cache_write_tokens", "model_uid", "api_provider")

    def __init__(self):
        self.input_tokens = 0
        self.output_tokens = 0
        self.cache_read_tokens = 0
        self.cache_write_tokens = 0
        self.model_uid = ""
        self.api_provider = 0


class ChatMessageResponse:
    __slots__ = ("message_id", "delta_text", "delta_tokens", "stop_reason",
                 "delta_thinking", "delta_signature", "delta_signature_type",
                 "thinking_id", "phase", "latency", "request_id",
                 "usage", "tool_calls")

    def __init__(self):
        self.message_id = ""
        self.delta_text = ""
        self.delta_tokens = 0
        self.stop_reason = 0
        self.delta_thinking = ""
        self.delta_signature = ""
        self.delta_signature_type = ""
        self.thinking_id = ""
        self.phase = ""
        self.latency = 0.0
        self.request_id = ""
        self.usage = None  # Usage
        self.tool_calls = []  # list[ToolCall]


def parse_tool_call(b: bytes) -> ToolCall:
    tc = ToolCall()
    for fno, wt, v in iter_fields(b):
        if fno == 1 and wt == 2:
            tc.id = v.decode("utf-8", "replace")
        elif fno == 2 and wt == 2:
            tc.name = v.decode("utf-8", "replace")
        elif fno == 3 and wt == 2:
            tc.arguments_json = v.decode("utf-8", "replace")
    return tc


def parse_usage(b: bytes) -> Usage:
    u = Usage()
    for fno, wt, v in iter_fields(b):
        if fno == 2 and wt == 0:
            u.input_tokens = v
        elif fno == 3 and wt == 0:
            u.output_tokens = v
        elif fno == 4 and wt == 0:
            u.cache_write_tokens = v
        elif fno == 5 and wt == 0:
            u.cache_read_tokens = v
        elif fno == 9 and wt == 2:
            u.model_uid = v.decode("utf-8", "replace")
        elif fno == 6 and wt == 0:
            u.api_provider = v
    return u


def parse_chat_message_response(b: bytes) -> ChatMessageResponse:
    r = ChatMessageResponse()
    for fno, wt, v in iter_fields(b):
        if fno == 1 and wt == 2:
            r.message_id = v.decode("utf-8", "replace")
        elif fno == 3 and wt == 2:
            r.delta_text = v.decode("utf-8", "replace")
        elif fno == 4 and wt == 0:
            r.delta_tokens = v
        elif fno == 5 and wt == 0:
            r.stop_reason = v
        elif fno == 6 and wt == 2:
            r.tool_calls.append(parse_tool_call(v))
        elif fno == 7 and wt == 2:
            r.usage = parse_usage(v)
        elif fno == 9 and wt == 2:
            r.delta_thinking = v.decode("utf-8", "replace")
        elif fno == 10 and wt == 2:
            r.delta_signature = v.decode("utf-8", "replace")
        elif fno == 12 and wt == 1:
            r.latency = struct.unpack("<d", v)[0]
        elif fno == 16 and wt == 2:
            r.thinking_id = v.decode("utf-8", "replace")
        elif fno == 17 and wt == 2:
            r.request_id = v.decode("utf-8", "replace")
        elif fno == 21 and wt == 2:
            r.delta_signature_type = v.decode("utf-8", "replace")
        elif fno == 25 and wt == 2:
            r.phase = v.decode("utf-8", "replace")
    return r