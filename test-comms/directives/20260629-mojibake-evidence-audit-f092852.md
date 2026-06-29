# Tester Directive: Corrected Mojibake Evidence Audit

Status: ACTIVE

Supersedes: `test-comms/directives/20260629-herenow-retest-f092852.md`.

Reason: the f092852 here.now retest reported PASS and produced `https://merry-frost-9arx.here.now`, but its JSON evidence contains mojibake-looking strings such as `cityâ€™s` and `LONGMONT Â· CO`. The downloaded HTML and screenshots appear clean from the coder side. This directive determines whether the remaining problem is real public output or tester evidence serialization.

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `f092852e9df3808f16cf56b829993f028e31d255`

Report path: `test-comms/reports/20260629-mojibake-evidence-audit-f092852-report.md`

Artifact evidence path: `test-comms/reports/20260629-mojibake-evidence-audit-f092852-evidence/`

## Machine Context

You are the tester on the separate cleanroom machine running as `msi\civic`.

Use this coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct`; that is the coder machine path and is invalid on the tester machine except as a warning example.

All new report and evidence files must be UTF-8 without BOM. Do not write UTF-16 evidence files.

## Scope

Do not reinstall, wipe, regenerate stories, or republish unless `https://merry-frost-9arx.here.now` has expired or is unreachable.

Audit these existing inputs:

- The local generated output folder used in the f092852 retest:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-cleanwipe-longmont-5a24a5a-evidence\publication-output\site`

- The f092852 downloaded here.now HTML:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-herenow-retest-f092852-evidence\herenow-index.html`

- The live here.now URL if still available:

`https://merry-frost-9arx.here.now`

- The f092852 JSON evidence files:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-herenow-retest-f092852-evidence\*.json`

## Critical Correction

These are mojibake and must fail when they appear in public output or in newly written evidence text:

- `â€™`
- `â€œ`
- `â€`
- `â€“`
- `â€”`
- `â†’`
- `Â©`
- `Â·`
- `Ã¢`
- the replacement character `�`

These are legitimate Unicode and must not fail:

- right curly apostrophe
- left/right curly quotation marks
- en dash
- em dash
- right arrow
- copyright sign
- middle dot

Do not type literal legitimate Unicode into this directive report from memory. Use codepoint construction in the scanner below.

## Required Scanner

Create and run a PowerShell scanner that:

1. Builds bad mojibake sequences by codepoint.
2. Builds legitimate Unicode sequences by codepoint.
3. Verifies the scanner catches a synthetic bad canary file before scanning real evidence.
4. Verifies the scanner does not fail a synthetic legitimate Unicode canary file.
5. Scans all `.html`, `.xml`, `.md`, `.txt`, `.json`, `.css`, and `.js` files in the local output folder, the downloaded here.now HTML, and the f092852 evidence folder.
6. Saves machine-readable results to `test-comms/reports/20260629-mojibake-evidence-audit-f092852-evidence/mojibake-audit.json`.

Use these exact bad sequences:

```powershell
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
```

Use these legitimate canaries:

```powershell
$GoodText = -join @(
  "city", [char]0x2019, "s ",
  [char]0x201C, "quote", [char]0x201D, " ",
  "a", [char]0x2013, "b ",
  "a", [char]0x2014, "b ",
  "next", [char]0x2192, "step ",
  [char]0x00A9, " ",
  "LONGMONT ", [char]0x00B7, " CO"
)
```

## Browser Rendered Text Check

Open `https://merry-frost-9arx.here.now` in a browser if still live. Capture `document.body.innerText` and save it as UTF-8 without BOM. Then run the same bad-sequence scanner against that file.

If the live URL has expired, say so and scan the downloaded HTML plus local output only.

## Required Report Contents

The report must include:

- PASS or FAIL.
- Whether the live here.now URL was still reachable.
- Whether local output files contain any bad sequences.
- Whether downloaded here.now HTML contains any bad sequences.
- Whether browser-rendered innerText contains any bad sequences.
- Whether existing f092852 JSON evidence contains bad sequences.
- Whether the scanner canary test caught the synthetic bad file.
- Whether the scanner canary test passed the synthetic legitimate Unicode file.
- Exact files/snippets for any bad-sequence hits.
- Clear conclusion:
  - real product/public-output mojibake,
  - tester evidence serialization mojibake only,
  - both,
  - or neither.

## Pass / Fail Bar

PASS only if:

- Scanner canary tests prove the scanner catches bad mojibake and allows legitimate Unicode.
- Local public output has no bad sequences.
- Downloaded/live here.now public output has no bad sequences.
- Browser-rendered public text has no bad sequences, if the live URL is still reachable.

FAIL if any public output contains bad sequences.

If only old JSON evidence contains bad sequences, mark the audit PASS WITH TESTER-EVIDENCE WARNING, and explain that the product output is clean but the tester evidence writer must be corrected for future reports.

Commit the report and evidence with `[skip ci]`.
