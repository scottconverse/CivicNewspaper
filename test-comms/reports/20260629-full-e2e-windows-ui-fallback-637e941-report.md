# Full E2E Windows UI Fallback Report - 637e941

Status: FAIL - release gate still open

Directive: `test-comms/directives/20260629-continue-full-e2e-with-windows-ui-fallback.md`

Product branch: `stable-readiness-local-gates`

Required product commit: `637e941ac77361033fc22b48fac33ae1aa50a6b3`

Artifact folder: `test-comms/artifacts/20260629-full-e2e-windows-ui-fallback-637e941/`

## Summary

Computer Use was tried once as required and failed before app control with:

`Computer Use native pipe is unavailable: Error: failed to connect native pipe: The system cannot find the file specified. (os error 2)`

The run continued with the directive-approved Windows-native fallback. The installed product was present and running from the previously verified 637e941 cleanroom state. The product state already contained the completed continuation work: Longmont profile, 18 leads, generated drafts, editor decisions, compiled output, ZIP, and here.now publish history.

The product completed the release-loop mechanics and produced a public here.now URL:

`https://oaken-bloom-z7nj.here.now`

HTTP verification returned 200 and the page contained Longmont/Civic publication content.

The result is still failed/not release-ready because the generated/published output contains mojibake-style encoding markers (`â€™`, `â€`) in multiple story HTML files. Scott should not use this generated issue for a real Longmont publication next week without fixing or cleaning the encoding problem.

## Control Method

- Computer Use: attempted once; failed with native pipe unavailable.
- Fallback used: Windows-native fallback with PowerShell/.NET, Win32 window focus/sizing, screenshots, and read-only DB/output verification.
- No app database rows were directly written by the tester.
- No prerequisites, models, browser helpers, or packages were installed manually.

## Resume / Install State

- Reinstall performed: no.
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Local database path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\civicdesk.db`
- Local AI visible state: `Local AI ready`
- Selected model visible state: `qwen2.5:7b`

## Counts And Sources

Read-only final database state:

- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10
- Drafts: 7
- Draft statuses: 6 `ready_to_publish`, 1 `killed`
- Publish runs: 2
- Published posts: 6

Sources present:

- Official record: Longmont Agenda Management Portal
- Official record: Longmont City Council Meetings
- Official record: Longmont Public Information
- Official record: Public Notice Colorado
- Community signal: Longmont subreddit
- Community signal: Longmont Colorado subreddit

## Story / Editor Flow Evidence

The continued app state shows:

- Already-drafted leads were represented as existing drafts/open-draft paths rather than duplicate-only draft actions.
- Draft count exceeded the five-story requirement.
- Six drafts were `ready_to_publish`.
- One nonessential story ended as `killed`, preserving at least five publishable stories.
- Prior visible editor-control evidence in the same installed state exercised title/body edit, save, advisor, hold, approve, and kill/cut.

The report evidence for this directive is saved in the artifact folder, including:

- `computer-use-attempt.txt`
- `01-visible-starting-state.png`
- `db-state.json`
- `here-now-http-verification.json`
- `mojibake-scan.json`
- `artifact-file-list.txt`

## Publishing

Latest publish run:

- Provider: `here_now`
- URL: `https://oaken-bloom-z7nj.here.now`
- Deployment ID: `slug=oaken-bloom-z7nj;version=01KW927XZK84SDSG09DGPXNW9Q;created_slug=oaken-bloom-z7nj`
- Article count: 6
- Skipped count: 0
- Files written: 23
- HTTP status: 200
- Visible-content check: response contained Longmont and Civic/publication content.

Copied publication output:

`test-comms/artifacts/20260629-full-e2e-windows-ui-fallback-637e941/publication-output/site/`

ZIP:

`test-comms/artifacts/20260629-full-e2e-windows-ui-fallback-637e941/publication-output/site/site-package.zip`

ZIP SHA256:

`3C6F79D14392B9279B35B05F23ACFFCCB34C4317B2B459F469CA6A8659CAE8A3`

## Findings

### Major - Published output contains mojibake markers

The output scan found garbled encoding markers in story pages:

- `watch/1.html`: `â€™`, `â€`
- `watch/3.html`: `â€™`, `â€`
- `watch/5.html`: `â€™`, `â€`
- `watch/6.html`: `â€™`, `â€`

Evidence: `test-comms/artifacts/20260629-full-e2e-windows-ui-fallback-637e941/mojibake-scan.json`

Impact: the issue is not ready for publication without cleanup.

### Minor - Computer Use unavailable in tester environment

The required Computer Use attempt failed before app control because the native pipe was unavailable. The Windows-native fallback was able to continue product verification.

Evidence: `test-comms/artifacts/20260629-full-e2e-windows-ui-fallback-637e941/computer-use-attempt.txt`

## Final Readiness Assessment

Not ready for Scott to use for a real Longmont publication next week.

The cleanroom product workflow can generate, approve, compile, package, and anonymously publish a Longmont issue through here.now, but the generated publication still contains mojibake-style encoding artifacts. That is a release-readiness blocker for real public use.
