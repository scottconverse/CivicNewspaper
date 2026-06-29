# Systemic Mojibake Fix Verification - 59eb271

Status: PASS

Directive: `test-comms/directives/20260629-rerun-mojibake-systemic-59eb271.md`

Product branch: `stable-readiness-local-gates`

Required product commit: `59eb271d323b0e051a01659494958594b6384cf1`

Evidence folder: `test-comms/reports/20260629-mojibake-systemic-59eb271-evidence/`

## Summary

The 59eb271 NSIS installer hash matched, the app upgraded successfully, the existing Longmont publication compiled/exported, the ZIP package exists, anonymous here.now publish succeeded, and the exact directive mojibake scanner passed against both local exported output and downloaded here.now HTML.

here.now URL:

`https://clear-canopy-5yvw.here.now`

HTTP verification: 200, with Longmont/Civic publication content present.

Focused blocker status:

- Known mojibake sequences: PASS, none found by exact scanner.
- Stale killed-story page cleanup: PASS, seeded `watch/7.html` was removed.
- Killed-story protection: PASS, killed draft shows `Current Status: killed` and `Approve for Static Publish` is disabled.
- Anonymous here.now publish: PASS in current state, nonempty display name present and publish succeeded without manual repair during this rerun.

Recommendation: next directive should be a full clean-wipe end-to-end Longmont publication run using commit `59eb271d323b0e051a01659494958594b6384cf1`.

## Installer / Upgrade

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

Installer used:

`test-comms/artifacts/20260629-rerun-mojibake-systemic-59eb271/The Civic Desk_0.2.8_x64-setup.exe`

Hash checks:

- NSIS expected: `0864D76EB0A382A641B03C1A3A65D6B4D6220307DC73FE764C95031E96F02B93`
- NSIS observed: `0864D76EB0A382A641B03C1A3A65D6B4D6220307DC73FE764C95031E96F02B93`
- MSI expected: `1DC37C593240EECC186486A6F2B750FD10CD69DFAE652043B7A4748DC88AF272`
- MSI observed: `1DC37C593240EECC186486A6F2B750FD10CD69DFAE652043B7A4748DC88AF272`

NSIS install/upgrade exit code: 0.

## App State

Visible state after upgrade:

- Community: Longmont, CO
- Local AI: ready
- Model: `qwen2.5:7b`
- Leads visible in app: 18
- Drafts visible in app: 7

Read-only final database state:

- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10
- Drafts: 7
- Draft statuses: 6 `ready_to_publish`, 1 `killed`
- Publish runs: 4
- Latest publish URL: `https://clear-canopy-5yvw.here.now`

## Compile / Export

Output folder:

`test-comms/reports/20260629-mojibake-systemic-59eb271-evidence/publication-output/site/`

ZIP:

`test-comms/reports/20260629-mojibake-systemic-59eb271-evidence/publication-output/site/site-package.zip`

ZIP SHA256:

`526862C51C3E108D16296B57C4500F7152D8DC7C7CE1FAFF1B750059372F0B6A`

Product compile result:

- Article count: 6
- Skipped count: 0
- Files written: 23

Manifest article paths:

- `watch/5.html`
- `watch/2.html`
- `watch/1.html`
- `watch/4.html`
- `watch/3.html`
- `watch/6.html`

Actual article files:

- `watch/1.html`
- `watch/2.html`
- `watch/3.html`
- `watch/4.html`
- `watch/5.html`
- `watch/6.html`

No extra or missing article files were found.

## Exact Mojibake Scanner

The exact directive scanner was run against:

1. Local exported output folder.
2. Downloaded here.now HTML pages for index and all manifest article paths.

Results:

- Local output: `PASS: no known mojibake sequences found`
- here.now downloaded output: `PASS: no known mojibake sequences found`

Evidence:

- `exact-mojibake-scan-local.json`
- `exact-mojibake-scan-herenow.json`

Note: this report does not fail on legitimate Unicode characters. It only reports the exact bad decoded sequences required by the directive.

## Stale Output Cleanup

Before compile, the test seeded:

`publication-output/site/watch/7.html`

After compile:

- `watch/7.html` did not exist.
- Manifest article paths matched actual `watch/*.html` files exactly.

Result: PASS.

Evidence:

- `stale-output-seed.txt`
- `manifest-vs-actual-articles.json`

## Killed Story Protection

Killed draft checked:

`Draft: Public Participation Rules: The city outlines specific rules for public participation...`

Workbench behavior:

- Displayed `Current Status: killed`.
- Displayed explanatory text: `This story is killed and will not be approved for publishing unless you move it back to Hold first.`
- `Approve for Static Publish` button was disabled.

Output behavior:

- Killed story was not included as `watch/7.html`.
- Killed story was not included in manifest article paths.

Result: PASS.

Evidence:

- `killed-story-ui-check.json`
- `02-killed-story-ui.png`

## here.now Publish

Publish URL:

`https://clear-canopy-5yvw.here.now`

Deployment ID:

`slug=clear-canopy-5yvw;version=01KW95QQH008XD0302T7QRP55D;created_slug=clear-canopy-5yvw`

HTTP verification:

- Status: 200
- Contains Longmont content: true
- Contains Civic/publication content: true

Evidence:

- `here-now-publish-result.json`
- `here-now-http-verification.json`
- `here-now-downloaded/`

## Screenshots / Evidence List

Key screenshots:

- `01-app-launched.png`
- `02-killed-story-ui.png`

Key evidence files:

- `installer-hashes.json`
- `install-result.txt`
- `compile-result.json`
- `here-now-publish-result.json`
- `here-now-http-verification.json`
- `exact-mojibake-scan-local.json`
- `exact-mojibake-scan-herenow.json`
- `manifest-vs-actual-articles.json`
- `killed-story-ui-check.json`
- `final-db-state.json`
- `site-package-zip-sha256.txt`
- `artifact-file-list.txt`

## Final Assessment

Focused blocker verification passes for commit `59eb271d323b0e051a01659494958594b6384cf1`.

The next directive should be a full clean-wipe end-to-end Longmont publication run on the same commit to prove the complete cleanroom path from fresh install through new draft generation and publication.
