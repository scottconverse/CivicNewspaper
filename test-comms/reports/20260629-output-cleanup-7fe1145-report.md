# Output Cleanup Verification Report - 7fe1145

Status: FAIL - mojibake blocker still present

Directive: `test-comms/directives/20260629-rerun-output-cleanup-7fe1145.md`

Product branch: `stable-readiness-local-gates`

Required product commit: `7fe11452ea7ccbb9425df291a030da58ff8e48bf`

Artifact folder: `test-comms/artifacts/20260629-output-cleanup-7fe1145/`

## Summary

The 7fe1145 NSIS installer hash matched and the existing cleanroom app was upgraded successfully. The app launched, preserved the Longmont state, showed Local AI ready with `qwen2.5:7b`, recompiled/exported the existing publication, created a ZIP, and anonymously published to here.now:

`https://yearly-plume-8e3t.here.now`

HTTP verification returned 200 and the response contained Longmont/Civic publication content.

Two focused fixes passed:

- Stale article cleanup passed. A seeded stale `watch/7.html` file was removed by the fixed compile/export.
- Killed-story protection passed. The killed draft opened with `Current Status: killed`, explanatory copy, and `Approve for Static Publish` disabled.

The rerun still fails because the generated output still contains mojibake marker codepoints in multiple story pages.

## Installer / Upgrade

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

Installer used:

`test-comms/artifacts/20260629-rerun-full-e2e-7fe1145/The Civic Desk_0.2.8_x64-setup.exe`

Hash checks:

- NSIS expected: `9F495209FFA6254B095EA946F5C2553067D5362834FC7BF62D662522B9F36C4A`
- NSIS observed: `9F495209FFA6254B095EA946F5C2553067D5362834FC7BF62D662522B9F36C4A`
- MSI expected: `18B9C45C7896A42C554177A063D08B4462A44C2563FF11437E19F5DA8ACFB154`
- MSI observed: `18B9C45C7896A42C554177A063D08B4462A44C2563FF11437E19F5DA8ACFB154`

NSIS install/upgrade exit code: 0.

## App State

Visible app state after upgrade:

- Community: Longmont, CO
- Local AI: ready
- Model: `qwen2.5:7b`

Read-only final database counts:

- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10
- Drafts: 7
- Draft statuses: 6 `ready_to_publish`, 1 `killed`
- Latest publish provider: `here_now`
- Latest here.now URL: `https://yearly-plume-8e3t.here.now`

## Compile / Export

Output folder:

`test-comms/artifacts/20260629-output-cleanup-7fe1145/publication-output/site/`

ZIP:

`test-comms/artifacts/20260629-output-cleanup-7fe1145/publication-output/site/site-package.zip`

ZIP SHA256:

`815A813227B5CE45921E462ABBF143A871ACEB5476FB5CAEA8C3A43272E1F9D9`

Article count reported by product: 6.

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

No extra actual article files and no missing manifest article files were found.

## Stale Output Cleanup

Before compiling, the test seeded a stale sentinel file:

`publication-output/site/watch/7.html`

After compile:

- `watch/7.html` did not exist.
- Manifest paths matched actual article files exactly.

Result: PASS for stale-output cleanup.

Evidence:

- `stale-output-seed.txt`
- `manifest-vs-actual-articles.json`

## Killed Story Behavior

Existing killed draft:

`Draft: Public Participation Rules: The city outlines specific rules for public participation...`

UI behavior:

- The killed draft was visible as `Draft exists` in Story Queue and could be opened.
- Workbench displayed `Current Status: killed`.
- Workbench displayed: `This story is killed and will not be approved for publishing unless you move it back to Hold first.`
- `Approve for Static Publish` was disabled.

Publish output behavior:

- Killed draft did not appear as `watch/7.html`.
- Killed draft was not included in manifest article paths.

Result: PASS for killed-story protection and exclusion.

Evidence:

- `killed-story-opened-ui.json`
- `02-killed-story-ui.png`
- `manifest-vs-actual-articles.json`

## Mojibake Scan

Scan method used directive codepoint markers:

- U+00C3
- U+00C2
- U+00E2

Result: FAIL. Markers were found in generated output:

- `watch/1.html`: U+00E2, snippet includes `Youth Centerâ€™s`
- `watch/2.html`: U+00C2 and U+00E2, snippets include `Â© 2026 City of Longmont` and `â†’`
- `watch/3.html`: U+00C2 and U+00E2, snippets include `Â© 2026 City of Longmont` and `Longmontâ€™s`
- `watch/4.html`: U+00C2 and U+00E2, snippets include `Â© 2026 City of Longmont` and `â†’`
- `watch/5.html`: U+00C2 and U+00E2, snippets include `Â© 2026 City of Longmont` and `Clerkâ€™s`
- `watch/6.html`: U+00C2 and U+00E2, snippets include `Â© 2026 City of Longmont` and `departmentâ€™s`

Evidence:

`test-comms/artifacts/20260629-output-cleanup-7fe1145/mojibake-codepoint-scan.json`

## here.now Publish

Publish result:

- URL: `https://yearly-plume-8e3t.here.now`
- Deployment ID: `slug=yearly-plume-8e3t;version=01KW942JAXGYKX9XWAGKSNGHNN;created_slug=yearly-plume-8e3t`
- HTTP status: 200
- Visible content check: Longmont/Civic content present.

Anonymous publish did not require manual repair during this directive. Note: the cleanroom state already had a persisted here.now config display name from the prior run (`Longmont Civic Desk Test`), so this specific run verifies that the upgraded product did not send an empty display name in the current state, but it does not isolate a brand-new no-config default.

Evidence:

- `here-now-publish-result.json`
- `here-now-http-verification.json`
- `herenow-index.html`

## Screenshots / Artifacts

Screenshots:

- `01-app-visible-state.png`
- `02-killed-story-ui.png`

Other key artifacts:

- `installer-hashes.json`
- `install-result.txt`
- `compile-result.json`
- `output-files-after-compile.json`
- `manifest-vs-actual-articles.json`
- `mojibake-codepoint-scan.json`
- `killed-story-opened-ui.json`
- `final-db-state.json`
- `site-package-zip-sha256.txt`
- `artifact-file-list.txt`

## UI Confusion / Notes

- Killed-story protection is much clearer in 7fe1145: status is visible and approve is disabled.
- The here.now display-name behavior was not fully isolated because the existing cleanroom state already had a saved nonempty display name.
- Mojibake remains the active blocker.

## Readiness Assessment

Not ready for Scott to use for a real Longmont publication next week.

The stale-output and killed-story fixes appear effective, and the app can publish to here.now, but mojibake remains in generated/public output. The next directive should focus on fixing the remaining output encoding problem before a full clean-wipe E2E rerun.
