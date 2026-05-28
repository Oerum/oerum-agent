mod commands;
mod install;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brain", about = "Shared AI session brain CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install CLI and register MCP config for coding agents.
    Install {
        /// Skip MCP registration and config snippet output.
        #[arg(long)]
        no_mcp: bool,
        /// Skip writing ~/.cursor/mcp.json.
        #[arg(long)]
        skip_cursor_mcp: bool,
    },
    /// Run the Model Context Protocol server on stdio.
    Mcp,
    Init,
    Resume,
    Checkpoint {
        note: String,
    },
    /// Record a key decision the AI/operator made this session.
    Decision {
        value: String,
    },
    /// Set or clear the currently active task.
    Task {
        /// New task description. Pass `--clear` instead to unset.
        value: Option<String>,
        #[arg(long, conflicts_with = "value")]
        clear: bool,
    },
    /// Record an artefact reference (file path, URL, PR link).
    Artifact {
        value: String,
    },
    Doctor,
    Sync,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    match cli.command {
        Command::Install {
            no_mcp,
            skip_cursor_mcp,
        } => install::run(&cwd, !no_mcp, skip_cursor_mcp)?,
        Command::Mcp => commands::mcp::run_stdio().await?,
        Command::Init => commands::init::run(&cwd)?,
        Command::Resume => commands::resume::run(&cwd)?,
        Command::Checkpoint { note } => commands::checkpoint::run(&cwd, note)?,
        Command::Decision { value } => commands::record::decision(&cwd, value)?,
        Command::Task { value, clear } => {
            let new = if clear { None } else { value };
            commands::record::task(&cwd, new)?
        }
        Command::Artifact { value } => commands::record::artifact(&cwd, value)?,
        Command::Doctor => commands::doctor::run(&cwd)?,
        Command::Sync => commands::sync::run(&cwd).await?,
    }
    Ok(())
}
