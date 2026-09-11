# OpenDevin — protocol & bridge internals

How the real `devin` CLI talks to Cognition's model backend, and how OpenDevin
reproduces it. Everything here was recovered from the shipped binary
(`devin 3000.10.21`, commit `611c1cba`) and verified against live captured
traffic (a capture-relay logged every request the real CLI made).

## The Devin CLI wire protocol (verified)

- **Host**: `server.codeium.com` — this is the `api_server_url` value stored in
  `~/.local/share/devin/credentials.toml` (NOT `api.devin.ai`, which is the
  REST/session API base).
- **Protocol**: ConnectRPC (connectrpc-0.4.2) over HTTPS. Paths are
  `/exa.<package>.<Service>/<Method>`.
- **Headers**: `content-type: application/connect+proto` (chat) /
  `application/proto` (auth), `connect-protocol-version: 1`, and (for the CLI)
  `authorization: Basic <api_key>-<api_key>` where
  `api_key = devin-session-token$<JWT>`.
- **Connect framing**: each payload is `[1 flag byte][4-byte big-endian
  length][payload]`. flag `0x01` = gzip, `0x02` = end-of-stream (trailer is a
  JSON error envelope).
- **Auth**: the key rides in `Metadata.api_key` (protobuf field 3) plus the
  Basic header. A short-lived `user_jwt` (from `GetUserJwt`) is used by some
  clients — the CLI itself does **not** send one for chat.

### RPCs observed in one `devin -p "hi"` run

```
/exa.seat_management_pb.SeatManagementService/GetUserStatus
/exa.seat_management_pb.SeatManagementService/GetCliTeamSettings
/exa.api_server_pb.ApiServerService/GetCliModelConfigs
/exa.api_server_pb.ApiServerService/GetAccountManagedPlugins
/exa.api_server_pb.ApiServerService/GetChatMessage      (×2: title + main)
/exa.product_analytics_pb.ProductAnalyticsService/BatchRecordAnalyticsEvents
```

### GetChatMessageRequest (field numbers = official descriptors)

```
1  Metadata            {1 ide_name, 2 extension_version, 3 api_key, 4 locale,
                         5 os, 7 ide_version, 9 request_id, 10 session_id,
                         12 extension_name, 28 ide_type, 31 identity_digest}
2  prompt              (system prompt string)
3  chat_message_prompts (repeated)  {1 message_id, 2 source(1 user,2 assistant,4 tool),
                                      3 prompt, 6 tool_calls, 7 tool_call_id,
                                      10 images, 11 thinking, 12 signature}
7  request_type = 5    (CASCADE)
8  configuration       {1 num_completions, 2 max_tokens, 3 max_newlines,
                         5 temperature(f64), 7 top_k, 8 top_p(f64), 9 first_temperature}
10 tools (repeated)    {1 name, 2 description, 3 json_schema_string, 4 strict}
16 cascade_id          (stable per conversation → prompt cache)
20 planner_mode = 1
21 chat_model_uid      (string: "swe-2-high", "claude-opus-4-7-medium", …)
22 execution_id
```

### GetChatMessageResponse (streamed frames)

```
1  message_id, 2 timestamp, 3 delta_text, 4 delta_tokens,
5  stop_reason (3 = max tokens), 6 delta_tool_calls {1 id,2 name,3 arguments_json},
7  usage {2 input_tokens, 3 output_tokens, 4 cache_write_tokens, 5 cache_read_tokens,
          9 model_uid, 6 api_provider}, 9 delta_thinking, 10 delta_signature,
12 latency, 16 thinking_id, 17 request_id, 21 delta_signature_type, 25 phase,
28 response_dimension_groups
```

The model streams **thinking first** (`delta_thinking`), then the final answer
(`delta_text`), then `stop_reason`, then `usage` (with `cache_read_tokens`),
then a stats frame and an `{}` trailer.

## OpenDevin's implementation

`bridge/opendevin/`:
- `wire.py` — a dependency-free protobuf codec for exactly the messages above
  (the official `google.protobuf` upb runtime segfaults on this descriptor
  set, so we encode by hand — same approach the WindsurfAPI project takes).
- `protocol.py` — request construction + ConnectRPC transport (gzip, framing,
  trailers, retry).
- `server.py` — OpenAI-compatible HTTP server (`/v1/chat/completions`,
  `/v1/models`, `/healthz`).
- `tui.py` — a minimal terminal chat.

Key details that make it work (all verified by experiment):
1. **CLI-shaped metadata** (`ide_name=devin-cli`, `extension_name=chisel`,
   versions `3000.10.21`) — using Windsurf/IDE metadata gets rejected with
   `permission_denied`.
2. **No `user_jwt`** — minting one via `GetUserJwt` and attaching it also gets
   rejected for `devin-session-token$` keys.
3. **Basic header** `Basic <key>-<key>`.
4. **Stable cascade/message ids** — conversation-stable UUIDs so the backend
   prompt cache hits across turns.
5. **Thinking replay** — `delta_thinking`/`delta_signature`/`thinking_id` are
   stored per message and re-sent on later turns (the upstream strips them from
   replayed history, so without this the model re-derives its reasoning).

## Open-source ecosystem (for cross-reference)

- `devinx` (Python gist) — first working OpenAI shim; descriptor-based.
- `rsvedant/opencode-windsurf-auth` (TS) — OAuth + `127.0.0.1:42100/v1`.
- `dwgx/WindsurfAPI` (Node, 3k★) — OpenAI/Anthropic/Gemini gateway,
  `DEVIN_CONNECT=1` direct-to-cloud.
- `oxicode-ai` `devin.rs` (Rust) — full prost schema.
- Official protobuf descriptors: the `jeopi-catalog` npm package ships
  Cognition's generated code (`proto/*.fdp` in this repo).

See also: `reports/recon.md` (binary RE), `reports/api-auth.md` (endpoints &
auth), `analysis/devin_protocol.md` (captured wire log analysis).