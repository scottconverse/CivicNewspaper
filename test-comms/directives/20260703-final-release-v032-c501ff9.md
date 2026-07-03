# Final Cleanroom Release Verification - The Civic Desk v0.3.2

Tester: run this from `msi\civic` on the cleanroom Windows tester machine.

## Source Of Truth

This file is the active directive. Ignore older directive filenames.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Release target commit shown by GitHub release metadata: `c501ff9ce2c81757b4a6f67ac21c8eede9d696fd`
- Product build commit embedded in the Windows app build: `fa39c39d2cdb9e96df851c971992de8eb3720513`
- Windows installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Windows installer size: `5214089` bytes
- Windows installer SHA256: `9C3B6670A445233C0CDAF98F49505A89C6D88E034DD391471357762092872533`
- Checksum asset: `SHA256SUMS`
- Public docs: https://scottconverse.github.io/CivicNewspaper/

Do not use any local installer from `C:\Users\instynct`. Download the installer and SHA256SUMS from the GitHub release URL above.

## Required Reports

Write these reports on this branch:

- Visibility: `test-comms/reports/20260703-final-release-v032-c501ff9-visibility.md`
- Full report: `test-comms/reports/20260703-final-release-v032-c501ff9-report.md`

Also attach any screenshots, logs, exported ZIP path notes, or publication URLs under `test-comms/evidence/20260703-final-release-v032-c501ff9/`.

## Visibility Check

Before installing, write the visibility report confirming:

1. You are on machine/user `msi\civic`.
2. Your repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
3. Branch is `test-comms/cleanroom-coder-tester`.
4. You read `test-comms/ACTIVE_DIRECTIVE.md`.
5. You can reach the GitHub release URL and public docs URL.

## Cleanroom Test Procedure

1. Pull this branch and verify this ACTIVE_DIRECTIVE is current.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS` from the GitHub release page.
3. Verify the installer SHA256 exactly matches `9C3B6670A445233C0CDAF98F49505A89C6D88E034DD391471357762092872533` and the `SHA256SUMS` file.
4. Confirm the release page and install docs explain that the Windows installer is unsigned, why SmartScreen may warn, and that the expected install path is **More info** then **Run anyway** when the hash matches.
5. Clean uninstall/remove prior The Civic Desk app state for this tester profile, then install from the downloaded GitHub release installer.
6. Launch the app from the installed Windows app/shortcut, not from source.
7. Complete first-run setup for Longmont, Colorado using the app-guided local AI setup. If the local model is already installed, note that. If setup downloads or verifies a model, note that.
8. Verify core UI navigation: onboarding, source discovery/import, Daily Scan, Workbench, Publishing, Settings, backup/restore visibility, and diagnostics visibility. Confirm no route bounces to the top nav instead of the intended workflow.
9. Run Longmont source discovery/import. Confirm it produces reusable, non-city-hardcoded source behavior and does not depend on `C:\Users\instynct` paths.
10. Run Daily Scan and inspect leads. Confirm weak, generic, recurring, unsupported, or non-jurisdictional items are clearly labeled/suppressed/downgraded rather than presented as strong ready-to-publish news.
11. Draft at least one credible Longmont story from linked source evidence. Confirm public copy has a real headline, no reporter scaffolding, no `EDITOR_NOTE`, no `Body:`, no `Headline:`, no `Nut graf`, no `[Source needed]`, no `[Verification needed]`, no `[End of Report]`, no mojibake marker code points, and no unsupported facts.
12. Exercise editor workflow including hold/send-back/approve/cut where available. Confirm a held draft can be clearly sent back for more work.
13. Compile/export the static site and record the exported ZIP path.
14. Publish through the default here.now anonymous preview flow. Record the public here.now URL.
15. Inspect the public here.now site as a visitor. Confirm there are no duplicate topic stories, no raw Markdown, no internal tester/developer paths, no unsupported Mac/Linux installer claims, and no broken story links.
16. Inspect the public docs/landing page from https://scottconverse.github.io/CivicNewspaper/ and release page. Confirm v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, and discussion links are all present and honest.
17. Write a full human-readable report with PASS/FAIL and explicit blocker/critical/major/minor findings. The release only passes if there are zero blocker, critical, and major findings.

## Do Not Do

- Do not merge, tag, or publish product code.
- Do not use an installer copied from the coder machine.
- Do not treat historical Mac/Linux artifacts as release-supported.
- Do not omit failures because they seem small.
