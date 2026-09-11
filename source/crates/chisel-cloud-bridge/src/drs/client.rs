//! DRS API client (blueprints, sandbox sessions, builds, secrets).
//!
//! Reconstruction skeleton.

use anyhow::Result;

/// Client for the DRS (Declarative Repo Setup) API.
///
/// TODO(reconstruction): the original `chisel-cloud-bridge/src/drs/client.rs`
/// implements the DRS endpoints used by `devin cloud drs` (whoami, sandbox
/// create/run, blueprint list/create/write, build start/logs/wait, secret
/// create).
#[derive(Debug, Clone, Default)]
pub struct DrsClient;

impl DrsClient {
    pub fn new(_api_key: Option<String>) -> Self {
        Self
    }

    pub async fn whoami(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }

    pub async fn create_sandbox(&self, _repo: &str) -> Result<String> {
        Ok(String::new())
    }

    pub async fn list_blueprints(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }

    pub async fn start_build(&self, _repo: &str, _blueprint: &str) -> Result<String> {
        Ok(String::new())
    }

    pub async fn build_logs(&self, _build_id: &str) -> Result<String> {
        Ok(String::new())
    }

    pub async fn create_secret(&self, _name: &str, _value: &str) -> Result<()> {
        Ok(())
    }
}