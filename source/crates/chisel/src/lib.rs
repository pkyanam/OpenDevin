//! Devin CLI — the `devin` binary crate.
//!
//! This crate was reconstructed from the shipped `devin` 3000.10.21 (611c1cba)
//! Mach-O arm64 binary (see `reports/recon.md`). Module layout mirrors the
//! embedded cargo source map of the original workspace.

pub mod app_state;
pub mod auth;
pub mod cli;
pub mod list;
pub mod rm;
pub mod session_manager;
pub mod setup;
pub mod ssh_cmd;
pub mod trusted_workspace;
pub mod version_update_cli;

/// The CLI version string, mirroring the original binary's
/// `3000.10.21 (611c1cba)`.
///
/// NOTE(reconstruction): added so `chisel::version()` resolves for the
/// entrypoint in `src/bin/devin/main.rs` (the `devin version` subcommand).
pub fn version() -> &'static str {
    concat!(env!("CARGO_PKG_VERSION"), " (reconstructed)")
}