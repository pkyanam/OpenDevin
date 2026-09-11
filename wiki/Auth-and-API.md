# Auth & API — Devin endpoints, tokens, and account mechanics

## Credential store

`~/.local/share/devin/credentials.toml`:
```toml
windsurf_api_key = "devin-session-token$<JWT>"   # OAuth-derived session token
api_server_url  = "https://server.codeium.com"   # ConnectRPC base
devin_webapp_host = "app.devin.ai"
devin_api_url   = "https://api.devin.ai"         # REST/session API base
```

## Auth flow

1. `devin auth login` → OAuth 2.0 **PKCE** at
   `https://app.devin.ai/devin/account/login` (marker `cli_pkce_marker=1`),
   browser loopback redirect → token stored as `windsurf_api_key`.
2. Every API call: `Metadata.api_key` (protobuf) + `Authorization: Basic <key>-<key>`.
3. Env overrides: `DEVIN_API_KEY`, `WINDSURF_API_KEY`, `WINDSURF_API_SERVER_URL`
   (ConnectRPC base), `DEVIN_API_URL` (REST base), `DEVIN_MODEL`, `DEVIN_PERMISSION_MODE`.

## API surface

**ConnectRPC (server.codeium.com)** — the model/chat path:
```
/exa.api_server_pb.ApiServerService/GetChatMessage       (streaming chat)
/exa.api_server_pb.ApiServerService/GetCliModelConfigs   (model registry)
/exa.api_server_pb.ApiServerService/AssignModel          (model assignment; assignment_jwt)
/exa.api_server_pb.ApiServerService/GetAccountManagedPlugins …
/exa.seat_management_pb.SeatManagementService/GetUserStatus / GetCliTeamSettings
/exa.auth_pb.AuthService/GetUserJwt                      (short-lived JWT; optional)
/exa.product_analytics_pb.ProductAnalyticsService/BatchRecordAnalyticsEvents
/exa.attribution_pb.AttributionService/Attribution
```

**REST (api.devin.ai)** — session management (documented at docs.devin.ai):
```
/v3/self
/v3/organizations/{org}/sessions          (create/list/get/archive)
/v3/organizations/{org}/sessions/{id}/messages   (list + send)
/v3beta1/organizations/…
/ssh/authorize, /ssh-info                 (SSH gateway for `devin ssh`/`forward`)
/local-tools/connect, /local-tools/pairing-status
```

**Other hosts**: `static.devin.ai` (update manifests + worker binaries),
`codeium-i5.sentry.io` (error telemetry), `unleash.codeium.com` (feature flags).

## Token formats

`devin-session-token$<JWT>` (current CLI), `sk-ws-01-<b64>` (Windsurf
self-serve), `cog_<id>` (Cognition team), legacy UUID. `apk_` / `apk_user_`
(legacy REST keys).

## Rate limits (observed)

Free plans gate `GetChatMessage` with a sliding-window message limit:
`resource_exhausted: Reached overall message rate limit for your free plan`.
`swe-2-high` is the dependable free default; other models are
entitlement-dependent (`permission_denied` / `failed_precondition`).