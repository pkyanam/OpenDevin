//! JSON helpers.
//!
//! Reconstruction skeleton.

use anyhow::Result;
use serde::de::DeserializeOwned;

/// Parse a JSON string into a typed value.
///
/// TODO(reconstruction): the original `utils/src/json.rs` adds pretty-printing
/// and streaming helpers used by session export and config serialization.
pub fn parse<T: DeserializeOwned>(text: &str) -> Result<T> {
    Ok(serde_json::from_str(text)?)
}

/// Serialize a value to a pretty-printed JSON string.
pub fn to_pretty<T: serde::Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}