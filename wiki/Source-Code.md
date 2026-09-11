# Source Code — the reconstructed Devin CLI workspace

> Full source: [`source/crates/**`](../source/crates) (124 `.rs` files, 44 crates).
> This is a **reconstruction** produced by static analysis + live protocol
> capture of the shipped binary — it mirrors the original workspace layout and
> CLI surface, with module stubs annotated `TODO(reconstruction)` awaiting
> logic-level decompilation.

## The binary it came from

- `devin 3000.10.21` (git `611c1cba`) — Mach-O arm64, **stripped** (no symbols/DWARF)
- 166 MB; 96 MB of ARM64 code (`__text`); 174,480 functions; 83,967 strings
- Rust stable (rustc `88d9e12…`, 2026-08-18)
- The binary embeds its own cargo source paths → the complete workspace module map was recovered (2270 source paths)

## Workspace layout (mirrors the original)

```
chisel                 the "devin" binary crate (src/bin/devin/main.rs)
  cli.rs               full clap CLI tree (reconstructed from --help + man pages)
  auth, app_state, list, rm, ssh_cmd, session_manager, trusted_workspace,
  version_update_cli, setup
chisel-agent           ACP server, session db, wiki, megaplan, revert, model defaults…
chisel-commands        cloud/drs, doctor, models, rules, skills, worker, connect
chisel-core            prompts, translator, flags, mcp event types, team settings
chisel-api             devin_api client, auth credentials+PKCE, telemetry, git info
chisel-ui              terminal UI (config editor, model selector, markdown, …)
chisel-server          acp, review/summarizer agents
chisel-mcp / chisel-acp-client / chisel-acp-relay / chisel-cloud-bridge / chisel-updater
affogato               agent core (control loop, effects, subagent, cog, stall watch)
agent-ext              skills, rules, hooks, plugins, compactor, smart permission
toolbox / toolbox-core 40+ agent tools (exec, edit, read, grep, webfetch, mcp, …)
local-agent / local-tools-client / config-importers / user-config / version / utils
sandbox-runtime (macOS seatbelt) / browser-preview (blitz HTML) / scrollback (TUI)
plugin / plugin-host / inference / connect-rpc / session-events / export / hands
cache / message-forest / web-framework-markdown / barista / fx / host-api
registrytoolbox(+core) / readline / vtparse
```

## Status of the reconstruction

- `cargo build -p chisel` — **compiles clean** (0 warnings); the rebuilt `devin`
  binary runs (`--version`, `--help`, `auth status`, `models list`, …).
- Decompiled artifacts from the original binary are in
  [`source/decompiled/`](../source/decompiled) and
  `source/functions.tsv` (174,480 functions) / `source/strings.tsv` (83,967 strings).
- The wire protocol (the hard part) is fully recovered — see [Protocol](Protocol).

## Third-party stack (from embedded cargo paths)

tokio 1.52.3, serde 1.0.228, serde_json 1.0.149, schemars 1.2.1, reqwest 0.13.2,
sentry 0.47.0, rustls 0.23.37, rmcp 3.1.0, tree-sitter 0.26.9, wezterm-*,
blitz-dom 0.3.0-beta.1 (Servo stylo 0.19), taffy, zip 7.2.0, zlib-rs 0.6.2,
zune-*, tiff/png/jpeg/rav1e, unleash-yggdrasil, similar, time, uuid,
agent-client-protocol 1.0.0, …