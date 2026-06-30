param(
  [string]$ArtifactsDir = "",
  [string]$ReleaseSmokeReceipt = "",
  [string]$ModelBakeoffReceipt = "",
  [string]$DependencyAuditReceipt = "",
  [string]$InstallerSmokeReceipt = "",
  [switch]$AllowDirty,
  [switch]$AllowMissingArtifacts,
  [switch]$AllowMissingReleaseEvidence,
  [switch]$AllowSkippedSmoke
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$Stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$RunDir = Join-Path $RepoRoot ".agent-runs\release-candidate-$Stamp"
New-Item -ItemType Directory -Force -Path $RunDir | Out-Null

Push-Location $RepoRoot
try {
  $status = git status --porcelain
  if ($status -and -not $AllowDirty) {
    throw "Working tree is dirty. Commit or rerun with -AllowDirty for a diagnostic RC receipt."
  }

  if ([string]::IsNullOrWhiteSpace($ArtifactsDir)) {
    $ArtifactsDir = Join-Path $RepoRoot "src-tauri\target\release\bundle"
  }

  if ([string]::IsNullOrWhiteSpace($ReleaseSmokeReceipt)) {
    throw "ReleaseSmokeReceipt is required for RC evidence. Run scripts\release-smoke.ps1 first and pass its release-smoke-receipt.json path."
  }
  $resolvedSmokeReceipt = Resolve-Path -ErrorAction Stop $ReleaseSmokeReceipt
  $smoke = Get-Content -Raw -LiteralPath $resolvedSmokeReceipt.Path | ConvertFrom-Json
  $headCommit = (git rev-parse HEAD).Trim()
  $smokeRepoPath = if ($smoke.repo) { (Resolve-Path -ErrorAction Stop $smoke.repo).Path } else { "" }
  if ($smokeRepoPath -ne $RepoRoot.Path) {
    throw "Release smoke receipt repo does not match this checkout. Smoke repo: $smokeRepoPath; current repo: $($RepoRoot.Path)"
  }
  if ($smoke.commit -ne $headCommit) {
    throw "Release smoke receipt commit $($smoke.commit) does not match current HEAD $headCommit."
  }
  if (($smoke.dirty -or $smoke.allow_dirty -or -not $smoke.stable_mode) -and -not $AllowDirty) {
    throw "Release smoke receipt is diagnostic (dirty=$($smoke.dirty), allow_dirty=$($smoke.allow_dirty), stable_mode=$($smoke.stable_mode)). Rerun smoke in stable clean mode or use -AllowDirty for a diagnostic RC receipt."
  }
  $failedSmoke = @($smoke.checks | Where-Object { $_ -and -not $_.ok -and -not $_.skipped })
  $skippedSmoke = @($smoke.skipped | Where-Object { $_ })
  if ($failedSmoke.Count -gt 0) {
    throw "Release smoke receipt contains failed checks: $($failedSmoke.name -join ', ')"
  }
  if ($skippedSmoke.Count -gt 0 -and -not $AllowSkippedSmoke) {
    throw "Release smoke receipt contains skipped checks: $($skippedSmoke.name -join ', '). Rerun smoke without skips or use -AllowSkippedSmoke for a diagnostic receipt."
  }

  $missingReleaseEvidence = @()
  $resolvedModelBakeoffReceipt = $null
  $resolvedDependencyAuditReceipt = $null
  $resolvedInstallerSmokeReceipt = $null
  if ([string]::IsNullOrWhiteSpace($ModelBakeoffReceipt)) {
    $missingReleaseEvidence += "model bakeoff receipt"
  } else {
    $resolvedModelBakeoffReceipt = Resolve-Path -ErrorAction Stop $ModelBakeoffReceipt
  }
  if ([string]::IsNullOrWhiteSpace($DependencyAuditReceipt)) {
    $missingReleaseEvidence += "dependency audit receipt"
  } else {
    $resolvedDependencyAuditReceipt = Resolve-Path -ErrorAction Stop $DependencyAuditReceipt
  }
  if ([string]::IsNullOrWhiteSpace($InstallerSmokeReceipt)) {
    $missingReleaseEvidence += "Windows installer smoke receipt"
  } else {
    $resolvedInstallerSmokeReceipt = Resolve-Path -ErrorAction Stop $InstallerSmokeReceipt
  }
  if ($missingReleaseEvidence.Count -gt 0 -and -not $AllowMissingReleaseEvidence) {
    throw "Missing required RC evidence: $($missingReleaseEvidence -join ', '). Pass -ModelBakeoffReceipt, -DependencyAuditReceipt, and -InstallerSmokeReceipt, or use -AllowMissingReleaseEvidence for a diagnostic receipt."
  }

  $package = Get-Content -Raw (Join-Path $RepoRoot "package.json") | ConvertFrom-Json
  $tauri = Get-Content -Raw (Join-Path $RepoRoot "src-tauri\tauri.conf.json") | ConvertFrom-Json
  $modelsConfig = Get-Content -Raw (Join-Path $RepoRoot "src\models.json") | ConvertFrom-Json

  $modelBakeoffOk = $false
  $dependencyAuditOk = $false
  $installerSmokeOk = $false
  if ($resolvedModelBakeoffReceipt) {
    $modelBakeoff = Get-Content -Raw -LiteralPath $resolvedModelBakeoffReceipt.Path | ConvertFrom-Json
    $defaultModel = [string]$modelsConfig.high
    $defaultResults = @($modelBakeoff.results | Where-Object { $_.model -eq $defaultModel })
    if ($defaultResults.Count -eq 0) {
      throw "Model bakeoff receipt does not include the configured default model '$defaultModel'."
    }
    $failedDefaultCases = @($defaultResults | Where-Object { -not $_.ok })
    if ($failedDefaultCases.Count -gt 0) {
      $failedNames = ($failedDefaultCases | ForEach-Object { "$($_.case): $($_.error)" }) -join "; "
      throw "Configured default model '$defaultModel' failed bakeoff case(s): $failedNames"
    }
    $modelBakeoffOk = $true
  }

  if ($resolvedDependencyAuditReceipt) {
    $dependencyAudit = Get-Content -Raw -LiteralPath $resolvedDependencyAuditReceipt.Path | ConvertFrom-Json
    if (-not $dependencyAudit.ok) {
      throw "Dependency audit receipt is not clean. npm_exit=$($dependencyAudit.npm_audit_exit); cargo_exit=$($dependencyAudit.cargo_audit_exit)."
    }
    $npmAuditLevel = if ($dependencyAudit.npm_audit_level) { [string]$dependencyAudit.npm_audit_level } else { "unknown" }
    if ($npmAuditLevel -notin @("low", "moderate")) {
      throw "Dependency audit receipt used npm audit level '$npmAuditLevel'. Release evidence requires 'moderate' or stricter."
    }
    if ($dependencyAudit.cargo_audit_available -ne $true) {
      throw "Dependency audit receipt did not run cargo-audit. Install cargo-audit or provide a clean receipt that includes Rust advisory checking."
    }
    $dependencyAuditOk = $true
  }

  if ($resolvedInstallerSmokeReceipt) {
    $installerSmoke = Get-Content -Raw -LiteralPath $resolvedInstallerSmokeReceipt.Path | ConvertFrom-Json
    if (-not $installerSmoke.ok) {
      throw "Windows installer smoke receipt is not clean."
    }
    $failedInstallerChecks = @($installerSmoke.checks | Where-Object { $_ -and -not $_.ok })
    if ($failedInstallerChecks.Count -gt 0) {
      throw "Windows installer smoke receipt contains failed checks: $($failedInstallerChecks.name -join ', ')"
    }
    $requiredInstallerChecks = @(
      "nsis-silent-install-exit",
      "nsis-installed-exe-present",
      "nsis-installed-app-starts",
      "nsis-installed-first-run-screenshot",
      "nsis-installed-isolated-db-created",
      "nsis-installed-dependency-absent-profile",
      "nsis-uninstaller-present",
      "nsis-silent-uninstall-exit",
      "nsis-uninstall-removes-installed-exe",
      "nsis-uninstall-leaves-no-install-root-files"
    )
    $presentInstallerChecks = @($installerSmoke.checks | ForEach-Object { $_.name })
    $missingInstallerChecks = @($requiredInstallerChecks | Where-Object { $_ -notin $presentInstallerChecks })
    if ($missingInstallerChecks.Count -gt 0) {
      throw "Windows installer smoke receipt is missing required checks: $($missingInstallerChecks -join ', ')"
    }
    $installerSmokeOk = $true
  }

  if ($package.version -ne $tauri.version) {
    throw "package.json version $($package.version) does not match Tauri version $($tauri.version)."
  }
  $artifactRoot = Resolve-Path -ErrorAction SilentlyContinue $ArtifactsDir
  $allArtifacts = @()
  if ($artifactRoot) {
    $allArtifacts = Get-ChildItem -Path $artifactRoot -Recurse -File |
      Where-Object { $_.Extension -in ".exe", ".msi", ".zip", ".sig", ".txt" } |
      ForEach-Object {
        $hash = Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName
        [ordered]@{
          name = $_.Name
          path = $_.FullName
          bytes = $_.Length
          sha256 = $hash.Hash
        }
      }
  }
  $currentVersionPattern = "(^|[^0-9])$([regex]::Escape([string]$package.version))([^0-9]|$)"
  $currentArtifacts = @($allArtifacts | Where-Object { $_.name -match $currentVersionPattern })
  $staleArtifacts = @($allArtifacts | Where-Object { $_.name -notmatch $currentVersionPattern })
  $artifacts = $currentArtifacts
  $configuredTargets = @($tauri.bundle.targets | Where-Object { $_ })
  $missingCurrentTargets = @()
  foreach ($target in $configuredTargets) {
    $targetName = ([string]$target).Trim().ToLowerInvariant()
    if ([string]::IsNullOrWhiteSpace($targetName)) { continue }
    $hasTargetArtifact = $false
    foreach ($artifact in $currentArtifacts) {
      $artifactPath = ([string]$artifact.path).ToLowerInvariant()
      $artifactName = ([string]$artifact.name).ToLowerInvariant()
      if (
        $artifactPath.Contains("\$targetName\") -or
        ($targetName -eq "msi" -and $artifactName.EndsWith(".msi")) -or
        ($targetName -eq "nsis" -and $artifactName.EndsWith(".exe"))
      ) {
        $hasTargetArtifact = $true
        break
      }
    }
    if (-not $hasTargetArtifact) {
      $missingCurrentTargets += $targetName
    }
  }
  if ($artifacts.Count -eq 0 -and -not $AllowMissingArtifacts) {
    $staleNames = if ($staleArtifacts.Count) { " Stale artifacts found: $($staleArtifacts.name -join ', ')." } else { "" }
    throw "No current-version ($($package.version)) installer artifacts found under $ArtifactsDir.$staleNames Build Windows installers before creating RC evidence, or use -AllowMissingArtifacts for a diagnostic receipt."
  }
  if ($missingCurrentTargets.Count -gt 0 -and -not $AllowMissingArtifacts) {
    throw "Missing current-version ($($package.version)) artifact(s) for configured bundle target(s): $($missingCurrentTargets -join ', '). Build every configured target or use -AllowMissingArtifacts for a diagnostic receipt."
  }

  $shaFile = Join-Path $RunDir "SHA256SUMS"
  "# SHA256 checksums for CivicNewspaper RC evidence" | Set-Content -Encoding ASCII $shaFile
  if ($artifacts.Count -eq 0) {
    "# No installer artifacts were present. This is diagnostic evidence only." | Add-Content -Encoding ASCII $shaFile
  }
  foreach ($artifact in $artifacts) {
    "$($artifact.sha256.ToLowerInvariant())  $($artifact.name)" | Add-Content -Encoding ASCII $shaFile
  }

  $receipt = [ordered]@{
    generated_at = (Get-Date).ToString("o")
    repo = $RepoRoot.Path
    branch = (git branch --show-current).Trim()
    commit = $headCommit
    dirty = [bool]$status
    package_version = $package.version
    tauri_version = $tauri.version
    tauri_bundle_targets = $tauri.bundle.targets
    artifacts_dir = if ($artifactRoot) { $artifactRoot.Path } else { $ArtifactsDir }
    artifact_count = $artifacts.Count
    artifacts = $artifacts
    stale_artifact_count = $staleArtifacts.Count
    stale_artifacts = $staleArtifacts
    missing_current_targets = $missingCurrentTargets
    sha256sums_path = $shaFile
    release_smoke_receipt = $resolvedSmokeReceipt.Path
    release_smoke_checks = $smoke.checks
    release_smoke_skipped = $smoke.skipped
    model_bakeoff_receipt = if ($resolvedModelBakeoffReceipt) { $resolvedModelBakeoffReceipt.Path } else { $null }
    model_bakeoff_ok = $modelBakeoffOk
    dependency_audit_receipt = if ($resolvedDependencyAuditReceipt) { $resolvedDependencyAuditReceipt.Path } else { $null }
    dependency_audit_ok = $dependencyAuditOk
    installer_smoke_receipt = if ($resolvedInstallerSmokeReceipt) { $resolvedInstallerSmokeReceipt.Path } else { $null }
    installer_smoke_ok = $installerSmokeOk
    missing_release_evidence = $missingReleaseEvidence
    diagnostic = [bool]($AllowDirty -or $AllowMissingArtifacts -or $AllowMissingReleaseEvidence -or $AllowSkippedSmoke -or $status -or $smoke.dirty -or $smoke.allow_dirty -or -not $smoke.stable_mode -or $missingCurrentTargets.Count -gt 0 -or $missingReleaseEvidence.Count -gt 0)
    notes = @(
      "Windows public beta evidence only.",
      "Mac and Linux installer proof is backlog/proof-needed.",
      "Unsigned installer status must remain visible in release notes and setup docs."
    )
  }

  $receiptPath = Join-Path $RunDir "release-candidate-receipt.json"
  $receipt | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $receiptPath
  Write-Host "Release candidate receipt: $receiptPath"
  if ($artifacts.Count -eq 0) {
    Write-Warning "No installer artifacts found under $ArtifactsDir. Build installers before treating this as release evidence."
  } else {
    Write-Host "SHA256SUMS: $shaFile"
  }
} finally {
  Pop-Location
}
