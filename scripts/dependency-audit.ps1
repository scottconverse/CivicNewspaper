param(
  [string]$OutputDir = "",
  [ValidateSet("low", "moderate", "high", "critical")]
  [string]$NpmAuditLevel = "moderate"
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$Stamp = Get-Date -Format "yyyyMMdd-HHmmss"
if (-not $OutputDir) {
  $OutputDir = Join-Path $RepoRoot ".agent-runs"
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$npmAuditPath = Join-Path $OutputDir "npm-audit-$Stamp.json"
$cargoAuditPath = Join-Path $OutputDir "cargo-audit-$Stamp.json"
$receiptPath = Join-Path $OutputDir "dependency-audit-$Stamp.json"
$waiverPath = Join-Path $RepoRoot "docs\security-advisory-waivers.json"

Push-Location $RepoRoot
try {
  $npmExit = 0
  & npm audit --audit-level=$NpmAuditLevel --json | Set-Content -Encoding UTF8 -LiteralPath $npmAuditPath
  if ($LASTEXITCODE -ne 0) {
    $npmExit = $LASTEXITCODE
  }

  $cargoAuditAvailable = $false
  $cargoExit = 0
  $cargoAuditIgnoreIds = @()
  $cargoAuditWaivers = @()
  $missingCargoAuditWaivers = @()
  $expiredCargoAuditWaivers = @()
  if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    $cargoAuditAvailable = $true
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      & cargo audit --json | Set-Content -Encoding UTF8 -LiteralPath $cargoAuditPath
      if ($LASTEXITCODE -ne 0) {
        $cargoExit = $LASTEXITCODE
      }
    } finally {
      Pop-Location
    }

    $cargoAuditJson = Get-Content -Raw -LiteralPath $cargoAuditPath | ConvertFrom-Json
    $cargoAuditIgnoreIds = @($cargoAuditJson.settings.ignore | Where-Object { $_ } | ForEach-Object { [string]$_ })
    if ($cargoAuditIgnoreIds.Count -gt 0) {
      if (-not (Test-Path -LiteralPath $waiverPath)) {
        $missingCargoAuditWaivers = $cargoAuditIgnoreIds
      } else {
        $waiverJson = Get-Content -Raw -LiteralPath $waiverPath | ConvertFrom-Json
        $waiverEntries = @($waiverJson.advisories | Where-Object { $_ })
        $today = (Get-Date).Date
        foreach ($id in $cargoAuditIgnoreIds) {
          $entry = $waiverEntries | Where-Object { $_.id -eq $id } | Select-Object -First 1
          if (-not $entry) {
            $missingCargoAuditWaivers += $id
            continue
          }
          try {
            $reviewBy = Get-Date -Date ([string]$entry.review_by) -ErrorAction Stop
          } catch {
            $reviewBy = $null
          }
          if (-not $reviewBy -or $reviewBy.Date -lt $today) {
            $expiredCargoAuditWaivers += $id
          }
          $cargoAuditWaivers += [ordered]@{
            id = [string]$entry.id
            crate = [string]$entry.crate
            source = [string]$entry.source
            rationale = [string]$entry.rationale
            release_note = [string]$entry.release_note
            owner = [string]$entry.owner
            review_by = [string]$entry.review_by
          }
        }
      }
    }
  }

  $waiverOk = ($missingCargoAuditWaivers.Count -eq 0 -and $expiredCargoAuditWaivers.Count -eq 0)
  $receipt = [ordered]@{
    generated_at = (Get-Date).ToString("o")
    repo = $RepoRoot.Path
    npm_audit_level = $NpmAuditLevel
    npm_audit_exit = $npmExit
    npm_audit_path = (Resolve-Path $npmAuditPath).Path
    cargo_audit_available = $cargoAuditAvailable
    cargo_audit_exit = $cargoExit
    cargo_audit_path = if ($cargoAuditAvailable) { (Resolve-Path $cargoAuditPath).Path } else { $null }
    cargo_audit_ignore_count = $cargoAuditIgnoreIds.Count
    cargo_audit_ignored_advisories = $cargoAuditIgnoreIds
    cargo_audit_waiver_path = if (Test-Path -LiteralPath $waiverPath) { (Resolve-Path $waiverPath).Path } else { $null }
    cargo_audit_waivers = $cargoAuditWaivers
    missing_cargo_audit_waivers = $missingCargoAuditWaivers
    expired_cargo_audit_waivers = $expiredCargoAuditWaivers
    ok = ($npmExit -eq 0 -and $cargoAuditAvailable -and $cargoExit -eq 0 -and $waiverOk)
  }

  $receipt | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 -LiteralPath $receiptPath
  Write-Host "Dependency audit receipt: $receiptPath"
  if (-not $receipt.ok) {
    throw "Dependency audit failed. Receipt: $receiptPath"
  }
} finally {
  Pop-Location
}
