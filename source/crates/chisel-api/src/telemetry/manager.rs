//! Telemetry manager — global analytics singleton.
//!
//! Reconstruction skeleton.

use serde_json::Value;

/// The global telemetry manager.
///
/// TODO(reconstruction): the original `chisel-api/src/telemetry/manager.rs`
/// owns a background analytics pipeline (batched events, session_id, user
/// hash, feature-flag sampling) that powers the Devin product analytics.
#[derive(Debug, Default)]
pub struct TelemetryManager {
    pub enabled: bool,
}

impl TelemetryManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) {
        self.enabled = true;
    }

    pub fn track(&self, _event: &str, _props: Value) {
        // TODO(reconstruction): enqueue the event for batched upload.
    }
}