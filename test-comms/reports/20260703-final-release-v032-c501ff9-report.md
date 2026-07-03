# Final Cleanroom Release Verification - The Civic Desk v0.3.2

Directive: `test-comms/directives/20260703-final-release-v032-c501ff9.md`  
Tester machine/user: `msi\civic`  
Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`  
Comms branch: `test-comms/cleanroom-coder-tester`  
Product release: `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2`  
Public docs: `https://scottconverse.github.io/CivicNewspaper/`  
Result: FAIL

## Verdict

The v0.3.2 release does not meet the directive's final-release bar. Installer download, checksum verification, clean install, first-run Longmont setup, local AI runtime installation, starter source intake, and Daily Scan mechanics all worked. However, release/docs checksum guidance is not sufficient, Daily Scan produced major reader-facing/editor-quality defects, and the workflow did not reach a completed draft/export/publish artifact.

Because the directive says the release passes only with zero blocker/critical/major findings, this is a FAIL.

## Evidence Paths

- Visibility report: `test-comms/reports/20260703-final-release-v032-c501ff9-visibility.md`
- Release assets evidence: `test-comms/reports/20260703-final-release-v032-c501ff9-evidence/`
- Installer: `test-comms/reports/20260703-final-release-v032-c501ff9-evidence/The.Civic.Desk_0.3.2_x64-setup.exe`
- Checksums: `test-comms/reports/20260703-final-release-v032-c501ff9-evidence/SHA256SUMS`

## Release Asset Verification

Downloaded from the GitHub v0.3.2 release, not from a local build path.

- Installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Expected size: `5214089`
- Actual size: `5214089`
- Expected SHA256: `9C3B6670A445233C0CDAF98F49505A89C6D88E034DD391471357762092872533`
- Actual SHA256: `9C3B6670A445233C0CDAF98F49505A89C6D88E034DD391471357762092872533`
- `SHA256SUMS` content matched the installer hash.

Pass: release asset size and checksum matched the directive.

## Cleanroom Install and First Run

Initial install attempt found residual app state under the app data path, so that attempt was discarded.

Corrected cleanroom steps:

- Uninstalled the existing app with the shipped uninstaller.
- Stopped residual app-managed `ollama` and `llama-server` processes that held files open.
- Removed app state directories, including the active `com.scottconverse.civicdesk` state.
- Reinstalled the downloaded GitHub release installer silently.
- Launched installed app: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.

First-run setup result:

- Workspace setup appeared on first launch after the corrected wipe.
- Identity was set to Longmont / CO.
- The app initially could not reach local AI.
- The in-app "Install local AI runtime" path downloaded the Ollama runtime, started it, detected the recommended model, and continued.
- Main app showed `Local AI ready` with `phi4-mini:latest`.
- Starter source intake completed.

Pass: clean install and first-run Longmont setup worked after a corrected state wipe.

## Source and Daily Scan Results

Database inspected after the run:

- `sources`: 9
- `daily_scan_runs`: 1
- `daily_scan_runs.run_status`: `completed`
- Scan started: `2026-07-03T17:56:06.667186700+00:00`
- Scan completed: `2026-07-03T17:57:26.845613800+00:00`
- `evidence_items`: 26
- `leads`: 24
- `daily_scan_leads`: 21
- `verification_tasks`: 56
- `dark_signals`: 9

Source status snapshot:

- Online: Longmont official city website, Longmont public notices search, St. Vrain Valley Schools, Longmont Leader local news, Longmont Area Chamber of Commerce, Longmont city events.
- Offline/failing: Longmont agendas and minutes, Longmont Public Safety, r/Longmont.

Pass: source intake and Daily Scan ran to completion.

## Major Findings

### M1 - Release/docs checksum guidance does not show the expected hash

The release and docs are supposed to explain checksum verification with the expected hash. The release page contains the literal text `SHA256: $hash` and does not contain the expected installer SHA256. The public docs also did not contain the expected installer SHA256.

Observed checks:

- Release page contains literal `SHA256: $hash`: yes.
- Release page contains expected SHA256: no.
- Public docs contain expected SHA256: no.
- Release page mentions SmartScreen/unsigned installer: yes.
- Public docs mention SmartScreen/unsigned installer: yes.
- Public docs contain literal `More info`: no.
- Public docs contain literal `Run anyway`: no.

Impact: users cannot verify the installer from the release/docs guidance without relying on a separate checksum asset. This fails the release-readiness requirement for public installer verification.

### M2 - Daily Scan output still surfaces non-news and malformed leads as draftable/lead material

Daily Scan completed, but the resulting queue still included multiple output-quality failures:

- A `ready_to_draft` lead whose title/body is a concatenated city homepage/event scrape: "Longmont official city website: Independence Weekend Free Concert Friday, July 3..." followed by multiple unrelated event snippets and HTML entity text such as `&#8211;`.
- A lead titled "Review community signal from Longmont city events: -->", which exposes markup/navigation debris as a civic signal.
- A lead claiming "City of Longmont Library Closure for Independence Day" says the downtown library will be closed from July 3 to August 6, a high-risk claim that appears to be an event/date parsing error.
- A Chamber lead pulls broad Colorado General Assembly session text into Longmont civic-signal output.

Some weak items were marked `needs_verification` or `watch`, which is good, but at least one malformed concatenated event scrape was marked `ready_to_draft`. This misses the directive's quality bar for duplicate-topic, mojibake/entity, and reporter-note/output-scaffolding checks.

Impact: a cleanroom user can be guided toward drafting from noisy or misleading material after the first scan.

### M3 - Draft/export/publish workflow did not complete

After Daily Scan, the app reported 24 leads and 0 drafts. Multiple UI attempts were made to focus/filter/select a lead and use the draft path. The UI repeatedly selected page text instead of accepting filter input cleanly. The database confirmed:

- `drafts`: 0
- `publish_runs`: 0
- `published_posts`: 0

Because no draft was produced, the editor workflow, static export ZIP, and here.now anonymous preview publication were not completed in this cleanroom run.

Impact: the final release walkthrough did not produce the required reader-facing publish artifact.

## Other Observations

- The app displays an in-app unsigned beta notice: "You're running an unsigned public beta of The Civic Desk. Windows SmartScreen may warn on install; that's expected."
- The active app data for this run appeared under the Codex package LocalCache because the app was launched from the Codex desktop environment: `C:\Users\civic\AppData\Local\Packages\OpenAI.Codex_2p2nqsd0c76g0\LocalCache\Roaming\com.scottconverse.civicdesk\civicdesk.db`.
- The release installer itself verified correctly; the checksum finding is about public guidance, not binary integrity.

## Reproduction Notes

1. Pull `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md` and `test-comms/directives/20260703-final-release-v032-c501ff9.md`.
3. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS` from the v0.3.2 GitHub release.
4. Verify installer size and SHA256.
5. Uninstall existing The Civic Desk.
6. Stop app-managed `ollama`/`llama-server` if they hold app data open.
7. Remove app state and reinstall the downloaded release.
8. Launch installed `civicnews.exe`.
9. Complete Longmont / CO setup.
10. Use in-app local AI runtime installer when AI is unreachable.
11. Let starter source intake finish and run Daily Scan.
12. Inspect Story Queue output and the active `civicdesk.db`.

## Final Status

FAIL. The installer asset itself is correct and the app can run first-run setup with local AI, but the release is not ready under this directive because public checksum guidance is incomplete/wrong, Daily Scan quality still has major defects, and the cleanroom workflow did not reach draft/export/publish completion.
