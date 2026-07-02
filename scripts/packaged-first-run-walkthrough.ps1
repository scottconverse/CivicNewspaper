param(
  [string]$NsisInstaller = "",
  [string]$OutputDir = "",
  [string]$Python = "",
  [switch]$CompleteOnboarding
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

function Resolve-UnusedLoopbackUrl {
  for ($port = 65200; $port -le 65300; $port++) {
    if (Test-PortFree -Port $port) {
      return "http://127.0.0.1:$port"
    }
  }
  throw "Could not find an unused loopback port."
}

Add-Type @"
using System;
using System.Runtime.InteropServices;
public class CivicDeskWalkthroughWin32 {
  [StructLayout(LayoutKind.Sequential)]
  public struct RECT {
    public int Left;
    public int Top;
    public int Right;
    public int Bottom;
  }
  [DllImport("user32.dll")]
  public static extern bool GetWindowRect(IntPtr hWnd, out RECT rect);
  [DllImport("user32.dll")]
  public static extern bool MoveWindow(IntPtr hWnd, int X, int Y, int nWidth, int nHeight, bool bRepaint);
  [DllImport("user32.dll")]
  public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
  [DllImport("user32.dll")]
  public static extern bool SetForegroundWindow(IntPtr hWnd);
  [DllImport("user32.dll", CharSet = CharSet.Unicode)]
  public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder text, int count);
  [DllImport("user32.dll")]
  public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBlt, int nFlags);
  [DllImport("user32.dll")]
  public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")]
  public static extern bool SetCursorPos(int X, int Y);
  [DllImport("user32.dll")]
  public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint dwData, UIntPtr dwExtraInfo);
}
"@

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
[void][CivicDeskWalkthroughWin32]::SetProcessDPIAware()

function Capture-WindowScreenshot {
  param(
    [System.Diagnostics.Process]$Process,
    [string]$Path
  )

  $rect = New-Object CivicDeskWalkthroughWin32+RECT
  if (-not [CivicDeskWalkthroughWin32]::GetWindowRect($Process.MainWindowHandle, [ref]$rect)) {
    throw "Could not read app window bounds."
  }
  $width = [Math]::Max(1, $rect.Right - $rect.Left)
  $height = [Math]::Max(1, $rect.Bottom - $rect.Top)
  $bitmap = New-Object System.Drawing.Bitmap($width, $height)
  $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
  try {
    $hdc = $graphics.GetHdc()
    try {
      $rendered = [CivicDeskWalkthroughWin32]::PrintWindow($Process.MainWindowHandle, $hdc, 2)
    } finally {
      $graphics.ReleaseHdc($hdc)
    }
    if (-not $rendered) {
      $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, $bitmap.Size)
    }
    $bitmap.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
  } finally {
    $graphics.Dispose()
    $bitmap.Dispose()
  }
}

function Click-WindowPoint {
  param(
    [System.Diagnostics.Process]$Process,
    [int]$X,
    [int]$Y
  )

  $rect = New-Object CivicDeskWalkthroughWin32+RECT
  if (-not [CivicDeskWalkthroughWin32]::GetWindowRect($Process.MainWindowHandle, [ref]$rect)) {
    throw "Could not read app window bounds for click."
  }
  [void][CivicDeskWalkthroughWin32]::SetForegroundWindow($Process.MainWindowHandle)
  Start-Sleep -Milliseconds 150
  [void][CivicDeskWalkthroughWin32]::SetCursorPos($rect.Left + $X, $rect.Top + $Y)
  Start-Sleep -Milliseconds 100
  [CivicDeskWalkthroughWin32]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
  Start-Sleep -Milliseconds 75
  [CivicDeskWalkthroughWin32]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
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
  output_dir = $OutputDir
  install_root = $installRoot
  app_data_dir = $appDataRoot
  python = $pythonExe
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

  $absentOllamaBaseUrl = Resolve-UnusedLoopbackUrl
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $installedExe.FullName
  $psi.WorkingDirectory = Split-Path $installedExe.FullName
  $psi.UseShellExecute = $false
  $psi.Environment["CIVICNEWS_APP_DATA_DIR"] = $appDataRoot
  $psi.Environment["CIVICNEWS_OLLAMA_BASE_URL"] = $absentOllamaBaseUrl
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

  [void][CivicDeskWalkthroughWin32]::ShowWindow($appProcess.MainWindowHandle, 9)
  [void][CivicDeskWalkthroughWin32]::MoveWindow($appProcess.MainWindowHandle, 40, 40, 1250, 900, $true)
  [void][CivicDeskWalkthroughWin32]::SetForegroundWindow($appProcess.MainWindowHandle)
  Start-Sleep -Seconds 3

  $titleBuilder = New-Object System.Text.StringBuilder 512
  [void][CivicDeskWalkthroughWin32]::GetWindowText($appProcess.MainWindowHandle, $titleBuilder, $titleBuilder.Capacity)
  $windowTitle = $titleBuilder.ToString()
  Add-Check "window-title" ($windowTitle -match "Civic Desk") @{ title = $windowTitle }

  $beforeScreenshot = Join-Path $OutputDir "01-before-identity-entry.png"
  Capture-WindowScreenshot -Process $appProcess -Path $beforeScreenshot
  Add-Check "before-screenshot" (Test-Path $beforeScreenshot) @{ path = $beforeScreenshot }

  $publication = "Longmont Local Beta Desk"
  $editor = "Local Packaged Walkthrough"
  $city = "Longmont"
  $state = "CO"
  [System.Windows.Forms.SendKeys]::SendWait($publication)
  Start-Sleep -Milliseconds 250
  [System.Windows.Forms.SendKeys]::SendWait("{TAB}")
  Start-Sleep -Milliseconds 150
  [System.Windows.Forms.SendKeys]::SendWait($editor)
  Start-Sleep -Milliseconds 250
  [System.Windows.Forms.SendKeys]::SendWait("{TAB}{TAB}")
  Start-Sleep -Milliseconds 150
  [System.Windows.Forms.SendKeys]::SendWait($city)
  Start-Sleep -Milliseconds 250
  [System.Windows.Forms.SendKeys]::SendWait("{TAB}")
  Start-Sleep -Milliseconds 150
  [System.Windows.Forms.SendKeys]::SendWait($state)
  Start-Sleep -Milliseconds 250

  $afterTypingScreenshot = Join-Path $OutputDir "02-after-identity-entry.png"
  Capture-WindowScreenshot -Process $appProcess -Path $afterTypingScreenshot
  Add-Check "after-typing-screenshot" (Test-Path $afterTypingScreenshot) @{ path = $afterTypingScreenshot }

  [System.Windows.Forms.SendKeys]::SendWait("{ENTER}")
  Start-Sleep -Seconds 5

  $afterNextScreenshot = Join-Path $OutputDir "03-after-identity-next.png"
  Capture-WindowScreenshot -Process $appProcess -Path $afterNextScreenshot
  Add-Check "after-next-screenshot" (Test-Path $afterNextScreenshot) @{ path = $afterNextScreenshot }

  if ($CompleteOnboarding) {
    Click-WindowPoint -Process $appProcess -X 748 -Y 817
    Start-Sleep -Seconds 1
    $skipConfirmScreenshot = Join-Path $OutputDir "04-skip-confirmation.png"
    Capture-WindowScreenshot -Process $appProcess -Path $skipConfirmScreenshot
    Add-Check "skip-confirmation-screenshot" (Test-Path $skipConfirmScreenshot) @{ path = $skipConfirmScreenshot }

    [System.Windows.Forms.SendKeys]::SendWait("{TAB}{ENTER}")
    Start-Sleep -Seconds 2
    $afterSkipScreenshot = Join-Path $OutputDir "05-after-skip-setup.png"
    Capture-WindowScreenshot -Process $appProcess -Path $afterSkipScreenshot
    Add-Check "after-skip-setup-screenshot" (Test-Path $afterSkipScreenshot) @{ path = $afterSkipScreenshot }

    Click-WindowPoint -Process $appProcess -X 940 -Y 817
    Start-Sleep -Seconds 1
    $doneStepScreenshot = Join-Path $OutputDir "06-done-step.png"
    Capture-WindowScreenshot -Process $appProcess -Path $doneStepScreenshot
    Add-Check "done-step-screenshot" (Test-Path $doneStepScreenshot) @{ path = $doneStepScreenshot }

    Click-WindowPoint -Process $appProcess -X 940 -Y 817
    Start-Sleep -Seconds 3
    $workspaceScreenshot = Join-Path $OutputDir "07-workspace-reached.png"
    Capture-WindowScreenshot -Process $appProcess -Path $workspaceScreenshot
    Add-Check "workspace-reached-screenshot" (Test-Path $workspaceScreenshot) @{ path = $workspaceScreenshot }
  }

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

if (-not $receipt.ok) {
  throw "Packaged first-run walkthrough failed. Receipt: $receiptPath"
}

Write-Host "Packaged first-run walkthrough receipt: $receiptPath"
