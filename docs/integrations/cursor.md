# Cursor Integration

1. Install: `irm .../install/bootstrap.ps1 | iex` (runs `brain install` and registers MCP).
2. In your repo: `brain init`.
3. Restart Cursor.

Agents should use MCP tools `brain_resume` and `brain_checkpoint`. CLI fallback: `brain resume` / `brain checkpoint "note"`.

State is stored in `.brain/`.
