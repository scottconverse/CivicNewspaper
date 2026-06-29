# Full Clean-Wipe Longmont Publication E2E Report - 4f0b09d

Status: FAIL - functional E2E completed, but clean-wipe certification is not valid

Directive: `test-comms/directives/20260629-full-cleanwipe-longmont-4f0b09d.md`

Product branch: `stable-readiness-local-gates`

Product commit: `4f0b09d9099ca5426c6e75ef36f962906634811a`

Evidence folder: `test-comms/reports/20260629-full-cleanwipe-longmont-4f0b09d-evidence/`

## Summary

The 4f0b09d NSIS installer hash matched, the app installed successfully after a CivicNewspaper/Ollama clean wipe, and the app-owned setup flow installed its local AI runtime/model without tester-installed dependencies.

The product then generated a Longmont issue from 6 sources, 27 evidence items, and 18 leads. Five drafts were generated and approved through the Workbench, the site compiled, a ZIP package was produced, anonymous here.now publishing succeeded, and the public URL returned HTTP 200:

`https://zen-vow-kmmb.here.now`

The exact mojibake scanner passed against local output, ZIP-extracted output, and downloaded here.now pages. Public article titles did not begin with `Draft:`.

I am marking this run failed because the full clean-wipe proof is contaminated: the product recorded its output path as the superseded c3db2ac evidence folder, not the 4f0b09d report folder or the default app-data output path. That means some output-path configuration from a prior tester run survived the wipe boundary, or the tester wipe missed a product/WebView state location. I cannot truthfully certify a clean-machine run from this result.

Additional fail/caveat: the directive asked to kill/cut at least one non-publish item if available. The UI kill path was attempted twice, including a confirmation modal, but the final database still contained only 5 drafts, all `ready_to_publish`; no killed draft persisted.

## Installer And Clean Wipe

Preferred NSIS installer:

`test-comms/artifacts/20260629-full-cleanwipe-longmont-4f0b09d/The Civic Desk_0.2.8_x64-setup.exe`

Hash checks:

- NSIS expected: `7B1A15005679678E1E3E99861D83F4B2BC0741266758C0EEA1898AB56D745CA0`
- NSIS observed: `7B1A15005679678E1E3E99861D83F4B2BC0741266758C0EEA1898AB56D745CA0`
- MSI expected: `5EA52BA952052E600C3736171365C328289A10E87A720180EDD7930D8217F871`
- MSI observed: `5EA52BA952052E600C3736171365C328289A10E87A720180EDD7930D8217F871`

Clean wipe actions completed:

- Stopped `civicnews.exe`.
- Stopped app-owned `ollama.exe`.
- Ran The Civic Desk uninstaller silently; exit code 0.
- Removed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`.
- Removed `C:\Users\civic\.ollama`.
- Verified common Ollama app/runtime directories were absent after wipe.

Evidence:

- `installer-hashes.json`
- `clean-wipe-log.json`
- `install-result.json`

## App-Owned Setup

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

The app launched and completed its own setup:

- Community shown: Longmont, CO.
- Local AI state: ready.
- Model: `qwen2.5:7b`.
- Ollama runtime installed under app data by the product.

No manual Ollama, model, PATH, browser helper, or dependency installation was performed.

First-run note: the app displayed this message during setup:

`The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile. You can edit identity later in Settings.`

The Publishing screen later showed:

- Publication: `My Local Publication`
- Tagline: `Local news and community information.`
- Warning: `Publication name still uses starter text.`

I did not manually invent or repair a Longmont masthead. The public site therefore uses neutral starter identity, not an invented city publication name.

## Sources And Scan

Sources present after the clean setup/scan:

- Longmont Agenda Management Portal - official record
- Longmont City Council Meetings - official record
- Longmont Public Information - official record
- Public Notice Colorado - official record
- Longmont subreddit - community signal
- Longmont Colorado subreddit - community signal

Read-only final database counts:

- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10
- Drafts: 5
- Draft statuses: 5 `ready_to_publish`
- Publish runs: 1
- Published posts: 5

Evidence:

- `03-sources.png`
- `04-daily-scan-progress.png`
- `05-daily-scan-after-wait.png`
- `06-story-queue.png`
- `final-db-state.json`

## Writer / Editor / Advisor Flow

The Workbench was exercised through the visible installed app UI:

- Opened generated drafts from Story Queue.
- Edited title/body on the first draft and saved.
- Ran the press-freedom/legal-risk advisor.
- Used Hold on the first draft before approving.
- Approved five drafts for static publishing using the attestation checkbox and `Approve for Static Publish`.
- Attempted the kill workflow twice. The UI showed the kill confirmation path, but no killed draft persisted in the final database.

Evidence screenshots include:

- `07-current-before-drafting-retry.png`
- `18-editor-save.png`
- `19-advisor-result.png`
- `20-hold-state.png`
- `21-approved-1.png` through `26-approved-6.png` where captured
- `31-kill-candidate-workbench.png`
- `32-kill-confirm-modal.png`
- `33-after-kill-story-confirmed.png`
- `34-story-queue-after-killed-draft.png`

Note: one automation loop attempted to approve a sixth draft, but the final read-only database confirms exactly five persisted drafts, all ready to publish.

## Compile / Export / Publish

The app compiled:

- Article count: 5
- Files written: 22
- Skipped: 0

Public here.now URL:

`https://zen-vow-kmmb.here.now`

HTTP verification:

- Status: 200
- Contains Longmont content: true
- Contains publication/news/story content: true

ZIP SHA256:

`DC2F4B37060A3F7D2A1A72F4FF0F8DB3DCD26BFDD4C1CFB107B53064A8E8D27A`

Critical contamination finding:

The product database recorded the output path as:

`C:/Users/civic/Desktop/CODE/civicnewspaper-test-comms/test-comms/reports/20260629-full-cleanwipe-longmont-c3db2ac-evidence/publication-output/site`

That is the superseded c3db2ac evidence path. The actual output was copied from that product-recorded path into this directive's evidence folder:

`test-comms/reports/20260629-full-cleanwipe-longmont-4f0b09d-evidence/publication-output/site/`

Evidence:

- `35-publishing-screen.png`
- `36-compile-checklist.png`
- `38-publishing-after-actions.png`
- `here-now-http-verification.json`
- `herenow-index.html`
- `copied-output-files.json`
- `publication-output/site/`
- `publication-output/site/site-package.zip`

## Required Output Checks

Exact mojibake scanner:

- Local exported output: PASS
- ZIP-extracted output: PASS
- Downloaded here.now pages: PASS

Draft prefix check:

- Local public HTML titles/H1: PASS, no public titles beginning with `Draft:`
- Downloaded here.now HTML titles/H1: PASS, no public titles beginning with `Draft:`

Publication quality spot checks:

- Homepage lists five real Longmont stories/briefs.
- Article pages load.
- RSS exists.
- About, ethics, how-we-report, and corrections pages exist.
- Share package, newsletter, Substack draft, Facebook copy, subreddit post, and Nextdoor copy files exist.
- Desktop and mobile screenshots render the public page.

Evidence:

- `output-checks.json`
- `zip-extracted/`
- `here-now-downloaded/`
- `39-herenow-desktop.png`
- `40-herenow-mobile.png`

## Findings

### Major - Clean-wipe proof contaminated by stale output path

The product used a superseded c3db2ac evidence folder as its output path during the 4f0b09d clean-wipe run. This means the run cannot be certified as clean-machine proof.

Possible causes:

- The tester wipe missed a product/WebView/local-storage state location.
- The product persists output-path state somewhere outside the wiped app data locations.
- The product restored a stale output path from prior test state.

Impact: fail the full clean-wipe directive even though the functional publish path worked.

### Major - Starter publication identity remains in public output

The public site title is `My Local Publication`, and the Publishing screen warns that the publication name still uses starter text. This avoids the prior invented `Longmont Civic Desk` bug, but it is not ready as a real public Longmont publication identity unless the publisher chooses and saves a real name.

Impact: not ready for Scott to use publicly without identity setup.

### Minor / Workflow - Kill/cut did not persist

The kill workflow was attempted, including confirmation, but the final database still contained only five drafts, all `ready_to_publish`. No killed item persisted.

Impact: the kill/cut requirement was not successfully proven in this clean run.

## Final Assessment

Functional E2E mechanics are working in this run:

- Clean install succeeded.
- App-owned AI/runtime setup succeeded.
- Official and public community sources were present.
- Daily scan produced 18 leads.
- Five stories were drafted and approved.
- Export and ZIP succeeded.
- here.now publish succeeded.
- Mojibake and public `Draft:` title checks passed.

However, this is not a passing full clean-wipe result because stale tester/product output-path state leaked into the run. The next run should wipe any additional WebView/local product state that can store output paths, then repeat the clean-wipe test on the same or next artifact. The product should also make starter publication identity completion a clearer first-run gate before public publishing.
