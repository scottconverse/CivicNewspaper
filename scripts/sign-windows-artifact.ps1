param(
  [Parameter(Mandatory = $true)]
  [string]$File
)

$ErrorActionPreference = "Stop"
$logPath = Join-Path $(if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [IO.Path]::GetTempPath() }) "civicnewspaper-signing.log"
function Write-SigningDiagnostic([string]$Message) {
  foreach ($secretName in @("AZURE_CLIENT_ID", "AZURE_CLIENT_SECRET", "AZURE_TENANT_ID")) {
    $secret = [System.Environment]::GetEnvironmentVariable($secretName)
    if (-not [string]::IsNullOrEmpty($secret)) { $Message = $Message.Replace($secret, "[REDACTED]") }
  }
  Add-Content -LiteralPath $logPath -Encoding UTF8 -Value "$(Get-Date -Format o) $Message"
}
Set-Content -LiteralPath $logPath -Encoding UTF8 -Value "$(Get-Date -Format o) signer-start"
foreach ($name in @("AZURE_CLIENT_ID", "AZURE_CLIENT_SECRET", "AZURE_TENANT_ID")) {
  if ([string]::IsNullOrWhiteSpace([System.Environment]::GetEnvironmentVariable($name))) {
    throw "$name is required for Windows package signing."
  }
}

$File = (Resolve-Path -LiteralPath $File).Path
Write-SigningDiagnostic "artifact-resolved name=$(Split-Path -Leaf $File)"
try {
Import-Module ArtifactSigning -RequiredVersion 0.1.8 -Force
Write-SigningDiagnostic "module-imported"
$params = @{
  Endpoint = "https://wcus.codesigning.azure.net"
  CodeSigningAccountName = "scottconverse-signing"
  CertificateProfileName = "ScottConversePublic"
  Files = $File
  FileDigest = "SHA256"
  TimestampRfc3161 = "http://timestamp.acs.microsoft.com"
  TimestampDigest = "SHA256"
  Description = "The Civic Desk"
  ExcludeWorkloadIdentityCredential = $true
  ExcludeManagedIdentityCredential = $true
  ExcludeSharedTokenCacheCredential = $true
  ExcludeVisualStudioCredential = $true
  ExcludeVisualStudioCodeCredential = $true
  ExcludeAzureCliCredential = $true
  ExcludeAzurePowerShellCredential = $true
  ExcludeAzureDeveloperCliCredential = $true
  ExcludeInteractiveBrowserCredential = $true
}
Invoke-ArtifactSigning @params
Write-SigningDiagnostic "artifact-signing-returned"

$signature = Get-AuthenticodeSignature -LiteralPath $File
if ($signature.Status -ne "Valid" -or -not $signature.TimeStamperCertificate) {
  throw "Artifact Signing did not produce a valid timestamped signature: status=$($signature.Status); file=$File"
}
Write-SigningDiagnostic "signature-valid timestamp=true"
} catch {
  Write-SigningDiagnostic "failure type=$($_.Exception.GetType().FullName) message=$($_.Exception.Message)"
  throw
}
