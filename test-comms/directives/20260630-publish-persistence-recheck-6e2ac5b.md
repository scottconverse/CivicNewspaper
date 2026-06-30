# CivicNewspaper Publish Persistence Recheck

Status: ACTIVE
Issued by: coder
Issued at: 2026-06-30T16:25:00Z

Single source of truth:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Coordination branch: test-comms/cleanroom-coder-tester
- Tester coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-only path on the tester machine: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product under test:

- Product branch: main
- Product commit: 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad
- Product version: 0.3.0

Use the same verified attempt-9 installer artifacts:

- Artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b
- NSIS installer: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64-setup.exe
- NSIS SHA256: 8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719
- NSIS size bytes: 5626730
- MSI installer: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64_en-US.msi
- MSI SHA256: AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A
- MSI size bytes: 9117696

Reports to write:

- Visibility report: test-comms/reports/20260630-publish-persistence-recheck-6e2ac5b-visibility.md
- Final report: test-comms/reports/20260630-publish-persistence-recheck-6e2ac5b-report.md
- Evidence folder: test-comms/evidence/20260630-publish-persistence-recheck-6e2ac5b

Why this recheck exists:

Attempt 9 proved the app showed `Connector publish completed`, saved the here.now URL, updated publish history, exported the ZIP, and published a live reachable site. The written report still marked FAIL because the app or browser-control target was later unavailable. This recheck must determine whether Civic Desk actually exits after connector publish, or whether the tester automation lost/released its browser target after a successful publish.

Scope:

- Do not wipe Windows.
- Do not manually install Ollama or models.
- Do not generate a new product build.
- A full cleanroom wipe is not required unless the existing app state cannot be opened. Prefer reusing the attempt-9 installed app and database so this test isolates publish persistence.

Steps:

1. Fetch/pull this branch and read ACTIVE_DIRECTIVE.md.
2. Verify installer hashes still match the values above.
3. Open the installed Civic Desk app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
4. Confirm the app is on product commit 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad if visible or infer from the installed installer hash.
5. Navigate to Publishing.
6. Verify the existing attempt-9 compile output and here.now URL are visible if present.
7. If a compiled issue is present, use it. If not, compile from the existing approved drafts.
8. Test here.now connection.
9. Click `Publish with connector`.
10. Capture the immediate post-publish UI text and screenshot.
11. Wait 30 seconds. Capture:
    - whether the Civic Desk process is running;
    - whether the WebView/browser-control target is still reachable;
    - current UI text and screenshot if reachable.
12. Wait another 60 seconds. Capture the same process and UI checks.
13. Do not click `Open here.now` until after the process persistence checks are complete.
14. After the persistence checks, fetch the live here.now URL and verify HTTP 200 for `/`, `/briefs/1.html`, `/briefs/2.html`, and `/feed.xml` if those paths exist in the manifest.

Pass criteria:

- Connector publish completes.
- The app displays a success message or saved live URL after publish.
- The Civic Desk process remains running 30 seconds and 90 seconds after publish.
- The app UI remains reachable or can be reattached without relaunch.
- The live here.now site is reachable.

Fail criteria:

- The Civic Desk process exits by itself after publish.
- The app UI becomes unreachable while the process is still running and cannot be reattached.
- The publish succeeds but no in-app success/saved URL state is shown.
- The here.now site is not reachable after a claimed successful publish.

Final report requirements:

- PASS or FAIL.
- Exact product commit and installer hash checked.
- Whether the app process was running immediately before publish, immediately after publish, after 30 seconds, and after 90 seconds.
- Whether the WebView/browser target remained reachable at each checkpoint.
- Exact in-app success/failure text after publish.
- here.now URL and HTTP status checks.
- If FAIL, state whether this is a product process exit, product UI target loss, tester automation target loss, or unknown.

After writing the final report, keep the 15-minute watcher armed and continue treating test-comms/ACTIVE_DIRECTIVE.md as the only active directive pointer.
