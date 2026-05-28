# Quickstart (30 seconds)

## Windows

```powershell
irm https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.ps1 | iex
cd <your-repo>
brain init
```

## macOS/Linux

```sh
curl -fsSL https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.sh | sh
cd <your-repo>
brain init
```

Restart your editor so the **oerum-agent** MCP server loads. Agents should call `brain_resume` at session start and `brain_checkpoint` after milestones.

CLI fallback:

```sh
brain resume
brain checkpoint "short note"
```

See [`mcp-setup.md`](mcp-setup.md) for client-specific MCP configuration.

If you get `404: Not Found` from the `raw.githubusercontent.com` URL, the
repository/branch/path may differ or the repo may be private. In that case,
run the script locally from a checked-out repo:

```powershell
.\install\bootstrap.ps1
brain init
```

```sh
./install/bootstrap.sh
brain init
```
