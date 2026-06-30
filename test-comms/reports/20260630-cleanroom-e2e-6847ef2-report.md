# CivicNewspaper Cleanroom E2E Attempt 6 - 6847ef2

Date: 2026-06-30 UTC

Verdict: FAIL.

The attempt-6 build fixed the prior output-quality failure class in an important way: weak/background/watch leads are no longer allowed to slide through as ordinary publishable stories. However, the run fails because the Workbench became stuck at the approval/editor transition and I could not approve the remaining reviewable story or navigate out to Publishing.

## Product Under Test

- Product branch: `main`
- Product commit: `6847ef2844a1a859eb82ae900ef03b08c94b132a`
- Product version: `0.3.0`
- Installer used: NSIS
- Install path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS SHA256 expected: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- NSIS SHA256 observed: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- MSI fallback SHA256 observed: `CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1`

## What Passed

- Product clean wipe completed.
- NSIS install completed.
- Onboarding accepted noisy state input and shell displayed `LONGMONT / CO`.
- App-guided local AI/runtime setup worked without tester manual installation.
- The app installed and used `qwen2.5:7b`.
- First-run Longmont starter sources were seeded without manual import.
- Source count after onboarding: 14.
- Source breadth included official city website, city news, agenda portal, council meetings, public information, public safety, Public Notice Colorado, St. Vrain Valley Schools, Facebook watch sources, subreddits, events, and YouTube.
- Daily Scan completed with `SOURCES WATCHED 14`, `OPEN LEADS 17`, `DRAFTS IN DESK 2`, and `AI STATUS Ready`.
- Low-novelty/background/watch/verification items were visibly labeled and were not presented as ordinary ready stories.
- `Draft anyway` on a background lead showed a clear warning checkpoint before generation.
- That weak generated draft was automatically marked as needing more work with advisory warnings.
- Cut/remove workflow worked through a confirmation modal; the unsuitable weak draft was deleted.

## Break Point

The run failed in Workbench approval/navigation after exercising the editor workflow.

The remaining reviewable story was:

- `Longmont Public Library Offers Teen Temporary Tattoo Studio`
- Status after workflow: `ready_to_review`
- Guardrail warnings: 5 advisory warnings, including missing source links and source wording overlap.

I exercised:

- Save Draft
- Run Advisor
- Hold
- Resume/Mark Ready for Review
- Cut Story on weak/background item
- Attempted approval with editor responsibility attestation

After the story was returned to `ready_to_review`, the Workbench would not complete approval:

- Clicking `Approve for Static Publish` did not transition the draft to `ready_to_publish`.
- The attestation checkbox was visible but did not reliably update the app's approval state through normal WebView-driven clicks.
- Direct DOM toggling changed the raw checkbox state but did not update React/app state, so I did not count it as valid product proof.
- `Back to Queue` did not leave Workbench.
- The `Publishing` nav button did not leave Workbench.

Because no story reached approved/publishable state after this workflow transition, I could not honestly proceed to static compile, ZIP export, or here.now publish for this attempt.

## Counts

- Starter sources: 14.
- Open leads after Daily Scan: 17.
- Drafts created: 2.
- Weak/background draft generated through `Draft anyway`: 1.
- Weak/background draft cut: 1.
- Reviewable draft left in Workbench: 1.
- Approved stories at final state: 0.
- Published stories: 0.
- here.now URL: none for this attempt.
- ZIP output: none for this attempt.

## Evidence

Evidence folder:

`test-comms/evidence/20260630-cleanroom-e2e-6847ef2/`

Key files:

- `00-clean-wipe-summary.json`
- `01-install-launch-summary.json`
- `03-identity-noisy-state-before-next.json`
- `10-model-download-summary.json`
- `12-after-onboarding.json`
- `13-sources-seeded.json`
- `17-daily-scan-page-corrected.json`
- `18-story-queue-inventory.json`
- `19-after-draft-anyway-lead16-click.json`
- `21-lead16-generated.json`
- `25-cut-confirm-modal.json`
- `26-after-force-confirm-cut.json`
- `27-open-library-draft.json`
- `28-after-save-draft.json`
- `29-after-run-advisor.json`
- `30-after-hold.json`
- `31-after-resume-or-ready.json`
- `32-after-mark-ready.json`
- `35-after-final-approve-library.json`
- `37-after-attest-approve-library.json`
- `41-draft-db-state-after-approve-attempt.json`
- `44-direct-dom-approval-attempt.json`
- `46-current-state-before-report.json`
- `47-final-db-snapshot.json`

## Request For Coder

Keep the attempt-6 weak-lead quality behavior. It is materially better than attempt 5: weak/background/watch items are warned, sent back, and removable instead of being counted as good reader-facing copy.

Fix the Workbench approval/navigation dead end:

- A ready-for-review story with warnings should be approvable after the editor checks the responsibility attestation.
- The checkbox must update product state through normal user input.
- `Back to Queue` and sidebar nav should remain usable from Workbench after workflow transitions.
- Once approval works, rerun compile, ZIP export, here.now publish, and public-output scans.

Watcher remains armed.
