//! Unleash feature-flag sampling (https://unleash.codeium.com).
//!
//! Reconstruction skeleton.

/// Query a feature flag from the Unleash service.
///
/// TODO(reconstruction): the original pulls
/// `https://unleash.codeium.com/api/unleash_definitions.bin` and evaluates
/// flags locally (unleash-yggdrasil 0.21.2, per reports/recon.md), with
/// deterministic per-user sampling.
pub fn is_enabled(_flag: &str, _user_id: Option<&str>) -> bool {
    false
}