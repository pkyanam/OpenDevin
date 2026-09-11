//! Attribution — attribution setting handling.
//!
//! Reconstruction skeleton.

/// Attribution preference for agent output.
///
/// TODO(reconstruction): the original `chisel-agent/src/attribution/mod.rs`
/// handles the `attribution` config key (whether tool output attribution is
/// shown to the model).
#[derive(Debug, Clone, Default)]
pub struct Attribution {
    pub enabled: bool,
}

pub fn resolve(_config: &serde_json::Value) -> Attribution {
    Attribution::default()
}