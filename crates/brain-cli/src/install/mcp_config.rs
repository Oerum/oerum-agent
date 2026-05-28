use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::{json, Value};

const SERVER_KEY: &str = "oerum-agent";

pub fn cursor_mcp_path() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var_os("USERPROFILE")
            .map(|home| PathBuf::from(home).join(".cursor").join("mcp.json"))
    }
    #[cfg(not(windows))]
    {
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cursor").join("mcp.json"))
    }
}

pub fn merge_cursor_mcp(brain_command: &Path, skip: bool) -> Result<Option<PathBuf>> {
    if skip {
        return Ok(None);
    }
    let Some(path) = cursor_mcp_path() else {
        return Ok(None);
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }

    let mut root: Value = if path.exists() {
        let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?
    } else {
        json!({})
    };

    let servers = root
        .as_object_mut()
        .and_then(|obj| {
            if !obj.contains_key("mcpServers") {
                obj.insert("mcpServers".to_string(), json!({}));
            }
            obj.get_mut("mcpServers").and_then(Value::as_object_mut)
        })
        .context("mcp.json must contain an object mcpServers field")?;

    servers.insert(
        SERVER_KEY.to_string(),
        json!({
            "command": brain_command.to_string_lossy(),
            "args": ["mcp"]
        }),
    );

    let pretty = serde_json::to_string_pretty(&root)?;
    fs::write(&path, format!("{pretty}\n")).with_context(|| format!("write {}", path.display()))?;
    Ok(Some(path))
}

pub fn snippet(brain_command: &str) -> String {
    format!(
        r#"{{
  "mcpServers": {{
    "{SERVER_KEY}": {{
      "command": "{brain_command}",
      "args": ["mcp"]
    }}
  }}
}}"#
    )
}
