# Devin CLI — Reverse Engineering Recon Report

**Target**: `devin` CLI (Cognition AI / Windsurf) — version **3000.10.21** (git `611c1cba`)
**Binary**: `/Users/preetham/.local/share/devin/cli/_versions/3000.10.21/bin/devin`
**Hash (SHA-256)**: `e7a86b3d4c8b198e1cbf0cab974b80d1a811f38251ceed28e28c5507ba3949b2`
**Size**: 166,420,944 bytes (~159 MiB), Mach-O 64-bit arm64, **stripped** (no DWARF, no symbol table)

## Binary anatomy

| Section | Size (hex) | Notes |
|---|---|---|
| `__TEXT.__text` | 0x5c01790 (96 MB) | ARM64 code |
| `__TEXT.__const` | 0x28fddd8 (43 MB) | embedded data: prompts, protocol schemas, JS |
| `__TEXT.__cstring` | 0xa4049 | strings |
| `__TEXT.__eh_frame` | 0xafb344 | unwind info |
| `__DATA` | ~0x3M | globals |

- Built with **Rust stable 2026-08-18** (rustc commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, "stable-next" merge, i.e. the August 2026 stable release).
- Frameworks: AppKit, CoreVideo, CoreData, CoreImage, CoreGraphics, CloudKit, QuartzCore, Foundation, CoreText, IOKit, CFNetwork, Security, SystemConfiguration, libiconv — consistent with a native terminal app + WebKit (`wry`) integration.
- One embedded gzip blob; 5 `<!DOCTYPE html>` markers (small wry bootstrap HTML).
- 355,012 printable strings extracted → `analysis/strings/all_strings.txt` (offset + string).

## Source structure (recovered from embedded cargo-registry paths)

The binary embeds the full path map of its own Cargo workspace (`~/.cargo/registry/src/pkg.cognition.build-<hash>/<crate>/...`). The CLI workspace is:

```
chisel            -> src/bin/devin/main.rs  (the "devin" binary entrypoint)
  src/lib.rs, app_state, analytics, auth/{mod,enterprise_policy}, auto_setup,
  coding_agent_detector, forward, forward_cmd, internal_cmd/{mod,self_manage},
  list, log_maintenance, migrations/*, plugin/mod, repl/*, repl_mode, rm,
  session_manager, setup, setup_git, shell/commands, ssh_cmd, trusted_workspace,
  ui/mod, version_update_cli, wiki_repl, acp_repl/{backend,bootstrap,reducer,typewriter}

chisel-agent      -> ACP server + local agent glue (acp_server/*, acp_tools/*, ...)
chisel-commands   -> CLI subcommands: cloud, cloud_secrets, connect, doctor, models,
                     rules, skills, worker/{api,bootstrap,cache,mod,runner,update},
                     checks, manual_migrations
chisel-core       -> prompts, translator, flags, convert, ref_tags, mcp_event_types,
                     team_settings/models, turn_stats
chisel-api        -> devin_api, auth/{credentials,pkce}, distribution, git_info,
                     team_settings, telemetry/{manager,sentry,skill_spool,unleash_sampling},
                     bug_report
chisel-ui         -> terminal UI: config_editor, hint_bar, mini_apps, model_selector,
                     paste_burst, readbox, select, session_stats, side_chat_panel,
                     spinner, syntax_highlight, terminal_markdown, terminal_theme, tips,
                     tool_content, user_question_panel
chisel-server     -> acp, agents/{mod,review,summarizer}
chisel-mcp        -> mcp, mcp_description_cache
chisel-acp-client -> bootstrap, child, connection, handshake
chisel-acp-relay  -> connection, jsonrpc, lib, sessions, state
chisel-cloud-bridge -> drs/client, handoff/{mod,flow}, org, setup_git, share
chisel-proxy-config, chisel-updater (manifest)

affogato          -> agent core: agent/{control_loop,manager,runner,effects/*,tool_pipeline/scheduler},
                     subagent, cog, image, event_types, stall_watch, cache_keepalive, client
agent-ext         -> skills/*, rules/*, hooks/*, plugins/*, compactor/*, smart_permission/*,
                     lints/*, revise, title, looper, btw, tool_arg_leak_recovery
toolbox           -> 40+ tools: apply_patch, edit, exec/*, grep, glob_tool, read, write,
                     webfetch, web_search, todo, subagent, mcp/*, notebook_*, view_image,
                     write_plan, user_question, tool_search, ...
toolbox-core      -> registry, typed_tool, plans, types, tools/{context,format,schema}
local-agent       -> permissions, tool_registry, sandbox_mode, web_search, plugins/*
local-tools-client-> api, gateway, session, service, fingerprint
config-importers  -> importers/{claude,cursor,windsurf,zed,copilot,opencode,cognition,
                     standard,system,mod,config,hooks,mcp,plugin_mcp}, jsonc, mcp_writer,
                     repo_config, resource/{parse,traverse}
user-config       -> project, user, trusted_workspaces
version           -> lib, self_manage/{mod,bundle,common}
utils             -> json, process, string
workspace         -> lib
sandbox-runtime   -> macos, http_proxy, socks_proxy, manager, utils
browser-preview   -> csp, inject, manager, proxy, server, service
scrollback        -> alt_screen, app, clipboard, input/*, renderer, tui/*
plugin            -> manifest, discover, install, lockfile, governance, policy, store, ...
plugin-host       -> actions, bundle_fetcher, cloud, ingest, lib, list, manifest, mcp
inference         -> backend, compat, request, retry, stream
connect-rpc       -> client, stream
session-events, export, hands, cache, message-forest, web-framework-markdown,
barista (harness, event_sender, model_registry, resume, tool_ui), fx (context,driver,lib),
host-api (filesystem, network), registrytoolbox(+core), readline, vtparse
```

Third-party crates (selected, from embedded paths): tokio 1.52.3, serde 1.0.228, serde_json 1.0.149,
serde_yaml 0.9.34, schemars 1.2.1, rmcp 3.1.0, tokio-tungstenite 0.28.0, tungstenite 0.28.0,
reqwest, sentry 0.47.0, rustls-webpki 0.103.9, tree-sitter 0.26.9 + highlight, wezterm-*,
termwiz, vtparse, taffy 0.12.2, blitz-dom 0.3.0-beta.1, dioxus-native-dom 0.8.0-alpha.1,
stylo 0.19.0 (Servo), html5ever/xml5ever/selectors, similar 3.2.0, zip 7.2.0, zlib-rs 0.6.2,
zune-*, tiff/png/jpeg, unleash-yggdrasil 0.21.2, portable-pty 0.9.0, terminal-colorsaurus,
time 0.3.47, uuid 1.21.0, toml 0.9.12+spec-1.1.0, agent-client-protocol 1.0.0, icu_segmenter,
addr2line/gimli/object, hashbrown 0.17.1, sysinfo 0.38.0, security-framework 3.7.0,
sailfish 0.10.1, urlpattern 0.3.0, winnow 1.0.4, serde_with 3.21.0, smol_str, ...

## Network surface

- API base: `https://api.devin.ai` (also `app.devin.ai`, `app.beta.devin.ai`, `staging.itsdev.in`)
- Static: `https://static.devin.ai/cli/current/manifest.json` (+ enterprise + windsurf variants)
- Sentry DSN: `https://codeium-i5.sentry.io`
- Feature flags: `https://unleash.codeium.com/api/unleash_definitions.bin`
- Env vars: DEVIN_API_KEY, DEVIN_ORG_ID, WINDSURF_API_KEY, DEVIN_MODEL, DEVIN_PERMISSION_MODE,
  DEVIN_SANDBOX, DEVIN_OUTPOSTS_TOKEN, DEVIN_WORKER_CACHE_DIR, DEVIN_WORKER_STATIC_BASE_URL,
  DEVIN_OUTPOST_GATEWAY_URL, WINDSURF_API_SERVER_URL, DEVIN_REFUSAL_FALLBACK, CHISEL_ACP_WIRE_LOG
- ACP protocol over stdio / WebSocket (`wss://`), JSON-RPC with `cognition.ai/...` metadata keys

## Config schema (user + project, ~/.config/devin/config.json and .devin/config.json)

agent{model, preferred_family_models, show_history_on_continue, model_mixture, compaction_threshold_tokens},
permissions{allow,deny,ask}, theme_mode, theme_auto_detect, show_path, unicode_mode, show_hints,
include_gitignored_files, respect_gitignore, attribution, subagents_enabled, disabled_tools,
keymap, auto_update, notify, proxy{mode,url,no_proxy}, sandbox{allowed_domains,denied_domains,network_mode},
devin, mouse_capture, legacy_terminal, disable_osc, skip_workspace_trust, skip_home_directory_warnings,
pty_for_noninteractive_exec, hooks, mcpServers, read_config_from{cursor,claude,zed,copilot,...},
org_id, exec_shell, setup_complete, startup_messages_remaining, codex_tools, dispatched
(see reference/docs/docs/extensibility/configuration.mdx + reference/configuration/config-file.mdx)

## CLI surface (clap)

```
devin [OPTIONS] [PATH]... [-- <PROMPT>...] [COMMAND]
  auth login|logout|status
  mcp add|list|remove|get|enable|disable|login|logout
  models list
  doctor
  rules list|paths|show
  skills list|paths|show
  plugins install|list|remove|info|update|prune
  cloud drs blueprint {create|list|write} | build {start|logs|wait} | run | sandbox create | secret create | whoami
  desktop, list (ls), rm, ssh, forward, update, version, migrate hooks|workflows,
  sandbox setup, setup, uninstall, acp (--agent-type), help
Flags: --prompt-file --config --permission-mode --sandbox --model -p/--print --export
       -c/--continue -r/--resume --respect-workspace-trust -V/--version
```

## Tools / methodology

- Ghidra 12.1.3 headless (JAVA_HOME = Android Studio JBR 21) — analysis in `analysis/ghidra_project`,
  exports in `analysis/ghidra_export/`
- Strings: `analysis/strings/all_strings.txt` (355k entries, file offsets)
- Man pages: `reference/man/man1/*.1` (full clap docs for every subcommand)
- Help text: `reference/help/*.help.txt` (captured from the live binary)
- Docs: `reference/docs/docs/**/*.mdx` (official CLI documentation shipped with the install)