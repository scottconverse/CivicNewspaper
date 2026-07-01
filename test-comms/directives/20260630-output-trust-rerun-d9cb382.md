# CivicNewspaper v0.3.1 Output Trust Cleanroom Rerun

Directive issued by: coder
Directive date: 2026-06-30
Coordination repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester

## Single Source Of Truth

Read this directive only through:

1. Fetch and check out `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Follow the directive path listed there.

Stop using any old CivicCast context or any older CivicNewspaper directive. This directive replaces `test-comms/directives/20260630-stage5-cleanroom-v031-1273bc7.md`.

Approved tester coordination path on the tester machine:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use this coder-machine path on the tester machine:

`C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms`

## Product Under Test

Product branch: `main`

Product commit represented by this installer:

`d9cb38210b3c71d0406219ed8c42f25212bcb977`

Important: this product commit may not yet be pushed to the product branch. For this cleanroom run, the executable installer artifact is the product under test.

Installer artifact:

`test-comms/artifacts/20260630-output-trust-rerun-d9cb382/The Civic Desk_0.3.1_x64-setup.exe`

Expected SHA256:

`F0558BE2E21EED4C83152E376E2FA8DDAFDB35D2CE657CFF4A798E2B8C0395BA`

Expected size:

`5635913`

## Required Reports And Evidence

Write the first visibility report as soon as you read this directive and verify the artifact:

`test-comms/reports/20260630-output-trust-rerun-d9cb382-visibility.md`

Write the final report here:

`test-comms/reports/20260630-output-trust-rerun-d9cb382-report.md`

Put screenshots, logs, output ZIPs, copied publication output, and any exported review artifacts here:

`test-comms/evidence/20260630-output-trust-rerun-d9cb382/`

After writing either report, commit and push it to `test-comms/cleanroom-coder-tester` with `[skip ci]`.

Arm or re-arm your 15 minute repo watcher after the final report. Continue treating `test-comms/ACTIVE_DIRECTIVE.md` as the only source of truth.

## Cleanroom Boundary

Before testing, wipe only CivicNewspaper/The Civic Desk related state:

- Installed CivicNewspaper/The Civic Desk app.
- CivicNewspaper/The Civic Desk app data.
- Prior CivicNewspaper test outputs.
- Prior cleanroom evidence output for this run.
- Prior product-installed Ollama/runtime/model state if it was installed only for CivicNewspaper testing.

Known app-data roots to check on Windows:

- `%LOCALAPPDATA%\The Civic Desk`
- `%LOCALAPPDATA%\com.scottconverse.civicdesk`
- `%APPDATA%\com.scottconverse.civicdesk`
- `%USERPROFILE%\.ollama` only when that Ollama/model state was created for this app test.

Do not wipe Windows, the tester Windows user account, browser, Git, Codex Desktop, or the coordination checkout.

Do not manually install Ollama, local models, Node, Rust, or any product dependency to help the app. If the app needs a runtime or model, the app must guide the setup. If the app cannot, report it as a product failure.

## Test Scope

Run this as an end user, not as a developer.

1. Verify the installer SHA256 and size before running it.
2. Install the app from the artifact.
3. Launch the app from the normal Windows path.
4. Complete first-run setup with Longmont, Colorado as the publication/community.
5. Let the app inspect hardware and guide any AI/runtime/model setup it needs. Record model choice, progress messaging, time, and any failures.
6. Discover/import Longmont sources. Include official public sources and public readable social/community sources. Do not log into private sites or private groups.
7. Run a scan and generate leads. A healthy run should target roughly 10-25 leads if enough current source material exists.
8. Use the writer workflow to draft multiple stories from leads.
9. Use the editor workflow to edit, approve, hold, cut, and send at least one draft back for more work if the UI exposes that path.
10. Confirm the editor can recover from held, cut, sent-back, and approved states without getting trapped.
11. Produce a reviewable publication. Target 5-10 reader-facing stories or briefs. If the app cannot create that many, report exactly where and why it stops.
12. Export the publication ZIP/static site through the app.
13. Publish anonymously to here.now through the app. This publish is authorized for this test.
14. Save the here.now URL, expiration if shown, local output path, and exported ZIP path.
15. Inspect the published site, local ZIP extract, RSS/feed/share artifacts, and article pages.

## Required Regression Checks From The Last Failed Run

The previous cleanroom run failed because bad output still reached public pages. This rerun must verify the fixes.

Fail the run if any generated public article page, ZIP extract, RSS/feed item, newsletter/share artifact, or here.now page contains only an approval note or tester note as the story body, including:

- `Approved during cleanroom mechanics test`
- `despite quality warnings`
- `see tester report`
- `mechanics test`
- `tester report`

Fail the run if any lead-based public story compiles or publishes with no source links attached.

Fail the run if a lead-based story has linked source material but no inline `evidence:` citation in the public body.

Fail the run if a story cites one source but makes a materially different claim that the cited source does not support. Specifically watch for the earlier failure class where a downtown/events source was used to support an unrelated contract/vendor claim.

If the app blocks package generation for one of these reasons and gives an actionable error, record that as a pass for the regression gate, then fix the story as an editor would and continue the publication workflow.

## Output Quality Checks

Fail the run if public output contains reporter scaffolding or internal notes, including:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`
- `Source check:`

Fail the run if public output contains mojibake marker code points:

- `U+00C2`
- `U+00C3`
- `U+00E2`
- `U+FFFD`

Fail the run if duplicate-topic stories appear as separate stories instead of being merged, suppressed, or clearly separated by new facts.

Flag as a quality issue, even if mechanics pass, if a story is only evergreen background material, a stale city webpage rewrite, or a reporter note masquerading as a finished article.

## Documentation And UX Checks

Check that the app and docs are honest about:

- Unsigned Windows installer status.
- here.now as the default/easiest anonymous publishing path.
- Mac and Linux installers not being available yet.
- Local AI setup being app-guided rather than tester-installed.
- Cleanroom reset paths for app data and app-managed Ollama/model state.

Check for narrow/mobile or small-window traps where content becomes unreachable.

Check that long-running work has visible progress or a clear status message.

Check that approval, hold, send-back, cut, and restore controls do not stack confusing modals.

## Report Format

Write the final report for Scott, not only for coder. Plain English first, evidence second.

Include:

- Pass/fail verdict.
- Exact app version and installer hash.
- Hardware summary.
- Model/runtime setup result.
- Number of sources.
- Number of leads.
- Number of drafts.
- Number of approved stories/briefs.
- Export ZIP path.
- Local output path.
- here.now URL.
- Screenshots and evidence file list.
- Every failure with exact step, observed behavior, expected behavior, severity, and reproduction notes.
- Whether the app installed and ran without manual dependency help.
- Whether each required regression check passed or failed.

If the answer is no, say exactly where it breaks. Then stop and report so coder can fix and issue the next directive.
