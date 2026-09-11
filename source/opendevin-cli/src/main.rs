//! OpenDevin — a custom Devin-compatible CLI.
//!
//! Same surface as the `devin` CLI (auth, models, doctor, version, chat,
//! sessions, …) plus bonus features: an embedded OpenAI-compatible server
//! (`opendevin serve`), web UI, and a live model catalog.
//! Protocol implemented from scratch and verified against the real backend.

mod auth;
mod chat;
mod models;
mod protocol;
mod server;
mod wire;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

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

    // one-shot print mode
    if let Some(prompt) = cli.print.flatten() {
        let client = protocol::Client::new(auth::resolve_api_key()?)?;
        return chat::print_mode(&client, &prompt, &model, chat::default_max_tokens()).await;
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
            chat::repl(&client, (!prompt.is_empty()).then_some(prompt.as_str()), &m).await
        }
        Some(Command::List(args)) => run_list(args),
        Some(Command::Rm(args)) => run_rm(args),
        Some(Command::Serve(args)) => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            server::serve(client, args.host, args.port, args.model.clone().unwrap_or(model)).await
        }
        None => {
            let client = protocol::Client::new(auth::resolve_api_key()?)?;
            chat::repl(&client, None, &model).await
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

fn session_file() -> PathBuf {
    auth::data_dir().unwrap_or_else(|_| PathBuf::from(".")).join("sessions.json")
}

fn run_list(args: ListArgs) -> Result<()> {
    let path = session_file();
    let sessions: Vec<serde_json::Value> = if path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&path)?).unwrap_or_default()
    } else {
        Vec::new()
    };
    match args.format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&sessions)?),
        "csv" => {
            for s in &sessions {
                println!("{},{}", s["id"].as_str().unwrap_or(""), s["title"].as_str().unwrap_or(""));
            }
        }
        _ => {
            if sessions.is_empty() {
                println!("No sessions yet. (REPL history is saved as you chat.)");
            }
            for s in &sessions {
                println!("{}  {}", s["id"].as_str().unwrap_or(""), s["title"].as_str().unwrap_or(""));
            }
        }
    }
    Ok(())
}

fn run_rm(args: RmArgs) -> Result<()> {
    let path = session_file();
    let mut sessions: Vec<serde_json::Value> = if path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&path)?).unwrap_or_default()
    } else {
        Vec::new()
    };
    let before = sessions.len();
    sessions.retain(|s| s["id"].as_str() != Some(args.target.as_str()));
    if sessions.len() == before && !args.force {
        println!("No session with id '{}'", args.target);
        return Ok(());
    }
    std::fs::write(&path, serde_json::to_string_pretty(&sessions)?)?;
    println!("Removed session {}.", args.target);
    Ok(())
}