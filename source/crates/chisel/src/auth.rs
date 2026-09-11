//! `devin auth` — login / logout / status.
//!
//! Reconstruction skeleton: no real authentication is attempted. The original
//! uses the PKCE browser flow (chisel-api::auth::pkce) and stores credentials
//! in `~/.local/share/devin/credentials.toml`.

use std::path::PathBuf;

use anyhow::Result;

use crate::cli::{AuthArgs, AuthCommand};

/// The credentials file used by the original CLI.
fn credentials_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".local/share/devin/credentials.toml"))
}

/// Dispatch `devin auth login|logout|status`.
pub fn run(args: AuthArgs) -> Result<()> {
    match args.command {
        AuthCommand::Login(login_args) => {
            println!("Logging in to Devin…");
            if login_args.force_manual_token_flow {
                println!("  (--force-manual-token-flow requested: manual token entry)");
            }
            // TODO(reconstruction): run the browser OAuth PKCE flow via
            // chisel-api::auth::pkce and store the token in credentials.toml.
            // Deliberately NOT attempted here — no network access in the
            // reconstruction.
            println!(
                "The browser OAuth PKCE flow is part of the reconstruction; \
                 no credentials were created. Run `devin auth login` again once \
                 auth is implemented."
            );
        }
        AuthCommand::Logout => {
            // TODO(reconstruction): remove the token from credentials.toml.
            println!(
                "Logout: credential removal is part of the reconstruction; \
                 no credentials were touched."
            );
        }
        AuthCommand::Status => {
            match credentials_path() {
                Some(path) if path.exists() => {
                    println!("Logged in (credentials found at {})", path.display());
                }
                _ => {
                    println!(
                        "Not logged in (no credentials at \
                         ~/.local/share/devin/credentials.toml)"
                    );
                }
            }
        }
    }
    Ok(())
}