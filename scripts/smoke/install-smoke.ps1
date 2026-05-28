$ErrorActionPreference = "Stop"
Write-Host "Running Windows install smoke test"
if (-not (Test-Path "install/bootstrap.ps1")) { throw "Missing bootstrap.ps1" }
Write-Host "Smoke check passed"
