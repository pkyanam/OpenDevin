# Devin CLI — Decompilation & Reconstruction Report

Status: **in progress** (analysis complete; module-by-module reconstruction underway).

## Target

| | |
|---|---|
| Binary | `devin 3000.10.21` (git `611c1cba`) — Cognition AI / Windsurf "Devin for Terminal" |
| Path | `~/.local/share/devin/cli/_versions/3000.10.21/bin/devin` |
| Size | 166,420,944 bytes; Mach-O 64-bit **arm64**; **stripped** (no DWARF, no symbol table; 421 dyld stubs only) |
| SHA-256 | `e7a86b3d4c8b198e1cbf0cab974b80d1a811f38251ceed28e28c5507ba3949b2` |
| Toolchain | Rust stable, rustc commit `88d9e12ae178fab0fb5cc050a94da85685d449ea` (2026-08-18 stable-next) |
| Frameworks | AppKit, CoreVideo, CoreData, CoreImage, CoreGraphics, CloudKit, QuartzCore, CFNetwork, Security, … (native macOS app + WebKit/`wry`) |

## Toolchain

- **Ghidra 12.1.3** (NSA release), run with Android Studio's JBR 21 as JAVA_HOME.
  - Built the **native arm64 decompiler** from source (the release zip only
    ships linux/win binaries): `make ghidra_opt ARCH_TYPE="-arch arm64" OSDIR=mac_arm_64`
    → deployed to `Ghidra/Features/Decompiler/os/mac_arm_64/decompile`.
  - Headless analysis completed in 1674 s (~28 min) across the 96 MB `__text`:
    Decompiler Switch Analysis 684 s, Stack 327 s, Basic Constant Reference
    247 s, Disassemble 155 s, Data Reference 28 s, + 30 more analyzers.
  - Rust language module is x86_64-only in this release → generic AArch64
    analysis used (function recovery unaffected for a stripped binary).
  - Project: `analysis/ghidra_project/devin_cli`. Python post-scripts need
    PyGhidra (not bundled); Java scripts hit an OSGi path issue → exports are
    being produced with **rizin** instead.
- **rizin 0.9.1** — full `aaa` analysis + `aflj`/`axtj` JSON exports running on
  the same binary (see `analysis/rizin/` when it finishes).

## What the binary is (module map from embedded cargo paths)

The binary embeds its own build's source paths (`~/.cargo/registry/src/...`).
2270 source files recovered → the CLI workspace is a set of Cognition crates:

```
chisel (the "devin" binary: src/bin/devin/main.rs)   chisel-agent (ACP server, session db)
chisel-commands (cloud, doctor, models, rules, skills, worker)   chisel-core (prompts, translator)
chisel-api (devin_api client, auth credentials+pkce, telemetry)  chisel-ui (terminal UI)
chisel-server (acp, review/summarizer agents)        chisel-mcp
chisel-acp-client / chisel-acp-relay / chisel-cloud-bridge (drs, handoff) / chisel-updater
affogato (agent core: control_loop, effects, subagent, cog)   agent-ext (skills, rules, hooks, plugins)
toolbox / toolbox-core (40+ agent tools)             local-agent / local-tools-client
config-importers (claude, cursor, windsurf, zed, opencode, cognition, standard)
user-config (config schema)   version (self-manage/update)   utils   workspace
sandbox-runtime (macOS seatbelt)   browser-preview (blitz HTML renderer)
scrollback (terminal scrollback/TUI)   plugin / plugin-host   inference
connect-rpc   session-events   export   hands   cache   message-forest
web-framework-markdown   barista   fx   host-api   registrytoolbox(+core)   readline   vtparse
```

Third-party (from cargo paths): tokio 1.52.3, serde 1.0.228, serde_json
1.0.149, schemars 1.2.1, reqwest 0.13.2, sentry 0.47.0, rustls 0.23.37,
tokio-tungstenite 0.28.0, rmcp 3.1.0, tree-sitter 0.26.9, wezterm-*,
termwiz, vtparse, taffy 0.12.2, blitz-dom 0.3.0-beta.1 (Servo stylo 0.19),
zip 7.2.0, zlib-rs 0.6.2, zune-*, tiff/png/jpeg/rav1e, unleash-yggdrasil,
similar 3.2.0, time 0.3.47, uuid 1.21.0, agent-client-protocol 1.0.0, ...

## Key reverse-engineered artifacts

1. **Full CLI surface** — captured `devin --help` for every subcommand
   (`reference/help/*.help.txt`) + all 227 man pages (`reference/man/`) →
   complete clap definition reconstructed in `src/crates/chisel/src/cli.rs`.
2. **Config schema** — full user/project config keys from docs + strings
   (`reports/recon.md` §config).
3. **API endpoints + auth** — `reports/api-auth.md`: OAuth PKCE login at
   `app.devin.ai/devin/account/login`, `credentials.toml`
   (`windsurf_api_key`, `api_server_url=https://server.codeium.com`, …),
   `Authorization: Basic <key>-<key>`, ConnectRPC at
   `/exa.api_server_pb.ApiServerService/GetChatMessage` etc.
4. **Inference protocol** — captured live (capture-relay + `WINDSURF_API_SERVER_URL`
   override): request/response protobuf fully decoded (see
   `bridge/docs/PROTOCOL.md`), cross-verified against the official descriptors
   from the `jeopi-catalog` npm package (`tools/reference-bridges/*.fdp`,
   copied to `bridge/proto/`).
5. **Model registry** — `~/.cache/devin/cli/model_configs_v5.<digest>.bin` is
   JSON `{version, identity_digest, fetched_at_secs, payload(base64 protobuf)}`;
   decoded → full per-account model catalog (display name, temperature, top_p,
   max_tokens, alias UIDs like `swe-1-6-slow`, `gpt-5-6-sol`…).
6. **System prompt** — the agent's full system prompt (18,433 bytes) recovered
   from the captured live request (stored in `analysis/capture/capture.log`).

## Reconstruction status

- ✅ `cli.rs` — complete clap CLI tree (all subcommands, flags, aliases).
- ✅ `bin/devin/main.rs` — entrypoint + dispatch (reconstructed).
- 🔄 Crate skeleton — `src/crates/*` compile-ready stubs being generated
  (subagent running; workspace members per the module map above).
- 🔄 Ghidra/rizin function+xref exports — analysis done, exports generating.
- ⏳ Module-by-module logic reconstruction — next phase; each module annotated
  with `TODO(reconstruction)` describing the original behavior from the
  recovered strings/paths/docs.
- ✅ **OpenDevin bridge** (functional + verified) — `bridge/` in this repo;
  translates OpenAI chat.completions ↔ Devin ConnectRPC. Verified with Hermes
  (`hermes -z …` through the bridge).

## Where to look

| What | Where |
|---|---|
| Recon notes (crates, config, endpoints, protocol) | `reports/recon.md`, `reports/api-auth.md` |
| Protocol internals + bridge docs | `bridge/docs/PROTOCOL.md`, `bridge/docs/HERMES.md` |
| Live wire captures | `analysis/capture/capture.log` (+ decode scripts) |
| Strings (355k, offset+text) | `analysis/strings/all_strings.txt` |
| Man pages / help | `reference/man/`, `reference/help/` |
| Official docs shipped with the CLI | `reference/docs/docs/**/*.mdx` |
| Ghidra project (analysis done) | `analysis/ghidra_project/devin_cli` |
| Reconstructed source | `src/crates/**` |
| Bridge | `bridge/` |