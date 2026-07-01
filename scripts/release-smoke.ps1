param(
  [string]$FixtureDir = "",
  [string]$Model = "phi4-mini:latest",
  [switch]$SkipLiveModel,
  [switch]$SkipHereNow,
  [switch]$SkipDesktopSmoke,
  [switch]$SkipImportFixtures,
  [switch]$AllowDirty,
  [switch]$Stable
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$RepoImportReviewDir = Join-Path $RepoRoot "test-fixtures\source-import-extracted"
if ([string]::IsNullOrWhiteSpace($FixtureDir)) {
  $FixtureDir = $RepoImportReviewDir
}
$FixtureDir = (Resolve-Path -ErrorAction Stop $FixtureDir).Path
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
  stable_mode = [bool]$Stable
  allow_dirty = [bool]$AllowDirty
  branch = $null
  commit = $null
  dirty = $null
  skipped = @()
  checks = @()
}

function Write-Receipt {
  $receiptPath = Join-Path $RunDir "release-smoke-receipt.json"
  $receipt | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $receiptPath
  Write-Host "Release smoke receipt: $receiptPath"
}

function Add-SkippedCheck {
  param(
    [string]$Name,
    [string]$Reason
  )
  $receipt.skipped += [ordered]@{
    name = $Name
    reason = $Reason
  }
  $receipt.checks += [ordered]@{
    name = $Name
    ok = $false
    skipped = $true
    seconds = 0
    reason = $Reason
  }
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
  if ($Stable -and ($SkipLiveModel -or $SkipHereNow -or $SkipImportFixtures)) {
    throw "Stable release smoke cannot skip live model, here.now, or import fixture gates."
  }

  Invoke-Check "git-status" {
    $branch = (git branch --show-current).Trim()
    $commit = (git rev-parse HEAD).Trim()
    $status = git status --porcelain
    $receipt.branch = $branch
    $receipt.commit = $commit
    $receipt.dirty = [bool]$status
    git status --short --branch
    git rev-parse HEAD
    if ($status -and -not $AllowDirty) {
      throw "Working tree is dirty. Commit/stash changes or rerun with -AllowDirty for a non-release diagnostic smoke."
    }
  }

  Invoke-Check "frontend-tests" {
    npm test -- --run
  }

  Invoke-Check "browser-ui-smoke" {
    npm run test:ui-smoke
  }

  Invoke-Check "rust-tests" {
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      cmd /d /c "cargo test --all --locked 2>&1"
    } finally {
      Pop-Location
    }
  }

  Invoke-Check "clean-profile-app-data-override" {
    Push-Location (Join-Path $RepoRoot "src-tauri")
    try {
      cmd /d /c "cargo test app_data_override -- --nocapture 2>&1"
    } finally {
      Pop-Location
    }
  }

  if ($SkipDesktopSmoke) {
    if ($Stable) {
      throw "Stable release smoke cannot skip desktop first-run smoke."
    }
    Add-SkippedCheck "desktop-first-run-loopback-smoke" "Skipped by -SkipDesktopSmoke."
  } else {
    Invoke-Check "desktop-first-run-loopback-smoke" {
      powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $RepoRoot "scripts\desktop-smoke.ps1") -RunDir (Join-Path $RunDir "desktop-smoke")
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

  if ($SkipHereNow) {
    Add-SkippedCheck "here-now-anonymous-live-publish" "Skipped by -SkipHereNow."
  } else {
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

  if ($SkipLiveModel) {
    Add-SkippedCheck "live-local-model-scan" "Skipped by -SkipLiveModel."
  } else {
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

  if ($SkipImportFixtures) {
    Add-SkippedCheck "source-import-fixture-extraction" "Skipped by -SkipImportFixtures."
    Add-SkippedCheck "source-import-fixture-review" "Skipped by -SkipImportFixtures."
  } else {
    $hasOriginalFixtures = Test-Path (Join-Path $FixtureDir "colorado-source-list-clean.csv")
    $hasReviewFixtures = Test-Path (Join-Path $FixtureDir "colorado-source-list-clean.csv.txt")
    $reviewDir = $ExtractedDir

    if ($hasOriginalFixtures) {
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
    } elseif ($hasReviewFixtures -and -not $Stable) {
      $reviewDir = $FixtureDir
      Add-SkippedCheck "source-import-fixture-extraction" "Using repo-local extracted fixture text; parser extraction is covered by per-format Rust unit tests."
    } else {
      throw "Source import fixtures not found in $FixtureDir. Stable runs require the full original fixture folder."
    }

    Invoke-Check "source-import-fixture-review" {
      $env:CIVICNEWS_IMPORT_EXTRACTED_DIR = $reviewDir
      try {
        npm test -- --run src/bulkImportFixture.test.ts
      } finally {
        Remove-Item Env:\CIVICNEWS_IMPORT_EXTRACTED_DIR -ErrorAction SilentlyContinue
      }
    }
  }
} finally {
  Write-Receipt
  Pop-Location
}
