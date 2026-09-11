# OpenDevin Wiki — Home

> **Note**: GitHub's wiki repo for private repos materializes on first web-UI
> access; until then the wiki content lives in the `wiki/` directory of this
> repo and is structured exactly like a GitHub wiki (each page is a `.md` file
> you can copy into the wiki).

OpenDevin is a from-scratch reconstruction of **Cognition's Devin CLI**
(`devin 3000.10.21`, git `611c1cba`), produced by reverse-engineering the
shipped Mach-O binary on a Mac, plus a working OpenAI-compatible bridge that
lets any client (Hermes, Claude Code, Cursor) use Devin's models.

## Pages

| Page | What it covers |
|---|---|
| [Home](Home) | You are here |
| [Source Code](Source-Code) | The reconstructed Devin CLI workspace (`source/crates/**`) — 44 crates, module map |
| [OpenDevin CLI](OpenDevin-CLI) | The standalone `opendevin` binary — same features + bonus server/web UI |
| [Decompilation](Decompilation) | How the binary was analyzed: Ghidra 12.1.3, the built arm64 decompiler, PyGhidra, live wire capture; decompiled functions |
| [Protocol](Protocol) | The ConnectRPC `GetChatMessage` wire format, fully decoded from live traffic + official descriptors |
| [Auth & API](Auth-and-API) | Endpoints, OAuth PKCE login, credentials.toml, token formats |
| [Usage & Ideas](Usage-and-Ideas) | Using OpenDevin (bridge, Hermes, `swe-2-high`), and ideas for the standalone CLI |

## Quick facts

- **Model backend**: `server.codeium.com/exa.api_server_pb.ApiServerService/GetChatMessage` (ConnectRPC, protobuf, `devin-session-token$…` auth)
- **Free-tier default model**: `swe-2-high` (current SWE gen; Claude Opus 5, GPT-6 Astra, Gemini 3.8 also available per-account)
- **Live catalog**: `GET /v1/models` on the bridge returns the account's live registry (209 models, fetched 2026-09-11)
- **Working bridge**: `bridge/` — OpenAI-compatible server, terminal chat, web UI — verified end-to-end with Hermes

## Repo layout

```
bridge/      the working OpenAI-compatible bridge (+ web UI, TUI)
source/      the reconstructed Devin CLI source (Rust workspace) + decompiled functions
wiki/        this wiki (markdown pages)
docs/        reverse-engineering reports (recon, api-auth, decompilation, protocol)
proto/       official protobuf descriptors (jeopi-catalog) + live model catalog
```

*Last updated: 2026-09-11*