//! Session store: persist conversations locally (JSON, like the Devin CLI's
//! per-directory sessions).

use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created: i64,
    pub updated: i64,
    pub messages: Vec<Value>,
}

pub fn session_file() -> PathBuf {
    auth::data_dir().unwrap_or_else(|_| PathBuf::from(".")).join("sessions.json")
}

pub fn load_all() -> Vec<Session> {
    let path = session_file();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_all(sessions: &[Session]) -> Result<()> {
    let path = session_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(sessions)?)?;
    Ok(())
}

pub fn find_by_id(id: &str) -> Option<Session> {
    load_all().into_iter().find(|s| s.id.starts_with(id))
}

pub fn most_recent() -> Option<Session> {
    let mut all = load_all();
    all.sort_by(|a, b| b.updated.cmp(&a.updated));
    all.into_iter().next()
}

/// Save a conversation (creating a new session, or updating an existing one).
pub fn upsert(id: Option<String>, messages: Vec<Value>) -> Result<String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let mut all = load_all();
    let existing = id
        .as_ref()
        .and_then(|i| all.iter().position(|s| s.id.starts_with(i)));
    let title = messages
        .iter()
        .find(|m| m["role"] == "user")
        .and_then(|m| m["content"].as_str())
        .map(|c| c.chars().take(60).collect())
        .unwrap_or_else(|| "untitled".to_string());
    let (sid, _) = match existing {
        Some(idx) => {
            let id = all[idx].id.clone();
            all[idx].title = title;
            all[idx].updated = now;
            all[idx].messages = messages;
            (id, idx)
        }
        None => {
            let id = format!("ses_{:016x}", rand::random::<u64>());
            all.push(Session {
                id: id.clone(),
                title,
                created: now,
                updated: now,
                messages,
            });
            (id, all.len() - 1)
        }
    };
    save_all(&all)?;
    Ok(sid)
}