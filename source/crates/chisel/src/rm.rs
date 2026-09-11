//! `devin rm` — delete a session by id or name.
//!
//! Reconstruction skeleton.

use anyhow::Result;

use crate::cli::RmArgs;

/// Delete a session by id or name.
///
/// TODO(reconstruction): the original `chisel/src/rm.rs` resolves the target
/// (id, name, or unambiguous id prefix) against the local session DB and
/// confirms before deleting (unless `--force`). The session DB is not yet
/// reconstructed, so nothing is deleted here.
pub fn run(_args: RmArgs) -> Result<()> {
    println!(
        "`devin rm` requires the local session DB (chisel-agent::session_db); \
         deletion is part of the reconstruction. Nothing was removed."
    );
    Ok(())
}