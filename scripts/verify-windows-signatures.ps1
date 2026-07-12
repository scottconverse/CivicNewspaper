param(
  [Parameter(Mandatory = $true)]
  [string]$Installer,
  [Parameter(Mandatory = $true)]
  [string]$ReceiptPath,
  [string]$ExpectedSigner = "Scott Converse"
)

$ErrorActionPreference = "Stop"
$Installer = (Resolve-Path -LiteralPath $Installer).Path
$ReceiptPath = [System.IO.Path]::GetFullPath($ReceiptPath)
$runRoot = Join-Path ([System.IO.Path]::GetTempPath()) "civicdesk-signature-$([guid]::NewGuid().ToString('N'))"
$installRoot = Join-Path $runRoot "install"
New-Item -ItemType Directory -Force -Path $installRoot | Out-Null

function Get-SignatureRecord {
  param(
    [string]$Name,
    [string]$Path
  )

  if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
    throw "$Name executable was not found at $Path"
  }
  $file = Get-Item -LiteralPath $Path
  $signature = Get-AuthenticodeSignature -LiteralPath $Path
  $valid = $signature.Status -eq "Valid"
  $hasExpectedSigner = $signature.SignerCertificate -and
    $signature.SignerCertificate.Subject -like "*$ExpectedSigner*"
  $hasTimestamp = $null -ne $signature.TimeStamperCertificate
  if (-not $valid -or -not $hasExpectedSigner -or -not $hasTimestamp) {
    throw "$Name signature failed: status=$($signature.Status); signer=$($signature.SignerCertificate.Subject); timestamp=$hasTimestamp; path=$Path"
  }

  return [ordered]@{
    name = $Name
    path = $Path
    size = $file.Length
    sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash
    status = [string]$signature.Status
    signer_subject = $signature.SignerCertificate.Subject
    signer_thumbprint = $signature.SignerCertificate.Thumbprint
    timestamp_subject = $signature.TimeStamperCertificate.Subject
    timestamp_thumbprint = $signature.TimeStamperCertificate.Thumbprint
  }
}

$records = @()
$uninstaller = $null
try {
  $records += Get-SignatureRecord -Name "installer" -Path $Installer
  $install = Start-Process -FilePath $Installer -ArgumentList @("/S", "/D=$installRoot") -Wait -PassThru -WindowStyle Hidden
  if ($install.ExitCode -ne 0) {
    throw "Silent installer exited $($install.ExitCode)."
  }

  $application = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" -File |
    Where-Object { $_.Name -notmatch "unins|uninstall" } |
    Sort-Object FullName |
    Select-Object -First 1
  $uninstaller = Get-ChildItem -LiteralPath $installRoot -Recurse -Filter "*.exe" -File |
    Where-Object { $_.Name -match "unins|uninstall" } |
    Sort-Object FullName |
    Select-Object -First 1
  if (-not $application) { throw "Installed application executable was not found under $installRoot" }
  if (-not $uninstaller) { throw "Installed uninstaller executable was not found under $installRoot" }

  $records += Get-SignatureRecord -Name "application" -Path $application.FullName
  $records += Get-SignatureRecord -Name "uninstaller" -Path $uninstaller.FullName

  $receipt = [ordered]@{
    generated_at = (Get-Date).ToUniversalTime().ToString("o")
    installer_name = Split-Path -Leaf $Installer
    installer_sha256 = $records[0].sha256
    installer_size = $records[0].size
    signer_subject = $records[0].signer_subject
    signer_thumbprint = $records[0].signer_thumbprint
    timestamp_subject = $records[0].timestamp_subject
    timestamp_thumbprint = $records[0].timestamp_thumbprint
    executables = $records
    ok = $true
  }
  $receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $ReceiptPath -Encoding utf8
  $receipt | ConvertTo-Json -Depth 6
} finally {
  if ($uninstaller -and (Test-Path -LiteralPath $uninstaller.FullName)) {
    Start-Process -FilePath $uninstaller.FullName -ArgumentList @("/S") -Wait -WindowStyle Hidden
  }
  if (Test-Path -LiteralPath $runRoot) {
    Remove-Item -LiteralPath $runRoot -Recurse -Force -ErrorAction SilentlyContinue
  }
}
