# Hermes Agent ↔ Devin models (via OpenDevin bridge) — verified config

Tested 2026-09-11: `hermes -z "Reply with exactly: HERMES_VIA_OPENDEVIN"` → `HERMES_VIA_OPENDEVIN`.

## Why a bridge?

The Devin CLI does **not** expose an OpenAI-compatible endpoint itself. Its
model access is a proprietary ConnectRPC protocol against `server.codeium.com`
(`/exa.api_server_pb.ApiServerService/GetChatMessage`, protobuf payloads,
`devin-session-token$…` auth). Hermes (a harness) needs an OpenAI-compatible
`/v1/chat/completions` endpoint. OpenDevin (this repo's `bridge/`) translates
between the two.

## Config (add to `~/.hermes/config.yaml`)

```yaml
model:
  default: swe-2-high
  provider: opendevin
  base_url: http://127.0.0.1:8321/v1

custom_providers:
  - name: opendevin
    base_url: http://127.0.0.1:8321/v1
    api_mode: chat_completions
    models:
      - swe-2-high      # reliably callable on free plans
      - swe-1-6
      - swe-1-6-fast      # may hit per-account limits
      - swe-1-7
      - claude-opus-4-7-medium
      - gpt-5-5-high
```

> **Plan limits**: on free plans the backend enforces a message-rate limit
> ("Reached overall message rate limit for your free plan") with a sliding
> window. `swe-2-high` is the dependable default; other models are
> entitlement-dependent. The bridge surfaces the upstream error verbatim.

## Run order

```bash
devin auth login                      # once — writes credentials.toml
cd bridge && ./setup.sh               # once — venv + descriptors
.venv/bin/python -m opendevin serve   # keep running (port 8321)
hermes                               # then just use hermes normally
```

## Notes

- No API key is needed in the Hermes config; the bridge reads
  `~/.local/share/devin/credentials.toml` (or `DEVIN_API_KEY`) itself.
- The config above is currently applied to this machine
  (`~/.hermes/config.yaml`; backup: `~/.hermes/config.yaml.bak-opendevin`).
- Any model UID your Devin account is entitled to works; list yours in
  `~/.cache/devin/cli/model_configs_v5.*.bin`.