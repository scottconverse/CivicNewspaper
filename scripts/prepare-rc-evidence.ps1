param(
  [string]$ArtifactsDir = "",
  [string]$ReleaseSmokeReceipt = "",
  [string]$ModelBakeoffReceipt = "",
  [string]$DependencyAuditReceipt = "",
  [string]$InstallerSmokeReceipt = "",
  [string]$PackagedWalkthroughReceipt = "",
  [switch]$AllowDirty,
  [switch]$AllowMissingArtifacts,
  [switch]$AllowMissingReleaseEvidence,
  [switch]$AllowSkippedSmoke,
  [switch]$AllowStaleArtifacts
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
  $requiredUiResults = @(
    "keyboard-tab-focus-visible",
    "visible-controls-have-accessible-names",
    "primary-text-contrast-aa"
  )
  $uiSmokeResults = @($smoke.ui_smoke_results | Where-Object { $_ } | ForEach-Object { $_.name })
  $missingUiResults = @($requiredUiResults | Where-Object { $_ -notin $uiSmokeResults })
  if ($missingUiResults.Count -gt 0) {
    throw "Release smoke receipt is missing required UI accessibility evidence: $($missingUiResults -join ', '). Rerun scripts\release-smoke.ps1 with the current UI smoke."
  }

  $missingReleaseEvidence = @()
  $resolvedModelBakeoffReceipt = $null
  $resolvedDependencyAuditReceipt = $null
  $resolvedInstallerSmokeReceipt = $null
  $resolvedPackagedWalkthroughReceipt = $null
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
  if ([string]::IsNullOrWhiteSpace($PackagedWalkthroughReceipt)) {
    $missingReleaseEvidence += "packaged first-run walkthrough receipt"
  } else {
    $resolvedPackagedWalkthroughReceipt = Resolve-Path -ErrorAction Stop $PackagedWalkthroughReceipt
  }
  if ($missingReleaseEvidence.Count -gt 0 -and -not $AllowMissingReleaseEvidence) {
    throw "Missing required RC evidence: $($missingReleaseEvidence -join ', '). Pass -ModelBakeoffReceipt, -DependencyAuditReceipt, -InstallerSmokeReceipt, and -PackagedWalkthroughReceipt, or use -AllowMissingReleaseEvidence for a diagnostic receipt."
  }

  $package = Get-Content -Raw (Join-Path $RepoRoot "package.json") | ConvertFrom-Json
  $tauri = Get-Content -Raw (Join-Path $RepoRoot "src-tauri\tauri.conf.json") | ConvertFrom-Json
  $modelsConfig = Get-Content -Raw (Join-Path $RepoRoot "src\models.json") | ConvertFrom-Json

  $modelBakeoffOk = $false
  [string[]]$modelBakeoffDefaultRepairedCases = @()
  $dependencyAuditOk = $false
  $dependencyAuditIgnoredAdvisories = @()
  $dependencyAuditWaivers = @()
  $dependencyAuditWaiverPath = $null
  $installerSmokeOk = $false
  $installerSmoke = $null
  $installerSmokeMatchedArtifacts = @()
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
    [string[]]$modelBakeoffDefaultRepairedCases = @(
      $defaultResults |
        Where-Object { $_.status -eq "repaired" -or $_.repaired -eq $true } |
        ForEach-Object { [string]$_.case }
    )
    if ($modelBakeoffDefaultRepairedCases.Count -gt 0) {
      throw "Configured default model '$defaultModel' required JSON repair for bakeoff case(s): $($modelBakeoffDefaultRepairedCases -join ', '). Release evidence requires clean first-pass structured output."
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
    $dependencyAuditIgnoredAdvisories = @($dependencyAudit.cargo_audit_ignored_advisories | Where-Object { $_ } | ForEach-Object { [string]$_ })
    $dependencyAuditWaivers = @($dependencyAudit.cargo_audit_waivers | Where-Object { $_ })
    $dependencyAuditWaiverPath = if ($dependencyAudit.cargo_audit_waiver_path) { [string]$dependencyAudit.cargo_audit_waiver_path } else { $null }
    if ($dependencyAuditIgnoredAdvisories.Count -gt 0) {
      if (-not $dependencyAuditWaiverPath) {
        throw "Dependency audit receipt has ignored RustSec advisories but no waiver file path."
      }
      if ($dependencyAuditWaivers.Count -ne $dependencyAuditIgnoredAdvisories.Count) {
        throw "Dependency audit receipt has $($dependencyAuditIgnoredAdvisories.Count) ignored RustSec advisories but $($dependencyAuditWaivers.Count) waiver entries."
      }
      $ignoredIds = @($dependencyAuditIgnoredAdvisories | ForEach-Object { ([string]$_).Trim() } | Where-Object { $_ })
      $waiverIds = @($dependencyAuditWaivers | ForEach-Object { ([string]$_.id).Trim() } | Where-Object { $_ })
      $duplicateWaiverIds = @($waiverIds | Group-Object | Where-Object { $_.Count -gt 1 } | ForEach-Object { $_.Name })
      $missingWaiverIds = @($ignoredIds | Where-Object { $_ -notin $waiverIds })
      $extraWaiverIds = @($waiverIds | Where-Object { $_ -notin $ignoredIds })
      if ($duplicateWaiverIds.Count -gt 0 -or $missingWaiverIds.Count -gt 0 -or $extraWaiverIds.Count -gt 0) {
        throw "Dependency audit waiver IDs do not exactly match ignored RustSec advisory IDs. Missing: $($missingWaiverIds -join ', '); extra: $($extraWaiverIds -join ', '); duplicate: $($duplicateWaiverIds -join ', ')."
      }
      $today = (Get-Date).Date
      foreach ($waiver in $dependencyAuditWaivers) {
        foreach ($field in @("id", "crate", "source", "rationale", "release_note", "owner", "review_by")) {
          if ([string]::IsNullOrWhiteSpace([string]$waiver.$field)) {
            throw "Dependency audit waiver for '$($waiver.id)' is missing required field '$field'."
          }
        }
        try {
          $reviewBy = Get-Date -Date ([string]$waiver.review_by) -ErrorAction Stop
        } catch {
          $reviewBy = $null
        }
        if (-not $reviewBy -or $reviewBy.Date -lt $today) {
          throw "Dependency audit waiver '$($waiver.id)' is expired or has an invalid review_by date: $($waiver.review_by)."
        }
      }
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
    if ($installerSmoke.commit -ne $headCommit) {
      throw "Windows installer smoke receipt commit $($installerSmoke.commit) does not match current HEAD $headCommit."
    }
    if (($installerSmoke.dirty -eq $true) -and -not $AllowDirty) {
      throw "Windows installer smoke receipt was produced from a dirty checkout. Rerun installer smoke from a clean checkout or use -AllowDirty for a diagnostic RC receipt."
    }
    $installerSmokeOk = $true
  }

  $packagedWalkthroughOk = $false
  $packagedWalkthrough = $null
  if ($resolvedPackagedWalkthroughReceipt) {
    $packagedWalkthrough = Get-Content -Raw -LiteralPath $resolvedPackagedWalkthroughReceipt.Path | ConvertFrom-Json
    if (-not $packagedWalkthrough.ok) {
      throw "Packaged first-run walkthrough receipt is not clean."
    }
    if ($packagedWalkthrough.commit -ne $headCommit) {
      throw "Packaged first-run walkthrough receipt commit $($packagedWalkthrough.commit) does not match current HEAD $headCommit."
    }
    if (($packagedWalkthrough.dirty -eq $true) -and -not $AllowDirty) {
      throw "Packaged first-run walkthrough receipt was produced from a dirty checkout. Rerun from a clean checkout or use -AllowDirty for a diagnostic RC receipt."
    }
    $failedWalkthroughChecks = @($packagedWalkthrough.checks | Where-Object { $_ -and -not $_.ok })
    if ($failedWalkthroughChecks.Count -gt 0) {
      throw "Packaged first-run walkthrough receipt contains failed checks: $($failedWalkthroughChecks.name -join ', ')"
    }
    $requiredWalkthroughChecks = @(
      "forced-ollama-base-url-absent",
      "nsis-silent-install",
      "installed-exe-present",
      "window-handle-present",
      "webview-cdp-ready",
      "first-run-webview-driver",
      "webview-build-id-matches-commit",
      "webview-dependency-absent-choice-visible",
      "webview-workspace-reached",
      "webview-zero-source-guidance-usable",
      "webview-07-first-run-workspace",
      "sqlite-db-present",
      "finish-onboarding-complete",
      "ai-setup-skipped",
      "zero-source-first-run",
      "setting-paths.publish-isolated",
      "setting-paths.backup-isolated",
      "setting-onboarding_complete",
      "packaged-local-api-ready",
      "packaged-live-pairing-positive",
      "packaged-live-protected-queue",
      "downloaded-runtime-absent",
      "core-flow-webview-cdp-ready",
      "core-flow-webview-driver",
      "core-live-model-ready",
      "core-controlled-sources-added",
      "core-daily-scan-created-linked-lead",
      "core-draft-persisted-through-command",
      "core-draft-reloaded-in-workbench",
      "core-03-core-flow-reloaded-workbench",
      "core-flow-database-persistence",
      "nsis-silent-uninstall"
    )
    $presentWalkthroughChecks = @($packagedWalkthrough.checks | ForEach-Object { $_.name })
    $missingWalkthroughChecks = @($requiredWalkthroughChecks | Where-Object { $_ -notin $presentWalkthroughChecks })
    if ($missingWalkthroughChecks.Count -gt 0) {
      throw "Packaged first-run walkthrough receipt is missing required completion checks: $($missingWalkthroughChecks -join ', ')"
    }
    $workspaceCheck = @($packagedWalkthrough.checks | Where-Object { $_.name -eq "webview-07-first-run-workspace" } | Select-Object -First 1)
    if ($workspaceCheck.Count -eq 0 -or -not (Test-Path -LiteralPath ([string]$workspaceCheck[0].details.path))) {
      throw "Packaged first-run walkthrough workspace screenshot is missing on disk."
    }
    if ([string]::IsNullOrWhiteSpace([string]$packagedWalkthrough.forced_ollama_base_url)) {
      throw "Packaged first-run walkthrough receipt is missing forced_ollama_base_url."
    }
    if (-not $packagedWalkthrough.core_flow) {
      throw "Packaged walkthrough receipt does not include the required live-model core flow."
    }
    if ([string]::IsNullOrWhiteSpace([string]$packagedWalkthrough.installer_sha256) -or -not $packagedWalkthrough.installer_size) {
      throw "Packaged walkthrough receipt is missing installer SHA256 or size."
    }
    if (-not (Test-Path -LiteralPath "$($resolvedPackagedWalkthroughReceipt.Path).sha256")) {
      throw "Packaged walkthrough receipt is missing its SHA256 sidecar."
    }
    $packagedWalkthroughOk = $true
  }

  if ($package.version -ne $tauri.version) {
    throw "package.json version $($package.version) does not match Tauri version $($tauri.version)."
  }
  $frontendBuildIdPath = Join-Path $RepoRoot "dist\build-id.txt"
  if (-not (Test-Path -LiteralPath $frontendBuildIdPath)) {
    throw "Missing dist\build-id.txt. Run npm run build before bundling release artifacts."
  }
  $frontendBuildId = (Get-Content -Raw -LiteralPath $frontendBuildIdPath).Trim()
  if ($frontendBuildId -ne $headCommit) {
    throw "dist\build-id.txt contains '$frontendBuildId' but current HEAD is $headCommit. Rebuild with npm run build before bundling."
  }
  $frontendBuildIdTimeUtc = (Get-Item -LiteralPath $frontendBuildIdPath).LastWriteTimeUtc
  $headCommitTimeUtc = [DateTime]::Parse((git show -s --format=%cI HEAD)).ToUniversalTime()
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
          last_write_time_utc = $_.LastWriteTimeUtc.ToString("o")
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
  if ($staleArtifacts.Count -gt 0 -and -not $AllowStaleArtifacts) {
    throw "Stale installer artifact(s) found under ${ArtifactsDir}: $($staleArtifacts.name -join ', '). Remove old bundle artifacts before creating release evidence, or use -AllowStaleArtifacts for a diagnostic receipt."
  }
  $preCommitArtifacts = @($artifacts | Where-Object { [DateTime]::Parse([string]$_.last_write_time_utc).ToUniversalTime() -lt $headCommitTimeUtc })
  if ($preCommitArtifacts.Count -gt 0 -and -not $AllowStaleArtifacts) {
    throw "Installer artifact(s) predate current HEAD commit time: $($preCommitArtifacts.name -join ', '). Rebuild the installer from the audited commit."
  }

  if ($packagedWalkthroughOk -and $artifacts.Count -gt 0) {
    $walkthroughHash = ([string]$packagedWalkthrough.installer_sha256).Trim().ToUpperInvariant()
    $walkthroughSize = [int64]$packagedWalkthrough.installer_size
    $walkthroughArtifacts = @($artifacts | Where-Object {
      ([string]$_.sha256).ToUpperInvariant() -eq $walkthroughHash -and [int64]$_.bytes -eq $walkthroughSize
    })
    if ($walkthroughArtifacts.Count -eq 0) {
      throw "Current release artifact does not match the packaged walkthrough receipt SHA256 and size."
    }
  }

  if ($installerSmokeOk -and $artifacts.Count -gt 0) {
    $smokedHashes = @()
    if ($installerSmoke.nsis_installer_sha256) {
      $smokedHashes += [string]$installerSmoke.nsis_installer_sha256
    }
    if ($installerSmoke.msi_installer_sha256) {
      $smokedHashes += [string]$installerSmoke.msi_installer_sha256
    }
    $smokedHashes = @($smokedHashes | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | ForEach-Object { $_.Trim().ToUpperInvariant() })
    if ($smokedHashes.Count -eq 0) {
      throw "Windows installer smoke receipt does not include installer SHA256 digest(s). Rerun scripts\windows-installer-smoke.ps1 with the current release scripts."
    }
    foreach ($smokedHash in $smokedHashes) {
      $matched = @($artifacts | Where-Object { ([string]$_.sha256).ToUpperInvariant() -eq $smokedHash })
      if ($matched.Count -eq 0) {
        throw "Windows installer smoke receipt hash $smokedHash does not match any current release artifact hash."
      }
      $installerSmokeMatchedArtifacts += $matched
    }
  }

  $shaFile = Join-Path $RunDir "SHA256SUMS"
  "# SHA256 checksums for CivicNewspaper RC evidence" | Set-Content -Encoding ASCII $shaFile
  if ($artifacts.Count -eq 0) {
    "# No installer artifacts were present. This is diagnostic evidence only." | Add-Content -Encoding ASCII $shaFile
  }
  foreach ($artifact in $artifacts) {
    "$($artifact.sha256.ToLowerInvariant())  $($artifact.name)" | Add-Content -Encoding ASCII $shaFile
  }

  $evidenceDir = Join-Path $RunDir "evidence"
  New-Item -ItemType Directory -Force -Path $evidenceDir | Out-Null
  $evidenceBundles = [System.Collections.Generic.List[object]]::new()
  function Resolve-RelativePath {
    param(
      [string]$BaseDir,
      [string]$Path
    )
    $baseFull = [System.IO.Path]::GetFullPath($BaseDir)
    if (-not $baseFull.EndsWith([System.IO.Path]::DirectorySeparatorChar)) {
      $baseFull += [System.IO.Path]::DirectorySeparatorChar
    }
    $baseUri = [System.Uri]::new($baseFull)
    $pathUri = [System.Uri]::new([System.IO.Path]::GetFullPath($Path))
    return [System.Uri]::UnescapeDataString($baseUri.MakeRelativeUri($pathUri).ToString()).Replace('/', [System.IO.Path]::DirectorySeparatorChar)
  }
  function Copy-EvidenceBundle {
    param(
      [string]$ReceiptPath,
      [string]$Name
    )
    if ([string]::IsNullOrWhiteSpace($ReceiptPath) -or -not (Test-Path -LiteralPath $ReceiptPath)) {
      return
    }
    $sourceDir = Split-Path -Parent $ReceiptPath
    $destDir = Join-Path $evidenceDir $Name
    New-Item -ItemType Directory -Force -Path $destDir | Out-Null
    $copied = @()
    $agentRunsDir = Join-Path $RepoRoot ".agent-runs"
    $sourceFiles = if (
      [System.IO.Path]::GetFullPath($sourceDir).TrimEnd([System.IO.Path]::DirectorySeparatorChar) -eq
      [System.IO.Path]::GetFullPath($agentRunsDir).TrimEnd([System.IO.Path]::DirectorySeparatorChar)
    ) {
      @(Get-Item -LiteralPath $ReceiptPath)
    } else {
      @(Get-ChildItem -LiteralPath $sourceDir -Recurse -File)
    }
    $sourceFiles |
      Where-Object {
        $_.FullName -notmatch "\\install\\" -and
        $_.FullName -notmatch "\\app-data\\" -and
        $_.FullName -notlike "$RunDir*"
      } |
      ForEach-Object {
        $relative = Resolve-RelativePath -BaseDir $sourceDir -Path $_.FullName
        $target = Join-Path $destDir $relative
        New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
        Copy-Item -LiteralPath $_.FullName -Destination $target -Force
        $copied += [ordered]@{
          source = $_.FullName
          copied_to = $target
          bytes = $_.Length
          sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $target).Hash
        }
      }
    $evidenceBundles.Add([ordered]@{
      name = $Name
      source_dir = $sourceDir
      copied_file_count = $copied.Count
      files = $copied
    }) | Out-Null
  }

  Copy-EvidenceBundle -ReceiptPath $resolvedSmokeReceipt.Path -Name "release-smoke"
  if ($smoke.ui_smoke_receipt) { Copy-EvidenceBundle -ReceiptPath ([string]$smoke.ui_smoke_receipt) -Name "ui-smoke" }
  if ($resolvedModelBakeoffReceipt) { Copy-EvidenceBundle -ReceiptPath $resolvedModelBakeoffReceipt.Path -Name "model-bakeoff" }
  if ($resolvedDependencyAuditReceipt) { Copy-EvidenceBundle -ReceiptPath $resolvedDependencyAuditReceipt.Path -Name "dependency-audit" }
  if ($resolvedInstallerSmokeReceipt) { Copy-EvidenceBundle -ReceiptPath $resolvedInstallerSmokeReceipt.Path -Name "windows-installer-smoke" }
  if ($resolvedPackagedWalkthroughReceipt) { Copy-EvidenceBundle -ReceiptPath $resolvedPackagedWalkthroughReceipt.Path -Name "packaged-first-run-walkthrough" }

  $receipt = [ordered]@{
    generated_at = (Get-Date).ToString("o")
    repo = $RepoRoot.Path
    branch = (git branch --show-current).Trim()
    commit = $headCommit
    dirty = [bool]$status
    package_version = $package.version
    tauri_version = $tauri.version
    tauri_bundle_targets = $tauri.bundle.targets
    frontend_build_id_path = $frontendBuildIdPath
    frontend_build_id = $frontendBuildId
    frontend_build_id_time_utc = $frontendBuildIdTimeUtc.ToString("o")
    artifacts_dir = if ($artifactRoot) { $artifactRoot.Path } else { $ArtifactsDir }
    artifact_count = $artifacts.Count
    artifacts = $artifacts
    evidence_dir = $evidenceDir
    evidence_bundles = @($evidenceBundles)
    stale_artifact_count = $staleArtifacts.Count
    stale_artifacts = $staleArtifacts
    missing_current_targets = $missingCurrentTargets
    sha256sums_path = $shaFile
    release_smoke_receipt = $resolvedSmokeReceipt.Path
    release_smoke_checks = $smoke.checks
    release_smoke_skipped = $smoke.skipped
    model_bakeoff_receipt = if ($resolvedModelBakeoffReceipt) { $resolvedModelBakeoffReceipt.Path } else { $null }
    model_bakeoff_ok = $modelBakeoffOk
    model_bakeoff_default_repaired_cases = @($modelBakeoffDefaultRepairedCases)
    dependency_audit_receipt = if ($resolvedDependencyAuditReceipt) { $resolvedDependencyAuditReceipt.Path } else { $null }
    dependency_audit_ok = $dependencyAuditOk
    dependency_audit_ignored_advisories = $dependencyAuditIgnoredAdvisories
    dependency_audit_waiver_path = $dependencyAuditWaiverPath
    dependency_audit_waivers = $dependencyAuditWaivers
    installer_smoke_receipt = if ($resolvedInstallerSmokeReceipt) { $resolvedInstallerSmokeReceipt.Path } else { $null }
    installer_smoke_ok = $installerSmokeOk
    installer_smoke_commit = if ($installerSmoke) { $installerSmoke.commit } else { $null }
    installer_smoke_dirty = if ($installerSmoke) { $installerSmoke.dirty } else { $null }
    installer_smoke_nsis_sha256 = if ($installerSmoke) { $installerSmoke.nsis_installer_sha256 } else { $null }
    installer_smoke_msi_sha256 = if ($installerSmoke) { $installerSmoke.msi_installer_sha256 } else { $null }
    installer_smoke_matched_artifacts = $installerSmokeMatchedArtifacts
    packaged_walkthrough_receipt = if ($resolvedPackagedWalkthroughReceipt) { $resolvedPackagedWalkthroughReceipt.Path } else { $null }
    packaged_walkthrough_ok = $packagedWalkthroughOk
    missing_release_evidence = $missingReleaseEvidence
    diagnostic = [bool]($AllowDirty -or $AllowMissingArtifacts -or $AllowMissingReleaseEvidence -or $AllowSkippedSmoke -or $AllowStaleArtifacts -or $status -or $smoke.dirty -or $smoke.allow_dirty -or -not $smoke.stable_mode -or $missingCurrentTargets.Count -gt 0 -or $missingReleaseEvidence.Count -gt 0 -or $staleArtifacts.Count -gt 0)
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
