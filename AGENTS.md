# oerum-agent continuity policy

All agents working in this repository should keep shared context in `.brain/`.

## Setup (once per machine)

```powershell
irm https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.ps1 | iex
```

Then `brain init` in this repo. MCP server name: **oerum-agent** (`brain mcp`).

## MCP-first workflow

Prefer MCP tools:

- `brain_resume` at session start
- `brain_checkpoint` at meaningful milestones
- `brain_decision` for major choices
- `brain_task` to track active work
- `brain_scope` when unsure about active storage paths

## CLI fallback

```sh
brain resume
brain checkpoint "short milestone note"
```

## End of session

Record one final checkpoint summary before leaving the task.
