# Usage & Ideas

## Using the bridge today

```bash
devin auth login                      # one-time
cd bridge && ./setup.sh               # one-time
.venv/bin/python -m opendevin serve   # OpenAI-compatible server on :8321
```

- Web chat: open http://127.0.0.1:8321
- Terminal chat: `.venv/bin/python -m opendevin chat -m swe-2-high`
- Any OpenAI client: point at `http://127.0.0.1:8321/v1`

### Hermes config (verified)

```yaml
model:
  default: swe-2-high
  provider: opendevin
  base_url: http://127.0.0.1:8321/v1
custom_providers:
  - name: opendevin
    base_url: http://127.0.0.1:8321/v1
    api_mode: chat_completions
```

## The standalone `opendevin` CLI (in progress)

A Rust binary (`source/crates/opendevin`) being built from this reconstruction:

**Same features as Devin CLI:**
- `auth login|status|logout`, `models list`, `doctor`, `version`, `update`, `setup`
- Interactive chat / REPL (`-p` one-shot, `-c` continue, `-r` resume)
- `list` / `rm` sessions, `mcp` (config), `rules`/`skills`/`plugins` (config mgmt)
- `acp` (Agent Client Protocol server over stdio)

**Bonus features:**
- `opendevin serve` — OpenAI-compatible server built into the CLI
- Live model catalog (`models list` = real registry; `swe-2-high` default)
- Web UI mode
- Multi-model switching with thinking replay
- Usage/credit reporting (from `usage` frames)

## Ideas / roadmap

- Full agent loop (tools: read/write/edit/exec/grep/glob/webfetch) inside the CLI
- Session store (SQLite) with `list`/`resume`/`rm`
- Config importers (Claude Code, Cursor, Windsurf) via `migrate`
- Plugin/skill system (`.devin/skills`, `/skill-name` slash commands)
- Team settings + org switching
- Workspace trust + permission modes (auto / accept-edits / smart / bypass)

## Legal / ethics

OpenDevin is a research reconstruction. It is **not** affiliated with
Cognition AI. Use it responsibly: it consumes your Devin account's quota
(free plans have rate limits), and reverse-engineering notes are provided for
interoperability research. The reconstructed source is our own code written
from observable behavior — not Cognition's proprietary source.