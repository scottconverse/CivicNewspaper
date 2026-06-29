# CivicNewspaper cleanroom rerun report - output scaffolding cleanup c01e32f

Status: FAIL

UTC run window: 2026-06-29T16:05Z to 2026-06-29T16:46Z

Coordination branch: `test-comms/cleanroom-coder-tester`

Directive: `test-comms/directives/20260629-output-scaffolding-rerun-c01e32f.md`

Product commit under test: `c01e32fdccb50b5a19182b7128f666e8de5cc304`

Evidence folder: `test-comms/reports/20260629-output-scaffolding-rerun-c01e32f-evidence/`

## Summary

The fixed installer installed and launched, app-guided AI setup completed, Longmont starter sources loaded, Daily Scan produced leads, five fresh drafts were generated and approved, the static site compiled with five articles, ZIP export worked, and here.now publish returned a live URL.

The rerun still fails the required output-quality bar. A reader-facing article still contains the forbidden internal marker `EDITOR_NOTE` in:

- local static output: `publication-output/site/watch/2.html`, line 45
- extracted ZIP output: `zip-extract-check/watch/2.html`, line 45
- live here.now output: `public-herenow-article-5.txt`, line 15

Live here.now URL:

`https://sentient-hill-qczg.here.now`

## Installer

Preferred NSIS installer:

`test-comms/artifacts/20260629-output-scaffolding-rerun-c01e32f/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`9A2828D9B98EBBDEA2F625F5BD3EEFAB824B79E6A80FF8FD57AF7EF534D415DE`

Observed SHA256:

`9A2828D9B98EBBDEA2F625F5BD3EEFAB824B79E6A80FF8FD57AF7EF534D415DE`

Install result:

- Method: NSIS
- Exit code: 0
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App launched with WebView CDP available.

Evidence:

- `installer-hashes.json`
- `clean-wipe-log.json`
- `clean-wipe-followup-log.json`
- `install-result.json`
- `launch-result.json`

## Cleanroom Notes

The first wipe attempt found an app-owned `ollama.exe` locked under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\...`. I stopped the app-owned Ollama process, removed the roaming app data, and reinstalled the NSIS artifact before continuing. No reboot was used.

Evidence:

- `clean-wipe-log.json`
- `clean-wipe-followup-log.json`

## App Setup and Sources

The app completed its normal first-run AI setup and reported:

- city/state: `LONGMONT / CO`
- AI status: `Local AI ready`
- model: `qwen2.5:7b`

The app added six starter Longmont sources:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit
- Longmont Colorado subreddit

Evidence:

- `01-first-launch.txt`
- `02-ai-setup-*.png`
- `ai-setup-states.json`
- `24-leads-tab.txt`

## Daily Scan

Daily Scan ran from the UI and produced 14-15 visible leads, with 6 watched sources and high-priority civic leads.

Evidence:

- `03-daily-scan-before.png`
- `04-daily-scan-state-*.png`
- `05-story-queue-after-scan.txt`
- `daily-scan-states.json`

## Editorial Workflow

The first automation pass selected the Drafts tab by mistake and produced no drafts; this is documented in:

- `draft-editor-results.json`
- `23-current-state.txt`

I corrected the path by using the Leads tab's per-lead Draft buttons. Five distinct fresh leads were drafted and approved:

- Longmont Seeks Human Service Agency Funding
- Public Records Request Options
- City Council Meeting Schedule and Agendas
- Upcoming City Council Meeting
- New City Council Meeting Portal

The press-freedom/legal-risk advisor was exercised during the first corrected Workbench pass. The app also allowed hold on one draft. I did not find an exposed return/send-back control after hold in the state tested, so I recorded that as unavailable rather than fabricating it.

Evidence:

- `draft-editor-results-corrected.json`
- `draft-editor-results-recovery.json`
- `hold-return-result-corrected.json`
- `32-advisor-result-1.png`
- `33-after-approve-*.png`
- `42-after-hold.png`

## Publication

The first compile produced 0 articles because no drafts were actually approved yet. After correcting the draft path, the second compile produced:

- 5 articles
- 22 files
- 0 skipped
- ZIP package created
- here.now URL returned after direct click on the exact `Publish with connector` button

Generated output folder:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-output-scaffolding-rerun-c01e32f-evidence\publication-output\site`

ZIP:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-output-scaffolding-rerun-c01e32f-evidence\publication-output\site\site-package.zip`

here.now URL:

`https://sentient-hill-qczg.here.now`

Evidence:

- `86-publishing-final.txt`
- `91-after-final-publish-click.txt`
- `publish-ui-final-click-result.json`
- `zip-hash.json`
- `zip-check.json`
- `public-herenow-home.txt`
- `public-herenow-article-*.txt`

## Blocking Failure

The mandatory marker audit failed.

`output-quality-audit.json` found:

```text
local-output publication-output/site/watch/2.html line 45:
<p>[EDITOR_NOTE: This looks like background material, not a publishable news story yet.] A specific announcement date or significant updates regarding the application process would make this more relevant for immediate readership and action.</p>

here-now public-herenow-article-5.txt line 15:
[EDITOR_NOTE: This looks like background material, not a publishable news story yet.] A specific announcement date or significant updates regarding the application process would make this more relevant for immediate readership and action.
```

`zip-check.json` found the same marker in the extracted ZIP:

```text
zip-extract-check/watch/2.html line 45:
<p>[EDITOR_NOTE: This looks like background material, not a publishable news story yet.] A specific announcement date or significant updates regarding the application process would make this more relevant for immediate readership and action.</p>
```

This is still a public reader-facing article page. The directive explicitly fails on any reader-facing public artifact containing `EDITOR_NOTE`.

## Additional Observations

The generated drafts still stored editor-note content internally in the database even when the app warned the editor. The fixed output cleanup appears to strip some prior markers but not bracketed editor-note text of the form `[EDITOR_NOTE: ...]`.

The here.now publish path worked only after clicking the exact `Publish with connector` button. Earlier broader publish-click automation reached only local export. The final successful here.now evidence is in `publish-ui-final-click-result.json`.

## Reproduction Steps

1. Use coordination branch `test-comms/cleanroom-coder-tester`.
2. Run directive `test-comms/directives/20260629-output-scaffolding-rerun-c01e32f.md`.
3. Install the NSIS artifact with SHA256 `9A2828D9B98EBBDEA2F625F5BD3EEFAB824B79E6A80FF8FD57AF7EF534D415DE`.
4. Complete app-guided setup for Longmont until `Local AI ready / qwen2.5:7b`.
5. Run Daily Scan.
6. Draft and approve the lead `Longmont Seeks Human Service Agency Funding`.
7. Compile approved stories and export ZIP.
8. Publish to here.now.
9. Open `publication-output/site/watch/2.html`, `zip-extract-check/watch/2.html`, or `https://sentient-hill-qczg.here.now/watch/2.html`.
10. Observe `[EDITOR_NOTE: ...]` in reader-facing article body.

## Final Determination

FAIL.

The install/setup/scan/editorial/ZIP/here.now mechanics are functional enough to execute the run, but the intended scaffolding cleanup is incomplete. A public issue still leaks the forbidden `EDITOR_NOTE` marker into local static output, extracted ZIP output, and live here.now output.
