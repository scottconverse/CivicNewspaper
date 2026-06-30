# CivicNewspaper Cleanroom E2E Attempt 5 - 3017410

UTC report time: 2026-06-30T12:57:00Z

Verdict: FAIL

## Product Under Test

- Product commit installed: `301741042b1a392885ac2de458cc8985a3084bac`
- Product version: `0.3.0`
- Installer used: NSIS
- Install path used: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS path: `test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256 observed: `0C79098D0B8720978E7AE056430B2DB7F3247D0072574DE05EC5F5AA9737D35C`
- NSIS SHA256 matched directive: yes
- MSI fallback SHA256 observed during visibility check: `2F601F00402ACDA01ECA29597A5866526678F9855F6FB6F5A9DBAD8E2C6D6135`
- MSI SHA256 matched directive: yes

## What Passed

The attempt-5 build fixed the primary identity/state regression from attempt 4.

- During onboarding, I deliberately filled the State field with noisy text `CO94 TES`.
- Before clicking Next, the captured form value had already normalized to `CO`.
- After setup, the main shell displayed `LONGMONT / CO`.
- Daily Scan did not fail with `Invalid city or state format`.

The build also fixed the starter-source regression.

- After first-run setup, the app reported `Added 6 starter Longmont source(s).`
- It seeded official/public Longmont sources without manual Discover/import.

App-guided AI/runtime/model setup also worked.

- The tester did not manually install Ollama or models.
- The app downloaded/installed its local AI runtime.
- The app downloaded/activated `qwen2.5:7b`.
- The shell reported `Local AI ready`.

Daily Scan produced a usable queue.

- Sources watched: 6 starter sources at scan time, later shown as 12 after app/background refresh.
- Open leads / lead count: 16.
- Normal `Draft` leads: 1.
- `Draft anyway` leads: 15.
- High priority: 1.

The single normal `Draft` lead generated one draft:

- Lead: `City Council to vote on roof contract for library`
- Draft count after generation: 1.
- The generated draft body had no forbidden scaffolding markers in the extracted editable field values.

## Break Point

The run failed in the editor/workbench workflow before publishing.

After generating the normal draft, I could open and inspect the workbench once. The editable field extraction showed:

- Title: `City Council to Vote on Roof Contract for Longmont Library`
- Body field present at `draft-editor-textarea`
- Forbidden scaffolding scan on the workbench text/body: no hits for `EDITOR_NOTE`, `Editor Note:`, `TESTER EDIT`, `Nut graf:`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, `[End of Report]`, `Body:`, `[insert`, or `Headline:`.

However, the editor workflow was not stable enough to complete the required edit/save/hold/send-back/approve/publish path:

1. The app repeatedly returned from the workbench/editor back to Story Queue while I was attempting to edit/save the generated draft.
2. Reopening via the lead card showed `Open draft`, but the editor textarea was not consistently available.
3. Opening the dedicated Workbench tab showed the draft picker, but attempting to open from that picker did not reliably load the existing draft editor.
4. The final captured state after trying to open from the Workbench picker instead showed a different `Drafting Article` flow for a verification lead with `Generate anyway`, not the previously generated normal draft editor.

This prevented completing the directive-required editor workflow and prevented local/static/ZIP/here.now publication.

I did not force publication or edit app storage directly because the directive is a product cleanroom workflow test.

## Output and Publishing

- Local static output path: none produced.
- ZIP output path: none produced.
- here.now URL: none.
- Published story count: 0.
- Public output quality scan: not reached because publication was not produced.

## Draft and Lead Counts

- Daily Scan leads: 16.
- Normal Draft leads: 1.
- Draft anyway leads: 15.
- Drafts generated: 1.
- Clean generated draft before manual editing: 1.
- Clean approved stories: 0.
- Published stories/briefs: 0.

Draft-anyway handling:

- I did not approve a cluster of `Draft anyway` items.
- I did not use `Draft anyway` to force five stories after the editor workflow became unstable on the first normal draft.
- The queue clearly labeled most non-ready leads as `Draft anyway`, `Background`, `Watch`, `Editor review`, `Seen before`, or similar cautionary statuses.

Duplicate-topic clustering:

- Not passed, because the issue did not reach approval/publish.
- The scan output still included many city-council process/background items, but the app labeled most of them as background/watch/draft-anyway rather than normal ready stories.

## Evidence

Evidence folder:

`test-comms/evidence/20260630-cleanroom-e2e-3017410/`

Key files:

- `00-clean-wipe-summary.json`
- `01-install-launch-summary.json`
- `02-first-launch.png`
- `02-first-launch-dom.json`
- `03-identity-noisy-state-before-next.json`
- `03-identity-noisy-state-before-next.png`
- `04-after-identity-next.json`
- `04-after-identity-next.png`
- `06-ai-setup-summary.json`
- `08-model-download-summary.json`
- `10-after-coordinate-download-click.json`
- `12-model-download-summary.json`
- `13-daily-scan-page-before-run.json`
- `15-daily-scan-summary.json`
- `15-daily-scan-final.png`
- `17-lead-inventory-text.json`
- `18-normal-draft-click-result.json`
- `20-normal-draft-generated.json`
- `24-after-draft-button-disappeared.json`
- `25-open-normal-draft-workbench.json`
- `26-normal-draft-field-values.json`
- `29-reopened-draft-workbench.json`
- `32-workbench-tab-opened.json`
- `33-workbench-picker-draft-open.json`
- `35-workbench-open-did-not-show-editor.json`
- `35-workbench-open-did-not-show-editor.png`

## Watcher

The watcher remains armed and `test-comms/ACTIVE_DIRECTIVE.md` remains the active directive pointer.
