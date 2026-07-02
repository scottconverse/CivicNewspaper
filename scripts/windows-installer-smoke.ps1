param(
  [string]$NsisInstaller = "",
  [string]$MsiInstaller = "",
  [string]$OutputDir = ""
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

function Get-GitHead {
  param([string]$Repo)
  Push-Location $Repo
  try {
    return (git rev-parse HEAD).Trim()
  } finally {
    Pop-Location
  }
}

function Test-GitDirty {
  param([string]$Repo)
  Push-Location $Repo
  try {
    return [bool](git status --porcelain)
  } finally {
    Pop-Location
  }
}

function Get-ArtifactSha256 {
  param([string]$Path)
  if ([string]::IsNullOrWhiteSpace($Path)) {
    return $null
  }
  return (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash
}

function Resolve-Artifact {
  param(
    [string]$ExplicitPath,
    [string]$SearchRoot,
    [string]$Pattern,
    [string]$Version,
    [switch]$AllowMissing
  )

  if ($ExplicitPath) {
    $resolved = Resolve-Path -LiteralPath $ExplicitPath -ErrorAction Stop
    return $resolved.Path
  }

  $artifact = Get-ChildItem -LiteralPath $SearchRoot -Recurse -Filter $Pattern |
    Where-Object { $_.Name -like "*$Version*" } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

  if (-not $artifact) {
    if ($AllowMissing) {
      return $null
    }
    throw "Could not find installer artifact matching $Pattern under $SearchRoot"
  }

  return $artifact.FullName
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
  throw "Could not find an unused loopback port for dependency-absent installer smoke."
}

Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Win32WindowRect {
  [StructLayout(LayoutKind.Sequential)]
  public struct RECT {
    public int Left;
    public int Top;
    public int Right;
    public int Bottom;
  }
  [DllImport("user32.dll")]
  public static extern bool GetWindowRect(IntPtr hWnd, out RECT rect);
  [DllImport("user32.dll", CharSet = CharSet.Unicode)]
  public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder text, int count);
  [DllImport("user32.dll")]
  public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBlt, int nFlags);
  [DllImport("user32.dll")]
  public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
  [DllImport("user32.dll")]
  public static extern bool SetForegroundWindow(IntPtr hWnd);
}
"@

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms

function Capture-WindowScreenshot {
  param(
    [System.Diagnostics.Process]$Process,
    [string]$Path
  )

  $deadline = (Get-Date).AddSeconds(20)
  while ((Get-Date) -lt $deadline) {
    $Process.Refresh()
    if ($Process.MainWindowHandle -ne [IntPtr]::Zero) {
      break
    }
    Start-Sleep -Milliseconds 500
  }

  if ($Process.MainWindowHandle -eq [IntPtr]::Zero) {
    throw "Installed app did not expose a main window handle."
  }

  $titleBuilder = New-Object System.Text.StringBuilder 512
  [void][Win32WindowRect]::GetWindowText($Process.MainWindowHandle, $titleBuilder, $titleBuilder.Capacity)
  $windowTitle = $titleBuilder.ToString()
  if ($windowTitle -notmatch "Civic Desk|CivicNewspaper|civicnews") {
    throw "Installed app window title was '$windowTitle', not The Civic Desk."
  }

  $rect = New-Object Win32WindowRect+RECT
  if (-not [Win32WindowRect]::GetWindowRect($Process.MainWindowHandle, [ref]$rect)) {
    throw "Could not read installed app window bounds."
  }

  $width = [Math]::Max(1, $rect.Right - $rect.Left)
  $height = [Math]::Max(1, $rect.Bottom - $rect.Top)
  $bitmap = New-Object System.Drawing.Bitmap($width, $height)
  $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
  $renderedFromWindow = $false
  try {
    [void][Win32WindowRect]::ShowWindow($Process.MainWindowHandle, 9)
    [void][Win32WindowRect]::SetForegroundWindow($Process.MainWindowHandle)
    Start-Sleep -Milliseconds 750
    $hdc = $graphics.GetHdc()
    try {
      $renderedFromWindow = [Win32WindowRect]::PrintWindow($Process.MainWindowHandle, $hdc, 2)
    } finally {
      $graphics.ReleaseHdc($hdc)
    }
    if (-not $renderedFromWindow) {
      $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, $bitmap.Size)
    }
    $bitmap.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
  } finally {
    $graphics.Dispose()
    $bitmap.Dispose()
  }

  $image = [System.Drawing.Bitmap]::new($Path)
  try {
    $samples = 0
    $different = 0
    $first = $image.GetPixel(0, 0).ToArgb()
    for ($x = 0; $x -lt $image.Width; $x += [Math]::Max(1, [int]($image.Width / 12))) {
      for ($y = 0; $y -lt $image.Height; $y += [Math]::Max(1, [int]($image.Height / 12))) {
        $samples++
        if ($image.GetPixel($x, $y).ToArgb() -ne $first) {
          $different++
        }
      }
    }
    return [ordered]@{
      path = $Path
      width = $image.Width
      height = $image.Height
      sampled_pixels = $samples
      varied_pixels = $different
      nonblank = ($different -gt 3)
      window_title = $windowTitle
      render_method = if ($renderedFromWindow) { "PrintWindow" } else { "CopyFromScreen fallback" }
    }
  } finally {
    $image.Dispose()
  }
}

function Stop-SmokeProcess {
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
    Write-Warning "Could not stop smoke process $($Process.Id): $_"
  }
}

$repo = Resolve-RepoRoot
$appVersion = Resolve-AppVersion -Repo $repo
$bundleDir = Join-Path $repo "src-tauri\target\release\bundle"
$runId = Get-Date -Format "yyyyMMdd-HHmmss"
if (-not $OutputDir) {
  $OutputDir = Join-Path $repo ".agent-runs\windows-installer-smoke-$runId"
} elseif (-not [System.IO.Path]::IsPathRooted($OutputDir)) {
  $OutputDir = Join-Path $repo $OutputDir
}
$OutputDir = [System.IO.Path]::GetFullPath($OutputDir)
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$nsisPath = Resolve-Artifact -ExplicitPath $NsisInstaller -SearchRoot $bundleDir -Pattern "*.exe" -Version $appVersion
$msiPath = Resolve-Artifact -ExplicitPath $MsiInstaller -SearchRoot $bundleDir -Pattern "*.msi" -Version $appVersion -AllowMissing

$installRoot = Join-Path $OutputDir "nsis-install"
$appDataRoot = Join-Path $OutputDir "app-data"
$msiExtractRoot = Join-Path $OutputDir "msi-extract"
New-Item -ItemType Directory -Force -Path $installRoot, $appDataRoot, $msiExtractRoot | Out-Null
$absentOllamaBaseUrl = Resolve-UnusedLoopbackUrl

$receipt = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  repo = $repo
  commit = Get-GitHead -Repo $repo
  dirty = Test-GitDirty -Repo $repo
  app_version = $appVersion
  nsis_installer = $nsisPath
  nsis_installer_sha256 = Get-ArtifactSha256 -Path $nsisPath
  msi_installer = $msiPath
  msi_installer_sha256 = Get-ArtifactSha256 -Path $msiPath
  output_dir = $OutputDir
  forced_ollama_base_url = $absentOllamaBaseUrl
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

try {
  $installArgs = @("/S", "/D=$installRoot")
  $install = Start-Process -FilePath $nsisPath -ArgumentList $installArgs -Wait -PassThru -WindowStyle Hidden
  Add-Check "nsis-silent-install-exit" ($install.ExitCode -eq 0) @{ exit_code = $install.ExitCode; install_root = $installRoot }
  if ($install.ExitCode -ne 0) {
    throw "NSIS installer exited $($install.ExitCode)"
  }

  $installedExe = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" |
    Where-Object { $_.Name -notmatch "unins|uninstall" } |
    Sort-Object FullName |
    Select-Object -First 1
  Add-Check "nsis-installed-exe-present" ($null -ne $installedExe) @{ exe = if ($installedExe) { $installedExe.FullName } else { "" } }
  if (-not $installedExe) {
    throw "NSIS install did not produce an application executable under $installRoot"
  }

  $env:CIVICNEWS_APP_DATA_DIR = $appDataRoot
  $env:CIVICNEWS_OLLAMA_BASE_URL = $absentOllamaBaseUrl
  $appProcess = Start-Process -FilePath $installedExe.FullName -PassThru
  Start-Sleep -Seconds 8
  Add-Check "nsis-installed-app-starts" (-not $appProcess.HasExited) @{ pid = $appProcess.Id; app_data = $appDataRoot }
  if ($appProcess.HasExited) {
    throw "Installed app exited early with code $($appProcess.ExitCode)"
  }
  $screenshotPath = Join-Path $OutputDir "nsis-installed-first-run.png"
  $screenshot = Capture-WindowScreenshot -Process $appProcess -Path $screenshotPath
  Add-Check "nsis-installed-first-run-screenshot" ([bool]$screenshot.nonblank) @{
    path = $screenshot.path
    width = $screenshot.width
    height = $screenshot.height
    window_title = $screenshot.window_title
    render_method = $screenshot.render_method
    installed_exe = $installedExe.FullName
    sampled_pixels = $screenshot.sampled_pixels
    varied_pixels = $screenshot.varied_pixels
  }
  if (-not $screenshot.nonblank) {
    throw "Installed app screenshot looked blank: $screenshotPath"
  }

  $dbPath = Join-Path $appDataRoot "civicdesk.db"
  Add-Check "nsis-installed-isolated-db-created" (Test-Path $dbPath) @{
    db_path = $dbPath
    app_data = $appDataRoot
  }
  if (-not (Test-Path $dbPath)) {
    throw "Installed app did not write the isolated SQLite DB at $dbPath"
  }
  $runtimePath = Join-Path $appDataRoot "ollama-runtime"
  Add-Check "nsis-installed-dependency-absent-profile" (-not (Test-Path $runtimePath)) @{
    forced_ollama_base_url = $absentOllamaBaseUrl
    downloaded_runtime_path = $runtimePath
    downloaded_runtime_present = (Test-Path $runtimePath)
  }
  if (Test-Path $runtimePath) {
    throw "Installed app downloaded the local AI runtime before explicit user action: $runtimePath"
  }
  Stop-SmokeProcess -Process $appProcess

  $uninstaller = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" |
    Where-Object { $_.Name -match "unins|uninstall" } |
    Sort-Object FullName |
    Select-Object -First 1
  Add-Check "nsis-uninstaller-present" ($null -ne $uninstaller) @{ uninstaller = if ($uninstaller) { $uninstaller.FullName } else { "" } }
  if ($uninstaller) {
    $uninstall = Start-Process -FilePath $uninstaller.FullName -ArgumentList @("/S") -Wait -PassThru -WindowStyle Hidden
    Add-Check "nsis-silent-uninstall-exit" ($uninstall.ExitCode -eq 0) @{ exit_code = $uninstall.ExitCode }
    Start-Sleep -Seconds 2
    $installedExeGone = -not (Test-Path $installedExe.FullName)
    $remainingInstallItems = @()
    if (Test-Path $installRoot) {
      $remainingInstallItems = @(Get-ChildItem -LiteralPath $installRoot -Force -ErrorAction SilentlyContinue)
    }
    Add-Check "nsis-uninstall-removes-installed-exe" $installedExeGone @{ exe = $installedExe.FullName }
    Add-Check "nsis-uninstall-leaves-no-install-root-files" ($remainingInstallItems.Count -eq 0) @{
      install_root = $installRoot
      remaining_count = $remainingInstallItems.Count
      remaining = @($remainingInstallItems | ForEach-Object { $_.FullName })
    }
  }

  if ($msiPath) {
    $msiLog = Join-Path $OutputDir "msi-admin-extract.log"
    $msiArgs = @("/a", "`"$msiPath`"", "/qn", "/norestart", "TARGETDIR=`"$msiExtractRoot`"", "/L*v", "`"$msiLog`"")
    $msi = Start-Process -FilePath "msiexec.exe" -ArgumentList $msiArgs -Wait -PassThru -WindowStyle Hidden
    Add-Check "msi-administrative-extract-exit" ($msi.ExitCode -eq 0) @{ exit_code = $msi.ExitCode; extract_root = $msiExtractRoot; log = $msiLog }
    if ($msi.ExitCode -ne 0) {
      throw "MSI administrative extraction exited $($msi.ExitCode)"
    }

    $msiExe = Get-ChildItem -LiteralPath $msiExtractRoot -Recurse -Filter "*.exe" |
      Where-Object { $_.Name -notmatch "unins|uninstall" } |
      Select-Object -First 1
    Add-Check "msi-extracted-exe-present" ($null -ne $msiExe) @{ exe = if ($msiExe) { $msiExe.FullName } else { "" } }
    if (-not $msiExe) {
      throw "MSI administrative extraction did not produce an application executable under $msiExtractRoot"
    }
  } else {
    Add-Check "msi-not-public-beta-target" $true @{ note = "No current MSI artifact was found. MSI is backlog/proof-needed and not required for this Windows public beta smoke." }
  }
} finally {
  Remove-Item Env:\CIVICNEWS_APP_DATA_DIR -ErrorAction SilentlyContinue
  Remove-Item Env:\CIVICNEWS_OLLAMA_BASE_URL -ErrorAction SilentlyContinue
}

$receipt.ok = -not ($receipt.checks | Where-Object { -not $_.ok })
$receiptPath = Join-Path $OutputDir "windows-installer-smoke-receipt.json"
$receipt | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $receiptPath -Encoding UTF8

if (-not $receipt.ok) {
  throw "Windows installer smoke failed. Receipt: $receiptPath"
}

Write-Host "Windows installer smoke receipt: $receiptPath"
