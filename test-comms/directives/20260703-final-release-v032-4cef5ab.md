# Final Cleanroom Release Verification - The Civic Desk v0.3.2 Repair Rerun

Tester: run this from `msi\civic` on the cleanroom Windows tester machine.

## Source Of Truth

This file is archived from `test-comms/ACTIVE_DIRECTIVE.md`. The active directive remains the source of truth while this test is running.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Release/docs commit on `main`: `8ac4fc3eb7246205074960c638ab1f3eaa0bde44`
- Product build commit embedded in the Windows app build: `4cef5ab218b6fe7b6167f143a0db57377e6ac3fe`
- Windows installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Windows installer size: `5232809` bytes
- Windows installer SHA256: `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`
- Checksum asset: `SHA256SUMS.txt`

Do not use any installer copied from `C:\Users\instynct`. Download the installer and checksum file from the GitHub release URL above.

## Required Reports

Write these reports on this branch:

- Visibility: `test-comms/reports/20260703-final-release-v032-4cef5ab-visibility.md`
- Full report: `test-comms/reports/20260703-final-release-v032-4cef5ab-report.md`

Also attach screenshots, logs, exported ZIP path notes, downloaded release asset notes, and publication URLs under:

- `test-comms/evidence/20260703-final-release-v032-4cef5ab/`

## Visibility Check

Before installing, write the visibility report confirming:

1. You are on machine/user `msi\civic`.
2. Your repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
3. Branch is `test-comms/cleanroom-coder-tester`.
4. You read `test-comms/ACTIVE_DIRECTIVE.md`.
5. You can reach the GitHub release URL and public docs URL.
6. The release page shows installer SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A` and does not show any unreplaced shell variable where the checksum should be.
7. The public docs URL shows installer SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`, `More info`, and `Run anyway`.

## Cleanroom Test Procedure

1. Pull this branch and verify this ACTIVE_DIRECTIVE is current.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release page.
3. Verify the installer SHA256 exactly matches `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`.
4. Verify `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
5. Confirm the release page has exactly one Windows installer asset and one checksum asset for this release, with no stale duplicate checksum asset.
6. Confirm the release page and public docs explain that the Windows installer is unsigned, why SmartScreen may warn, and that the expected install path is `More info` then `Run anyway` when the hash matches.
7. Clean uninstall/remove prior The Civic Desk app state for this tester profile, then install from the downloaded GitHub release installer.
8. Launch the app from the installed Windows app/shortcut, not from source.
9. Complete first-run setup for Longmont, Colorado using the app-guided local AI setup. If the local model is already installed, note that. If setup downloads or verifies a model, note that.
10. Verify core UI navigation: onboarding, source discovery/import, Daily Scan, Workbench, Publishing, Settings, backup/restore visibility, and diagnostics visibility. Confirm no route bounces to the top nav instead of the intended workflow.
11. Run Longmont source discovery/import. Confirm it produces reusable, non-city-hardcoded source behavior and does not depend on coder-machine paths.
12. Run Daily Scan and inspect leads. Confirm weak, generic, recurring, unsupported, non-jurisdictional, navigation/index, markup-debris, multi-item event listing, and broad legislative/chamber rollup items are clearly labeled, suppressed, or downgraded rather than presented as strong ready-to-publish news.
13. Specifically check that malformed leads like encoded `&#8211;` text, stray `-->` text, multi-event calendar concatenations, generic chamber pages, broad state legislative pages, and old static background pages are not draftable as strong news stories.
14. Draft at least one credible Longmont story from linked source evidence. Use the explicit `Draft` or `Open Draft` control; do not rely on clicking passive card text.
15. Confirm public copy has a real headline, no reporter scaffolding, no `EDITOR_NOTE`, no `Body:`, no `Headline:`, no `Nut graf`, no `[Source needed]`, no `[Verification needed]`, no `[End of Report]`, no mojibake marker code points, and no unsupported facts.
16. Exercise editor workflow including hold/send-back/approve/cut where available. Confirm a held draft can be clearly sent back for more work.
17. Compile/export the static site and record the exported ZIP path.
18. Publish through the default here.now anonymous preview flow. Record the public here.now URL.
19. Inspect the public here.now site as a visitor. Confirm there are no duplicate topic stories, no raw Markdown, no internal tester/developer paths, no unsupported Mac/Linux installer claims, and no broken story links.
20. Inspect the public docs/landing page from https://scottconverse.github.io/CivicNewspaper/ and the release page. Confirm v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, discussion links, and release download links are all present and honest.
21. Write a full human-readable report with PASS/FAIL and explicit blocker/critical/major/minor findings. The release only passes if there are zero blocker, critical, and major findings.

## Do Not Do

- Do not merge, tag, or publish product code.
- Do not use an installer copied from the coder machine.
- Do not treat historical Mac/Linux artifacts as release-supported.
- Do not omit failures because they seem small.
