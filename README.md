# brain

Shared AI session brain for cross-tool and cross-machine continuity.

Local-first, optional encrypted S3 sync, one binary per platform, no
runtime dependencies on the host.

## Quickstart

1. **One command install** (registers MCP for Cursor when possible):
   - Windows: `irm https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.ps1 | iex`
   - macOS/Linux: `curl -fsSL https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.sh | sh`
2. In your repository: `brain init`
3. Restart your editor and use MCP tools (`brain_resume`, `brain_checkpoint`, …) or the CLI (`brain resume`).

Details: [`docs/quickstart.md`](docs/quickstart.md), [`docs/mcp-setup.md`](docs/mcp-setup.md).

## Commands

- `brain install` — register MCP config and print setup snippet
- `brain mcp` — run the Model Context Protocol server (stdio)
- `brain init` — seed `.brain/` in the current repository
- `brain resume` — print the shared resume brief (active task, recent decisions, suggested next actions)
- `brain checkpoint <note>` — append a free-form continuity note
- `brain decision <value>` — record a key decision
- `brain task <value>` / `brain task --clear` — set or clear the active task
- `brain artifact <value>` — record an artefact reference (file, URL, PR link)
- `brain doctor` — actionable diagnostics on `.brain/` state and sync config
- `brain sync` — push the encrypted snapshot to the configured remote (requires `BRAIN_SYNC_PASSPHRASE`)

## State layout

```
.brain/
  snapshot.json         # materialised state (committed)
  events/00000001.json  # append-only event log (committed)
  locks/                # transient cross-process locks (gitignored)
  sync.json             # optional remote config (committed; no secrets)
```

The snapshot is written atomically (tmp + rename) and all mutations are
serialised behind an OS file lock, so concurrent `brain` calls from
different tools cannot lose updates.

## Architecture

See the ADRs under [`docs/adr/`](docs/adr/) for the runtime, state model,
and sync/encryption design.
