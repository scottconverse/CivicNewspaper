$ErrorActionPreference = "Stop"
$Repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$EvidenceDir = $PSScriptRoot
$Utf8NoBom = New-Object System.Text.UTF8Encoding($false)
$BadSequences = @(
  @{ Name = "curly_apostrophe_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x2122) },
  @{ Name = "left_double_quote_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x0153) },
  @{ Name = "right_double_quote_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x009D) },
  @{ Name = "en_dash_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x201C) },
  @{ Name = "em_dash_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x201D) },
  @{ Name = "right_arrow_mojibake"; Text = -join ([char]0x00E2, [char]0x2020, [char]0x2019) },
  @{ Name = "copyright_mojibake"; Text = -join ([char]0x00C2, [char]0x00A9) },
  @{ Name = "middle_dot_mojibake"; Text = -join ([char]0x00C2, [char]0x00B7) },
  @{ Name = "double_encoded_utf8_starter"; Text = -join ([char]0x00C3, [char]0x00A2) },
  @{ Name = "replacement_character"; Text = [string][char]0xFFFD }
)
$GoodText = -join @(
  "city", [char]0x2019, "s ",
  [char]0x201C, "quote", [char]0x201D, " ",
  "a", [char]0x2013, "b ",
  "a", [char]0x2014, "b ",
  "next", [char]0x2192, "step ",
  [char]0x00A9, " ",
  "LONGMONT ", [char]0x00B7, " CO"
)
function Write-Utf8NoBom([string]$Path, [string]$Text) { [IO.File]::WriteAllText($Path, $Text, $Utf8NoBom) }
function Get-Snippet([string]$Text, [int]$Index, [int]$Length) {
  $start = [Math]::Max(0, $Index - 45)
  $end = [Math]::Min($Text.Length, $Index + $Length + 45)
  return $Text.Substring($start, $end - $start).Replace("`r", "\\r").Replace("`n", "\\n")
}
function Scan-Text([string]$Text, [string]$Path, [string]$Scope) {
  $hits = New-Object System.Collections.Generic.List[object]
  foreach ($seq in $BadSequences) {
    $idx = $Text.IndexOf($seq.Text, [StringComparison]::Ordinal)
    while ($idx -ge 0) {
      $line = 1 + ($Text.Substring(0, $idx).ToCharArray() | Where-Object { $_ -eq "`n" }).Count
      $hits.Add([pscustomobject]@{
        scope = $Scope
        file = $Path
        sequence = $seq.Name
        index = $idx
        line = $line
        snippet = Get-Snippet $Text $idx $seq.Text.Length
      })
      $nextStart = $idx + [Math]::Max(1, $seq.Text.Length)
      if ($nextStart -ge $Text.Length) { break }
      $idx = $Text.IndexOf($seq.Text, $nextStart, [StringComparison]::Ordinal)
    }
  }
  return $hits
}
function Scan-File([string]$Path, [string]$Scope) {
  $text = [IO.File]::ReadAllText($Path, [Text.Encoding]::UTF8)
  return Scan-Text $text $Path $Scope
}
$badCanary = Join-Path $EvidenceDir 'canary-bad.txt'
$goodCanary = Join-Path $EvidenceDir 'canary-good.txt'
Write-Utf8NoBom $badCanary ("bad canary: city" + $BadSequences[0].Text + "s LONGMONT " + $BadSequences[7].Text + " CO")
Write-Utf8NoBom $goodCanary $GoodText
$badCanaryHits = @(Scan-File $badCanary 'canary_bad')
$goodCanaryHits = @(Scan-File $goodCanary 'canary_good')
$localOutput = Join-Path $Repo 'test-comms\reports\20260629-full-cleanwipe-longmont-5a24a5a-evidence\publication-output\site'
$downloadedHtml = Join-Path $Repo 'test-comms\reports\20260629-herenow-retest-f092852-evidence\herenow-index.html'
$f092Evidence = Join-Path $Repo 'test-comms\reports\20260629-herenow-retest-f092852-evidence'
$browserText = Join-Path $EvidenceDir 'browser-innerText.txt'
$liveHtml = Join-Path $EvidenceDir 'live-herenow-index.html'
$liveUrl = 'https://merry-frost-9arx.here.now'
$live = [ordered]@{ url = $liveUrl; reachable = $false; statusCode = $null; error = $null; htmlPath = $liveHtml }
try {
  $resp = Invoke-WebRequest -Uri $liveUrl -UseBasicParsing -TimeoutSec 30
  $live.reachable = $true
  $live.statusCode = [int]$resp.StatusCode
  Write-Utf8NoBom $liveHtml $resp.Content
} catch {
  $live.error = $_.Exception.Message
}
$node = 'C:\Users\civic\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\bin\node.exe'
$browserJs = Join-Path $EvidenceDir 'capture-browser-innertext.cjs'
$js = @"
const { chromium } = require('playwright');
const fs = require('fs');
(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: 'C:/Program Files/Google/Chrome/Application/chrome.exe' });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  let result = { url: '$liveUrl', reachable: false, status: null, error: null };
  try {
    const response = await page.goto('$liveUrl', { waitUntil: 'networkidle', timeout: 45000 });
    result.status = response ? response.status() : null;
    result.reachable = !!response && response.ok();
    const text = await page.evaluate(() => document.body ? document.body.innerText : '');
    fs.writeFileSync(process.argv[2], text, { encoding: 'utf8' });
    await page.screenshot({ path: process.argv[3], fullPage: true });
  } catch (err) {
    result.error = err && err.message ? err.message : String(err);
  } finally {
    await browser.close();
    fs.writeFileSync(process.argv[4], JSON.stringify(result, null, 2), { encoding: 'utf8' });
  }
})();
"@
Write-Utf8NoBom $browserJs $js
$browserResultPath = Join-Path $EvidenceDir 'browser-capture-result.json'
$browserPng = Join-Path $EvidenceDir 'browser-rendered-page.png'
if ($live.reachable) {
  $env:NODE_PATH='C:\Users\civic\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\node_modules;C:\Users\civic\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\node_modules\.pnpm\node_modules'
  & $node $browserJs $browserText $browserPng $browserResultPath | Out-Null
  if ($LASTEXITCODE -ne 0) { throw 'Browser capture failed' }
} else {
  Write-Utf8NoBom $browserResultPath (@{ url = $liveUrl; reachable = $false; skipped = $true; reason = 'Live URL unreachable during Invoke-WebRequest precheck' } | ConvertTo-Json -Depth 5)
}
$extensions = @('.html','.xml','.md','.txt','.json','.css','.js')
$allHits = New-Object System.Collections.Generic.List[object]
$scopeHits = [ordered]@{}
function Add-ScopeHits([string]$Scope, [object[]]$Hits) {
  $scopeHits[$Scope] = @($Hits)
  foreach ($hit in $Hits) { $allHits.Add($hit) }
}
$localFiles = @(Get-ChildItem -LiteralPath $localOutput -Recurse -File | Where-Object { $extensions -contains $_.Extension.ToLowerInvariant() })
$localHits = @($localFiles | ForEach-Object { Scan-File $_.FullName 'local_output' })
Add-ScopeHits 'local_output' $localHits
$downloadHits = @()
if (Test-Path -LiteralPath $downloadedHtml) { $downloadHits = @(Scan-File $downloadedHtml 'downloaded_herenow_html') }
Add-ScopeHits 'downloaded_herenow_html' $downloadHits
$liveHtmlHits = @()
if (Test-Path -LiteralPath $liveHtml) { $liveHtmlHits = @(Scan-File $liveHtml 'live_herenow_html') }
Add-ScopeHits 'live_herenow_html' $liveHtmlHits
$browserHits = @()
if (Test-Path -LiteralPath $browserText) { $browserHits = @(Scan-File $browserText 'browser_innerText') }
Add-ScopeHits 'browser_innerText' $browserHits
$f092Files = @(Get-ChildItem -LiteralPath $f092Evidence -Recurse -File | Where-Object { $extensions -contains $_.Extension.ToLowerInvariant() })
$f092Hits = @($f092Files | ForEach-Object { Scan-File $_.FullName 'f092852_existing_evidence' })
Add-ScopeHits 'f092852_existing_evidence' $f092Hits
$result = [ordered]@{
  generatedUtc = (Get-Date).ToUniversalTime().ToString('o')
  scanner = [ordered]@{
    badCanaryCaught = ($badCanaryHits.Count -gt 0)
    badCanaryHitCount = $badCanaryHits.Count
    goodCanaryAllowed = ($goodCanaryHits.Count -eq 0)
    goodCanaryHitCount = $goodCanaryHits.Count
    badCanaryHits = $badCanaryHits
    goodCanaryHits = $goodCanaryHits
  }
  live = $live
  inputs = [ordered]@{
    localOutput = $localOutput
    downloadedHtml = $downloadedHtml
    f092Evidence = $f092Evidence
    browserInnerText = $browserText
  }
  counts = [ordered]@{
    localOutputFiles = $localFiles.Count
    f092EvidenceTextFiles = $f092Files.Count
    localOutputHits = $localHits.Count
    downloadedHtmlHits = $downloadHits.Count
    liveHtmlHits = $liveHtmlHits.Count
    browserInnerTextHits = $browserHits.Count
    f092EvidenceHits = $f092Hits.Count
    totalRealInputHits = $allHits.Count
  }
  hitsByScope = $scopeHits
}
$json = $result | ConvertTo-Json -Depth 20
Write-Utf8NoBom (Join-Path $EvidenceDir 'mojibake-audit.json') $json
if (($badCanaryHits.Count -gt 0) -and ($goodCanaryHits.Count -eq 0)) { exit 0 } else { exit 2 }