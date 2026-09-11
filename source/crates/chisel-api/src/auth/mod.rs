//! Authentication: credential storage and PKCE OAuth.
//!
//! Reconstruction skeleton.

pub mod credentials;
pub mod pkce;

/// Where credentials are persisted on disk.
///
/// TODO(reconstruction): the original stores credentials in
/// `~/.local/share/devin/credentials.toml`.
pub fn credentials_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".local/share/devin/credentials.toml"))
}