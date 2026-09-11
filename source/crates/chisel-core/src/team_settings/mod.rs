//! Team settings (enterprise configuration delivered by the org).
//!
//! Reconstruction skeleton.

pub mod models;

/// Team-wide settings fetched from the Devin API.
///
/// TODO(reconstruction): the original `chisel-core/src/team_settings/mod.rs`
/// caches org-level settings (default model, permissions, disabled tools)
/// delivered by the enterprise endpoint.
#[derive(Debug, Clone, Default)]
pub struct TeamSettings {
    pub org_id: Option<String>,
    pub settings: models::TeamSettingsModels,
}