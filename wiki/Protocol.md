# Protocol — the Devin CLI wire format (recovered & verified)

Captured live from `devin 3000.10.21` (via a local relay overriding
`WINDSURF_API_SERVER_URL`), cross-checked against the official descriptors
shipped in the `jeopi-catalog` npm package (`proto/*.fdp` here).

## Transport

- Host: `server.codeium.com` (the `api_server_url` in `credentials.toml`).
- **ConnectRPC** (`connectrpc-0.4.2`), paths `/exa.<pkg>.<Service>/<Method>`.
- Headers: `content-type: application/connect+proto` (chat) /
  `application/proto` (auth), `connect-protocol-version: 1`,
  `authorization: Basic <api_key>-<api_key>`.
- Connect framing: `[flags:1][len:4 BE][payload]`, flags `0x01`=gzip,
  `0x02`=end-of-stream (JSON error trailer).

## RPCs observed in one `devin -p "hi"` run

```
GetUserStatus  GetCliTeamSettings  GetCliModelConfigs  GetAccountManagedPlugins
GetChatMessage (×2: title + main)  BatchRecordAnalyticsEvents
```

## GetChatMessageRequest (field numbers = official descriptors)

```
1  Metadata   {1 ide_name, 2 extension_version, 3 api_key, 4 locale, 5 os,
               7 ide_version, 9 request_id, 10 session_id, 12 extension_name,
               28 ide_type, 31 identity_digest}
2  prompt                      (system prompt string)
3  chat_message_prompts        {1 message_id, 2 source(1 user,2 assistant,4 tool),
                                3 prompt, 6 tool_calls, 7 tool_call_id,
                                10 images, 11 thinking, 12 signature}
7  request_type = 5            (CASCADE)
8  configuration               {1 num_completions, 2 max_tokens, 3 max_newlines,
                                5 temperature, 7 top_k, 8 top_p}
10 tools (repeated)            {1 name, 2 description, 3 json_schema_string, 4 strict}
16 cascade_id                  (stable per conversation → prompt cache)
20 planner_mode = 1
21 chat_model_uid              ("swe-2-high", "claude-opus-5-medium", …)
22 execution_id
```

## GetChatMessageResponse (streamed frames)

```
1 message_id  2 timestamp  3 delta_text  4 delta_tokens  5 stop_reason(3=length)
6 delta_tool_calls {1 id, 2 name, 3 arguments_json}
7 usage {2 input, 3 output, 4 cache_write, 5 cache_read, 9 model_uid}
9 delta_thinking  10 delta_signature  12 latency  16 thinking_id
17 request_id  21 delta_signature_type  25 phase  28 response_dimension_groups
```

Models stream **thinking first** (`delta_thinking`), then the answer
(`delta_text`), then `stop_reason`, `usage` (with `cache_read_tokens`), a
stats frame, and an `{}` trailer.

## Auth

- `devin auth login`: OAuth PKCE at `app.devin.ai/devin/account/login`
  (`cli_pkce_marker=1`), browser loopback redirect.
- Credentials: `~/.local/share/devin/credentials.toml`
  (`windsurf_api_key = devin-session-token$<JWT>`, `api_server_url`,
  `devin_webapp_host`, `devin_api_url`).
- Every RPC: `Metadata.api_key` + `Basic <key>-<key>` header.
- No `user_jwt` for chat (sending one from `GetUserJwt` gets rejected for
  `devin-session-token$` keys — verified empirically).

## Why the bridge works

- CLI-shaped metadata (`devin-cli`, `chisel`, versions `3000.10.21`) is
  required — Windsurf/IDE metadata gets `permission_denied`.
- Stable cascade/message ids keep the backend prompt cache warm.
- Thinking/signature replay across turns preserves reasoning.

## Model registry

`GetCliModelConfigs` (unary `application/proto`) returns the raw
`ModelConfigs` protobuf: repeated entries `{1 display_name, 22 uid}` — e.g.
`SWE-2 High → swe-2-high`, `Claude Opus 5 Medium → claude-opus-5-medium`.
OpenDevin caches it as `proto/model_catalog.json` and serves it at
`GET /v1/models`. (Also cached locally by the CLI as
`~/.cache/devin/cli/model_configs_v5.*.bin`.)