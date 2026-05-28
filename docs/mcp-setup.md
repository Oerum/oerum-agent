# MCP setup (recommended)

Use the built-in MCP server so agents can call `brain_resume`, `brain_checkpoint`, and related tools without manual shell commands.

## 1. Install (one command)

**Windows**

```powershell
irm https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.ps1 | iex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.sh | sh
```

The bootstrap script installs `brain`, runs `brain install`, and registers the MCP server in Cursor (`~/.cursor/mcp.json`) when possible.

**Local development fallback** (requires Rust toolchain):

```sh
cargo install --path crates/brain-cli --locked --root "$HOME/.brain/bin"
export PATH="$HOME/.brain/bin:$PATH"
brain install
```

## 2. Initialize your repository

From the repo you are working in:

```sh
brain init
```

## 3. MCP configuration

`brain install` writes an `oerum-agent` entry like:

```json
{
  "mcpServers": {
    "oerum-agent": {
      "command": "brain",
      "args": ["mcp"]
    }
  }
}
```

Use the full path to `brain` if it is not on your `PATH` (common on Windows until you open a new shell).

For other clients (Claude Code, Codex, Gemini, Copilot), paste the same `mcpServers` block into that tool's MCP settings file.

## 4. Agent usage

At session start, the agent should call **`brain_resume`**.

After meaningful progress, call **`brain_checkpoint`** with a short note.

Optional: `brain_decision`, `brain_task`, `brain_scope`.

## Troubleshooting

- **MCP server fails to start**: run `brain doctor` and confirm `brain mcp` works in a terminal from your project root.
- **Wrong repository context**: set `BRAIN_REPO_ROOT` to your git root in the MCP server `env` block.
- **Cursor still uses an old server path**: open `~/.cursor/mcp.json` and ensure `oerum-agent` points at `brain` with `args: ["mcp"]`, not a missing `.exe` path.
