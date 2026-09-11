//! Team-settings model types.
//!
//! Reconstruction skeleton.

/// Model-level team settings.
///
/// TODO(reconstruction): the original `chisel-core/src/team_settings/models.rs`
/// defines the typed org settings schema (per-model defaults, allowed models,
/// feature gates).
#[derive(Debug, Clone, Default)]
pub struct TeamSettingsModels {
    pub default_model: Option<String>,
    pub allowed_models: Vec<String>,
}