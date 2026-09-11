//! `devin list` (`devin ls`) — list sessions in the current directory.
//!
//! Reconstruction skeleton: session listing requires the local session DB
//! (chisel-agent::session_db), which is not yet reconstructed.

use anyhow::Result;

use crate::cli::{ListArgs, ListFormat};

/// List sessions in the current directory.
///
/// TODO(reconstruction): the original `chisel/src/list.rs` reads the local
/// session DB and renders an interactive picker (default), JSON, or CSV. Only
/// the format dispatch is reconstructed here.
pub fn run(args: ListArgs) -> Result<()> {
    match args.format {
        ListFormat::Interactive => {
            println!(
                "Session listing requires the local session DB \
                 (chisel-agent::session_db); it is part of the reconstruction."
            );
        }
        ListFormat::Json => {
            println!("[]");
        }
        ListFormat::Csv => {
            println!("id,name,created_at");
        }
    }
    Ok(())
}