//! Deferred fields — lazily-resolved context fields.
//!
//! Reconstruction skeleton.

/// A field whose value is resolved lazily.
///
/// TODO(reconstruction): the original `chisel-agent/src/deferred_fields.rs`
/// defers expensive context (git diff, file contents) until the model actually
/// references it.
#[derive(Debug, Clone, Default)]
pub struct DeferredFields {
    pub keys: Vec<String>,
}

pub fn defer(_key: &str) {
    // TODO(reconstruction): register a lazy resolver.
}