//! PKCE OAuth browser flow for `devin auth login`.
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// A PKCE code verifier/challenge pair.
///
/// TODO(reconstruction): the original `chisel-api/src/auth/pkce.rs` implements
/// RFC 7636 PKCE (S256) for the browser login flow, spinning up a localhost
/// redirect server that captures the authorization code. No network access is
/// performed in the reconstruction.
#[derive(Debug, Clone)]
pub struct PkcePair {
    pub verifier: String,
    pub challenge: String,
}

/// Generate a fresh PKCE pair.
///
/// TODO(reconstruction): generate a random 43-128 char verifier and its S256
/// base64url challenge (the original uses `rand` + `sha2`).
pub fn generate() -> Result<PkcePair> {
    todo!("PKCE generation is part of the reconstruction")
}

/// Run the browser login flow (blocking).
///
/// TODO(reconstruction): open a browser, wait for the localhost redirect with
/// the auth code, exchange it for a token.
pub async fn run_browser_flow(_client_id: &str, _scopes: &[String]) -> Result<String> {
    todo!("browser OAuth PKCE flow is part of the reconstruction")
}