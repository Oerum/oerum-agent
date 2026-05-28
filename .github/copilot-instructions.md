# Copilot repository instructions

This repository uses `oerum-agent` shared context in `.brain/`.

Install once: see `docs/mcp-setup.md` (bootstrap + `brain install` registers MCP).

- MCP-first: `brain_resume` at session start.
- MCP-first: `brain_checkpoint` after meaningful milestones.
- Optional: `brain_decision`, `brain_task`, `brain_scope`.
- CLI fallback: `brain resume`, `brain checkpoint "short milestone note"`.
- End each task with a final checkpoint summary.
