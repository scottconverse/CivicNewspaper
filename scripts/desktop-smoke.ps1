param(
  [string]$RunDir = "",
  [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
if ([string]::IsNullOrWhiteSpace($RunDir)) {
  $Stamp = Get-Date -Format "yyyyMMdd-HHmmss"
  $RunDir = Join-Path $RepoRoot ".agent-runs\desktop-smoke-$Stamp"
}
$RunDir = [System.IO.Path]::GetFullPath($RunDir)
$AppDataDir = Join-Path $RunDir "app-data"
$LogPath = Join-Path $RunDir "desktop-smoke.log"
$ReceiptPath = Join-Path $RunDir "desktop-smoke-receipt.json"
New-Item -ItemType Directory -Force -Path $RunDir, $AppDataDir | Out-Null

function Write-Log {
  param([string]$Message)
  $line = "$(Get-Date -Format o) $Message"
  $line | Tee-Object -FilePath $LogPath -Append
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

Push-Location $RepoRoot
$proc = $null
try {
  if (-not (Test-PortFree 12053)) {
    throw "Port 12053 is already in use before desktop smoke launch."
  }

  if (-not $SkipBuild) {
    Write-Log "Building Tauri executable without bundle."
    cmd /d /c "npm run tauri -- build --no-bundle 2>&1" | Tee-Object -FilePath $LogPath -Append
    if ($LASTEXITCODE -ne 0) {
      throw "Tauri no-bundle build failed with exit code $LASTEXITCODE."
    }
  }

  $exe = Join-Path $RepoRoot "src-tauri\target\release\civicnews.exe"
  if (-not (Test-Path $exe)) {
    throw "Desktop executable not found at $exe."
  }

  Write-Log "Launching desktop app with isolated app data: $AppDataDir"
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $exe
  $psi.WorkingDirectory = Split-Path $exe
  $psi.UseShellExecute = $false
  $psi.Environment["CIVICNEWS_APP_DATA_DIR"] = $AppDataDir
  $proc = [System.Diagnostics.Process]::Start($psi)

  $dbPath = Join-Path $AppDataDir "civicdesk.db"
  $logsDir = Join-Path $AppDataDir "logs"
  $dbSeen = $false
  $serverSeen = $false
  $pairStatus = $null
  $deadline = (Get-Date).AddSeconds(45)
  while ((Get-Date) -lt $deadline) {
    if ($proc.HasExited) {
      throw "Desktop process exited early with code $($proc.ExitCode)."
    }
    if (-not $dbSeen -and (Test-Path $dbPath)) {
      $dbSeen = $true
      Write-Log "Observed isolated SQLite database: $dbPath"
    }
    if (-not $serverSeen) {
      try {
        $body = '{"pin":"000000"}'
        Invoke-WebRequest -Uri "http://127.0.0.1:12053/api/pair" -Method Post -Body $body -ContentType "application/json" -Headers @{ "Host" = "127.0.0.1:12053" } -UseBasicParsing | Out-Null
        $serverSeen = $true
        $pairStatus = 200
      } catch {
        $response = $_.Exception.Response
        if ($response) {
          $serverSeen = $true
          $pairStatus = [int]$response.StatusCode
          Write-Log "Observed loopback pairing server with HTTP $pairStatus."
        } elseif (-not (Test-PortFree 12053)) {
          $serverSeen = $true
          $pairStatus = "tcp-open"
          Write-Log "Observed loopback pairing server TCP listener."
        }
      }
    }
    if ($dbSeen -and $serverSeen) { break }
    Start-Sleep -Milliseconds 500
  }

  if (-not $dbSeen) {
    throw "Timed out waiting for desktop app to write isolated SQLite DB at $dbPath."
  }
  if (-not $serverSeen) {
    throw "Timed out waiting for desktop app loopback server on 127.0.0.1:12053."
  }

  $receipt = [ordered]@{
    generated_at = (Get-Date).ToString("o")
    repo = $RepoRoot.Path
    commit = (git rev-parse HEAD).Trim()
    dirty = [bool](git status --porcelain)
    exe = $exe
    app_data_dir = $AppDataDir
    db_path = $dbPath
    db_exists = (Test-Path $dbPath)
    logs_dir = $logsDir
    logs_dir_exists = (Test-Path $logsDir)
    loopback_pair_status = $pairStatus
    process_id = $proc.Id
    downloaded_runtime_in_isolated_profile = (Test-Path (Join-Path $AppDataDir "ollama-runtime"))
    first_run_coverage_note = "Desktop process launched with isolated app data and no downloaded Ollama runtime in that profile; onboarding UI is covered by browser smoke and remote cleanroom."
  }
  $receipt | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 $ReceiptPath
  Write-Log "Desktop smoke receipt: $ReceiptPath"
  Write-Host "Desktop smoke receipt: $ReceiptPath"
} finally {
  if ($proc -and -not $proc.HasExited) {
    Write-Log "Stopping desktop process $($proc.Id)."
    Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
  }
  Pop-Location
}
