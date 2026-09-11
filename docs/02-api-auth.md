# Devin CLI — API Endpoints & Auth Mechanism (v3000.10.21)

Reverse-engineered from the shipped binary + the machine's own credential store.
All endpoints below were recovered from embedded strings/URL constants and the
credential file layout on this machine (values redacted).

## Credential store (this machine)

`~/.local/share/devin/credentials.toml`:
```toml
windsurf_api_key = "<redacted>"   # session/API key (OAuth-derived)
api_server_url  = "https://api.devin.ai"
devin_webapp_host = "https://app.devin.ai"
devin_api_url   = "https://api.devin.ai"
```

## Auth mechanism

1. **Login** (`devin auth login` / `devin setup`):
   - OAuth 2.0 **PKCE** flow. Marker string `cli_pkce_marker=1`, endpoint path
     `/devin/account/login` on `https://app.devin.ai`.
   - Opens browser to `https://app.devin.ai/devin/account/login?cli_pkce_marker=1&...`
     with `code_challenge` (S256) + state. Localhost redirect back to the CLI.
   - `--force-manual-token-flow` = paste a token instead (remote/SSH use).
2. **Token** → stored as `windsurf_api_key` in `credentials.toml`
   (`chisel-api/src/auth/credentials.rs`). Also accepted from env:
   - `DEVIN_API_KEY` / `WINDSURF_API_KEY`
   - `dev win-session-token`
   - `$WINDSURF_API_SERVER_URL` overrides the API base.
3. **Every API call** authenticates with `Authorization: Bearer <windsurf_api_key>`
   against `https://api.devin.ai`.
4. Logout deletes `credentials.toml` (plus `outposts_auth.<id>.json` for workers).

## API surface (api.devin.ai)

RPC (ConnectRPC / gRPC — connectrpc-0.4.2, protobuf envelopes):
```
/exa.api_server_pb.ApiServerService/AssignModel          # model assignment (returns assignment_jwt, model_uid, harness_uids)
/exa.seat_management_pb.SeatManagementService/GetUserStatus
```
REST:
```
/v3/self
/v3/organizations/{org_id}/sessions?session_ids=...
/v3/organizations/{org_id}/integrations
/v3beta1/organizations/...
/ssh/authorize            # SSH gateway approval for `devin ssh` / `devin forward`
/ssh-info
/sse | /streaming         # streaming channels
/local-tools/connect      # local-tools pairing (gateway)
/local-tools/pairing-status
```
Web (browser/oauth):
```
https://app.devin.ai/devin/account/login
https://app.devin.ai/settings/environment?tab=outposts   # outpost tokens
```
Other hosts:
```
https://static.devin.ai/cli/current/manifest.json        # self-update manifest (+ -enterprise / -windsurfcom)
https://static.devin.ai/devin-rs/remote                  # worker remote binaries
https://codeium-i5.sentry.io                              # error telemetry
https://unleash.codeium.com/api/unleash_definitions.bin  # feature flags
https://to.cognition.ai/inference                        # inference relay host
server.codeium.com / server-beta.codeium.com             # legacy codeium infra
```
Inference protocol: **proprietary**. `windsurf-api-client/src/inference_client.rs`
+ `connect-rpc` crate. Model registry fetched as `model_configs_v5.bin`.
Streaming via Connect RPC + SSE. NOT the OpenAI `/v1/chat/completions` wire format.

## The CLI can *consume* OpenAI-compatible endpoints (as a client)

`devin acp` supports an alternate backend:
```
ACP_BACKEND=openai OPENAI_API_BASE=https://openrouter.ai/api/v1 \
OPENAI_API_KEY=... USE_COMPLETIONS=true devin acp
```
→ the ACP server routes model calls to an OpenAI-compatible endpoint.
This is the CLI acting as a *client* — it does NOT expose such an endpoint itself.

## Verdict: OpenAI-compatible?

**No.** `api.devin.ai` speaks ConnectRPC/gRPC + a proprietary streaming protocol,
not OpenAI Chat Completions. There is no `/v1/chat/completions` anywhere in the
binary. The only OpenAI-compatible surface in the product is the *inbound* side
(`ACP_BACKEND=openai` client mode) which consumes, not serves.

## Hermes pluggability

Hermes (harness) supports `custom_providers` with api_modes:
`chat_completions | codex_responses | anthropic_messages | bedrock_converse | codex_app_server`.
Devin's ConnectRPC protocol matches **none** of these, and no OpenAI-compatible
endpoint is exposed by Devin. → **There is no Hermes config that can use Devin
for model access.** A working bridge would require a translation proxy
(OpenAI chat-completions ↔ Devin ConnectRPC) which does not exist today.
## Rate limits (observed)

Free plans enforce a message-rate limit on `GetChatMessage`:
`resource_exhausted: Reached overall message rate limit for your free plan.
Upgrade to Pro or Max for more generous rate limits.` — sliding-window gate.
`swe-2-high` is reliably callable on the free tier (as of 2026-09-11); most
other models return `permission_denied` / `failed_precondition`
(entitlement-dependent).
