param(
  [Parameter(Mandatory = $true)]
  [string]$File
)

$ErrorActionPreference = "Stop"
foreach ($name in @("AZURE_CLIENT_ID", "AZURE_CLIENT_SECRET", "AZURE_TENANT_ID")) {
  if ([string]::IsNullOrWhiteSpace([System.Environment]::GetEnvironmentVariable($name))) {
    throw "$name is required for Windows package signing."
  }
}

$File = (Resolve-Path -LiteralPath $File).Path
Import-Module ArtifactSigning -RequiredVersion 0.1.8 -Force
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

$signature = Get-AuthenticodeSignature -LiteralPath $File
if ($signature.Status -ne "Valid" -or -not $signature.TimeStamperCertificate) {
  throw "Artifact Signing did not produce a valid timestamped signature: status=$($signature.Status); file=$File"
}
