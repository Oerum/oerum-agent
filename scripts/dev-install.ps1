# Local developer install: build brain from source, register MCP.
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$Dest = Join-Path $env:USERPROFILE ".brain"

Write-Host "Installing brain to $Dest ..."
cargo install --path (Join-Path $Root "crates/brain-cli") --locked --root $Dest

$Brain = Join-Path $Dest "bin/brain.exe"
if (-not (Test-Path $Brain)) {
    $Brain = Join-Path $Dest "bin/brain"
}

& $Brain install
Write-Host ""
Write-Host "Done. Add to PATH if needed:"
Write-Host "  $Dest\bin"
