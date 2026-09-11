//! Credentials handling: read/write ~/.local/share/devin/credentials.toml
//! (same store the real Devin CLI uses).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub windsurf_api_key: String,
    pub api_server_url: String,
    #[serde(default)]
    pub devin_webapp_host: String,
    #[serde(default)]
    pub devin_api_url: String,
}

pub fn data_dir() -> Result<std::path::PathBuf> {
    // The real Devin CLI uses XDG_DATA_HOME semantics even on macOS:
    // ~/.local/share/devin
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        if !xdg.is_empty() {
            return Ok(std::path::PathBuf::from(xdg).join("devin"));
        }
    }
    Ok(dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".local/share/devin"))
}

pub fn credentials_path() -> Result<std::path::PathBuf> {
    Ok(data_dir()?.join("credentials.toml"))
}

pub fn load() -> Result<Credentials> {
    let path = credentials_path()?;
    let text = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "no credentials at {} — run `devin auth login` first or set DEVIN_API_KEY",
            path.display()
        )
    })?;
    parse(&text)
}

pub fn parse(text: &str) -> Result<Credentials> {
    let mut creds = Credentials::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let k = k.trim();
        let v = v.trim().trim_matches('"').trim_matches('\'');
        match k {
            "windsurf_api_key" => creds.windsurf_api_key = v.to_string(),
            "api_server_url" => creds.api_server_url = v.to_string(),
            "devin_webapp_host" => creds.devin_webapp_host = v.to_string(),
            "devin_api_url" => creds.devin_api_url = v.to_string(),
            _ => {}
        }
    }
    Ok(creds)
}

pub fn env_api_key() -> Option<String> {
    std::env::var("DEVIN_API_KEY").ok().or_else(|| std::env::var("WINDSURF_API_KEY").ok())
}

pub fn resolve_api_key() -> Result<String> {
    if let Some(k) = env_api_key() {
        return Ok(k);
    }
    let creds = load()?;
    if creds.windsurf_api_key.is_empty() {
        anyhow::bail!("credentials.toml has no windsurf_api_key");
    }
    Ok(creds.windsurf_api_key)
}

pub fn status() -> Result<bool> {
    Ok(credentials_path()?.exists())
}