# Tester Report - Publish Persistence Recheck 6e2ac5b

Date: 2026-06-30
Tester machine: MSI\civic
Repo: https://github.com/scottconverse/CivicNewspaper.git
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad
Directive: test-comms/directives/20260630-publish-persistence-recheck-6e2ac5b.md

## Verdict

PASS.

The app did not exit after connector publish. Civic Desk remained running and the WebView debug target remained reachable immediately after publish, after 30 seconds, and after 90 seconds. The in-app UI showed connector success, a saved here.now URL, and updated publish history. The live here.now site returned HTTP 200 for all required paths.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- Tester user: MSI\civic
- Installed app: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Product commit: inferred from the verified attempt-9 installer hash for product commit `6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad`
- NSIS SHA256: `8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719`
- NSIS size: `5626730`
- MSI fallback SHA256: `AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A`
- MSI fallback size: `9117696`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread the active directive plus protocol files.
2. Verified installer hashes and wrote the required visibility report.
3. Reused the attempt-9 installed app and database, per directive scope.
4. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` with WebView debug port `9333`.
5. Navigated to Publishing.
6. Confirmed previous attempt-9 publish history was visible.
7. Current compile controls had reset to pending, so I reviewed the compile checklist and compiled from the existing approved drafts.
8. Ran here.now test connection.
9. Clicked `Publish with connector`.
10. Captured immediate post-publish UI, process state, and WebView target state.
11. Waited 30 seconds and captured process/UI/WebView state.
12. Waited another 60 seconds and captured process/UI/WebView state.
13. Fetched the live here.now URL after persistence checks.

## Results

- Connector publish completed: PASS.
- In-app success state shown: PASS.
  - Exact UI text included: `Connector publish completed and publish history was updated.`
  - Saved URL shown: `https://quick-nutmeg-xwxq.here.now`
- Publish history updated: PASS.
  - New issue: `issue-20260630-162414-439329900`
  - Provider: `here now`
  - Articles: `2`
- App process before publish: running, PID `7336`.
- App process immediately after publish: running, PID `7336`.
- App process after 30 seconds: running, PID `7336`.
- App process after 90 seconds: running, PID `7336`.
- WebView/browser target before publish: reachable at `http://127.0.0.1:9333/json/list`.
- WebView/browser target immediately after publish: reachable.
- WebView/browser target after 30 seconds: reachable.
- WebView/browser target after 90 seconds: reachable.
- UI reattach status: no relaunch required; the same target remained available.

## Live URL Checks

Base URL: https://quick-nutmeg-xwxq.here.now

- `/`: HTTP 200
- `/briefs/1.html`: HTTP 200
- `/briefs/2.html`: HTTP 200
- `/feed.xml`: HTTP 200

## Evidence

- Visibility report: `test-comms/reports/20260630-publish-persistence-recheck-6e2ac5b-visibility.md`
- Evidence folder: `test-comms/evidence/20260630-publish-persistence-recheck-6e2ac5b`
- Launch/debug target: `00-launch-debug.json`
- Publishing before test: `01-publishing-before-test.json`, `01-publishing-before-test.png`
- Test connection: `12-after-test-connection-retry.json`, `12-after-test-connection-retry.png`
- Compile retry: `11-after-compile-retry.json`, `11-after-compile-retry.png`
- Before publish checkpoint: `13-before-publish-checkpoint-retry.json`
- Publish poll: `14-publish-retry-poll.json`
- Immediate post-publish: `15-immediate-post-publish-retry.json`, `15-immediate-post-publish-retry.png`
- 30-second checkpoint: `16-after-30s-retry.json`, `16-after-30s-retry.png`
- 90-second checkpoint: `17-after-90s-retry.json`, `17-after-90s-retry.png`
- Live HTTP status: `18-live-http-status.json`

## Findings

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

No product failure reproduced. The earlier automation-target loss was not reproduced in this recheck. Classification: PASS; if attempt 9 observed target loss after success, this run points to tester automation target loss or a non-reproducible transient, not a Civic Desk process exit.

## Request For Coder

No fix requested from this recheck.
