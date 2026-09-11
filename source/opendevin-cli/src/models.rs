//! Model catalog: live fetch from the Devin registry + bundled fallback.

use anyhow::Result;
use serde_json::json;

use crate::protocol::Client;

pub const DEFAULT_MODEL: &str = "swe-2-high";

/// Fetch the live catalog from the backend, with a bundled fallback.
pub async fn list(client: &Client) -> Result<Vec<(String, String)>> {
    match client.fetch_model_registry().await {
        Ok(models) if !models.is_empty() => Ok(models),
        _ => Ok(bundled_fallback()),
    }
}

pub fn bundled_fallback() -> Vec<(String, String)> {
    vec![
        ("SWE-2 Max".into(), "swe-2-max".into()),
        ("SWE-2 High".into(), "swe-2-high".into()),
        ("SWE-2 Medium".into(), "swe-2-medium".into()),
        ("Claude Opus 5 Medium".into(), "claude-opus-5-medium".into()),
        ("GPT-6 Astra Medium".into(), "gpt-6-astra-medium".into()),
        ("Gemini 3.8 Flash Medium".into(), "gemini-3-8-flash-medium".into()),
        ("Grok 4.6 Medium".into(), "grok-4-6-medium".into()),
    ]
}

pub fn models_json(models: &[(String, String)]) -> serde_json::Value {
    json!({
        "object": "list",
        "data": models.iter().map(|(_, uid)| json!({
            "id": uid, "object": "model", "created": 0, "owned_by": "devin"
        })).collect::<Vec<_>>()
    })
}