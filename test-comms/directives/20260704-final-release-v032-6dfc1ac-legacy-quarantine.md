# Final Cleanroom Release Verification - The Civic Desk v0.3.2 6dfc1ac Legacy Quarantine Rerun

Tester: run this from `msi\civic` on the cleanroom Windows tester machine.

## Source Of Truth

This file is archived from `test-comms/ACTIVE_DIRECTIVE.md`. If the active directive changes, follow `test-comms/ACTIVE_DIRECTIVE.md`.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Release/docs commit on `main`: `4c01de4488e311851a846d6ed4ef5421a125a24b`
- Product build commit embedded in the Windows app build: `6dfc1ac8239a920251fca44d1fdbc9e5a960c58b`
- Windows installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Windows installer size: `5253971` bytes
- Windows installer SHA256: `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`
- Checksum asset: `SHA256SUMS.txt`

Do not use any installer copied from `C:\Users\instynct`. Download the installer and checksum file from the GitHub release URL above.

## Required Reports

Write these reports on this branch:

- Visibility: `test-comms/reports/20260704-final-release-v032-6dfc1ac-legacy-quarantine-visibility.md`
- Full report: `test-comms/reports/20260704-final-release-v032-6dfc1ac-legacy-quarantine-report.md`

Also attach screenshots, logs, exported ZIP path notes, downloaded release asset notes, and publication URLs under:

- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/`

## Why This Rerun Exists

The prior `eab6a31` cleanroom run failed because stale malformed calendar/listing leads and drafts could survive reinstall/profile cleanup and remain visible as draftable/openable work. It also still leaked encoded text such as `&#8211;`, mojibake bullets, and copied multi-event listing text into Workbench.

This build adds shared public-text quality normalization and legacy database remediation:

1. Old encoded/mojibake evidence excerpts are cleaned before display.
2. Malformed scan/story leads are downgraded to verification-needed, low-priority items rather than strong news.
3. Contaminated existing drafts are cut, detached from leads, and replaced with a quarantine note.
4. Quarantined draft rows must not appear as `Ready to draft`, `Open draft`, Workbench body copy, or publishable source text.

## Visibility Check

Before installing, write the visibility report confirming:

1. You are on machine/user `msi\civic`.
2. Your repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
3. Branch is `test-comms/cleanroom-coder-tester`.
4. You read `test-comms/ACTIVE_DIRECTIVE.md`.
5. You can reach the GitHub release URL and public docs URL.
6. The release page shows installer SHA256 `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`, size `5253971`, and does not show any older checksum from a prior rerun.
7. `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains SHA256 `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`.
8. The public docs URL shows installer SHA256 `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`, `More info`, and `Run anyway`.

## Cleanroom Test Procedure

1. Pull this branch and verify this ACTIVE_DIRECTIVE is current.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release page.
3. Verify the installer SHA256 exactly matches `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`.
4. Verify `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
5. Confirm the release page has exactly one Windows installer asset and one checksum asset for this release, with no stale duplicate checksum asset.
6. Confirm the release page and public docs explain that the Windows installer is unsigned, why SmartScreen may warn, and that the expected install path is `More info` then `Run anyway` when the hash matches.
7. Clean uninstall/remove prior The Civic Desk app state for this tester profile before installing.
8. Record whether any of these state paths existed before cleanup, then remove them if present: `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%APPDATA%\The Civic Desk`, and `%LOCALAPPDATA%\The Civic Desk`.
9. Install from the downloaded GitHub release installer.
10. Launch the app from the installed Windows app/shortcut, not from source.
11. Complete first-run setup for Longmont, Colorado using the app-guided local AI setup. If local AI is unavailable, verify that the AI setup step exposes usable above-the-fold controls, including `Skip for now`, and that skipping reaches the rest of setup without trapping focus in diagnostics.
12. Verify core UI navigation: onboarding, source discovery/import, Daily Scan, Workbench, Publishing, Settings, backup/restore visibility, and diagnostics visibility. Confirm no route bounces to the top nav instead of the intended workflow.
13. Run Longmont source discovery/import. Confirm it produces reusable, non-city-hardcoded source behavior and does not depend on coder-machine paths.
14. Run Daily Scan and inspect leads. Confirm weak, generic, recurring, unsupported, non-jurisdictional, navigation/index, markup-debris, multi-item event listing, and broad legislative/chamber rollup items are clearly labeled, suppressed, or downgraded rather than presented as strong ready-to-publish news.
15. Specifically check that malformed leads like encoded `&#8211;` text, stray `-->` text, mojibake bullets, multi-event calendar concatenations, generic chamber pages, broad state legislative pages, and old static background pages are not draftable as strong news stories.
16. If inherited old Longmont state appears despite cleanup, verify the remediation behavior: no malformed event rollup is `Ready to draft`; no malformed event rollup has an attached `Open draft`; any old contaminated draft is cut or quarantined with title `Quarantined draft needs fresh source review` and no copied event-list body text.
17. Draft at least one credible Longmont story from linked source evidence. Use the explicit `Draft` or `Open Draft` control; do not rely on clicking passive card text.
18. Confirm public copy has a real headline, no reporter scaffolding, no `EDITOR_NOTE`, no `Body:`, no `Headline:`, no `Nut graf`, no `[Source needed]`, no `[Verification needed]`, no `[End of Report]`, no mojibake marker code points, no encoded HTML entity leakage, and no unsupported facts.
19. Exercise editor workflow including hold/send-back/approve/cut where available. Confirm a held draft can be clearly sent back for more work.
20. Compile/export the static site and record the exported ZIP path.
21. Publish through the default here.now anonymous preview flow. Record the public here.now URL.
22. Inspect the public here.now site as a visitor. Confirm there are no duplicate topic stories, no raw Markdown, no internal tester/developer paths, no unsupported Mac/Linux installer claims, and no broken story links.
23. Inspect the public docs/landing page from https://scottconverse.github.io/CivicNewspaper/ and the release page. Confirm v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, discussion links, and release download links are all present and honest.
24. Write a full human-readable report with PASS/FAIL and explicit blocker/critical/major/minor findings. The release only passes if there are zero blocker, critical, and major findings.

## Do Not Do

- Do not merge, tag, or publish product code.
- Do not use an installer copied from the coder machine.
- Do not treat historical Mac/Linux artifacts as release-supported.
- Do not omit failures because they seem small.
- Do not use any `C:\Users\instynct` path except to confirm that it is forbidden as a tester input.
