# Reconstructed Devin CLI source

Reconstructed from `devin 3000.10.21` (git `611c1cba`) — see the wiki
(`wiki/Source-Code.md`, `wiki/Decompilation.md`, `wiki/Protocol.md`) for how
it was produced and what each crate does.

## Layout

```
crates/        the 44-crate Cargo workspace (chisel = the "devin" binary crate)
decompiled/    decompiled key functions from the original binary (Ghidra output)
functions.tsv  all 174,480 functions (address, size, name)
strings.tsv    all 83,967 strings (address, length, value)
all_strings.txt  raw string dump with file offsets
```

## Build

```bash
cd source
cargo build -p chisel          # builds the `devin` binary (reconstructed)
cargo build                    # builds the whole workspace
```

## Reconstructed `devin` binary behavior

```
devin --version      -> devin 3000.10.21
devin --help         -> full CLI tree
devin auth status    -> checks credentials.toml
devin models list    -> (requires the API client — see TODO comments)
```

Not-yet-implemented logic is marked `// TODO(reconstruction): ...` in each
module, describing the original behavior from the recovered
strings/paths/docs.

## Notice

This is a clean-room-style reconstruction written from observable behavior of
the shipped binary (help output, man pages, captured wire traffic, embedded
source-path metadata). It is not Cognition AI's proprietary source.