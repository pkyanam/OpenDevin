//! Project configuration (`.devin/config.json`).
//!
//! Reconstruction skeleton. Field names follow the recovered config schema in
//! reports/recon.md; all fields default sensibly so partial config files parse.

use serde::{Deserialize, Serialize};

/// Agent-related settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentConfig {
    /// The default model for new sessions.
    pub model: Option<String>,
    /// Per-family model preferences.
    pub preferred_family_models: Option<serde_json::Value>,
    /// Show conversation history when continuing a session.
    pub show_history_on_continue: Option<bool>,
    /// Model mixture configuration.
    pub model_mixture: Option<serde_json::Value>,
    /// Context-compaction threshold in tokens.
    pub compaction_threshold_tokens: Option<u64>,
}

/// Permission rules (tool allow/deny/ask).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PermissionConfig {
    pub allow: Vec<String>,
    pub deny: Vec<String>,
    pub ask: Vec<String>,
}

/// Proxy settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProxyConfig {
    pub mode: Option<String>,
    pub url: Option<String>,
    pub no_proxy: Option<String>,
}

/// Sandbox settings (research preview).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SandboxConfig {
    pub allowed_domains: Vec<String>,
    pub denied_domains: Vec<String>,
    pub network_mode: Option<String>,
}

/// Where to read config from (other tools).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ReadConfigFrom {
    pub cursor: Option<bool>,
    pub claude: Option<bool>,
    pub zed: Option<bool>,
    pub copilot: Option<bool>,
    pub opencode: Option<bool>,
    pub cognition: Option<bool>,
    pub windsurf: Option<bool>,
    pub standard: Option<bool>,
    pub system: Option<bool>,
}

/// Project-level configuration (`.devin/config.json`).
///
/// TODO(reconstruction): the original `user-config/src/project.rs` adds
/// load/save/discovery logic (walking up to the git root) and a typed
/// `Hooks`/`McpServers` model; only the schema shell is reconstructed here.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectConfig {
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
    pub exec_shell: Option<String>,
    pub setup_complete: Option<bool>,
    pub startup_messages_remaining: Option<u64>,
    pub codex_tools: Option<bool>,
    pub dispatched: Option<bool>,
}

impl ProjectConfig {
    /// Load the project config by walking up from `dir` to the git root.
    ///
    /// TODO(reconstruction): discover `.devin/config.json` (or
    /// `./devin/config.json` variants) and merge with defaults.
    pub fn load(_dir: &std::path::Path) -> anyhow::Result<Self> {
        Ok(Self::default())
    }

    pub fn save(&self, _dir: &std::path::Path) -> anyhow::Result<()> {
        Ok(())
    }
}