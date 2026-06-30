# CivicNewspaper cleanroom E2E report - c4c10b0 attempt 2

Date: 2026-06-30 UTC
Tester machine: Windows cleanroom tester, user `MSI\civic`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit: `c4c10b0bcbce8fee789a6209ee10a8c216d88dc9`
Directive: `test-comms/directives/20260630-cleanroom-e2e-c4c10b0-attempt2.md`

## Plain-English verdict

FAIL, evidence-backed.

Attempt 2 improves several attempt-1 failure areas: the compile checklist no longer claims the ZIP is complete before compile, the final manifest truthfully reports `article_count: 0`, a real `site-package.zip` exists after compile, and the copied public output has no forbidden marker hits. The here.now preview also published and loaded.

The full E2E still fails because the app could not produce any clean reader-facing stories without manual rewriting. I generated six drafts from non-background `Draft` leads. All six generated drafts contained internal notes, placeholders, copied boilerplate, or other non-public markers. I sent them back/held them rather than approving polluted output. Compile therefore produced a zero-article site and here.now published a zero-article preview.

here.now URL: https://humble-canvas-82dn.here.now

## Installer and cleanroom setup

- Preferred NSIS installer used: `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256: `BF12F1B020D355B95ABBF79597EB629A505C5E966C892B57338BD3AE5AFC498C`
- NSIS size: `5611790`
- Fallback MSI visible and verified: `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/The Civic Desk_0.3.0_x64_en-US.msi`
- MSI SHA256: `46EDAC61E261D1E17BFA9BE26C0664554486FC826F6B91DCE01DD8264D5A3DA1`
- MSI size: `9113600`
- Visibility report was already present and pushed at `test-comms/reports/20260630-cleanroom-e2e-c4c10b0-visibility-attempt-2.md`.

Evidence: `tester-output/evidence/attempt2-final-run-summary.json`, first-run screenshots, installer/app process evidence under `tester-output/evidence/`.

## AI setup and model

- First-run flow reached local AI setup.
- App-installed local Ollama runtime was used.
- Model shown by app: `qwen2.5:7b`.
- App status during test: `Local AI ready`.

Evidence: `first-run-launch.png`, `setup-after-identity.png`, `ai-setup-model-installed.png`.

## Sources, leads, drafts

- Sources visible: 6.
- Leads visible after scan: 14.
- Drafts generated during retest: 6.
- Approved clean stories: 0.
- Published stories: 0.

Observed source set included official Longmont city pages and public readable sources. Evidence: `sources-screen.png`, `story-queue-opened.png`, `attempt2-draft-loop-summary.json`.

## Draft/story results

I generated drafts from normal `Draft` leads rather than background-only `Draft anyway` leads. Results:

1. `City Council to Review Vision Zero Initiatives` - held/sent back. Body included placeholder text such as `[insert date if available]` and was not publication-ready.
2. `Temporary Overnight Closure of Hover Street/CO 119 Intersection Scheduled for June 28` - held/sent back. Body retained uncertainty and copied boilerplate/source text markers.
3. `City Council Approves Ordinances Through Majority Votes` - held/sent back. Body began with `EDITOR_NOTE`.
4. `Meeting Notification for Longmont City Council` - held/sent back. Body began with `EDITOR_NOTE`.
5. `City Council Meeting Process Updated` - held/sent back. Body began with `EDITOR_NOTE`.
6. `New Official Document from Longmont Public Information Available` - held/sent back. Body began with `Editor Note`.

Evidence: `attempt2-draft-loop-summary.json` and screenshots `attempt2-draft-*-generated.png`, `attempt2-draft-*-sent-back-and-held-marker-draft.png`.

## Editor workflow and advisor

Workflow exercised:

- Draft generation.
- Workbench open.
- Send back / rework.
- Hold.
- Held draft recovery path.
- Press-freedom/legal-risk advisor.

Positive retest note: held drafts exposed a clear `Send Back for More Work` path, and Workbench opened without a selected draft showed a visible useful state rather than a blank pane.

Advisor result: the advisor/guardrail area produced visible non-blocking warnings, including reporter-note marker warnings, evergreen-background warnings, source-link warnings, and source-wording warnings. It did not block editor action, which matches the directive expectation that advice remains advisory.

Evidence: `workbench-empty-state.png`, `attempt2-drafts-tab-before-advisor.png`, `attempt2-open-held-draft-for-advisor.png`, `attempt2-advisor-after-run.png`, `attempt2-advisor-publish-summary.json`.

## Compile, ZIP, here.now

- Local output folder: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Committed output copy: `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/site-output-copy/`
- ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- Copied ZIP: `test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/site-output-copy/site-package.zip`
- ZIP SHA256: `F6ACFFFEDA804FFC40A835E9C7EF556BAFC21BCA91FC93EB28DBEB7CD2960EAE`
- here.now URL: https://humble-canvas-82dn.here.now
- here.now fetch: HTTP 200

Compile receipt:

- Articles: 0
- Files: 17
- Skipped: 0

The UI clearly warned: `No approved stories were included. Approve a story in Workbench, then compile again.`

Evidence: `attempt2-publish-checklist-no-approved.png`, `attempt2-publish-compile-no-approved-result.png`, `attempt2-publish-connector-summary.json`, `attempt2-here-now-fetch.json`, `attempt2-final-run-summary.json`.

## Attempt-1 failure retest notes

- Stale ZIP/manifest claim: improved. Manifest says `article_count: 0`, includes `site-package.zip`, and ZIP exists.
- Checklist premature completion: improved. Checklist showed pending items before compile.
- Public markers in final output: improved for zero-article output. Marker scan found no forbidden marker hits in committed output copy.
- Longmont footer boilerplate leak: improved for zero-article output. Marker scan found no `Employee Login`, `InsideLongmont`, `Terms of Use`, or `Privacy Policy` hits in final copied output.
- Held draft send-back path: improved. Held/review state exposed `Send Back for More Work`.
- Blank Workbench state: improved. Workbench showed a useful visible state.

These improvements do not make the full E2E pass because the app still did not produce a usable newspaper issue.

## Public output quality scan

Marker scan file:

`test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/evidence/attempt2-public-output-marker-scan.json`

Result: no forbidden marker findings in the final copied zero-article output.

## Exact failure point

The product did not hard-crash. The failure point is editorial/product quality:

`Daily Scan -> Draft generation -> Workbench`

Generated drafts were not clean reader-facing stories. Because all generated drafts required hold/send-back, the only publishable output the app could produce without manual rewriting was a zero-article site. That is not a successful civic newspaper E2E.

## Watcher status

The CivicNewspaper watcher remains armed for follow-up directives. CivicCast context was not used for this run.
