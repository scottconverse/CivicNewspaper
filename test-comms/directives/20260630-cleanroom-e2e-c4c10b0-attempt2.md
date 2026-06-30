# CivicNewspaper cleanroom E2E retest - c4c10b0 attempt 2

Status: ACTIVE
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: c4c10b0bcbce8fee789a6209ee10a8c216d88dc9
Directive path: test-comms/directives/20260630-cleanroom-e2e-c4c10b0-attempt2.md
Visibility report path: test-comms/reports/20260630-cleanroom-e2e-c4c10b0-visibility-attempt-2.md
Final report path: test-comms/reports/20260630-cleanroom-e2e-c4c10b0-report.md
Tester artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/

## Stop old context first

Stop any CivicCast watcher, CivicCast repo context, or old local path assumption. This directive is for CivicNewspaper only.

Do not use `C:\Users\instynct` on the tester. Use the tester machine's actual coordination checkout path. The approved tester checkout path remains:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## First action: prove visibility

Before starting the long retest, write:

`test-comms/reports/20260630-cleanroom-e2e-c4c10b0-visibility-attempt-2.md`

Include tester user/host, local coordination path, coordination HEAD, active directive confirmation, artifact visibility, artifact SHA256 verification, and watcher-armed status. Commit and push with `[skip ci]`, then continue.

## Installer artifacts to use

- `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/The Civic Desk_0.3.0_x64-setup.exe`
  - SHA256: `BF12F1B020D355B95ABBF79597EB629A505C5E966C892B57338BD3AE5AFC498C`
  - Size: `5611790`
- `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/The Civic Desk_0.3.0_x64_en-US.msi`
  - SHA256: `46EDAC61E261D1E17BFA9BE26C0664554486FC826F6B91DCE01DD8264D5A3DA1`
  - Size: `9113600`

Use NSIS first. Use MSI only if NSIS fails, and report why.

## Retest scope

Run the full product cleanroom E2E again from a product clean wipe:

- CivicNewspaper install/app data/test files/output.
- Ollama install/model/service/processes related to prior CivicNewspaper testing.
- Prior tester output for this product run.

Do not reset Windows or remove unrelated software.

The tester must not manually install Ollama, models, runtime pieces, or missing product prerequisites. The product must handle them or fail with evidence.

## Required checks

Repeat the full E2E path:

1. Install and launch.
2. First-run setup and app-guided local AI setup.
3. Longmont identity/source discovery/import with official plus public readable social/community sources.
4. Daily Scan.
5. Generate leads and create multiple drafts.
6. Exercise editor workflow across at least three items: edit, save, send back/rework, hold, cut, restore/reopen if available, approve.
7. Invoke press-freedom/legal-risk advisor on at least one story.
8. Compile static output.
9. Confirm `site-package.zip` exists on disk after successful compile.
10. Publish anonymously to here.now if compile succeeds.
11. Verify here.now URL loads.
12. Compare here.now output against local output.

## Attempt-1 failures that must be rechecked

This build is specifically expected to address:

- A failed public-output quality gate must not leave `publish-manifest.json` claiming `site-package.zip` exists when the ZIP was not created.
- The publish checklist must not show `Export hosting package` complete before a successful compile result exists.
- Public output must not leak `EDITOR_NOTE`, `editor_note`, `Editor Note:`, `[EDITOR_NOTE`, `[Editor Note:`, `TESTER EDIT`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.
- Evidence excerpts/public pages must not leak combined Longmont footer boilerplate such as `Employee Login`, `InsideLongmont`, `Terms of Use`, or `Privacy Policy`.
- Held drafts must expose a clear `Send Back for More Work` path.
- Workbench opened without a selected draft must show a visible draft picker or useful empty state, not a blank pane.

## Quality expectations

Pass only if:

- The app produces a reviewable local output folder and ZIP package.
- The app publishes a here.now URL from that output.
- The public output has no duplicate story topics.
- The public output is reader-facing newspaper copy, not reporter notes.
- Background/reference pages are held or sent back instead of published as news.
- No mojibake marker code points appear: `U+00C2`, `U+00C3`, `U+00E2`, `U+FFFD`.
- No unconfigured claims are invented about AI, ads, nonprofit status, public-record-only coverage, or publisher identity.

If the app blocks compilation because it correctly detects non-public scaffolding, report that as a product safety pass for the specific gate, but still fail the E2E if the user cannot recover inside the app and produce a clean publication without manual file editing.

## Evidence required

Final report must be plain English for the human product owner. Include:

- Pass/fail summary.
- here.now URL if produced.
- Local output folder and ZIP path.
- Committed output copy under `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/` if size allows.
- Installer hash verification.
- Clean wipe evidence.
- AI model chosen and setup evidence.
- Source list, lead count, story/brief count.
- Story list with source URLs and statuses.
- Screenshots for the failed attempt-1 areas and their retest result.
- Exact failure point and evidence if anything still fails.
- Confirmation that the 15-minute CivicNewspaper watcher remains armed.
