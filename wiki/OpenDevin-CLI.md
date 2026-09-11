# OpenDevin CLI — the standalone `opendevin` binary

A from-scratch **Rust** reimplementation of the Devin CLI, invoked as
`opendevin`. Same experience as `devin` — including the interactive TUI and the
full local agent loop with tools — plus bonus features. Implements the verified
wire protocol directly (no `devin` binary involved).

Source: `source/opendevin-cli/` (Rust; `cargo build -p opendevin --release`).

## Commands (Devin-CLI compatible)

```
opendevin                     interactive TUI (alternate screen, streaming,
                              thinking, tool calls, status bar)
opendevin auth login|status|logout
opendevin models list         live registry from the backend
opendevin doctor | version | update
opendevin chat [prompt]       TUI REPL
opendevin -p "prompt" -m swe-2-high   one-shot agent (tools included)
opendevin -c | -r <id>        continue / resume a saved conversation
opendevin list | rm           session store (JSON, per-machine)
```

## Bonus features (not in the original CLI)

```
opendevin serve --port 8321   OpenAI-compatible server (stream + non-stream)
                              + web chat UI at http://127.0.0.1:8321
                              + /v1/models live catalog
                              → point Hermes/Cursor/Claude Code at it
opendevin models list --format json
```

## Interactive TUI (Devin-CLI-class)

- Alternate screen, message pane with scroll, status bar, input box.
- **Streaming**: every model frame renders the instant it arrives (incremental
  Connect-frame parsing — no response buffering).
- **Thinking** shown dimmed; toggle with `/thinking`.
- **Tool calls** shown live (`⚙ name args`) with results (`↩ …`).
- Slash commands handled locally (never sent to the model):

```
/model <uid>    switch model at runtime (e.g. /model swe-2-medium)
/fusion [uid]   Fusion model — list fusion-capable models / pick one
/mode <auto|accept-edits|bypass>   permission mode
/models         list available models
/usage          last turn token usage
/thinking       toggle thinking
/clear          clear conversation
/quit           exit
```

## Agent loop + tools

`opendevin -p` and the TUI run the full agent loop: stream a turn → execute
tool calls → feed results back → repeat until the model finishes.

Tools (Devin toolset): `read`, `write`, `edit`, `exec`, `grep`, `glob`,
`webfetch`, `todo_write` (+ `kill_shell`/`get_output` plumbing). Permission
modes match Devin: `auto` (read-only auto-approved, write/exec prompt),
`accept-edits` (edits auto-approved), `bypass` (all).

## Fusion

- `/fusion` lists fusion-capable models; `/fusion <uid>` or `/model <uid>`
  selects one (the backend routes lead/sidekick).
- Harness/env overrides are honored: `DEVIN_HARNESS`, `DEVIN_HARNESS_LEAD_ONLY`,
  `DEVIN_HARNESS_SIDEKICK_ONLY`, `DEVIN_SIDEKICK_PREFER_EXEC_TOOL`.

## Sessions

Conversations are saved to `~/.local/share/devin/sessions.json` on TUI exit;
`-c` resumes the most recent, `-r <id>` resumes a specific one, `list`/`rm`
manage them.

## Architecture

```
wire.rs       minimal protobuf codec (exa.* schemas from descriptors + capture)
protocol.rs   ConnectRPC client — GetChatMessage / GetCliModelConfigs,
              CLI-shaped metadata, Basic auth, gzip framing, stable cascade IDs
auth.rs       ~/.local/share/devin/credentials.toml (or DEVIN_API_KEY)
chat.rs       streaming turn (incremental frames, tool-call merge-by-id)
agent.rs      agent loop + permission modes
tools.rs      tool definitions + executors
tui.rs        ratatui terminal UI
server.rs     embedded OpenAI-compatible server + web UI
models.rs     live model registry
sessions.rs   conversation persistence
```

*Last updated: 2026-09-11*