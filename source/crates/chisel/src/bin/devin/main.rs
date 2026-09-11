//! `devin` — the Devin CLI binary entrypoint.
//!
//! Reconstructed from the shipped `devin 3000.10.21` (611c1cba) Mach-O arm64
//! binary. The original crate layout puts this binary in `chisel`:
//! `chisel/src/bin/devin/main.rs`. See `reports/recon.md` for the full
//! reverse-engineering notes.

use std::process::ExitCode;

use chisel::cli::{Cli, Command};
use clap::Parser;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // The original wires up tracing + sentry + telemetry here; the local
    // reconstruction keeps logging minimal until the agent core is in place.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .try_init();

    match dispatch(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Command::Auth(args)) => chisel::auth::run(args),
        Some(Command::Mcp(_args)) => {
            eprintln!("Error: MCP management requires the full agent runtime (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Models(_args)) => chisel_commands::models::run(),
        Some(Command::Doctor(args)) => chisel_commands::doctor::run(args),
        Some(Command::Rules(_args)) => chisel_commands::rules::run(),
        Some(Command::Skills(_args)) => chisel_commands::skills::run(),
        Some(Command::Plugins(_args)) => {
            eprintln!("Error: plugin management requires the plugin runtime (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Cloud(_args)) => {
            eprintln!("Error: cloud commands require the windsurf API client (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Desktop(_args)) => chisel::app_state::open_desktop(),
        Some(Command::List(args)) => chisel::list::run(args),
        Some(Command::Rm(args)) => chisel::rm::run(args),
        Some(Command::Ssh(args)) => chisel::ssh_cmd::run(args),
        Some(Command::Forward(_args)) => {
            eprintln!("Error: forwarding requires the SSH gateway client (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Update(args)) => chisel::version_update_cli::run(args),
        Some(Command::Version) => {
            println!("devin {}", chisel::version());
            Ok(())
        }
        Some(Command::Migrate(_args)) => {
            eprintln!("Error: configuration migration requires the importers crate (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Sandbox(_args)) => {
            eprintln!("Error: sandbox setup requires the sandbox-runtime crate (not yet reconstructed)");
            Ok(())
        }
        Some(Command::Setup(args)) => chisel::setup::run(args),
        Some(Command::Uninstall(args)) => chisel::version_update_cli::run_uninstall(args),
        Some(Command::Acp(_args)) => {
            eprintln!("Error: the ACP server requires the agent runtime (not yet reconstructed)");
            Ok(())
        }
        None => {
            // Interactive REPL — requires the terminal UI (scrollback/ratatui).
            // For now: print the help and note that the REPL is pending.
            println!("Devin CLI {} (reconstructed)", chisel::version());
            println!(
                "The interactive REPL and the agent loop are part of the ongoing \
                 reconstruction; see the docs/ folder. Try `devin --help`."
            );
            Ok(())
        }
    }
}

/// The CLI version, matching the original binary's `3000.10.21 (611c1cba)`.
pub fn version() -> &'static str {
    concat!(env!("CARGO_PKG_VERSION"), " (reconstructed)")
}