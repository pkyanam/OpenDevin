# Devin CLI — Live-Captured Wire Protocol (v3000.10.21)

Captured 2026-09-11 by pointing the real `devin -p "say hi"` at a local
capture-relay via `WINDSURF_API_SERVER_URL=http://127.0.0.1:8899`, which
forwarded to `https://server.codeium.com`. Raw capture: `analysis/capture/capture.log`.

## Transport

- Host: `server.codeium.com` (the `api_server_url` from `~/.local/share/devin/credentials.toml`;
  `devin_api_url` (`https://api.devin.ai`) is the REST/session API base).
- Protocol: **ConnectRPC** (connectrpc-0.4.2), paths `/exa.<pkg>.<Service>/<Method>`.
- Headers: `content-type: application/proto` (auth) / `application/connect+proto` (chat),
  `connect-protocol-version: 1`.
- **Auth header observed**: `authorization: Basic devin-session-token$<JWT>-devin-session-token$<JWT>`
  where `<JWT>` = HS256 session JWT with payload `{"session_id":"windsurf-session-<hex>"}`.
  The API key ALSO rides inside the request protobuf (`Metadata.api_key`).
- Connect framing: `[1 flags byte][4-byte big-endian payload length][payload]` repeated.
  flags `0x01` = gzip, `0x02` = end-of-stream.

## RPCs observed in a single `devin -p "say hi"` run

```
POST /exa.seat_management_pb.SeatManagementService/GetUserStatus
POST /exa.seat_management_pb.SeatManagementService/GetCliTeamSettings
POST /exa.api_server_pb.ApiServerService/GetCliModelConfigs
POST /exa.api_server_pb.ApiServerService/GetAccountManagedPlugins
POST /exa.api_server_pb.ApiServerService/GetChatMessage        (×2: title gen + main)
POST /exa.product_analytics_pb.ProductAnalyticsService/BatchRecordAnalyticsEvents
```

## GetChatMessageRequest (recovered from 55,356-byte live request)

```
f1  Metadata { f1 "devin-cli", f2 "3000.10.21", f3 "devin-session-token$<JWT>", f4 "en",
               f5 "darwin", f7 "3000.10.21", f12 "chisel", f28 "chisel", f31 identity_digest }
f2  system_prompt  (string; "You are Devin, an interactive command line agent from Cognition...", 18,433 bytes)
f3  repeated ChatMessagePrompt { f1 message_id(uuid), f2 source(1=user), f3 prompt(text) }
f7  request_type = 5 (CASCADE)
f8  CompletionConfiguration { f1 num_completions=1, f2 max_tokens=128000, f3 400, f5 1.0, f7 40, f8 0.9 }
f10 repeated Tool { f1 name, f2 description, f3 input_schema(JSON string) }
f15 { f1 request_id(uuid), f3 4, f4 14 }
f16 cascade_id (uuid string)
f20 planner_mode = 1
f21 chat_model_uid (string, e.g. "swe-1-6-slow")
```

Title-generation request uses model `swe-1-6-fast`; main request `swe-1-6-slow`.

## GetChatMessageResponse (streamed; 37 Connect frames for one turn)

```
f1  message_id / bot_id  (uuid string)
f2  { f1 timestamp, f2 seq }            (CortexStepMetadata)
f7  { f6 varint, f8 { "x-request-id": ... }, f9 str }   (per-chunk metadata)
f9  delta_text                          (string; token deltas — the model text)
f12 fixed64                             (float, per-token probability/score)
f17 request_id (uuid)
final frames:
  f3 + f4 (full message / completion),
  f5 (stop reason),
  f28 (final stats/usage),
  f15 (end-of-stream marker)
```

## Reference implementations (community, verified compatible)

- Python: `devinx` gist (future3OOO) — OpenAI shim, protobuf descriptors from `jeopi-catalog` npm.
- TypeScript: `rsvedant/opencode-windsurf-auth` — `127.0.0.1:42100/v1` OpenAI-compatible proxy.
- Node: `dwgx/WindsurfAPI` — OpenAI/Anthropic/Gemini gateway with `DEVIN_CONNECT=1`.
- Rust: `oxicode-ai` `devin.rs` — full prost schema (docs.rs).
- Full research: `research/devin-windsurf-backend-research.md`.

## Key env vars for the bridge/tooling

- `WINDSURF_API_SERVER_URL` — overrides the ConnectRPC base (used for capture).
- `DEVIN_API_URL` — overrides the REST API base.
- `CHISEL_ACP_WIRE_LOG` / `CHISEL_PURE_ACP_STDERR` — ACP wire logging.
- Credentials: `~/.local/share/devin/credentials.toml`
  (`windsurf_api_key`, `api_server_url`, `devin_webapp_host`, `devin_api_url`).
- API keys: `devin-session-token$<JWT>`, `sk-ws-01-*`, `cog_*`, legacy UUID.