# OpenDevin CLI — the standalone `opendevin` binary

A from-scratch **Rust** reimplementation of the Devin CLI, invoked as
`opendevin`. Same surface as `devin`, plus bonus features. Implements the
verified wire protocol directly (no `devin` binary involved).

Source: `source/opendevin-cli/` (Rust, workspace-compatible; `cargo build -p opendevin`).

## Commands (Devin-CLI compatible)

```
opendevin auth login|status|logout
opendevin models list                # live registry from the backend
opendevin doctor                     # config/credential diagnostics
opendevin version | update
opendevin chat [prompt]              # interactive REPL (streaming + thinking)
opendevin -p "prompt" -m swe-2-high  # one-shot print mode (same as `devin -p`)
opendevin list | rm                  # local session store
```

## Bonus features (not in the original CLI)

```
opendevin serve --port 8321          # OpenAI-compatible server (stream + non-stream)
                                     #   + web chat UI at http://127.0.0.1:8321
                                     #   + /v1/models (live catalog)
                                     #   → point Hermes/Cursor/Claude Code at it
opendevin models list --format json  # machine-readable live catalog
```

## How it works

- `wire.rs` — minimal protobuf codec for the `exa.*` schemas (field numbers
  from official descriptors + live capture).
- `protocol.rs` — ConnectRPC client (`GetChatMessage`, `GetCliModelConfigs`),
  CLI-shaped metadata (`devin-cli`/`chisel`), `Basic <key>-<key>` auth,
  gzip Connect framing, stable cascade/message ids for prompt-cache warmth.
- `auth.rs` — reads `~/.local/share/devin/credentials.toml` (or `DEVIN_API_KEY`).
- `chat.rs` — streaming conversation with thinking + tool-call support.
- `server.rs` — embedded OpenAI-compatible server (axum) + web UI.
- `models.rs` — live model registry with a bundled fallback.

## Verify

```
opendevin auth status     # Logged in (…credentials.toml)
opendevin models list     # 200+ live models (SWE-2, Claude Opus 5, GPT-6 Astra…)
opendevin -p "hi"         # streams thinking, prints the answer (default swe-2-high)
```

*Last updated: 2026-09-11*