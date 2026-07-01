# CivicNewspaper v0.3.1 Stage 5 Cleanroom Result

Date: 2026-07-01T00:37:26Z
Tester: cleanroom tester
Coordination branch: `test-comms/cleanroom-coder-tester`
Directive: `test-comms/directives/20260630-stage5-cleanroom-v031-1273bc7.md`

## Verdict

FAIL.

The v0.3.1 installer installed and launched as an end-user app, guided local AI/runtime setup without manual dependency help, discovered/imported sources, generated leads, exercised draft/editor states, compiled a static publication package, and published a here.now site successfully.

The run fails the Stage 5 directive because the final public output does not meet the publication quality bar:

- The directive targeted 5-10 reader-facing stories or briefs; the compiled publication contains 3 briefs.
- Two published article pages contain only title/metadata/notes and no story body.
- One published article contains public-facing reporter-workflow language: "Next steps should include verifying details..."
- Published output includes stories approved during mechanics testing despite quality warnings and with no attached source links.

## Product And Installer

- Product under test: The Civic Desk / CivicNewspaper
- Installed app version observed in Windows uninstall metadata: `0.3.1`
- Product commit represented by installer: `1273bc73ea660be6852a9ced6b3954fc494b5e29`
- Installer artifact: `test-comms/artifacts/20260630-stage5-cleanroom-v031-1273bc7/The Civic Desk_0.3.1_x64-setup.exe`
- Expected SHA256: `12FF893863684996045A6802406698D825CA6B411006B5355AC8F5C2A4B319B6`
- Observed SHA256: `12FF893863684996045A6802406698D825CA6B411006B5355AC8F5C2A4B319B6`
- Expected size: `5633364`
- Observed size: `5633364`

## Hardware Summary

- OS: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- GPUs observed in earlier visibility report: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU

## Setup And Runtime

- Cleanroom boundary wipe was performed for CivicNewspaper/The Civic Desk state before the run.
- The app installed from the verified NSIS installer.
- The app launched from the normal installed path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- First-run setup selected Longmont, Colorado.
- Local AI setup was app-guided. I did not manually install Ollama, a model, Node, Rust, or other product dependency.
- The app started its bundled Ollama runtime from `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`.
- The app reached "Local AI ready" with model `phi-4-mini:latest`.
- Setup and AI progress screenshots are in the evidence folder.

## Workflow Results

- Sources shown after scan: 19
- Leads shown after scan: 15
- Draft/editor surfaces exercised: at least 6 generated/opened drafts
- Editor states exercised: saved, sent back / needs work, ready for review, held, cut attempt, approved
- Approved/publication output count: 3 briefs in the compiled manifest
- The app did not reach the requested 5-10 reader-facing stories/briefs in the final publication.

## Export And Publish

- Evidence ZIP copied to: `test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/site-package.zip`
- Evidence ZIP SHA256: `0AD9C4C69447F40AEE79D124195A2DEBA6B130CD2A92693EE028A1E2D0000703`
- Manifest-reported local output path: `C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default`
- At report time, that manifest-reported local output folder was no longer present on disk. The copied ZIP evidence remains present in the coordination worktree.
- here.now URL: `https://serene-aurora-nz5r.here.now`
- here.now HTTP checks returned 200 for:
  - `https://serene-aurora-nz5r.here.now/`
  - `https://serene-aurora-nz5r.here.now/briefs/2.html`
  - `https://serene-aurora-nz5r.here.now/briefs/3.html`
  - `https://serene-aurora-nz5r.here.now/briefs/6.html`
  - `https://serene-aurora-nz5r.here.now/feed.xml`

## Public Output Checks

Checked the exported ZIP/static site for the directive's explicit marker list:

- `EDITOR_NOTE`: not found
- `[EDITOR_NOTE`: not found
- `Body:`: not found
- `Headline:`: not found
- `Nut graf`: not found
- `Reporting Steps`: not found
- `[Source needed]`: not found
- `[Verification needed]`: not found
- `[End of Report]`: not found

Checked public ZIP/static output for mojibake code points:

- `U+00C2`: not found
- `U+00C3`: not found
- `U+00E2`: not found
- `U+FFFD`: not found

Quality failures still remain even though explicit marker and mojibake scans passed:

1. `briefs/2.html` contains the title "Downtown Longmont's Creative District Announces July Events" plus metadata and sources/notes, but no article body in the extracted public page.
2. `briefs/3.html` contains the title "City of Longmont Library Announces Summer Reading Challenge Deadline" plus metadata and sources/notes, but no article body in the extracted public page.
3. `briefs/6.html` contains reader-facing workflow/scaffolding text: "Next steps should include verifying details such as exact dates..."
4. The public pages say "Source links attached by the editor: No source links were attached to this article."
5. The final output count is 3 briefs, below the directive's target of 5-10 reader-facing stories or briefs.

## Failures

### Failure 1: Final publication below requested story count

- Step: Directive step 11, produce a reviewable publication targeting 5-10 reader-facing stories or briefs.
- Observed: `publish-manifest.json` has `"article_count": 3`.
- Expected: 5-10 reader-facing stories or briefs if enough material exists, or a clear app-level reason why fewer could be produced.
- Severity: High for Stage 5 readiness.
- Reproduction notes: Run the cleanroom flow through compile/publish. The final manifest in the exported package lists only 3 article entries.

### Failure 2: Two public article pages have no story body

- Step: Directive step 15, inspect article pages.
- Observed: `briefs/2.html` and `briefs/3.html` contain title, metadata, site chrome, and sources/notes, but no article body text.
- Expected: Each public story/brief page should contain reader-facing story content.
- Severity: High.
- Reproduction notes: Open the exported ZIP or live here.now pages for `/briefs/2.html` and `/briefs/3.html`.

### Failure 3: Public article includes reporter workflow language

- Step: Output Quality Checks.
- Observed: `briefs/6.html` includes: "Next steps should include verifying details..."
- Expected: Public output should not contain reporter-note or verification-workflow language masquerading as a finished article.
- Severity: High.
- Reproduction notes: Open `/briefs/6.html` in the exported ZIP or live here.now site.

### Failure 4: Public output has no attached source links

- Step: Directive step 15, inspect article pages and share/feed artifacts.
- Observed: Public pages state: "Source links attached by the editor: No source links were attached to this article."
- Expected: Published civic stories should retain enough source context for reviewability.
- Severity: Medium to high.
- Reproduction notes: Open any published brief page in the exported ZIP.

### Residual Issue: App process and manifest local output path were not durable at report time

- Step: Export and publish verification.
- Observed: The app had no running `civicnews.exe` or product `ollama.exe` process at report time. The manifest-reported local output path `C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default` did not exist when checked after the run. The copied ZIP evidence still exists in the coordination worktree, and the here.now site remains reachable.
- Expected: Ideally the app remains inspectable, or the local output path remains available after export.
- Severity: Medium.
- Reproduction notes: After connector publish, check product processes and the manifest output path.

## Evidence Files

Evidence folder:

`test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/`

Key evidence:

- `first-run-initial.png`
- `fresh-launch-after-full-state-wipe.png`
- `setup-step1-longmont-selected.png`
- `setup-step2-after-next.png`
- `ai-service-wait-10s.png`
- `ai-service-wait-30s.png`
- `ai-service-wait-60s.png`
- `ai-service-wait-120s.png`
- `model-download-started.png`
- `model-download-final-or-latest.png`
- `workspace-after-onboarding.png`
- `daily-scan-started.png`
- `daily-scan-final-or-latest.png`
- `story-queue-after-scan.png`
- `first-generated-draft.png`
- `first-generated-draft-fields.json`
- `editor-state-saved.png`
- `editor-state-sent-back.png`
- `editor-state-ready-review.png`
- `editor-state-held.png`
- `editor-state-held-confirmed.png`
- `batch-draft-actions.json`
- `extra-approvals.json`
- `unsourced-generate-anyway-fields.json`
- `publishing-initial.png`
- `publishing-edit-identity.png`
- `identity-saved.png`
- `compile-checklist.png`
- `compile-result.png`
- `connector-test-result.png`
- `connector-publish-result.png`
- `site-package.zip`
- `site-package-extracted/`

## Bottom Line For Scott

The v0.3.1 app made real progress on first-run setup, app-guided local AI, source discovery, drafting, editor-state controls, export packaging, and here.now publishing. It should not be accepted as Stage 5 cleanroom pass yet. The public output is not publication-ready: it is too small, two article pages have no body, and one published article exposes verification/workflow language to readers.
