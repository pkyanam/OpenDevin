# Decompiled key functions (from the shipped `devin 3000.10.21`)

Real decompiler output (Ghidra 12.1.3, PyGhidra) from the stripped arm64
binary, driven by string xrefs. Full `.c` outputs live in
`analysis/decompiled/` (this repo) and `docs/decompiled/` (OpenDevin repo).

Method: every function that references a key string was located via Ghidra's
reference analysis (the binary is stripped — 174,480 functions recovered,
names like `FUN_<addr>`), then decompiled with the Ghidra decompiler.

## 1. Credentials loader — `FUN_101a46720` (chisel-api/src/auth/credentials.rs)

```c
void FUN_101a46720(long *param_1)
{
  FUN_1052c0d9c(&local_60);                          // data_dir = XDG data home
  if (local_60 == -1) {
    pvStack_40 = FUN_105b42a30("Failed to determine data directory", 0x22);
  } else {
    FUN_10464cd20(&local_48, ..., "devin", 5);       // data_dir/"devin"
    FUN_10464cd20(&local_48, ..., "credentials.toml", 0x10);
    param_1[1] = ...; *param_1 = local_48;           // path: <data>/devin/credentials.toml
  }
  *param_1 = -1; ...
}
```

→ Matches the captured behavior: credentials live at
`~/.local/share/devin/credentials.toml` (XDG data home + `devin` +
`credentials.toml`).

## 2. Secret redaction engine — `FUN_1036bd620` (the `[redacted]` tool)

This function builds the regex table the CLI uses to redact secrets from logs
and tool output (the `log.redact_upload` instrumentation). Recovered patterns:

```
-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?-----END [A-Z ]*PRIVATE KEY-----   # PEM private keys
devin-session-token\$[A-Za-z0-9._\-]+                                         # Devin session token
(?:gh[pousr]_|github_pat_)[A-Za-z0-9_]{16,}                                   # GitHub tokens
sk-(?:ant-)?[A-Za-z0-9_\-]{16,}                                               # OpenAI/Anthropic keys
xox[baprs]-[A-Za-z0-9\-]{8,}                                                  # Slack tokens
AKIA[0-9A-Z]{16}                                                              # AWS access keys
ey[A-Za-z0-9_\-]{8,}\.[A-Za-z0-9_\-]{8,}\.[A-Za-z0-9_\-]{8,}                  # JWTs
(?i)bearer\s+[A-Za-z0-9._\-]{20,}                                             # Bearer tokens
([A-Za-z][A-Za-z0-9+.\-]*://)([A-Za-z0-9._~%+\-]*):([^\s/"'`<>{}]*)@          # URL credentials
data:[a-z]+/[A-Za-z0-9.+\-]+;base64,[A-Za-z0-9+/=]+                           # base64 data blobs
(?i)\b([A-Za-z0-9_]*(?:secret|token|password|passwd|api[_-]?key|access[_-]?key|private[_-]?key|credential)[A-Za-z0-9_]*)(\s*[:=]\s*)\S{6,}
                                                                               # generic key=value secrets
```

## 3. Inference wrapper — `FUN_10552fb58` (windsurf-api-client / inference_client.rs)

75 KB of pseudocode — the `GetDevstralStream`/`GetChatMessage` request builder
and response parser (protobuf field handling, streaming, `Devstral API call
failed` error path). This is the function whose wire behavior we captured
live (see `docs/PROTOCOL.md`); the decompiled struct layout matches the
descriptor-derived field numbers exactly.

## How to reproduce

```bash
# one-time: build the arm64 decompiler, analyze, install PyGhidra
# (see reports/DECOMPILATION.md); then, for any key string:
GHIDRA_DECOMP_DIR=$PWD/analysis/decompiled pyghidraRun -H analysis/ghidra_project devin_cli \
  -process devin-3000.10.21 -noanalysis \
  -scriptPath $PWD/tools/ghidra_12.1.3_PUBLIC/Ghidra/Features/PyGhidra/ghidra_scripts \
  -postScript tools/ghidra_scripts/DecompileKeyStrings.py
```

Functions exported: `analysis/ghidra_export/functions.tsv` (174,480),
`strings.tsv` (83,967).