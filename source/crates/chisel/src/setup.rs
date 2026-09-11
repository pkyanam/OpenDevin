//! `devin setup` — interactive setup wizard.
//!
//! Reconstruction skeleton.

use anyhow::Result;

use crate::cli::SetupArgs;

/// Run the interactive setup wizard.
///
/// TODO(reconstruction): the original `chisel/src/setup.rs` walks the user
/// through auth (PKCE or manual token), config creation, and workspace trust.
/// Only a placeholder is reconstructed here — no network access.
pub fn run(_args: SetupArgs) -> Result<()> {
    println!(
        "The interactive setup wizard (auth + config) is part of the \
         reconstruction. Once auth is implemented, run `devin auth login`."
    );
    Ok(())
}