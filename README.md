# OpenDevin — Devin models for any OpenAI-compatible client

_Last updated: 2026-09-11_ — model list is live from your Devin account
(`GET /v1/models`).

OpenDevin is a small local bridge that exposes **Cognition's Devin models**
(current gen: SWE-2, Claude Opus 5, GPT-6 Astra, Gemini 3.8, Grok 4.6, Kimi K3,
DeepSeek V4, …) as a standard OpenAI-compatible API — so you can drive them
from **Hermes**, Claude Code, Cursor, or any tool that speaks
`POST /v1/chat/completions`.

It speaks the same wire protocol as the real `devin` CLI (ConnectRPC
`GetChatMessage` against `server.codeium.com`) — reverse-engineered and
verified against live traffic (see `docs/`).

## What you get

```
POST /v1/chat/completions   (stream + non-stream, tools, thinking, usage)
GET  /v1/models             (live catalog from your account)
GET  /healthz
GET  /                       (web chat UI)
```

## Setup (one command)

```bash
./setup.sh        # creates .venv, installs deps, fetches protobuf descriptors
```

You need a Devin CLI login already:
```bash
devin auth login          # creates ~/.local/share/devin/credentials.toml
```

## Run

```bash
.venv/bin/python -m opendevin serve --port 8321
```

Then point any OpenAI client at `http://127.0.0.1:8321/v1`:

```bash
# plain curl
curl http://127.0.0.1:8321/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{"model":"swe-2-high","messages":[{"role":"user","content":"hi"}]}'
```

Or use the built-in terminal chat:
```bash
.venv/bin/python -m opendevin chat -m swe-2-high
```

## Models (as of 2026-09-11)

Pick any model your Devin account is entitled to. `swe-2-high` is the
dependable free default; `GET /v1/models` returns your live catalog.
Current-gen UIDs include:

```
swe-2-max | swe-2-high | swe-2-medium        # SWE (current)
swe-1-7  | swe-1-7-lightning | swe-1-7-medium
claude-opus-5-*  | claude-sonnet-5-*  | claude-fable-5-*
gpt-6-astra-*    | gpt-5-6-sol-* | gpt-5-6-luna-* | gpt-5-6-terra-*
gemini-3-8-flash-* | gemini-3-7-flash-*
grok-4-6-* | kimi-k3-* | deepseek-v4-* | glm-5-3-* | inkling-*
```

## Notes

- Auth: uses the `windsurf_api_key` from `~/.local/share/devin/credentials.toml`
  (or `DEVIN_API_KEY`). No tokens are stored by OpenDevin.
- Thinking is streamed as `reasoning_content` and replayed across turns so the
  model keeps its context.
- Conversation ids are stable per conversation → the backend's prompt cache
  stays warm (big cost savings).
- Tools work: the model can emit `tool_calls`; execute them yourself and feed
  results back as `role:"tool"` messages.
- Free plans have message rate limits; paid models surface
  `permission_denied` / `resource_exhausted` verbatim — you won't accidentally
  burn quota.

See `docs/` for the full reverse-engineering notes and wire-protocol details.