# Codex local instructions

Register MCP server **oerum-agent** (`brain` + `args: ["mcp"]`). See `docs/mcp-setup.md`.

- `brain_resume` at session start.
- `brain_checkpoint` at milestones.
- `brain_decision` / `brain_task` as needed.
- CLI fallback: `brain resume`, `brain checkpoint "note"`.
