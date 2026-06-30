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

Push-Location $RepoRoot
try {
  $npmExit = 0
  & npm audit --audit-level=$NpmAuditLevel --json | Set-Content -Encoding UTF8 -LiteralPath $npmAuditPath
  if ($LASTEXITCODE -ne 0) {
    $npmExit = $LASTEXITCODE
  }

  $cargoAuditAvailable = $false
  $cargoExit = 0
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
  }

  $receipt = [ordered]@{
    generated_at = (Get-Date).ToString("o")
    repo = $RepoRoot.Path
    npm_audit_level = $NpmAuditLevel
    npm_audit_exit = $npmExit
    npm_audit_path = (Resolve-Path $npmAuditPath).Path
    cargo_audit_available = $cargoAuditAvailable
    cargo_audit_exit = $cargoExit
    cargo_audit_path = if ($cargoAuditAvailable) { (Resolve-Path $cargoAuditPath).Path } else { $null }
    ok = ($npmExit -eq 0 -and $cargoAuditAvailable -and $cargoExit -eq 0)
  }

  $receipt | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 -LiteralPath $receiptPath
  Write-Host "Dependency audit receipt: $receiptPath"
  if (-not $receipt.ok) {
    throw "Dependency audit failed. Receipt: $receiptPath"
  }
} finally {
  Pop-Location
}
