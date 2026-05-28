mod mcp_config;

use std::env;
use std::path::Path;

use anyhow::{Context, Result};

pub use mcp_config::snippet;

pub fn run(_cwd: &Path, mcp: bool, skip_cursor_mcp: bool) -> Result<()> {
    let brain_cmd = env::current_exe().context("resolve brain executable path")?;

    println!("brain install complete");
    println!("binary: {}", brain_cmd.display());
    println!();
    println!("Next in any git repository:");
    println!("  brain init");
    println!();

    if mcp {
        println!("MCP server: oerum-agent (stdio via `brain mcp`)");
        println!();
        println!("Add to your client MCP config:");
        println!("{}", snippet(&brain_cmd.to_string_lossy()));
        println!();

        match mcp_config::merge_cursor_mcp(&brain_cmd, skip_cursor_mcp)? {
            Some(path) => println!("Updated Cursor MCP config: {}", path.display()),
            None if skip_cursor_mcp => println!("Skipped Cursor MCP config (--skip-cursor-mcp)."),
            None => {
                println!("Could not resolve Cursor MCP path; paste the snippet above manually.")
            }
        }
        println!();
        println!("Restart Cursor (or reload MCP) after install.");
    }

    Ok(())
}
