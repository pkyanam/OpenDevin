//! `devin doctor` — diagnose the local Devin configuration.
//!
//! Reconstruction skeleton: the original runs a battery of checks (binary
//! integrity, config validity, git state, network reachability, sandbox
//! prerequisites) and prints a table, or JSON when `--json` is passed.

use anyhow::Result;

/// Run the doctor checks.
///
/// The argument is deliberately generic: `chisel/src/bin/devin/main.rs`
/// dispatches `chisel::cli::DoctorArgs` here, but `chisel-commands` cannot
/// depend on `chisel` (that would create a dependency cycle, since `chisel`
/// already depends on `chisel-commands`). A generic parameter avoids the cycle
/// while keeping the entrypoint call site unchanged.
///
/// TODO(reconstruction): implement the original checks (see
/// `chisel-commands/src/doctor.rs` in the original workspace) and honor the
/// `json` flag from the args.
pub fn run<T>(_args: T) -> Result<()> {
    println!(
        "doctor: configuration diagnostics are part of the reconstruction; \
         the original runs integrity/config/git/network checks here."
    );
    Ok(())
}