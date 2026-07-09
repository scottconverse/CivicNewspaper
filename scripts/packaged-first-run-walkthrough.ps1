param(
  [string]$NsisInstaller = "",
  [string]$OutputDir = "",
  [string]$Python = "",
  [string]$Model = "phi4-mini:latest",
  [switch]$CompleteOnboarding,
  [switch]$CoreFlow
)

$ErrorActionPreference = "Stop"

function Resolve-RepoRoot {
  $scriptDir = Split-Path -Parent $PSCommandPath
  return (Resolve-Path (Join-Path $scriptDir "..")).Path
}

function Resolve-AppVersion {
  param([string]$Repo)
  $package = Get-Content -Raw -LiteralPath (Join-Path $Repo "package.json") | ConvertFrom-Json
  return [string]$package.version
}

function Resolve-Installer {
  param(
    [string]$ExplicitPath,
    [string]$Repo,
    [string]$Version
  )

  if ($ExplicitPath) {
    return (Resolve-Path -LiteralPath $ExplicitPath -ErrorAction Stop).Path
  }

  $bundleDir = Join-Path $Repo "src-tauri\target\release\bundle"
  $artifact = Get-ChildItem -LiteralPath $bundleDir -Recurse -Filter "*.exe" |
    Where-Object { $_.Name -like "*$Version*" } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

  if (-not $artifact) {
    throw "Could not find NSIS installer for version $Version under $bundleDir"
  }
  return $artifact.FullName
}

function Resolve-Python {
  param([string]$ExplicitPython)
  if ($ExplicitPython) {
    return (Resolve-Path -LiteralPath $ExplicitPython -ErrorAction Stop).Path
  }
  $candidates = @(
    "C:\Users\instynct\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe",
    "python.exe",
    "python"
  )
  foreach ($candidate in $candidates) {
    try {
      $cmd = Get-Command $candidate -ErrorAction Stop
      return $cmd.Source
    } catch {
      continue
    }
  }
  throw "Python was not found for SQLite verification."
}

function Test-PortFree {
  param([int]$Port)
  $client = New-Object System.Net.Sockets.TcpClient
  try {
    $iar = $client.BeginConnect("127.0.0.1", $Port, $null, $null)
    if ($iar.AsyncWaitHandle.WaitOne(300, $false)) {
      $client.EndConnect($iar)
      return $false
    }
    return $true
  } catch {
    return $true
  } finally {
    $client.Close()
  }
}

function Resolve-UnusedLoopbackPort {
  param([int]$Start = 65200, [int]$End = 65300)
  for ($port = $Start; $port -le $End; $port++) {
    if (Test-PortFree -Port $port) {
      return $port
    }
  }
  throw "Could not find an unused loopback port."
}

function Resolve-UnusedLoopbackUrl {
  return "http://127.0.0.1:$(Resolve-UnusedLoopbackPort)"
}

function Test-LoopbackAbsent {
  param([string]$Url)
  try {
    Invoke-WebRequest -Uri "$Url/api/tags" -UseBasicParsing -TimeoutSec 2 | Out-Null
    return $false
  } catch {
    return $true
  }
}

function Test-CivicApiReady {
  try {
    Invoke-WebRequest -Uri "http://127.0.0.1:12053/api/queue" -Headers @{ Host = "127.0.0.1:12053" } -UseBasicParsing -TimeoutSec 2 | Out-Null
    return $true
  } catch {
    $statusCode = if ($_.Exception.Response) { $_.Exception.Response.StatusCode.value__ } else { 0 }
    return $statusCode -eq 401
  }
}

function Test-PathUnderRoot {
  param(
    [string]$Path,
    [string]$Root
  )
  if ([string]::IsNullOrWhiteSpace($Path)) {
    return $false
  }
  $fullPath = [System.IO.Path]::GetFullPath($Path)
  $fullRoot = [System.IO.Path]::GetFullPath($Root)
  if (-not $fullRoot.EndsWith([System.IO.Path]::DirectorySeparatorChar)) {
    $fullRoot += [System.IO.Path]::DirectorySeparatorChar
  }
  return $fullPath.StartsWith($fullRoot, [System.StringComparison]::OrdinalIgnoreCase)
}

function Stop-WalkthroughProcess {
  param([System.Diagnostics.Process]$Process)
  if (-not $Process -or $Process.HasExited) {
    return
  }
  try {
    $Process.CloseMainWindow() | Out-Null
    if (-not $Process.WaitForExit(5000)) {
      $Process.Kill()
      $Process.WaitForExit(5000) | Out-Null
    }
  } catch {
    Write-Warning "Could not stop walkthrough process $($Process.Id): $_"
  }
}

$repo = Resolve-RepoRoot
$version = Resolve-AppVersion -Repo $repo
$installer = Resolve-Installer -ExplicitPath $NsisInstaller -Repo $repo -Version $version
$pythonExe = Resolve-Python -ExplicitPython $Python
$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
if (-not $OutputDir) {
  $OutputDir = Join-Path $repo ".agent-runs\packaged-first-run-walkthrough-$stamp"
} elseif (-not [System.IO.Path]::IsPathRooted($OutputDir)) {
  $OutputDir = Join-Path $repo $OutputDir
}
$OutputDir = [System.IO.Path]::GetFullPath($OutputDir)
$installRoot = Join-Path $OutputDir "install"
$appDataRoot = Join-Path $OutputDir "app-data"
$logPath = Join-Path $OutputDir "packaged-first-run-walkthrough.log"
$receiptPath = Join-Path $OutputDir "packaged-first-run-walkthrough-receipt.json"
New-Item -ItemType Directory -Force -Path $OutputDir, $installRoot, $appDataRoot | Out-Null

function Write-Log {
  param([string]$Message)
  "$(Get-Date -Format o) $Message" | Tee-Object -FilePath $logPath -Append
}

$receipt = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  repo = $repo
  commit = (git -C $repo rev-parse HEAD).Trim()
  dirty = [bool](git -C $repo status --porcelain)
  app_version = $version
  installer = $installer
  installer_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $installer).Hash
  installer_size = (Get-Item -LiteralPath $installer).Length
  machine = $env:COMPUTERNAME
  windows_version = [System.Environment]::OSVersion.VersionString
  output_dir = $OutputDir
  install_root = $installRoot
  app_data_dir = $appDataRoot
  python = $pythonExe
  forced_ollama_base_url = $null
  webview_driver = (Join-Path $repo "scripts\packaged-webview-driver.mjs")
  cdp_url = $null
  core_flow = [bool]$CoreFlow
  checks = @()
}

function Add-Check {
  param(
    [string]$Name,
    [bool]$Ok,
    [hashtable]$Details = @{}
  )
  $receipt.checks += [ordered]@{
    name = $Name
    ok = $Ok
    details = $Details
  }
}

function Add-ScreenshotCheck {
  param(
    [string]$Name,
    [string]$Path
  )

  $file = Get-Item -LiteralPath $Path -ErrorAction SilentlyContinue
  $ok = $null -ne $file -and $file.Length -gt 1024
  Add-Check $Name $ok @{
    path = $Path
    bytes = if ($file) { $file.Length } else { 0 }
  }
  if (-not $ok) {
    throw "Screenshot check failed for $Name at $Path"
  }
}

$appProcess = $null
try {
  $install = Start-Process -FilePath $installer -ArgumentList @("/S", "/D=$installRoot") -Wait -PassThru -WindowStyle Hidden
  Add-Check "nsis-silent-install" ($install.ExitCode -eq 0) @{ exit_code = $install.ExitCode }
  if ($install.ExitCode -ne 0) {
    throw "NSIS installer exited $($install.ExitCode)"
  }

  $installedExe = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" |
    Where-Object { $_.Name -notmatch "unins|uninstall" } |
    Sort-Object FullName |
    Select-Object -First 1
  Add-Check "installed-exe-present" ($null -ne $installedExe) @{ exe = if ($installedExe) { $installedExe.FullName } else { "" } }
  if (-not $installedExe) {
    throw "No installed application executable found."
  }
  $receipt.installed_exe = $installedExe.FullName
  $receipt.installed_exe_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $installedExe.FullName).Hash

  $absentOllamaBaseUrl = Resolve-UnusedLoopbackUrl
  $receipt.forced_ollama_base_url = $absentOllamaBaseUrl
  $ollamaAbsent = Test-LoopbackAbsent -Url $absentOllamaBaseUrl
  Add-Check "forced-ollama-base-url-absent" $ollamaAbsent @{ forced_ollama_base_url = $absentOllamaBaseUrl }
  if (-not $ollamaAbsent) {
    throw "Expected no Ollama service at $absentOllamaBaseUrl"
  }
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $installedExe.FullName
  $psi.WorkingDirectory = Split-Path $installedExe.FullName
  $psi.UseShellExecute = $false
  $psi.Environment["CIVICNEWS_APP_DATA_DIR"] = $appDataRoot
  $psi.Environment["CIVICNEWS_OLLAMA_BASE_URL"] = $absentOllamaBaseUrl
  $cdpPort = Resolve-UnusedLoopbackPort -Start 65301 -End 65400
  $cdpUrl = "http://127.0.0.1:$cdpPort"
  $receipt.cdp_url = $cdpUrl
  $psi.Environment["WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS"] = "--remote-debugging-port=$cdpPort"
  $appProcess = [System.Diagnostics.Process]::Start($psi)

  $deadline = (Get-Date).AddSeconds(30)
  while ((Get-Date) -lt $deadline) {
    $appProcess.Refresh()
    if ($appProcess.HasExited) {
      throw "Installed app exited early with code $($appProcess.ExitCode)"
    }
    if ($appProcess.MainWindowHandle -ne [IntPtr]::Zero) {
      break
    }
    Start-Sleep -Milliseconds 500
  }
  Add-Check "window-handle-present" ($appProcess.MainWindowHandle -ne [IntPtr]::Zero) @{ pid = $appProcess.Id }
  if ($appProcess.MainWindowHandle -eq [IntPtr]::Zero) {
    throw "Installed app did not expose a main window."
  }

  if (-not $CompleteOnboarding) {
    throw "Packaged first-run proof requires -CompleteOnboarding."
  }
  $cdpDeadline = (Get-Date).AddSeconds(30)
  $cdpReady = $false
  while ((Get-Date) -lt $cdpDeadline) {
    try {
      $targets = Invoke-RestMethod -Uri "$cdpUrl/json" -TimeoutSec 2
      if ($targets) { $cdpReady = $true; break }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  Add-Check "webview-cdp-ready" $cdpReady @{ cdp_url = $cdpUrl }
  if (-not $cdpReady) {
    throw "Installed app did not expose WebView2 CDP at $cdpUrl"
  }

  $driverScript = Join-Path $repo "scripts\packaged-webview-driver.mjs"
  & node $driverScript --cdp-url $cdpUrl --mode first-run --output-dir $OutputDir --expected-build-id $($receipt.commit)
  if ($LASTEXITCODE -ne 0) {
    throw "Packaged first-run webview driver failed with exit code $LASTEXITCODE"
  }
  $driverResultPath = Join-Path $OutputDir "first-run-webview-result.json"
  $driverResult = Get-Content -Raw -LiteralPath $driverResultPath | ConvertFrom-Json
  Add-Check "first-run-webview-driver" ([bool]$driverResult.ok) @{ result_path = $driverResultPath }
  foreach ($driverCheck in $driverResult.checks) {
    Add-Check "webview-$($driverCheck.name)" ([bool]$driverCheck.ok) @{ details = $driverCheck.details }
  }
  foreach ($screenshot in $driverResult.screenshots) {
    Add-ScreenshotCheck "webview-$([System.IO.Path]::GetFileNameWithoutExtension($screenshot))" $screenshot
  }

  $publication = "Local Beta Desk"
  $editor = "Packaged Walkthrough"
  $city = ""
  $state = ""

  $dbPath = Join-Path $appDataRoot "civicdesk.db"
  Add-Check "sqlite-db-present" (Test-Path $dbPath) @{ db_path = $dbPath }
  if (-not (Test-Path $dbPath)) {
    throw "SQLite database was not created at $dbPath"
  }

  $settingsJsonPath = Join-Path $OutputDir "settings-after-identity.json"
  $queryScript = @"
import json, sqlite3, sys
db_path, out_path = sys.argv[1], sys.argv[2]
conn = sqlite3.connect(db_path)
rows = dict(conn.execute("select key, value from settings"))
rows["__source_count"] = str(conn.execute("select count(*) from sources").fetchone()[0])
with open(out_path, "w", encoding="utf-8") as f:
    json.dump(rows, f, indent=2, sort_keys=True)
"@
  $queryScriptPath = Join-Path $OutputDir "read-settings.py"
  Set-Content -LiteralPath $queryScriptPath -Value $queryScript -Encoding UTF8
  & $pythonExe $queryScriptPath $dbPath $settingsJsonPath
  if ($LASTEXITCODE -ne 0) {
    throw "SQLite settings read failed with exit code $LASTEXITCODE"
  }
  $settings = Get-Content -Raw -LiteralPath $settingsJsonPath | ConvertFrom-Json

  $onboardingComplete = [string]$settings.PSObject.Properties["onboarding_complete"].Value
  $aiSetupSkipped = [string]$settings.PSObject.Properties["ai.setup_skipped"].Value
  $sourceCount = [int]([string]$settings.PSObject.Properties["__source_count"].Value)
  Add-Check "finish-onboarding-complete" ($onboardingComplete -eq "1" -or $onboardingComplete -eq "true") @{ onboarding_complete = $onboardingComplete }
  Add-Check "ai-setup-skipped" ($aiSetupSkipped -eq "true") @{ ai_setup_skipped = $aiSetupSkipped }
  Add-Check "zero-source-first-run" ($sourceCount -eq 0) @{ source_count = $sourceCount }
  if (($onboardingComplete -ne "1" -and $onboardingComplete -ne "true") -or $aiSetupSkipped -ne "true" -or $sourceCount -ne 0) {
    throw "First-run database state does not match the dependency-absent walkthrough."
  }

  $identityChecks = @{
    "identity.newsroom_name" = $publication
    "identity.editor_name" = $editor
    "identity.city" = $city
    "identity.state" = $state
  }
  foreach ($key in $identityChecks.Keys) {
    $actual = [string]$settings.PSObject.Properties[$key].Value
    $expected = [string]$identityChecks[$key]
    Add-Check "setting-$key" ($actual -eq $expected) @{ expected = $expected; actual = $actual }
    if ($actual -ne $expected) {
      throw "Expected setting $key='$expected', got '$actual'"
    }
  }

  $onboardingStep = [string]$settings.PSObject.Properties["onboarding.step"].Value
  Add-Check "setting-onboarding.step-advanced" ($onboardingStep -ne "" -and [int]$onboardingStep -ge 2) @{ actual = $onboardingStep }
  if ($onboardingStep -eq "" -or [int]$onboardingStep -lt 2) {
    throw "Expected onboarding.step >= 2 after Identity Next, got '$onboardingStep'"
  }

  if ($CompleteOnboarding) {
    $onboardingComplete = [string]$settings.PSObject.Properties["onboarding_complete"].Value
    $isOnboardingComplete = $onboardingComplete -eq "true" -or $onboardingComplete -eq "1"
    Add-Check "setting-onboarding_complete" $isOnboardingComplete @{ actual = $onboardingComplete }
    if (-not $isOnboardingComplete) {
      throw "Expected onboarding_complete true/1, got '$onboardingComplete'"
    }
  }

  $publishPath = [string]$settings.PSObject.Properties["paths.publish"].Value
  $backupPath = [string]$settings.PSObject.Properties["paths.backup"].Value
  $publishIsolated = Test-PathUnderRoot -Path $publishPath -Root $appDataRoot
  $backupIsolated = Test-PathUnderRoot -Path $backupPath -Root $appDataRoot
  Add-Check "setting-paths.publish-isolated" $publishIsolated @{ actual = $publishPath; expected_root = $appDataRoot }
  Add-Check "setting-paths.backup-isolated" $backupIsolated @{ actual = $backupPath; expected_root = $appDataRoot }
  if (-not $publishIsolated -or -not $backupIsolated) {
    throw "Expected publish/backup settings under isolated app data root $appDataRoot. publish='$publishPath' backup='$backupPath'"
  }

  $downloadedRuntimePath = Join-Path $appDataRoot "ollama-runtime"
  $runtimeAbsent = -not (Test-Path -LiteralPath $downloadedRuntimePath)
  Add-Check "downloaded-runtime-absent" $runtimeAbsent @{ downloaded_runtime_path = $downloadedRuntimePath }
  if (-not $runtimeAbsent) {
    throw "Expected no downloaded Ollama runtime at $downloadedRuntimePath"
  }

  $apiDeadline = (Get-Date).AddSeconds(30)
  while ((Get-Date) -lt $apiDeadline) {
    if (Test-CivicApiReady) { break }
    Start-Sleep -Seconds 1
  }
  $apiReady = Test-CivicApiReady
  Add-Check "packaged-local-api-ready" $apiReady @{ url = "http://127.0.0.1:12053/api/queue" }
  if (-not $apiReady) {
    throw "Packaged local API did not become ready on 127.0.0.1:12053"
  }

  $pairPin = "packaged-walkthrough-pin"
  $pairScript = @"
import datetime, hashlib, sqlite3, sys, uuid
db_path, pin = sys.argv[1], sys.argv[2]
conn = sqlite3.connect(db_path)
hashed = hashlib.sha256(pin.encode("utf-8")).hexdigest()
token = str(uuid.uuid4())
now = datetime.datetime.now(datetime.timezone.utc)
expires = (now + datetime.timedelta(minutes=10)).isoformat().replace("+00:00", "Z")
conn.execute(
    "insert into paired_clients (token, label, pairing_pin, pin_expires_at, created_at, revoked) values (?, ?, ?, ?, ?, 0)",
    (token, "packaged-walkthrough", hashed, expires, now.isoformat().replace("+00:00", "Z")),
)
conn.commit()
print(token)
"@
  $pairScriptPath = Join-Path $OutputDir "seed-pairing-pin.py"
  Set-Content -LiteralPath $pairScriptPath -Value $pairScript -Encoding UTF8
  & $pythonExe $pairScriptPath $dbPath $pairPin | Out-Null
  if ($LASTEXITCODE -ne 0) {
    throw "Seeding packaged pairing pin failed with exit code $LASTEXITCODE"
  }

  $pairResponse = Invoke-RestMethod -Method Post -Uri "http://127.0.0.1:12053/api/pair" -Headers @{ Host = "127.0.0.1:12053"; "x-civicnews-pair" = "1" } -ContentType "application/json" -Body (@{ pin = $pairPin } | ConvertTo-Json -Compress) -TimeoutSec 10
  $pairedToken = [string]$pairResponse.token
  $pairOk = -not [string]::IsNullOrWhiteSpace($pairedToken)
  Add-Check "packaged-live-pairing-positive" $pairOk @{ token_present = $pairOk }
  if (-not $pairOk) {
    throw "Packaged live pairing did not return a token."
  }

  $queueResponse = Invoke-RestMethod -Method Get -Uri "http://127.0.0.1:12053/api/queue" -Headers @{ Host = "127.0.0.1:12053"; Authorization = "Bearer $pairedToken" } -TimeoutSec 10
  $queueOk = $null -ne $queueResponse
  Add-Check "packaged-live-protected-queue" $queueOk @{ response_type = if ($queueResponse) { $queueResponse.GetType().FullName } else { "" } }
  if (-not $queueOk) {
    throw "Packaged live protected queue request did not return a response."
  }

  if ($CoreFlow) {
    Stop-WalkthroughProcess -Process $appProcess
    $appProcess = $null
    $portDeadline = (Get-Date).AddSeconds(15)
    while ((Get-Date) -lt $portDeadline -and -not (Test-PortFree -Port 12053)) {
      Start-Sleep -Milliseconds 500
    }
    if (-not (Test-PortFree -Port 12053)) {
      throw "Packaged first-run process did not release port 12053."
    }

    $coreFlowDir = Join-Path $OutputDir "core-flow"
    $coreAppData = Join-Path $coreFlowDir "app-data"
    New-Item -ItemType Directory -Force -Path $coreFlowDir, $coreAppData | Out-Null
    $coreCdpPort = Resolve-UnusedLoopbackPort -Start 65401 -End 65500
    $coreCdpUrl = "http://127.0.0.1:$coreCdpPort"
    $corePsi = New-Object System.Diagnostics.ProcessStartInfo
    $corePsi.FileName = $installedExe.FullName
    $corePsi.WorkingDirectory = Split-Path $installedExe.FullName
    $corePsi.UseShellExecute = $false
    $corePsi.Environment["CIVICNEWS_APP_DATA_DIR"] = $coreAppData
    $corePsi.Environment["WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS"] = "--remote-debugging-port=$coreCdpPort"
    $appProcess = [System.Diagnostics.Process]::Start($corePsi)

    $coreCdpDeadline = (Get-Date).AddSeconds(30)
    $coreCdpReady = $false
    while ((Get-Date) -lt $coreCdpDeadline) {
      if ($appProcess.HasExited) {
        throw "Packaged core-flow app exited early with code $($appProcess.ExitCode)."
      }
      try {
        $targets = Invoke-RestMethod -Uri "$coreCdpUrl/json" -TimeoutSec 2
        if ($targets) { $coreCdpReady = $true; break }
      } catch {}
      Start-Sleep -Milliseconds 500
    }
    Add-Check "core-flow-webview-cdp-ready" $coreCdpReady @{ cdp_url = $coreCdpUrl; app_data_dir = $coreAppData }
    if (-not $coreCdpReady) {
      throw "Packaged core-flow app did not expose WebView2 CDP at $coreCdpUrl"
    }

    & node $driverScript --cdp-url $coreCdpUrl --mode core-flow --output-dir $coreFlowDir --model $Model --expected-build-id $($receipt.commit)
    if ($LASTEXITCODE -ne 0) {
      throw "Packaged core-flow webview driver failed with exit code $LASTEXITCODE"
    }
    $coreResultPath = Join-Path $coreFlowDir "core-flow-webview-result.json"
    $coreResult = Get-Content -Raw -LiteralPath $coreResultPath | ConvertFrom-Json
    Add-Check "core-flow-webview-driver" ([bool]$coreResult.ok) @{ result_path = $coreResultPath; details = $coreResult.details }
    foreach ($driverCheck in $coreResult.checks) {
      Add-Check "core-$($driverCheck.name)" ([bool]$driverCheck.ok) @{ details = $driverCheck.details }
    }
    foreach ($screenshot in $coreResult.screenshots) {
      Add-ScreenshotCheck "core-$([System.IO.Path]::GetFileNameWithoutExtension($screenshot))" $screenshot
    }

    $coreDbPath = Join-Path $coreAppData "civicdesk.db"
    $coreDbSummaryPath = Join-Path $coreFlowDir "database-summary.json"
    $coreQuery = @"
import json, sqlite3, sys
db_path, out_path = sys.argv[1], sys.argv[2]
conn = sqlite3.connect(db_path)
summary = {
    "sources": conn.execute("select count(*) from sources").fetchone()[0],
    "evidence": conn.execute("select count(*) from evidence_items").fetchone()[0],
    "scan_leads": conn.execute("select count(*) from daily_scan_leads").fetchone()[0],
    "queue_leads": conn.execute("select count(*) from leads").fetchone()[0],
    "drafts": conn.execute("select count(*) from drafts").fetchone()[0],
    "linked_drafts": conn.execute("select count(distinct d.id) from drafts d join lead_evidence le on le.lead_id = d.lead_id").fetchone()[0],
}
with open(out_path, "w", encoding="utf-8") as f:
    json.dump(summary, f, indent=2, sort_keys=True)
"@
    $coreQueryPath = Join-Path $coreFlowDir "read-core-flow.py"
    Set-Content -LiteralPath $coreQueryPath -Value $coreQuery -Encoding UTF8
    & $pythonExe $coreQueryPath $coreDbPath $coreDbSummaryPath
    if ($LASTEXITCODE -ne 0) {
      throw "Packaged core-flow SQLite verification failed with exit code $LASTEXITCODE"
    }
    $coreSummary = Get-Content -Raw -LiteralPath $coreDbSummaryPath | ConvertFrom-Json
    $coreDatabaseOk = $coreSummary.sources -ge 2 -and $coreSummary.evidence -gt 0 -and $coreSummary.scan_leads -gt 0 -and $coreSummary.queue_leads -gt 0 -and $coreSummary.drafts -gt 0 -and $coreSummary.linked_drafts -gt 0
    Add-Check "core-flow-database-persistence" $coreDatabaseOk @{ summary = $coreSummary; path = $coreDbSummaryPath }
    if (-not $coreDatabaseOk) {
      throw "Packaged core-flow database does not contain the required source-to-draft chain."
    }
  }
} finally {
  Stop-WalkthroughProcess -Process $appProcess
  if (Test-Path $installRoot) {
    $uninstaller = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" |
      Where-Object { $_.Name -match "unins|uninstall" } |
      Sort-Object FullName |
      Select-Object -First 1
    if ($uninstaller) {
      $uninstall = Start-Process -FilePath $uninstaller.FullName -ArgumentList @("/S") -Wait -PassThru -WindowStyle Hidden
      Add-Check "nsis-silent-uninstall" ($uninstall.ExitCode -eq 0) @{ exit_code = $uninstall.ExitCode }
    }
  }
}

$receipt.ok = -not ($receipt.checks | Where-Object { -not $_.ok })
$receipt | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $receiptPath -Encoding UTF8
$receiptHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $receiptPath).Hash
"$receiptHash  $([System.IO.Path]::GetFileName($receiptPath))" | Set-Content -LiteralPath "$receiptPath.sha256" -Encoding ascii

if (-not $receipt.ok) {
  throw "Packaged first-run walkthrough failed. Receipt: $receiptPath"
}

Write-Host "Packaged first-run walkthrough receipt: $receiptPath"
