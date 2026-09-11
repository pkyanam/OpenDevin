//! `devin update` / `devin uninstall` — CLI self-management.
//!
//! Reconstruction skeleton.

use anyhow::Result;

use crate::cli::{UninstallArgs, UpdateArgs};

/// `devin update` — check for updates and optionally install them.
///
/// TODO(reconstruction): the original `chisel/src/version_update_cli.rs`
/// consults `version::self_manage::bundle::update` (manifest at
/// `static.devin.ai/cli/current/manifest.json`) and swaps the bundle. Nothing
/// is downloaded here.
pub fn run(_args: UpdateArgs) -> Result<()> {
    println!("devin {} (reconstructed)", version::VERSION);
    println!(
        "Self-update (manifest-based bundle swap) is part of the \
         reconstruction; no update was performed."
    );
    Ok(())
}

/// `devin uninstall` — remove the CLI and (optionally) user data.
///
/// TODO(reconstruction): the original delegates to
/// `version::self_manage::bundle::uninstall` (+ `common::uninstall` for
/// `--clean`). Nothing is removed here.
pub fn run_uninstall(_args: UninstallArgs) -> Result<()> {
    println!(
        "Uninstall is part of the reconstruction; nothing was removed. \
         (The original removes ~/.local/share/devin/cli/ and, with --clean, \
         config + user data.)"
    );
    Ok(())
}