//! OpenDevin — a custom Devin-compatible CLI.
//!
//! Same surface as the `devin` CLI (auth, models, doctor, version, chat,
//! sessions, …) plus bonus features: an embedded OpenAI-compatible server
//! (`opendevin serve`), web UI, and a live model catalog.
//! Protocol implemented from scratch and verified against the real backend.

mod agent;
mod auth;
mod chat;
mod models;
mod protocol;
mod server;
mod sessions;
mod tools;
mod tui;
mod wire;



use anyhow::Result;
#[allow(unused_imports)]
use clap::{Args, Parser, Subcommand};
use serde_json::json;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "opendevin",
    version,
    about = "A custom Devin-compatible agent CLI — Devin's models, your tools, plus an OpenAI-compatible server."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Print response and exit (one-shot) — same as `devin -p`
    #[arg(short = 'p', long = "print", value_name = "PROMPT", num_args = 0..=1)]
    pub print: Option<Option<String>>,

    /// Model to use (default: swe-2-high)
    #[arg(short = 'm', long = "model", global = true)]
    pub model: Option<String>,

    /// Continue the most recent conversation (resume)
    #[arg(short = 'c', long = "continue")]
    pub continue_session: bool,

    /// Resume a specific conversation by id
    #[arg(short = 'r', long = "resume", value_name = "SESSION_ID")]
    pub resume: Option<String>,

    /// Permission mode: auto | accept-edits | bypass (dangerous/yolo)
    #[arg(long = "permission-mode", default_value = "auto", env = "DEVIN_PERMISSION_MODE")]
    pub permission_mode: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authentication related commands
    Auth(AuthArgs),
    /// List the models available to your account (live catalog)
    Models(ModelsArgs),
    /// Diagnose the local configuration
    Doctor,
    /// Print the current version
    Version,
    /// Check for updates and optionally install them
    Update(UpdateArgs),
    /// Interactive chat (REPL)
    Chat(ChatArgs),
    /// List sessions in the current directory
    List(ListArgs),
    /// Delete a session by id
    Rm(RmArgs),
    /// [bonus] Run an OpenAI-compatible server (also serves a web UI)
    Serve(ServeArgs),
}

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Log in to Devin (uses the devin CLI's PKCE flow)
    Login(LoginArgs),
    /// Log out and remove stored credentials
    Logout,
    /// Check authentication status
    Status,
}

#[derive(Args, Debug)]
pub struct LoginArgs {
    /// Skip browser-based auth and paste a token manually
    #[arg(long = "force-manual-token-flow")]
    pub force_manual_token_flow: bool,
}

#[derive(Args, Debug)]
pub struct ModelsArgs {
    #[command(subcommand)]
    pub command: Option<ModelsCommand>,
}

#[derive(Subcommand, Debug)]
pub enum ModelsCommand {
    /// List available models, organized by family (default)
    List(ModelsListArgs),
}

#[derive(Args, Debug)]
pub struct ModelsListArgs {
    /// Output format: text | json
    #[arg(long = "format", default_value = "text")]
    pub format: String,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Force re-check even if up to date
    #[arg(long = "force")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ChatArgs {
    /// Model to use
    #[arg(long = "model")]
    pub model: Option<String>,
    /// Initial prompt
    #[arg(value_name = "PROMPT", last = true)]
    pub prompt: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Output format: text | json | csv
    #[arg(long = "format", default_value = "text")]
    pub format: String,
}

#[derive(Args, Debug)]
pub struct RmArgs {
    /// Session id
    pub target: String,
    /// Skip confirmation
    #[arg(long = "force")]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Listen port
    #[arg(long = "port", default_value_t = 8321)]
    pub port: u16,
    /// Bind host
    #[arg(long = "host", default_value = "127.0.0.1")]
    pub host: String,
    /// Default model served at /v1/chat/completions
    #[arg(long = "model")]
    pub model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let model = cli.model.clone().unwrap_or_else(|| models::DEFAULT_MODEL.to_string());
    let resume_id = cli.resume.clone();
    let continue_flag = cli.continue_session;
    let perm_mode = agent::PermissionMode::parse(&cli.permission_mode);

    // one-shot print mode — runs the full agent (tools included), like `devin -p`
    if let Some(prompt) = cli.print.clone().flatten() {
        let client = protocol::Client::new(auth::resolve_api_key()?)?;
        let mut msgs = vec![json!({"role": "user", "content": prompt})];
        use std::io::Write;
        let noninteractive_deny = perm_mode != agent::PermissionMode::Bypass;
        agent::run_agent(
            &client,
            &mut msgs,
            &model,
            chat::default_max_tokens(),
            24,
            perm_mode,
            noninteractive_deny,
            |ev| {
                if let Some(t) = ev.text {
                    print!("{t}");
                    std::io::stdout().flush().ok();
                }
                if let Some(tc) = ev.tool_call {
                    let name = tc["function"]["name"].as_str().unwrap_or("?");
                    let args = tc["function"]["arguments"].as_str().unwrap_or("");
                    println!("\n⚙ {name} {args}");
                }
                if let Some(r) = ev.tool_result {
                    let first_line = r.lines().next().unwrap_or("").chars().take(120).collect::<String>();
                    println!("↩ {first_line}");
                }
            },
        )
        .await?;
        println!();
        return Ok(());
    }

    match cli.command {
        Some(Command::Auth(args)) => run_auth(args),
        Some(Command::Models(args)) => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            let format = match args.command {
                Some(ModelsCommand::List(l)) => l.format,
                None => "text".to_string(),
            };
            let models = models::list(&client).await?;
            match format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&models::models_json(&models))?),
                _ => {
                    for (display, uid) in &models {
                        println!("{uid:32} {display}");
                    }
                }
            }
            Ok(())
        }
        Some(Command::Doctor) => run_doctor(),
        Some(Command::Version) => {
            println!("opendevin {VERSION}");
            Ok(())
        }
        Some(Command::Update(args)) => run_update(args),
        Some(Command::Chat(args)) => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            let m = args.model.clone().unwrap_or(model);
            let prompt = args.prompt.join(" ");
            let mut initial = load_resume(resume_id.clone(), continue_flag).unwrap_or_default();
            if !prompt.is_empty() {
                initial.push(json!({"role": "user", "content": prompt}));
            }
            let msgs = tui::tui_repl(client, initial, m).await?;
            save_session(resume_id.clone(), msgs)?;
            Ok(())
        }
        Some(Command::List(args)) => run_list(args),
        Some(Command::Rm(args)) => run_rm(args),
        Some(Command::Serve(args)) => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            server::serve(client, args.host, args.port, args.model.clone().unwrap_or(model)).await
        }
        None => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            let mut initial = load_resume(resume_id.clone(), continue_flag).unwrap_or_default();
            let msgs = tui::tui_repl(client, initial, model).await?;
            save_session(resume_id.clone(), msgs)?;
            Ok(())
        }
    }
}

fn run_auth(args: AuthArgs) -> Result<()> {
    use clap::Args;
    let _ = &args;
    match args.command {
        AuthCommand::Login(_) => {
            eprintln!(
                "OpenDevin uses the Devin CLI credential store. Run `devin auth login` \
                 once, or set DEVIN_API_KEY."
            );
            Ok(())
        }
        AuthCommand::Logout => {
            let path = auth::credentials_path()?;
            if path.exists() {
                std::fs::remove_file(&path)?;
                println!("Removed {}", path.display());
            } else {
                println!("No credentials found.");
            }
            Ok(())
        }
        AuthCommand::Status => {
            if auth::status()? {
                println!("Logged in (credentials found at {})", auth::credentials_path()?.display());
            } else {
                println!("Not logged in. Run `devin auth login` or set DEVIN_API_KEY.");
            }
            Ok(())
        }
    }
}

fn run_doctor() -> Result<()> {
    let creds_ok = auth::status()?;
    let key_env = auth::env_api_key().is_some();
    println!("OpenDevin doctor — {VERSION}");
    println!("  credentials.toml: {}", if creds_ok { "present" } else { "missing" });
    println!("  DEVIN_API_KEY env: {}", if key_env { "set" } else { "not set" });
    println!("  default model: {}", models::DEFAULT_MODEL);
    if !creds_ok && !key_env {
        println!("  status: ACTION REQUIRED — run `devin auth login`");
    } else {
        println!("  status: OK");
    }
    Ok(())
}

fn run_update(args: UpdateArgs) -> Result<()> {
    let _ = args.force;
    eprintln!("opendevin is self-contained; updates are fetched from the OpenDevin repo.");
    Ok(())
}

fn load_resume(resume_id: Option<String>, continue_flag: bool) -> Option<Vec<serde_json::Value>> {
    if let Some(id) = resume_id {
        return sessions::find_by_id(&id).map(|s| s.messages);
    }
    if continue_flag {
        return sessions::most_recent().map(|s| s.messages);
    }
    None
}

fn save_session(resume_id: Option<String>, messages: Vec<serde_json::Value>) -> Result<()> {
    sessions::upsert(resume_id, messages)?;
    Ok(())
}

fn run_list(args: ListArgs) -> Result<()> {
    let sessions = sessions::load_all();
    match args.format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&sessions)?),
        "csv" => {
            for s in &sessions {
                println!("{},{}", s.id, s.title);
            }
        }
        _ => {
            if sessions.is_empty() {
                println!("No sessions yet. (conversations are saved as you chat)");
            }
            for s in &sessions {
                println!("{}  {}  {}", s.id, s.title, s.updated);
            }
        }
    }
    Ok(())
}

fn run_rm(args: RmArgs) -> Result<()> {
    let mut sessions = sessions::load_all();
    let before = sessions.len();
    sessions.retain(|s| !s.id.starts_with(&args.target));
    if sessions.len() == before && !args.force {
        println!("No session with id '{}'", args.target);
        return Ok(());
    }
    sessions::save_all(&sessions)?;
    println!("Removed session {}.", args.target);
    Ok(())
}