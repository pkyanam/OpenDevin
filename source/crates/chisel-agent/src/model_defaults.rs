//! Default model selection per family/alias.
//!
//! Reconstruction skeleton.

/// Resolve a fuzzy model name (family slug, alias, or partial name) to a
/// concrete model id.
///
/// TODO(reconstruction): the original `chisel-agent/src/model_defaults.rs`
/// knows the model families (opus, sonnet, codex, ...) and their current
/// default versions, and powers `--model` / `/model`.
pub fn resolve(_name: &str) -> Option<String> {
    None
}