param(
  [Parameter(Mandatory = $true)]
  [string]$Version,
  [string]$Repo = "Oerum/oerum-agent",
  [switch]$SkipTagPush,
  [switch]$NoWait
)

$ErrorActionPreference = "Stop"

function Require-Command {
  param([string]$Name)
  if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
    throw "Missing required command: $Name"
  }
}

function Invoke-Git {
  param([string[]]$CommandArgs)
  & git @CommandArgs
  if ($LASTEXITCODE -ne 0) {
    throw "git $($CommandArgs -join ' ') failed with exit code $LASTEXITCODE"
  }
}

function Invoke-Gh {
  param([string[]]$CommandArgs)
  & gh @CommandArgs
  if ($LASTEXITCODE -ne 0) {
    throw "gh $($CommandArgs -join ' ') failed with exit code $LASTEXITCODE"
  }
}

if ($Version -notmatch "^v\d+\.\d+\.\d+([\-+].+)?$") {
  throw "Version must look like v1.2.3 (optionally with pre-release/build suffixes)."
}

Require-Command -Name "git"
Require-Command -Name "gh"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
Push-Location $repoRoot
try {
  Invoke-Git -CommandArgs @("rev-parse", "--is-inside-work-tree") | Out-Null
  $headSha = (Invoke-Git -CommandArgs @("rev-parse", "HEAD")).Trim()

  # Ensure gh has valid auth before tagging.
  Invoke-Gh -CommandArgs @("auth", "status")

  $existingRemoteTagOutput = & git ls-remote --tags origin "refs/tags/$Version"
  $existingRemoteTag = ($existingRemoteTagOutput | Out-String).Trim()
  if (-not [string]::IsNullOrWhiteSpace($existingRemoteTag)) {
    throw "Remote tag '$Version' already exists."
  }

  $existingLocalTagOutput = & git tag --list $Version
  $existingLocalTag = ($existingLocalTagOutput | Out-String).Trim()
  if ([string]::IsNullOrWhiteSpace($existingLocalTag)) {
    Invoke-Git -CommandArgs @("tag", "-a", $Version, "-m", "Release $Version")
  }

  if (-not $SkipTagPush) {
    Invoke-Git -CommandArgs @("push", "origin", $Version)
  }

  if ($NoWait) {
    Write-Host "Tag ready: $Version"
    Write-Host "Release workflow will publish assets for all targets."
    exit 0
  }

  Write-Host "Waiting for release workflow to finish for $Version..."
  $maxAttempts = 120
  $attempt = 0
  $runId = $null

  while ($attempt -lt $maxAttempts) {
    $attempt += 1
    $runsJson = & gh run list --repo $Repo --workflow "release.yml" --limit 20 --json databaseId,headSha,status,conclusion,event
    if ($LASTEXITCODE -ne 0) {
      throw "Failed to query workflow runs."
    }

    $runs = $runsJson | ConvertFrom-Json
    $match = $runs |
      Where-Object { $_.event -eq "push" -and $_.headSha -eq $headSha } |
      Select-Object -First 1

    if ($null -ne $match) {
      $runId = $match.databaseId
      if ($match.status -eq "completed") {
        if ($match.conclusion -ne "success") {
          throw "Release workflow failed (run id: $runId, conclusion: $($match.conclusion))."
        }
        break
      }
      Write-Host "Release workflow running (run id: $runId, status: $($match.status))..."
    } else {
      Write-Host "Waiting for workflow run to appear..."
    }

    Start-Sleep -Seconds 10
  }

  if ($null -eq $runId) {
    throw "Timed out waiting for release workflow to start."
  }

  if ($attempt -ge $maxAttempts) {
    throw "Timed out waiting for release workflow to complete (run id: $runId)."
  }

  Write-Host "Release workflow succeeded."
  Invoke-Gh -CommandArgs @("release", "view", $Version, "--repo", $Repo)
}
finally {
  Pop-Location
}
