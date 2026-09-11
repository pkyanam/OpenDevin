//! Feature flags.
//!
//! Reconstruction skeleton.

/// A parsed feature flag.
///
/// TODO(reconstruction): the original `chisel-core/src/flags.rs` models the
/// server-delivered feature flag set (via Unleash, see
/// chisel-api::telemetry::unleash_sampling).
#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub entries: Vec<(String, bool)>,
}

impl Flags {
    pub fn is_enabled(&self, _name: &str) -> bool {
        false
    }
}