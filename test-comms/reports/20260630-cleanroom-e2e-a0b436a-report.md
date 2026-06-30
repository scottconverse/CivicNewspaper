# Cleanroom E2E report - a0b436a attempt 1

## Result

FAIL.

The installer, clean first-run AI setup, source ingestion, Daily Scan, local static-site compile, and anonymous here.now public load all worked at least partially. The run fails readiness because the published output contains reader-facing quality failures, the required ZIP package is missing from disk even though the UI marks "Export hosting package" complete, and the full editor workflow could not be completed through the app.

Public preview URL: `https://snowy-pumice-rq7m.here.now/`

## Build under test

- Coordination branch: `test-comms/cleanroom-coder-tester`
- Product branch: `main`
- Product commit: `a0b436af3009500714055a2bff01612716ee36c1`
- Directive: `test-comms/directives/20260630-cleanroom-e2e-a0b436a-attempt1.md`
- Visibility report was committed first at `49d0814`.

## Tester machine

- Windows user: `MSI\civic`
- Hostname: `MSI`
- Coordination clone: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- App data root observed: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- Static output folder: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`

## Installer and hash verification

Used NSIS installer:

- `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64-setup.exe`
- Size: `5605081`
- SHA256: `B6777C66A7330A46F6FC443576C06E648E516EC52EC845004044DB4663A23BD8`
- Installer exit code: `0`

The MSI was visible and hash-verified but not used because NSIS succeeded:

- `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64_en-US.msi`
- Size: `9117696`
- SHA256: `4C4F40178017853DFA5E65AFD10595306018C0F2B803190A1DB431A28CA8AA2E`

## Clean wipe evidence

Before install, the previous `The Civic Desk` install and a prior `.ollama` state existed. I stopped `civicnews.exe`, ran the app uninstaller, and removed CivicNewspaper/Ollama state within the directive boundary.

I initially missed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`; after observing it on launch, I stopped the app/WebView/Ollama processes and wiped:

- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- `C:\Users\civic\.ollama`
- `C:\Users\civic\AppData\Local\Ollama`
- `C:\Users\civic\AppData\Roaming\Ollama`

No Ollama user PATH entries were found.

## First-run and AI setup

- First-run setup displayed the unsigned beta/local setup path.
- The setup screen twice reported that it did not receive input events and advanced automatically.
- The app chose `qwen2.5:7b (Recommended)` on a 15 GB RAM machine.
- The app downloaded and started bundled Ollama runtime `v0.30.11`.
- Completed model: `qwen2.5:7b`, size `4.7 GB`.
- The dashboard reported `Local AI ready qwen2.5:7b`.

The generated community profile was neutral:

- Site title: `My Local Publication`
- City/state: `Longmont, CO`
- About text: `A locally edited publication for this community.`

## Sources and scan results

Configured/observed sources:

- `Longmont Agenda Management Portal` - `https://longmontcolorado.gov/city-clerk/agenda-management-portal/` - online
- `Longmont City Council Meetings` - `https://longmontcolorado.gov/government/city-council-meetings/` - online
- `Longmont Public Information` - `https://longmontcolorado.gov/public-information/` - online
- `Public Notice Colorado` - `https://www.publicnoticecolorado.com/` - online
- `Longmont subreddit` - `https://www.reddit.com/r/Longmont/` - offline
- `Longmont Colorado subreddit` - `https://www.reddit.com/r/LongmontColorado/` - offline

Observed database counts:

- Sources: `6`
- Evidence items: `26`
- Leads: `13`
- Daily scan runs: `2`, both completed
- Daily scan leads: `5`
- Drafts generated: `5`
- Published static watch pages: `4`

## Draft/story list

The app generated these drafts:

- Draft 1, lead 13: `City Council Meeting Details and Participation Rules` - status `draft_generated`
- Draft 2, lead 12: `City Council Set to Discuss Temporary Closure of Hover Street/CO 119 Intersection` - status `ready_to_publish`
- Draft 3, lead 11: `Overview of City Departments in Longmont` - status `ready_to_publish`
- Draft 4, lead 10: `Understanding How to Participate in City Council Meetings` - status `ready_to_publish`
- Draft 5, lead 9: `City to Close Intersection of Hover Street/CO 119 for Overnight Roof Work` - status `ready_to_publish`

Static output pages:

- `watch/2.html` - `City Council Set to Discuss Temporary Closure of Hover Street/CO 119 Intersection`
- `watch/3.html` - `Overview of City Departments in Longmont`
- `watch/4.html` - `Understanding How to Participate in City Council Meetings`
- `watch/5.html` - `City to Close Intersection of Hover Street/CO 119 for Overnight Roof Work`

## Workflow coverage

Completed:

- Install from artifact and launch app.
- First-run setup and app-managed local AI setup.
- Source discovery/import path populated six sources.
- Daily Scan produced leads.
- Draft creation produced five drafts.
- Static compile produced local HTML output.
- Anonymous here.now publish produced a public URL.

Partially completed or failed:

- The app repeatedly returned to a blank pane after draft generation. I had to recover by returning to the dashboard/restarting the app.
- The standalone Workbench tile opened a blank pane when no draft was selected.
- I saw and captured the "Cut this story?" confirmation modal, but canceled it; I did not complete cut/restore.
- I did not complete the full required edit/save/send back/rework/hold/cut/restore/approve sequence on three items through the UI.
- I did not successfully invoke the press-freedom/legal-risk advisor before the publish pass.
- Four drafts were marked `ready_to_publish`/attested as `Publisher`, but the public output quality shows they should not have passed.

## Export and publish

The app compiled local static output here:

`C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`

Committed copy:

`test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/static-output/`

Public URL:

`https://snowy-pumice-rq7m.here.now/`

Verification:

- `index.html`: HTTP `200`, length `3512`
- `watch/2.html`: HTTP `200`, length `3730`
- `watch/3.html`: HTTP `200`, length `9463`
- `watch/4.html`: HTTP `200`, length `10376`
- `watch/5.html`: HTTP `200`, length `8938`

Important publish caveat:

The app-generated output was published anonymously to here.now using the same here.now API flow against the compiled app output. I did not commit or record the one-time here.now claim token. I could not complete the app connector UI path cleanly before reporting because of the UI navigation/blank-pane failures above.

## Public output quality gates

FAIL.

- Duplicate topics: fail. Two pages are about the Hover Street/CO 119 closure, and two pages are about City Council meeting participation/background.
- Reporter/editor note leakage: fail. `watch/2.html` publicly includes `TESTER EDIT: saved during cleanroom workflow exercise; original draft began with editor_note...`
- Public pages are newspaper output, not reporter notes: fail. `watch/2.html` is verification questions and a tester/editor note, not reader-facing newspaper copy.
- Headlines read like reader headlines: mixed. Some are acceptable, but `Overview of City Departments in Longmont` is background/reference material, not a current reader headline.
- No mojibake marker code points: pass in scanned local output.
- No explicit `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, `[End of Report]`: fail in spirit because lower/mixed-case `editor_note`/tester-note material leaked publicly, even though the exact uppercase marker set was not present in the compiled HTML.
- No unconfigured claim that all articles used AI: pass in scanned local output.
- No unconfigured claim of no ads, nonprofit status, public-record-only coverage, or made-up publication identity: pass in scanned local output.
- here.now visible as default preview publish path in docs/UI: pass. The publishing panel text says "publish instantly with here.now" and the share package includes here.now hosting notes.

## ZIP/package failure

FAIL.

The UI marked "Export hosting package" complete and `publish-manifest.json` lists `site-package.zip`, but `site-package.zip` is missing from the actual output folder:

- Expected: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- Observed: file does not exist

This prevents satisfying the required local ZIP/output package check.

## Evidence files

Evidence is under:

`test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/evidence/`

Key screenshots include:

- `clean-first-run-screen.png`
- `identity-longmont-selected.png`
- `ai-service-setup-after-wait.png`
- `ai-model-complete-screen.png`
- `sources-screen-initial.png`
- `story-queue-leads-visible.png`
- `draft-wizard-first-lead.png`
- `draft-generating-first-lead.png`
- `publish-compile-after-click.png`
- `public-here-now-site-edge.png`

The browser screenshot attempt captured the app WebView rather than the public Edge tab because the foreground Edge handle belonged to the app WebView. Public load proof is therefore the HTTP `200` checks listed above.

## Reproduction notes

1. Install the NSIS artifact from the coordination branch.
2. Wipe `AppData\Roaming\com.scottconverse.civicdesk`, `AppData\Local\com.scottconverse.civicdesk`, and `.ollama` before first launch.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
4. Let first-run setup proceed. On this tester, it reported that the setup screen was not receiving input events and advanced automatically.
5. Let the app download `qwen2.5:7b`.
6. Open Story Queue/Daily Scan and generate drafts from the Longmont scan leads.
7. Observe blank-pane recovery issues after draft generation.
8. Compile from Publishing.
9. Check the output folder for `site-package.zip`; it is missing even though the manifest lists it.
10. Check `watch/2.html`; tester/editor note content is visible in public output.

## Watcher status

The 15 minute CivicNewspaper watcher remains armed for follow-up directives.
