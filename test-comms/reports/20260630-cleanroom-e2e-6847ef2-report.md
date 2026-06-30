# CivicNewspaper Cleanroom E2E Attempt 6 - Final Report

UTC report time: 2026-06-30T13:54:00Z

Verdict: FAIL.

The attempt-6 build fixed several major workflow mechanics, but the final public output still fails the directive's public-output quality gate. The app installed from the NSIS installer, completed app-guided AI/runtime/model setup, seeded Longmont sources, ran Daily Scan, enforced visible weak-lead checkpoints, compiled a static issue, exported a ZIP, and published to here.now. The generated public article still leaked source-check scaffolding and unlinked-evidence text, and local/ZIP output contained mojibake marker `Â`.

## Repo And Directive

- Coordination branch: `test-comms/cleanroom-coder-tester`
- Coordination HEAD observed before final report: `d94f808 test-comms: visibility for cleanroom e2e 6847ef2 [skip ci]`
- Active directive: `test-comms/directives/20260630-cleanroom-e2e-6847ef2-attempt6.md`
- Product branch: `main`
- Product commit installed: `6847ef2844a1a859eb82ae900ef03b08c94b132a`
- Product version: `0.3.0`

## Installer And Cleanroom Setup

- Clean wipe evidence: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/00-clean-wipe-summary.json`
- Installer used: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/The Civic Desk_0.3.0_x64-setup.exe`
- Installed executable: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS expected SHA256: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- NSIS observed SHA256: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- MSI fallback expected SHA256: `CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1`
- MSI fallback observed SHA256: `CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1`
- NSIS install result: PASS.
- MSI fallback used: no.

## App-Guided AI Setup

- App-guided runtime setup worked without tester manually installing Ollama or models.
- Evidence: `05-ai-setup-poll.json`, `06-ai-setup-summary.json`, `10-model-download-summary.json`
- The app downloaded/installed its local runtime and model through the first-run flow.
- Model shown ready: `qwen2.5:7b`
- Shell identity after onboarding: `LONGMONT / CO`
- Identity result: PASS.

## First-Run Sources

Starter sources were seeded without manual import.

Source count observed: 14.

Representative sources:

- Longmont official city website
- Longmont city news
- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Longmont Public Safety
- Public Notice Colorado
- St. Vrain Valley Schools
- City of Longmont Facebook
- Longmont Public Safety Facebook
- Longmont subreddit
- Longmont Colorado subreddit
- Longmont city events
- Longmont city YouTube

Evidence: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/13-sources-seeded.json`

Source breadth result: PASS.

## Daily Scan And Queue

- Daily Scan completed.
- Lead count: 17.
- Draft count after normal draft: 1.
- High priority count: 0.
- Source count in queue: 14.

The queue did not present weak/background/watch/verification leads as ordinary ready stories. It used labels such as:

- `Watch`
- `Background`
- `Needs verification`
- `Ready to draft`
- `Draft`
- `Draft anyway`

Only one lead had the normal `Draft` action:

- `Longmont Public Library hosting Teen Temporary Tattoo Studio`

All other visible lead actions were `Draft anyway` and had explicit risk/treatment/novelty context.

Queue labeling result: PASS.

## Editorial Workflow

Normal ready lead:

- Opened normal `Draft` action for the Teen Temporary Tattoo Studio brief.
- Generated draft.
- Edited copy.
- Saved draft.
- Put story on hold.
- Resumed editing.
- Marked ready for review.
- Attempted approval.
- App displayed a conscious review checkpoint before final approval because advisory warnings remained.
- Clicked `Publish anyway (logged)`.
- App reported: `Story approved for publishing; a verification record was saved.`

Approval checkpoint text included:

- `Publish with review warnings?`
- `This story has 5 review warning(s) from your newsroom's guardrail and story-quality checks. The app will not veto the editor, but this decision is recorded with the story.`
- Warning examples included citation coverage and verbatim overlap.

Weak/watch lead:

- Opened `Draft anyway` for `Vision Zero projects updates`.
- App did not present this as ordinary ready drafting. It showed `Generate anyway` and labeled the lead `Watch`.
- Generated anyway only to test the weak-lead path.
- App automatically reported: `Draft generated and marked as needing more work because the lead has watch, background, verification, recurrence, or low-novelty signals.`
- Workbench status: `Sent back / needs work`.
- Guardrail warning: `The linked lead is marked as watch. Confirm a current, specific, verified development before treating this as a reader-facing news story.`
- Cut the weak story.
- App required confirmation and then reported: `Story status updated to 'killed'.`
- The cut story's `Approve for Static Publish` control became disabled.

Workflow result: PASS.

## Static Output, ZIP, And here.now

- Local static output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Tester output copy: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/tester-output/`
- Evidence output copy: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/site-output-copy/`
- ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- ZIP copied to tester output folder: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/tester-output/site-package.zip`
- ZIP SHA256: `3B1BE5083A1F1E4C0EAAED35F3042DD2F871F181CEF6E83F45C82588200D0E3F`
- Publish manifest: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/tester-output/publish-manifest.json`
- Issue id: `issue-20260630-134431-264139800`
- here.now URL: `https://quaint-larch-pvqx.here.now`
- Deployment id: `slug=quaint-larch-pvqx;version=01KWCCE1DDG3GHRJWGWTEVJ39S;created_slug=quaint-larch-pvqx`
- Article count: 1
- Files written: 18
- here.now fetch result: HTTP 200 for `/` and `/watch/1.html`

Publish mechanics result: PASS.

## Public Output Quality Failure

Quality scan evidence: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/quality-scan/quality-scan-results.json`

Scanned:

- Local output folder
- ZIP extract
- here.now fetched pages

The public output failed the directive's public-output quality checks.

Failures:

1. Mojibake marker `Â` appears in local output and ZIP output:
   - `watch/1.html`
   - Example source text includes `Wednesday, July 1 Â· 6 pm - 7 pm`

2. Public article leaks source-check/reporting scaffolding:
   - Marker: `Source check:`
   - Marker: `unlinked-evidence`
   - Appears in local output, ZIP extract, and live here.now fetched page.

3. Public article includes a blockquote that reads like internal verification scaffolding:
   - `Source check: The AI draft referenced unlinked evidence ID(s) 355. Those citation markers were disabled automatically. Verify the claim against the linked sources before publishing.`

4. The generated article count is only 1, not 5. The app did not provide five ordinary ready reader-facing leads. The weak/watch path now correctly blocks or warns, but this means the run cannot honestly claim five successful reader-facing stories from this scan.

5. The published article is tagged `Format: watch` even though the approved lead was a brief/event item. This is confusing public taxonomy and should be reviewed.

Output quality result: FAIL.

## Exact User-Visible Result

The user could complete install, onboarding, AI setup, scan, draft, edit, approval-with-warning, compile, ZIP export, and here.now publication.

The user could not get a clean public issue that satisfies the directive's quality bar because the public article still includes mojibake and source-check/unlinked-evidence scaffolding.

## Evidence Index

- Visibility report: `test-comms/reports/20260630-cleanroom-e2e-6847ef2-visibility-attempt-6.md`
- Clean wipe: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/00-clean-wipe-summary.json`
- Install launch: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/01-install-launch-summary.json`
- Identity/onboarding: `02-first-launch*`, `03-identity-noisy-state-before-next*`, `04-after-identity-next*`
- AI setup/model: `05-ai-setup*`, `06-ai-setup*`, `08-model-download*`, `10-model-download*`
- Sources: `13-sources-seeded.json`
- Daily Scan/queue: `14-daily-scan*`, `15-daily-scan*`, `16-story-queue-before-drafting*`
- Normal draft workflow: `17-after-normal-draft-click*` through `28-back-to-queue-after-approval*`
- Weak-lead workflow: `29-weak-watch-draft-anyway-opened*` through `33-weak-watch-cut-confirmed*`
- Publishing: `34-publishing-tab-start*` through `43-here-now-publish-final-state*`
- Output copy: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/site-output-copy/`
- Tester output: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/tester-output/`
- Quality scan: `test-comms/evidence/20260630-cleanroom-e2e-6847ef2/quality-scan/quality-scan-results.json`

## Final Result

FAIL. Attempt 6 is substantially improved on source seeding, weak-lead workflow, and publishing mechanics, but public output still fails because mojibake and source-check/unlinked-evidence scaffolding appear in the generated public article and because the run produced only one approved article.
