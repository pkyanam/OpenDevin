//! Team settings fetched from the Devin API.
//!
//! Reconstruction skeleton.

/// Org/team settings for the current account.
///
/// TODO(reconstruction): the original `chisel-api/src/team_settings.rs` fetches
/// enterprise team settings from the API and exposes them to the agent
/// (chisel-agent::team_settings consumes the same data as chisel-core).
#[derive(Debug, Clone, Default)]
pub struct TeamSettings {
    pub org_id: Option<String>,
    pub default_model: Option<String>,
}

pub async fn fetch(_api_key: &str) -> anyhow::Result<TeamSettings> {
    Ok(TeamSettings::default())
}