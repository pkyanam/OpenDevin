//! Devin API client.
//!
//! Minimal reconstruction: the `Client` struct (base URL + API key) mirrors
//! the original; every request method is stubbed with `todo!()`.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Base URL of the Devin API (recovered from the binary's strings).
pub const DEVIN_API_BASE_URL: &str = "https://api.devin.ai";

/// A minimal Devin API client.
///
/// TODO(reconstruction): the original `chisel-api/src/devin_api.rs` implements
/// the full REST surface (auth, models, sessions, DRS, outposts, enterprise
/// team settings) over `reqwest`. Only the shell is reconstructed here.
#[derive(Clone, Debug)]
pub struct Client {
    pub base_url: String,
    pub api_key: Option<String>,
    pub http: reqwest::Client,
}

/// A session descriptor (subset of the original schema).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub status: String,
}

impl Client {
    /// Create a client for the production API.
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            base_url: DEVIN_API_BASE_URL.to_string(),
            api_key,
            http: reqwest::Client::new(),
        }
    }

    /// Create a client from the environment (`DEVIN_API_KEY`, `DEVIN_ORG_ID`).
    ///
    /// TODO(reconstruction): also honor `DEVIN_ORG_ID` and the staging base
    /// URLs recovered in reports/recon.md.
    pub fn from_env() -> Self {
        Self::new(std::env::var("DEVIN_API_KEY").ok())
    }

    /// GET /models — list models available to the account.
    ///
    /// TODO(reconstruction): implement the real request; used by
    /// `chisel-commands::models::run`.
    pub async fn list_models(&self) -> Result<serde_json::Value> {
        todo!("list_models request is part of the reconstruction")
    }

    /// GET /me — current authenticated user.
    ///
    /// TODO(reconstruction): implement the real request.
    pub async fn get_me(&self) -> Result<serde_json::Value> {
        todo!("get_me request is part of the reconstruction")
    }

    /// POST /sessions — create a cloud Devin session.
    ///
    /// TODO(reconstruction): implement the real request.
    pub async fn create_session(&self, _prompt: &str) -> Result<Session> {
        todo!("create_session request is part of the reconstruction")
    }

    /// GET /sessions/{id} — poll a cloud Devin session.
    ///
    /// TODO(reconstruction): implement the real request.
    pub async fn get_session(&self, _id: &str) -> Result<Session> {
        todo!("get_session request is part of the reconstruction")
    }
}