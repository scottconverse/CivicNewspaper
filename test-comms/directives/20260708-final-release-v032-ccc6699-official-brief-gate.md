# Final Cleanroom Release Verification - The Civic Desk v0.3.2 ccc6699 Official Brief Gate Rerun

Tester: run this from `msi\civic` on the cleanroom Windows tester machine.

## Source Of Truth

This file is the active directive. Ignore older directive filenames.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Product build commit embedded in the Windows app build: `ccc66997031d027e6187b4c1d5f95e117c7c8ac8`
- Release/docs commit on `main`: `e41c8cd5319dc954feb8f409766fd8ec36a65b8f`
- Windows installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Windows installer size: `5239817` bytes
- Windows installer SHA256: `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`
- Checksum asset: `SHA256SUMS.txt`

Do not use any installer copied from `C:\Users\instynct`. Download the installer and checksum file from the GitHub release URL above.

## Required Reports

Write these reports on this branch:

- Visibility: `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-visibility.md`
- Full report: `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-report.md`

Attach screenshots, logs, exported ZIP path notes, downloaded release asset notes, SQLite/app-data evidence, and publication URLs under:

- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/`

## Why This Rerun Exists

The previous `1467846` weak-lead and brief-gate rerun proved that unsupported/no-evidence Daily Scan leads are now low-priority verification work, `Draft anyway` is gone for weak queue cards, and Brief/Story format defaults no longer fall back to Watch. It still failed final cleanroom because the fresh Longmont run did not yield a credible linked-evidence Story or Brief lead to draft, and some verification cards still displayed raw page navigation/category debris.

Coder fixes in `ccc66997031d027e6187b4c1d5f95e117c7c8ac8`:

- Official `New Primary Record` leads remain `Watch` for generic home, directory, search, navigation, or index pages.
- Official `New Primary Record` leads become draftable `Brief` leads only when linked source evidence contains concrete civic action, timeline, meeting/deadline, amount, or public-impact language.
- Draftable official-record Brief leads keep their linked `lead_evidence` grounding.
- Daily Scan quality-gated verification cards rewrite raw navigation/category/index debris into a clean `Verify source-quality issue from ...` queue card instead of leading with scraped page chrome.
- Existing weak-lead protections remain in force: unsupported/no-source/model-suggested items are downgraded before Story Queue, weak/watch/background/verification/low-novelty leads show verification-first behavior, and Story/Brief drafting defaults to Brief rather than Watch.

## Visibility Check

Before installing, write the visibility report confirming:

1. You are on machine/user `msi\civic`.
2. Your repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
3. Branch is `test-comms/cleanroom-coder-tester`.
4. You read `test-comms/ACTIVE_DIRECTIVE.md`.
5. You can reach the GitHub release URL and public docs URL.
6. The release page shows installer SHA256 `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`, size `5239817`, and product commit `ccc66997031d027e6187b4c1d5f95e117c7c8ac8`.
7. The release page has exactly one Windows installer asset and one checksum asset.
8. `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains SHA256 `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`.
9. The public docs URL shows installer SHA256 `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`, `More info`, `Run anyway`, Windows-only beta language, and no stale `6CD5B8C6D3565AFAE8A39357DEAEC1CE53ADEDADB8316BEB6C44DCB86C87EE74` hash.

## Cleanroom Test Procedure

1. Pull this branch and verify this ACTIVE_DIRECTIVE is current.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release page.
3. Verify the installer SHA256 exactly matches `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`.
4. Verify `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
5. Confirm the release page and public docs explain that the Windows installer is unsigned, why SmartScreen may warn, and that the expected install path is `More info` then `Run anyway` when the hash matches.
6. Uninstall any prior The Civic Desk app instance so the installer lifecycle is tested. Do not delete unrelated user data.
7. Record whether these state paths exist before this test: `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%APPDATA%\The Civic Desk`, `%LOCALAPPDATA%\The Civic Desk`, `%APPDATA%\org.civicnews.app`, and `%LOCALAPPDATA%\org.civicnews.app`.
8. Install from the downloaded GitHub release installer.
9. Create and use a fresh isolated app-data folder for this cleanroom run:

```powershell
$cleanProfile = Join-Path $env:TEMP "civicdesk-final-v032-ccc6699-official-brief-gate"
Remove-Item -LiteralPath $cleanProfile -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $cleanProfile -Force | Out-Null
$env:CIVICNEWS_APP_DATA_DIR = $cleanProfile
& "$env:LOCALAPPDATA\The Civic Desk\civicnews.exe"
```

10. Launch only the installed EXE from the GitHub release installer. Do not launch from source. Keep the `CIVICNEWS_APP_DATA_DIR` environment variable set in the PowerShell process that starts the installed EXE.
11. Verify the first screen is first-run onboarding, not inherited Longmont state.
12. Complete first-run setup for Longmont, Colorado using the app-guided local AI setup. Use the app controls as a new user would. If direct field automation is unreliable, use the visible `Longmont` starter profile.
13. After onboarding finishes, do not manually repair city/state in Settings before the first Daily Scan. Go directly to Daily Scan and run it. This must pass without the prior `Choose your publication city and state in Settings before running Daily Scan.` blocker.
14. Capture the saved `community_profile.json` and any relevant settings/SQLite evidence showing city `Longmont` and state `CO` after onboarding and after Daily Scan.
15. Run source discovery/import twice if practical: once using `Longmont` / `Colorado`, and once using `Longmont` / `CO`. Full state name input should not produce an empty no-guidance result.
16. Run Daily Scan and inspect leads. Confirm unsupported/no-source/model-suggested items are low-priority verification work, not high-priority or ready-to-draft story work.
17. Specifically check that leads with no linked `lead_evidence` are not presented as strong draftable stories, do not show `Draft anyway`, and that Workbench offers verification-note behavior rather than a normal draft path for those leads.
18. Confirm weak, generic, recurring, unsupported, non-jurisdictional, navigation/index, markup-debris, multi-item event listing, and broad legislative/chamber rollup items are clearly labeled, suppressed, downgraded, or separated from ordinary publishable story leads.
19. Find at least one credible Longmont lead with linked evidence and Story or Brief treatment. Confirm the Workbench Article Format selector defaults to Brief for Story/Brief leads, not Watch.
20. Draft at least one credible Longmont story or brief from linked source evidence. Use the explicit `Draft` or `Open draft` control; do not rely on clicking passive card text.
21. Confirm public copy has a real headline, coherent reader-facing body, no watch-fragment phrasing for Brief output, no reporter scaffolding, no `EDITOR_NOTE`, no `Body:`, no `Headline:`, no `Nut graf`, no `[Source needed]`, no `[Verification needed]`, no `[End of Report]`, no mojibake marker code points, no encoded HTML entity leakage, and no unsupported facts.
22. Exercise editor workflow including hold/send-back/approve/cut where available. Confirm a held draft can be clearly sent back for more work.
23. Compile/export the static site and record the exported ZIP path.
24. Publish through the default here.now anonymous preview flow. This anonymous here.now preview is authorized for this release cleanroom test. Do not use credentialed external providers.
25. Record the public here.now URL.
26. Inspect the public here.now site as a visitor. Confirm there are no duplicate topic stories, no raw Markdown, no internal tester/developer paths, no unsupported Mac/Linux installer claims, and no broken story links.
27. Inspect the public docs/landing page from https://scottconverse.github.io/CivicNewspaper/ and the release page. Confirm v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, discussion links, and release download links are all present and honest.
28. Write a full human-readable report with PASS/FAIL and explicit blocker/critical/major/minor findings. The release only passes if there are zero blocker, critical, and major findings.

## Do Not Do

- Do not merge, tag, or publish product code.
- Do not use an installer copied from the coder machine.
- Do not treat historical Mac/Linux artifacts as release-supported.
- Do not omit failures because they seem small.
- Do not use any `C:\Users\instynct` path except to confirm that it is forbidden as a tester input.
