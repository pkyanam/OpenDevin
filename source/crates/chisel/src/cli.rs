//! Command-line interface definition for `devin`.
//!
//! Reconstructed from `devin --help` output and the shipped man pages
//! (`reference/man/man1/*.1`) of the original binary v3000.10.21.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// A fast and minimal agent that lives both in your terminal and in the cloud.
#[derive(Parser, Debug)]
#[command(
    name = "devin",
    version,
    about = "A fast and minimal agent that lives both in your terminal and in the cloud.",
    disable_help_subcommand = false,
    subcommand_negates_reqs = true,
    args_conflicts_with_subcommands = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Open Devin Desktop on the given path(s)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Your prompt (use `--` before the prompt)
    ///
    /// Starts an interactive session. Use -p/--print for non-interactive mode.
    #[arg(value_name = "PROMPT", last = true, allow_hyphen_values = true)]
    pub prompt: Vec<String>,

    /// Load the initial prompt from a file
    #[arg(long = "prompt-file", value_name = "FILE")]
    pub prompt_file: Option<PathBuf>,

    /// Configuration file path
    ///
    /// Override the default user config file (~/.config/devin/config.json).
    #[arg(long = "config", value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Permission mode
    ///
    /// Modes: "auto" auto-approves read-only tools, "accept-edits" also
    /// auto-approves workspace edits, "smart" additionally auto-runs actions a
    /// fast model judges safe, "dangerous" auto-approves all tools.
    #[arg(
        long = "permission-mode",
        value_name = "PERMISSION_MODE",
        default_value = "auto",
        env = "DEVIN_PERMISSION_MODE"
    )]
    pub permission_mode: String,

    /// [Research Preview] Sandbox exec-tool processes (macOS seatbelt / Linux bwrap+seccomp)
    ///
    /// When passed, commands can write only within the workspace and granted
    /// `Write(...)` scopes, and can read everything except paths hidden by
    /// `Deny(Read(...))` rules.
    #[arg(long = "sandbox", env = "DEVIN_SANDBOX")]
    pub sandbox: bool,

    /// Model to use (e.g. "claude-sonnet-4", "claude-opus-4.6", "opus", "codex")
    ///
    /// With -c/--continue or -r/--resume, switches the resumed conversation to
    /// this model; without it the session's saved model is kept.
    #[arg(long = "model", value_name = "MODEL", env = "DEVIN_MODEL")]
    pub model: Option<String>,

    /// Print response and exit
    ///
    /// Runs in non-interactive mode: processes the prompt and exits.
    /// Optionally accepts an inline prompt: -p "fix the bug"
    #[arg(short = 'p', long = "print", value_name = "PROMPT", num_args = 0..=1)]
    pub print: Option<Option<String>>,

    /// Export conversation to a file
    ///
    /// Exports after each turn. Uses a default path if no path is provided.
    #[arg(long = "export", value_name = "PATH", num_args = 0..=1)]
    pub export: Option<Option<PathBuf>>,

    /// Continue the most recent conversation
    #[arg(short = 'c', long = "continue")]
    pub continue_session: bool,

    /// Resume a conversation
    ///
    /// Provide a session ID to resume a specific session, or omit to select
    /// interactively.
    #[arg(short = 'r', long = "resume", value_name = "SESSION_ID", num_args = 0..=1)]
    pub resume: Option<Option<String>>,

    /// Respect workspace trust settings
    ///
    /// Defaults to true in every mode. Non-interactive (print) mode cannot show
    /// the trust prompt and fails in an untrusted directory; pass
    /// --respect-workspace-trust false to skip the check.
    #[arg(
        long = "respect-workspace-trust",
        value_name = "RESPECT_WORKSPACE_TRUST",
        num_args = 0..=1,
        default_missing_value = "true",
        value_parser = clap::builder::BoolishValueParser::new()
    )]
    pub respect_workspace_trust: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authentication related commands
    Auth(AuthArgs),

    /// Connect and log in to Model Context Protocol servers
    Mcp(McpArgs),

    /// List the models available to your account
    Models(ModelsArgs),

    /// Diagnose the local Devin configuration
    Doctor(DoctorArgs),

    /// Manage agent rules (always-on context blobs)
    Rules(RulesArgs),

    /// Manage agent skills (slash commands and agent-triggered context blobs)
    Skills(SkillsArgs),

    /// Manage plugins (install, list, info, update, remove)
    Plugins(PluginsArgs),

    /// Manage Devin Cloud resources (environment setup, sandbox sessions, builds)
    Cloud(CloudArgs),

    /// Open Devin Desktop
    Desktop(DesktopArgs),

    /// List sessions in the current directory
    #[command(alias = "ls")]
    List(ListArgs),

    /// Delete a session by id or name
    Rm(RmArgs),

    /// SSH into a cloud Devin session's box
    Ssh(SshArgs),

    /// Forward ports from a cloud Devin session's box to localhost
    Forward(ForwardArgs),

    /// Check for updates and optionally install them
    Update(UpdateArgs),

    /// Print the current version
    Version,

    /// Migrate configuration from other tools
    Migrate(MigrateArgs),

    /// [Research Preview] Process sandboxing for the exec tool
    Sandbox(SandboxArgs),

    /// Interactive setup wizard
    Setup(SetupArgs),

    /// Uninstall and remove data
    Uninstall(UninstallArgs),

    /// Run as an ACP (Agent Client Protocol) server over stdio
    Acp(AcpArgs),
}

// ---------------------------------------------------------------------------
// auth
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Log in to Devin
    Login(LoginArgs),
    /// Log out and remove stored credentials
    Logout,
    /// Check authentication status
    Status,
}

#[derive(Args, Debug)]
pub struct LoginArgs {
    /// Skip browser-based auth and paste a token manually
    ///
    /// Useful for remote or SSH sessions where the localhost redirect won't work.
    #[arg(long = "force-manual-token-flow")]
    pub force_manual_token_flow: bool,
}

// ---------------------------------------------------------------------------
// mcp
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct McpArgs {
    #[command(subcommand)]
    pub command: McpCommand,
}

#[derive(Subcommand, Debug)]
pub enum McpCommand {
    /// Add a new MCP server
    Add(McpAddArgs),
    /// List all configured MCP servers
    List,
    /// Get details for a specific MCP server
    Get(McpGetArgs),
    /// Remove an MCP server
    Remove(McpRemoveArgs),
    /// Authenticate with an MCP server via OAuth
    Login(McpLoginArgs),
    /// Remove stored OAuth credentials for an MCP server
    Logout(McpLogoutArgs),
    /// Enable a disabled MCP server
    Enable(McpNameArgs),
    /// Disable an MCP server without removing it
    Disable(McpNameArgs),
}

#[derive(Args, Debug)]
pub struct McpAddArgs {
    /// Server name
    pub name: String,

    /// Server URL (positional form; overrides `--url`)
    pub positional_url: Option<String>,

    /// Arguments passed to the stdio command (after `--`)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,

    /// Transport type (inferred when omitted: URL → http, trailing args → stdio)
    #[arg(short = 't', long = "transport")]
    pub transport: Option<McpTransport>,

    /// Scope of the server config
    #[arg(short = 's', long = "scope")]
    pub scope: Option<McpScope>,

    /// Server URL
    #[arg(long = "url")]
    pub url: Option<String>,

    /// Command to run (stdio transport)
    #[arg(long = "command")]
    pub command: Option<String>,

    /// Environment variables for the server (KEY=VAL)
    #[arg(short = 'e', long = "env")]
    pub env: Vec<String>,

    /// HTTP headers to send (NAME=VALUE)
    #[arg(short = 'H', long = "header")]
    pub header: Vec<String>,

    /// OAuth scopes to request (comma-separated)
    #[arg(long = "scopes")]
    pub scopes: Option<String>,

    /// Pre-registered OAuth client ID for servers that don't support dynamic
    /// client registration (e.g. GitHub)
    #[arg(long = "oauth-client-id")]
    pub oauth_client_id: Option<String>,

    /// OAuth client secret
    #[arg(long = "oauth-client-secret")]
    pub oauth_client_secret: Option<String>,

    /// OAuth resource parameter
    #[arg(long = "oauth-resource")]
    pub oauth_resource: Option<String>,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum McpTransport {
    Stdio,
    Http,
    Sse,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum McpScope {
    User,
    Project,
    Local,
}

#[derive(Args, Debug)]
pub struct McpGetArgs {
    /// Server name
    pub name: String,
}

#[derive(Args, Debug)]
pub struct McpRemoveArgs {
    /// Server name
    pub name: String,
}

#[derive(Args, Debug)]
pub struct McpLoginArgs {
    /// Server name
    pub name: String,
    /// OAuth scopes to request (comma-separated)
    #[arg(long = "scopes")]
    pub scopes: Option<String>,
    /// Pre-registered OAuth client ID for servers that don't support dynamic
    /// client registration (e.g. GitHub)
    #[arg(long = "oauth-client-id")]
    pub oauth_client_id: Option<String>,
    /// OAuth client secret
    #[arg(long = "oauth-client-secret")]
    pub oauth_client_secret: Option<String>,
    /// OAuth resource parameter
    #[arg(long = "oauth-resource")]
    pub oauth_resource: Option<String>,
}

#[derive(Args, Debug)]
pub struct McpLogoutArgs {
    /// Server name
    pub name: String,
}

#[derive(Args, Debug)]
pub struct McpNameArgs {
    /// Server name
    pub name: String,
}

// ---------------------------------------------------------------------------
// models / doctor
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct ModelsArgs {
    #[command(subcommand)]
    pub command: ModelsCommand,
}

#[derive(Subcommand, Debug)]
pub enum ModelsCommand {
    /// List available models, organized by model family
    List(ModelsListArgs),
}

#[derive(Args, Debug)]
pub struct ModelsListArgs {
    /// Output format
    #[arg(long = "format", default_value = "text")]
    pub format: String,
}

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Emit machine-readable JSON instead of the text table
    #[arg(long = "json")]
    pub json: bool,
}

// ---------------------------------------------------------------------------
// rules / skills
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct RulesArgs {
    #[command(subcommand)]
    pub command: RulesCommand,
}

#[derive(Subcommand, Debug)]
pub enum RulesCommand {
    /// List all available rules
    List,
    /// Show details for a specific rule
    Show(RulesShowArgs),
    /// Show rule directory locations
    Paths,
}

#[derive(Args, Debug)]
pub struct RulesShowArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct SkillsArgs {
    #[command(subcommand)]
    pub command: SkillsCommand,
}

#[derive(Subcommand, Debug)]
pub enum SkillsCommand {
    /// List all available skills
    List,
    /// Show details for a specific skill
    Show(SkillsShowArgs),
    /// Show skill directory locations
    Paths,
}

#[derive(Args, Debug)]
pub struct SkillsShowArgs {
    pub name: String,
}

// ---------------------------------------------------------------------------
// plugins
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct PluginsArgs {
    #[command(subcommand)]
    pub command: PluginsCommand,
}

#[derive(Subcommand, Debug)]
pub enum PluginsCommand {
    /// Install a plugin from a source (auto-installs its required plugins)
    Install(PluginsInstallArgs),
    /// List installed plugins with their version and blocked status
    List,
    /// Show a plugin's skills, hooks, rules, and its required/optional/forbidden lists
    Info(PluginsInfoArgs),
    /// Re-fetch and re-install a plugin (or all plugins) at the latest HEAD
    Update(PluginsUpdateArgs),
    /// Remove an installed plugin (auto-installed requireds are left in place)
    Remove(PluginsRemoveArgs),
    /// Prune plugin requirements from repos that no longer exist on disk, then
    /// garbage-collect any plugin content no live scope still requires
    Prune,
}

#[derive(Args, Debug)]
pub struct PluginsInstallArgs {
    /// A GitHub `owner/repo`, a git URL, or a local path. Append `#path/to/plugin`
    /// when the plugin is below a git repository's root.
    pub source: String,
    /// Skip the interactive trust prompt
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,
    /// Install on this machine only, without adding the plugin to your personal plugins
    #[arg(long = "local")]
    pub local: bool,
}

#[derive(Args, Debug)]
pub struct PluginsInfoArgs {
    /// The installed plugin name (or a disambiguated `owner/repo` label when
    /// several installed plugins share the same name)
    pub name: String,
}

#[derive(Args, Debug)]
pub struct PluginsUpdateArgs {
    /// The plugin to update; omit to update all installed plugins
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct PluginsRemoveArgs {
    /// The installed plugin name (or a disambiguated `owner/repo` label)
    pub name: String,
}

// ---------------------------------------------------------------------------
// cloud
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct CloudArgs {
    #[command(subcommand)]
    pub command: CloudCommand,
}

#[derive(Subcommand, Debug)]
pub enum CloudCommand {
    /// Manage Declarative Repo Setup (environment blueprints, sandbox sessions, and builds)
    Drs(DrsArgs),
}

#[derive(Args, Debug)]
pub struct DrsArgs {
    #[command(subcommand)]
    pub command: DrsCommand,
}

#[derive(Subcommand, Debug)]
pub enum DrsCommand {
    /// Print the current DRS configuration (org, API endpoint, auth status)
    Whoami,
    /// Create a sandbox Devin session for testing repo setup
    SandboxCreate(DrsSandboxCreateArgs),
    /// Run a shell command inside a sandbox session
    Run(DrsRunArgs),
    /// List all environment blueprints for the organization
    BlueprintList,
    /// Create a new environment blueprint
    BlueprintCreate(DrsBlueprintCreateArgs),
    /// Update an existing environment blueprint
    BlueprintWrite(DrsBlueprintWriteArgs),
    /// Trigger an environment build and wait for it to finish
    Build(DrsBuildArgs),
    /// Trigger an environment build without waiting
    BuildStart(DrsBuildArgs),
    /// Wait for a previously started build to finish
    BuildWait(DrsBuildWaitArgs),
    /// Fetch the log stream for a build job
    BuildLogs(DrsBuildLogsArgs),
    /// Create organization-level secrets
    SecretCreate(DrsSecretCreateArgs),
}

#[derive(Args, Debug)]
pub struct DrsSandboxCreateArgs {
    /// Repo slug to create the sandbox for, e.g. `owner/repo`
    #[arg(long = "repo")]
    pub repo: String,
    /// Blueprint name to use; when omitted a sandbox is created for the repo
    /// without an environment build
    #[arg(long = "blueprint")]
    pub blueprint: Option<String>,
    /// Build the environment before creating the sandbox
    #[arg(long = "build")]
    pub build: bool,
}

#[derive(Args, Debug)]
pub struct DrsRunArgs {
    /// Devin session ID (e.g. `devin-abc123...`)
    #[arg(long = "devin-id")]
    pub devin_id: String,
    /// Shell command to execute
    #[arg(long = "command")]
    pub command: String,
    /// Timeout in seconds (server-side)
    #[arg(long = "timeout", default_value_t = 600)]
    pub timeout: u64,
}

#[derive(Args, Debug)]
pub struct DrsBlueprintCreateArgs {
    /// Blueprint name
    pub name: String,
    /// Repo slug (e.g. `owner/repo`)
    #[arg(long = "repo")]
    pub repo: String,
    /// Blueprint YAML (either this or --file)
    #[arg(long = "yaml")]
    pub yaml: Option<String>,
    /// Path to a blueprint YAML file
    #[arg(long = "file")]
    pub file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DrsBlueprintWriteArgs {
    /// Blueprint name or ID
    pub name: String,
    /// Blueprint YAML (either this or --file)
    #[arg(long = "yaml")]
    pub yaml: Option<String>,
    /// Path to a blueprint YAML file
    #[arg(long = "file")]
    pub file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DrsBuildArgs {
    /// Repo slug (e.g. `owner/repo`)
    #[arg(long = "repo")]
    pub repo: String,
    /// Blueprint name
    #[arg(long = "blueprint")]
    pub blueprint: String,
}

#[derive(Args, Debug)]
pub struct DrsBuildWaitArgs {
    /// Build ID returned by `build start`
    pub build_id: String,
    /// Timeout in seconds to wait before giving up
    #[arg(long = "timeout", default_value_t = 3600)]
    pub timeout: u64,
}

#[derive(Args, Debug)]
pub struct DrsBuildLogsArgs {
    /// Build ID returned by `build start`
    pub build_id: String,
}

#[derive(Args, Debug)]
pub struct DrsSecretCreateArgs {
    /// Secret name
    pub name: String,
    /// Secret value (either this or --file)
    #[arg(long = "value")]
    pub value: Option<String>,
    /// Path to a file whose contents become the secret value
    #[arg(long = "file")]
    pub file: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// desktop / list / rm / ssh / forward / update / migrate / sandbox / setup / uninstall / acp
// ---------------------------------------------------------------------------

#[derive(Args, Debug)]
pub struct DesktopArgs {
    /// Arguments forwarded to the Devin Desktop launcher (e.g. paths)
    pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Output format
    #[arg(long = "format", default_value = "interactive")]
    pub format: ListFormat,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum ListFormat {
    /// Interactive session picker (default)
    Interactive,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

#[derive(Args, Debug)]
pub struct RmArgs {
    /// Session id or name, or an unambiguous id prefix
    pub target: String,
    /// Skip the confirmation prompt
    #[arg(long = "force")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct SshArgs {
    /// Session id or session URL
    pub session: String,
    /// Extra options passed through to ssh (e.g. `-L 8080:localhost:8080`)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub ssh_args: Vec<String>,
    /// SSH gateway host (defaults to the ssh. host of your Devin API)
    #[arg(long = "gateway")]
    pub gateway: Option<String>,
}

#[derive(Args, Debug)]
pub struct ForwardArgs {
    /// Session id or session URL
    pub session: String,
    /// Ports on the box to forward, as `<port>` or `<local>:<port>`
    /// (e.g. `3000` or `8080:3000`)
    pub ports: Vec<String>,
    /// SSH gateway host (defaults to the ssh. host of your Devin API)
    #[arg(long = "gateway")]
    pub gateway: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Force re-install even if already on the latest version
    #[arg(long = "force")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct MigrateArgs {
    #[command(subcommand)]
    pub command: MigrateCommand,
}

#[derive(Subcommand, Debug)]
pub enum MigrateCommand {
    /// Migrate Windsurf hooks (.windsurf/hooks.json) to Devin hooks (.devin/hooks.v1.json)
    Hooks,
    /// Migrate workflow files into skills and remove the originals
    Workflows,
}

#[derive(Args, Debug)]
pub struct SandboxArgs {
    #[command(subcommand)]
    pub command: SandboxCommand,
}

#[derive(Subcommand, Debug)]
pub enum SandboxCommand {
    /// [Research Preview] Print the sandbox prerequisites for the current platform
    Setup,
}

#[derive(Args, Debug)]
pub struct SetupArgs {
    /// Skip browser-based auth and paste a token manually
    ///
    /// Useful for remote or SSH sessions where the localhost redirect won't work.
    #[arg(long = "force-manual-token-flow")]
    pub force_manual_token_flow: bool,
}

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Remove all user data
    ///
    /// Includes configuration, history, and custom data.
    #[arg(long = "clean")]
    pub clean: bool,
    /// Skip confirmation prompt
    #[arg(long = "force")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct AcpArgs {
    /// The type of agent to run. When omitted, runs the default agent
    #[arg(long = "agent-type", value_enum)]
    pub agent_type: Option<AcpAgentType>,

    /// Default model for every new ACP session, overriding the
    /// enterprise-configured default. Accepts the same fuzzy names as `/model`
    /// (family slug, alias, or partial name), e.g. `--model opus`.
    #[arg(long = "model", env = "DEVIN_MODEL")]
    pub model: Option<String>,

    /// Models to switch to, in order, when the provider refuses a request under
    /// its usage policy — Anthropic content policy, Anthropic cyber
    /// verification, or OpenAI cyber policy.
    #[arg(long = "refusal-fallback", env = "DEVIN_REFUSAL_FALLBACK")]
    pub refusal_fallback: Option<String>,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum AcpAgentType {
    /// A summarizer agent with no tools. The model outputs the full summary as
    /// text and a `PostAgentIteration` cog persists it to
    /// `~/.local/share/devin/summaries/<session_id>.md`
    Summarizer,
    /// A code-review agent with read-only + shell tools. Reviews diffs and
    /// provides actionable feedback on correctness, style, security,
    /// performance, and completeness
    Review,
}