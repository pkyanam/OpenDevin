# Decompilation — how the Devin CLI was reverse-engineered

## Toolchain

| Tool | Role |
|---|---|
| Ghidra 12.1.3 | Full analysis of the 96 MB code image (28 min, 30+ analyzers). Arm64 native decompiler built from source (`make ghidra_opt ARCH_TYPE="-arch arm64"`). |
| Android Studio JBR 21 | JAVA_HOME (no separate JDK install). |
| PyGhidra | Headless Python scripting (after enabling, exports + decompilation work). |
| rizin 0.9.1 | Function map (252,548 functions) — but its xref table doesn't populate on this image, so string→function mapping was done with a custom ARM64 ADRP decoder + Ghidra's reference analysis. |
| Capture relay | Pointed the real CLI at a local relay (`WINDSURF_API_SERVER_URL` override) and recorded every request/response byte-for-byte. |

## Key results

- **174,480 functions** / 83,967 strings exported (`source/functions.tsv`, `source/strings.tsv`).
- **Complete module map** — the binary embeds its cargo source paths (2270 files → 44 Cognition crates; see [Source Code](Source-Code)).
- **Full CLI surface** — captured `--help` for every subcommand + 227 man pages.
- **Live wire protocol** — decoded request/response protobuf (see [Protocol](Protocol)).

## Decompiled functions (from the stripped binary)

Located via string xrefs, decompiled with the Ghidra decompiler
(full outputs in [`source/decompiled/`](../source/decompiled)):

1. **Credentials loader** `FUN_101a46720` — builds `<data>/devin/credentials.toml`
   from the XDG data directory (matches `chisel-api/src/auth/credentials.rs`).
2. **Secret-redaction engine** `FUN_1036bd620` — the regex table that redacts
   secrets from logs/tool output: PEM private keys, `devin-session-token$…`,
   GitHub `gh[pousr]_`/`github_pat_`, `sk-…` API keys, Slack `xox*`, AWS `AKIA`,
   JWTs, `Bearer` tokens, URL credentials, base64 blobs, generic
   `secret|token|password|api_key = …` patterns.
3. **Devstral inference wrapper** `FUN_10552fb58` — 75 KB pseudocode of the
   `GetChatMessage` request builder/response parser (the function whose wire
   behavior was captured live; struct layout matches the descriptor-derived
   field numbers exactly).

## How to reproduce

```bash
# 1. Build the arm64 decompiler (release zip ships only linux/win)
cd Ghidra/Features/Decompiler/src/decompile/cpp
make ghidra_opt ARCH_TYPE="-arch arm64" OSDIR=mac_arm_64
cp ghidra_opt ../../os/mac_arm_64/decompile

# 2. Analyze
analyzeHeadless <proj> devin_cli -import devin-3000.10.21

# 3. Enable PyGhidra (once; answers "y" to install)
echo y | pyghidraRun -e "print('ok')"

# 4. Export + decompile
GHIDRA_EXPORT_DIR=$PWD pyghidraRun -H <proj> devin_cli -process devin-3000.10.21 \
  -noanalysis -scriptPath <ghidra>/Ghidra/Features/PyGhidra/ghidra_scripts \
  -postScript ExportBasics.py
```