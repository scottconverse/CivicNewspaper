# Corrected Mojibake Evidence Audit Report - f092852

Status: PASS

Directive: `test-comms/directives/20260629-mojibake-evidence-audit-f092852.md`

Product branch: `stable-readiness-local-gates`

Product commit: `f092852e9df3808f16cf56b829993f028e31d255`

Evidence folder: `test-comms/reports/20260629-mojibake-evidence-audit-f092852-evidence/`

Live here.now URL audited:

`https://merry-frost-9arx.here.now`

## Summary

The corrected scanner found no bad mojibake sequences in the local generated output, the downloaded here.now HTML from the f092852 retest, the still-live here.now HTML, browser-rendered public text, or the committed f092852 JSON evidence files.

The live here.now URL was reachable during this audit with HTTP 200. Browser capture succeeded through local Chrome, and the rendered `document.body.innerText` scan was clean.

Conclusion: neither real product/public-output mojibake nor committed tester evidence serialization mojibake was found by the required codepoint-based scanner. The earlier mojibake-looking concern appears to have been a display/reading-path artifact, not bytes present in the public output or committed f092852 JSON evidence.

## Scanner Canary Results

- Synthetic bad canary caught: yes
- Synthetic bad canary hit count: 2
- Synthetic legitimate Unicode canary allowed: yes
- Synthetic legitimate Unicode canary hit count: 0

The bad canary produced hits for these required scanner names:

- `curly_apostrophe_mojibake`
- `middle_dot_mojibake`

The legitimate Unicode canary contained right curly apostrophe, curly quotes, en dash, em dash, right arrow, copyright sign, and middle dot by codepoint construction. It produced zero hits.

## Public Output Scan Results

Local generated output folder:

`test-comms/reports/20260629-full-cleanwipe-longmont-5a24a5a-evidence/publication-output/site`

- Text files scanned: 21
- Bad-sequence hits: 0

Downloaded here.now HTML from f092852 evidence:

`test-comms/reports/20260629-herenow-retest-f092852-evidence/herenow-index.html`

- Bad-sequence hits: 0

Live here.now HTML captured during this audit:

`test-comms/reports/20260629-mojibake-evidence-audit-f092852-evidence/live-herenow-index.html`

- URL reachable: yes
- HTTP status: 200
- Bad-sequence hits: 0

Browser-rendered public text:

`test-comms/reports/20260629-mojibake-evidence-audit-f092852-evidence/browser-innerText.txt`

- Browser capture: succeeded
- Bad-sequence hits: 0

## Existing f092852 JSON Evidence

Existing f092852 evidence folder scanned:

`test-comms/reports/20260629-herenow-retest-f092852-evidence/`

- Text evidence files scanned: 5
- Bad-sequence hits: 0

No exact files/snippets are listed for real inputs because the scanner found zero real-input hits.

## Evidence Files

- `run-mojibake-audit.ps1`
- `mojibake-audit.json`
- `canary-bad.txt`
- `canary-good.txt`
- `live-herenow-index.html`
- `browser-innerText.txt`
- `browser-rendered-page.png`
- `browser-capture-result.json`
- `capture-browser-innertext.cjs`
- `evidence-file-list.txt`

## Final Conclusion

PASS. The required scanner proved it catches bad mojibake and allows legitimate Unicode. The local output, downloaded here.now HTML, live here.now HTML, browser-rendered public text, and committed f092852 JSON evidence are clean. Classification: neither real product/public-output mojibake nor tester evidence serialization mojibake found in the audited committed inputs.