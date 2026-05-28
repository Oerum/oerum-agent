param(
  [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"
$Platform = "windows-x64"
$Archive  = "brain-$Platform.zip"

if ($Version -eq "latest") {
  $BaseUrl = "https://github.com/Oerum/oerum-agent/releases/latest/download"
} else {
  $BaseUrl = "https://github.com/Oerum/oerum-agent/releases/download/$Version"
}

$Url         = "$BaseUrl/$Archive"
$ChecksumUrl = "$BaseUrl/$Archive.sha256"
$DestDir     = Join-Path $env:USERPROFILE ".brain\bin"
$Tmp         = Join-Path $env:TEMP $Archive
$TmpSum      = Join-Path $env:TEMP "$Archive.sha256"

New-Item -ItemType Directory -Path $DestDir -Force | Out-Null

Write-Host "Downloading $Url"
try {
  Invoke-WebRequest -Uri $Url -OutFile $Tmp -UseBasicParsing
  Invoke-WebRequest -Uri $ChecksumUrl -OutFile $TmpSum -UseBasicParsing
} catch {
  $StatusCode = $null
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode) {
    $StatusCode = [int]$_.Exception.Response.StatusCode
  }

  if ($StatusCode -eq 404) {
    throw @"
Release asset not found at:
  $Url

No matching GitHub release asset is currently published.

Maintainer path:
  Publish a release containing '$Archive' and '$Archive.sha256'.

Local fallback:
  cargo install --path crates/brain-cli --locked --root "$DestDir"
  Then run: brain init
"@
  }

  throw
}

$ExpectedLine = (Get-Content $TmpSum | Select-Object -First 1).Trim()
if (-not $ExpectedLine) {
  throw "Empty checksum file at $ChecksumUrl"
}
$Expected = ($ExpectedLine -split "\s+")[0].ToLower()
$Actual   = (Get-FileHash -Algorithm SHA256 $Tmp).Hash.ToLower()

if ($Expected -ne $Actual) {
  Remove-Item $Tmp -Force -ErrorAction SilentlyContinue
  throw "Checksum mismatch for $Archive`nexpected: $Expected`nactual:   $Actual"
}

Write-Host "Checksum verified ($Actual)"
Expand-Archive -Path $Tmp -DestinationPath $DestDir -Force
Remove-Item $Tmp, $TmpSum -Force -ErrorAction SilentlyContinue

# Persistently add to user PATH if missing.
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not ($UserPath -split ";" | Where-Object { $_ -ieq $DestDir })) {
  $NewPath = if ([string]::IsNullOrEmpty($UserPath)) { $DestDir } else { "$UserPath;$DestDir" }
  [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
  Write-Host "Added $DestDir to user PATH (open a new shell to pick it up)."
}

Write-Host ""
Write-Host "Installed brain to $DestDir"
Write-Host "Next: open a new PowerShell window and run 'brain init'"
