# Tester Directive: Rerun Systemic Mojibake Fix

Status: ACTIVE

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `59eb271d323b0e051a01659494958594b6384cf1`

Artifact folder: `test-comms/artifacts/20260629-rerun-mojibake-systemic-59eb271/`

Preferred installer:

`test-comms/artifacts/20260629-rerun-mojibake-systemic-59eb271/The Civic Desk_0.2.8_x64-setup.exe`

Expected preferred NSIS SHA256:

`0864D76EB0A382A641B03C1A3A65D6B4D6220307DC73FE764C95031E96F02B93`

Fallback MSI:

`test-comms/artifacts/20260629-rerun-mojibake-systemic-59eb271/The Civic Desk_0.2.8_x64_en-US.msi`

Expected fallback MSI SHA256:

`1DC37C593240EECC186486A6F2B750FD10CD69DFAE652043B7A4748DC88AF272`

Report path:

`test-comms/reports/20260629-mojibake-systemic-59eb271-report.md`

Artifact evidence path:

`test-comms/reports/20260629-mojibake-systemic-59eb271-evidence/`

## Machine Context

You are the tester on the separate cleanroom machine running as `msi\civic`.

Use this coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct`; that is the coder machine path and is invalid on the tester machine except as a warning example.

## Why This Rerun Exists

The previous tester report correctly found that the published output was vulnerable to mojibake. The coder then fixed the product compiler so common Windows-1252-as-UTF-8 damage is repaired at the publication boundary, removed broken literal mojibake strings from tracked source/tests, normalized one tracked UTF-16 Markdown artifact to UTF-8, and fixed generated draft titles so newly created drafts no longer receive a hard-coded `Draft:` prefix.

Important: do not fail this rerun merely because the finished output contains legitimate Unicode characters such as curly apostrophe, copyright sign, or right arrow. Those are valid characters. This test must fail only when the decoded output contains known mojibake sequences.

## Required Steps

1. Fetch and read `test-comms/ACTIVE_DIRECTIVE.md`. Confirm it points to this directive.
2. Verify the installer hash before running it.
3. Install or upgrade with the NSIS installer. Use the MSI only if NSIS fails, and document why.
4. Launch The Civic Desk.
5. Recompile/export the existing Longmont issue that previously showed mojibake.
6. Publish anonymously to here.now. This publish is authorized for this test.
7. Export the ZIP/package from the app.
8. Save the generated output path, ZIP path, here.now URL, screenshots, and any app-visible warnings/errors.
9. Run the exact UTF-8 mojibake scanner below against the exported output folder and, if locally accessible, the published/downloaded here.now HTML files.
10. Write the report to the report path above and commit it with `[skip ci]`.

## Exact Mojibake Scanner

Use this scanner rather than broad visual searches. It constructs known bad decoded sequences by codepoint, so the report is not corrupted by terminal or Markdown encoding.

```powershell
$OutputRoot = Read-Host "Paste the exported output folder path"
if (-not (Test-Path -LiteralPath $OutputRoot)) {
  throw "Output folder does not exist: $OutputRoot"
}
$BadSequences = @(
  @{ Name = "copyright_mojibake"; Text = -join ([char]0x00C2, [char]0x00A9) },
  @{ Name = "right_arrow_mojibake"; Text = -join ([char]0x00E2, [char]0x2020, [char]0x2019) },
  @{ Name = "curly_apostrophe_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x2122) },
  @{ Name = "left_double_quote_mojibake"; Text = -join ([char]0x00E2, [char]0x20AC, [char]0x0153) },
  @{ Name = "double_encoded_utf8_starter"; Text = -join ([char]0x00C3, [char]0x00A2) }
)

$Hits = @()
Get-ChildItem -LiteralPath $OutputRoot -Recurse -File |
  Where-Object { $_.Extension -in ".html", ".xml", ".md", ".txt", ".json", ".css", ".js" } |
  ForEach-Object {
    $Path = $_.FullName
    $Text = [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
    foreach ($Bad in $BadSequences) {
      $Index = $Text.IndexOf($Bad.Text, [StringComparison]::Ordinal)
      if ($Index -ge 0) {
        $Start = [Math]::Max(0, $Index - 40)
        $Length = [Math]::Min(100, $Text.Length - $Start)
        $Snippet = $Text.Substring($Start, $Length).Replace("`r", "\r").Replace("`n", "\n")
        $Codepoints = ($Bad.Text.ToCharArray() | ForEach-Object { "U+{0:X4}" -f [int]$_ }) -join " "
        $Hits += [pscustomobject]@{
          File = $Path
          Sequence = $Bad.Name
          Codepoints = $Codepoints
          Snippet = $Snippet
        }
      }
    }
  }

if ($Hits.Count -eq 0) {
  "PASS: no known mojibake sequences found"
} else {
  "FAIL: known mojibake sequences found"
  $Hits | Format-List
  exit 1
}
```

## Acceptance Criteria

Pass only if:

- The app installs/upgrades from the provided artifact.
- Existing Longmont output can compile/export.
- here.now anonymous publish succeeds and returns a URL.
- The exported output ZIP/package exists on disk.
- The exact scanner above reports no known mojibake sequences.
- Stale killed-story pages remain absent from the generated output.
- Killed stories remain visibly protected from accidental approval.

Do not fail because legitimate characters such as `’`, `©`, or `→` appear in output. Those are expected to survive as valid UTF-8.

If a newly generated draft is created during this run, verify its title does not start with `Draft:`. Do not use pre-existing old database rows to judge this behavior, because old rows may keep old titles until regenerated or renamed.
