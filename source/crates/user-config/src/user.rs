//! User configuration (`~/.config/devin/config.json`).
//!
//! Reconstruction skeleton. Shares the schema recovered in reports/recon.md
//! with the project config; the user config is loaded from
//! `~/.config/devin/config.json`.

use serde::{Deserialize, Serialize};

use crate::project::{
    AgentConfig, PermissionConfig, ProxyConfig, ReadConfigFrom, SandboxConfig,
};

/// User-level configuration.
///
/// TODO(reconstruction): the original `user-config/src/user.rs` adds
/// load/save/merge with env-var overrides (`DEVIN_MODEL`, etc.) and the
/// per-tool config editor used by chisel-ui::config_editor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfig {
    pub agent: AgentConfig,
    pub permissions: PermissionConfig,
    pub theme_mode: Option<String>,
    pub theme_auto_detect: Option<bool>,
    pub show_path: Option<bool>,
    pub unicode_mode: Option<bool>,
    pub show_hints: Option<bool>,
    pub include_gitignored_files: Option<bool>,
    pub respect_gitignore: Option<bool>,
    pub attribution: Option<bool>,
    pub subagents_enabled: Option<bool>,
    pub disabled_tools: Vec<String>,
    pub keymap: Option<String>,
    pub auto_update: Option<bool>,
    pub notify: Option<bool>,
    pub proxy: ProxyConfig,
    pub sandbox: SandboxConfig,
    pub devin: Option<serde_json::Value>,
    pub mouse_capture: Option<bool>,
    pub legacy_terminal: Option<bool>,
    pub disable_osc: Option<bool>,
    pub skip_workspace_trust: Option<bool>,
    pub skip_home_directory_warnings: Option<bool>,
    pub pty_for_noninteractive_exec: Option<bool>,
    pub hooks: Option<serde_json::Value>,
    #[serde(rename = "mcpServers")]
    pub mcp_servers: Option<serde_json::Value>,
    pub read_config_from: ReadConfigFrom,
    pub org_id: Option<String>,
}

impl UserConfig {
    /// Path to the user config file.
    pub fn config_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("devin/config.json"))
    }

    /// Load the user config (defaults when the file is absent).
    ///
    /// TODO(reconstruction): parse `~/.config/devin/config.json`, falling back
    /// to legacy locations.
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self::default())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        Ok(())
    }
}