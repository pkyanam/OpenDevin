//! `devin models list` — list models available to the account.
//!
//! Reconstruction skeleton: the original implementation queries the Devin API
//! (see chisel-api::devin_api::Client::list_models) and groups results by
//! model family.

use anyhow::Result;

/// List available models.
///
/// TODO(reconstruction): query the Devin API via `chisel-api::devin_api::Client`
/// and print a table of model families (the original takes a `--format`
/// argument via `chisel::cli::ModelsListArgs`; the entrypoint in
/// `chisel/src/bin/devin/main.rs` dispatches here with no args).
pub fn run() -> Result<()> {
    println!(
        "models list is part of the reconstruction: it requires the Devin API \
         client (chisel-api::devin_api::Client::list_models), which is not yet \
         implemented."
    );
    Ok(())
}