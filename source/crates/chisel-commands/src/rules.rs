//! `devin rules` — manage agent rules (always-on context blobs).
//!
//! Reconstruction skeleton: the original implements `list`, `paths`, and
//! `show <name>` subcommands backed by the user-config / rules storage.

use anyhow::Result;

/// Manage agent rules.
///
/// TODO(reconstruction): implement the `rules list|paths|show` subcommands
/// (see `chisel-commands/src/rules.rs` in the original workspace).
pub fn run() -> Result<()> {
    println!(
        "rules: agent-rule management is part of the reconstruction; the \
         original implements `rules list|paths|show` here."
    );
    Ok(())
}