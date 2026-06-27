param(
  [string]$FixtureDir = "C:\Users\instynct\Desktop\CivicNewspaperTestFiles",
  [string]$Model = "qwen2.5:7b",
  [switch]$SkipLiveModel,
  [switch]$SkipHereNow,
  [switch]$SkipImportFixtures
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$Stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$RunDir = Join-Path $RepoRoot ".agent-runs\release-smoke-$Stamp"
$ExtractedDir = Join-Path $RunDir "import-extracted"
$StaticSiteDir = Join-Path $RunDir "static-site"
New-Item -ItemType Directory -Force -Path $RunDir, $ExtractedDir, $StaticSiteDir | Out-Null

$receipt = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  repo = $RepoRoot.Path
  fixture_dir = $FixtureDir
  model = $Model
  static_site_dir = $StaticSiteDir
  checks = @()
}

function Invoke-Check {
  param(
    [string]$Name,
    [scriptblock]$Script
  )

  $started = Get-Date
  Write-Host "==> $Name"
  try {
    $oldErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $global:LASTEXITCODE = 0
    & $Script *>&1 | Tee-Object -FilePath (Join-Path $RunDir "$($Name -replace '[^A-Za-z0-9_.-]', '_').log")
    $ErrorActionPreference = $oldErrorActionPreference
    if ($LASTEXITCODE -ne 0) {
      throw "$Name exited with code $LASTEXITCODE"
    }
    $receipt.checks += [ordered]@{
      name = $Name
      ok = $true
      seconds = [math]::Round(((Get-Date) - $started).TotalSeconds, 2)
    }
  } catch {
    $ErrorActionPreference = "Stop"
    $receipt.checks += [ordered]@{
      name = $Name
      ok = $false
      seconds = [math]::Round(((Get-Date) - $started).TotalSeconds, 2)
      error = $_.Exception.Message
    }
    throw
  }
}

Push-Location $RepoRoot
try {
  Invoke-Check "git-status" {
    git status --short --branch
    git rev-parse --short HEAD
  }

  Invoke-Check "frontend-tests" {
    npm test -- --run
  }

  Invoke-Check "rust-tests" {
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      cmd /d /c "cargo test --all --locked 2>&1"
    } finally {
      Pop-Location
    }
  }

  Invoke-Check "static-site-output-gate" {
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      $env:CIVIC_DESK_AUDIT_OUTPUT_DIR = $StaticSiteDir
      cmd /d /c "cargo test test_seeded_publish_fixture_generates_article_evidence_and_correction_package -- --nocapture 2>&1"
    } finally {
      Remove-Item Env:\CIVIC_DESK_AUDIT_OUTPUT_DIR -ErrorAction SilentlyContinue
      Pop-Location
    }
  }

  if (-not $SkipHereNow) {
    Invoke-Check "here-now-anonymous-live-publish" {
      Push-Location (Join-Path $RepoRoot "src-tauri")
      try {
        $env:CIVIC_DESK_HERENOW_OUTPUT_DIR = $StaticSiteDir
        $env:CIVIC_DESK_HERENOW_RECEIPT = (Join-Path $RunDir "here-now-receipt.json")
        cmd /d /c "cargo test local_herenow_anonymous_publishes_compiled_site -- --ignored --nocapture 2>&1"
      } finally {
        Remove-Item Env:\CIVIC_DESK_HERENOW_OUTPUT_DIR -ErrorAction SilentlyContinue
        Remove-Item Env:\CIVIC_DESK_HERENOW_RECEIPT -ErrorAction SilentlyContinue
        Pop-Location
      }
    }
  }

  Invoke-Check "live-colorado-source-scan" {
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      cmd /d /c "cargo test stage10_live_colorado_daily_scan_fetches_sources_first -- --ignored --nocapture 2>&1"
    } finally {
      Pop-Location
    }
  }

  if (-not $SkipLiveModel) {
    Invoke-Check "live-local-model-scan" {
      Push-Location (Join-Path $RepoRoot "src-tauri")
      try {
        $env:CIVICNEWS_STAGE10_REAL_MODEL = $Model
        cmd /d /c "cargo test stage10_live_ollama_daily_scan_completes_with_real_local_model -- --ignored --nocapture 2>&1"
      } finally {
        Remove-Item Env:\CIVICNEWS_STAGE10_REAL_MODEL -ErrorAction SilentlyContinue
        Pop-Location
      }
    }
  }

  if (-not $SkipImportFixtures) {
    Invoke-Check "source-import-fixture-extraction" {
      Push-Location (Join-Path $RepoRoot "src-tauri")
      try {
        $env:CIVICNEWS_IMPORT_FIXTURE_DIR = $FixtureDir
        $env:CIVICNEWS_IMPORT_EXTRACTED_DIR = $ExtractedDir
        cmd /d /c "cargo test local_source_import_fixtures_extract_reviewable_text -- --ignored --nocapture 2>&1"
      } finally {
        Remove-Item Env:\CIVICNEWS_IMPORT_FIXTURE_DIR -ErrorAction SilentlyContinue
        Remove-Item Env:\CIVICNEWS_IMPORT_EXTRACTED_DIR -ErrorAction SilentlyContinue
        Pop-Location
      }
    }

    Invoke-Check "source-import-fixture-review" {
      $env:CIVICNEWS_IMPORT_EXTRACTED_DIR = $ExtractedDir
      try {
        npm test -- --run src/bulkImportFixture.test.ts
      } finally {
        Remove-Item Env:\CIVICNEWS_IMPORT_EXTRACTED_DIR -ErrorAction SilentlyContinue
      }
    }
  }

  $receiptPath = Join-Path $RunDir "release-smoke-receipt.json"
  $receipt | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $receiptPath
  Write-Host "Release smoke receipt: $receiptPath"
} finally {
  Pop-Location
}
