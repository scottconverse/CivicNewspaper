# Tester Report - v0.3.2 feb5878 AI Setup Visibility Rerun

Date: 2026-07-04T23:23:39Z
Tester machine: `msi\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product branch: release installer from `v0.3.2`
Product commit: `feb5878e789ac09709531c26ad453cbce72bf1ff`
Directive: `test-comms/ACTIVE_DIRECTIVE.md`
Result: FAIL

## Environment

- Windows version: Windows 10 Home, HAL `10.0.26100.1`
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 16,870,060,032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 366,240,911,360 bytes on `C:`
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama CLI: not on PATH
- App-reported local AI: `Local AI ready`, selected model `phi4-mini:latest`

## Steps Run

1. Pulled `origin test-comms/cleanroom-coder-tester`; branch fast-forwarded from `2f0de2d` to `9869455`.
2. Reread:
   - `test-comms/ACTIVE_DIRECTIVE.md`
   - `test-comms/README.md`
   - `test-comms/protocol.md`
   - `test-comms/prompts/tester-codex-desktop-prompt.md`
   - `test-comms/directives/20260704-final-release-v032-feb5878.md`
3. Reached release URL and public docs URL.
4. Queried GitHub release API for release assets.
5. Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release.
6. Verified installer SHA256:
   - Expected: `CF7D7DBDE3A97486FED198490397A8A662B90A8A34CE5D3F73ACA3CF61A76522`
   - Actual: `CF7D7DBDE3A97486FED198490397A8A662B90A8A34CE5D3F73ACA3CF61A76522`
7. Verified `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same hash.
8. Uninstalled prior `The Civic Desk` via `%LOCALAPPDATA%\The Civic Desk\uninstall.exe /S`.
9. Removed prior app state:
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%APPDATA%\com.scottconverse.civicdesk`
10. Installed the downloaded GitHub release installer with `/S`.
11. Launched installed app from `%LOCALAPPDATA%\The Civic Desk\civicnews.exe` through Windows app automation.
12. Inspected AI Model, Sources, Daily Scan, Workbench, Publishing, and exposed Ethics/System nav entries.
13. Generated a draft from the source-linked `Ready to draft` lead.
14. Exercised Send Back workflow and saved the default decision note.

## Results

- Release/docs visibility: PASS.
- Installer download and checksum verification: PASS.
- Asset inventory: PASS. GitHub release API listed exactly two assets: installer and `SHA256SUMS.txt`.
- Clean uninstall/profile reset: PASS for the identified Civic Desk install and app-data folders.
- Installed-app launch: PASS.
- AI setup visibility: PASS for the available-model path. The AI Model page was above the fold and reported `AI service: Running on this computer`, `Selected model: phi4-mini:latest`, and `AI-assisted workflows ready`.
- AI-unavailable skip path: NOT EXERCISED because local AI was already available in this tester profile after reinstall.
- Source discovery/import: PARTIAL PASS. The app added starter Longmont sources and Sources showed discover/import controls. However, duplicate starter sources were visible after the flow.
- Daily Scan: FAIL. It produced 24 open leads, including correctly downgraded weak items, but also exposed malformed encoded text in a `Ready to draft` lead.
- Drafting: FAIL. The source-linked draft retained encoded HTML entity text and copied a multi-item event listing into the article body.
- Editor workflow: PARTIAL PASS. Send Back opened a note modal and saved state as `Sent back / needs work`; approval stayed disabled until ready/review responsibility is completed.
- Publishing/export/here.now: BLOCKED. Publishing page required at least one approved story before compile/export/publish. Because the only generated source-linked draft was not publication quality and was sent back, I did not force approval or publish a bad public site.
- Public docs/release wording: PASS for v0.3.2, Windows-only beta, unsigned installer guidance, SmartScreen `More info` / `Run anyway`, here.now support, and release download link visibility.

## Evidence

- Visibility report: `test-comms/reports/20260704-final-release-v032-feb5878-visibility.md`
- Checksum file: `test-comms/reports/20260704-final-release-v032-feb5878-evidence/SHA256SUMS.txt`
- Installer receipt:
  - Downloaded from `https://github.com/scottconverse/CivicNewspaper/releases/download/v0.3.2/The.Civic.Desk_0.3.2_x64-setup.exe`
  - Size: `5234632` bytes
  - SHA256: `CF7D7DBDE3A97486FED198490397A8A662B90A8A34CE5D3F73ACA3CF61A76522`
- Release API asset inventory:
  - `SHA256SUMS.txt`, size `102`
  - `The.Civic.Desk_0.3.2_x64-setup.exe`, size `5234632`
- Installed app path: `%LOCALAPPDATA%\The Civic Desk\civicnews.exe`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 2
- Minor: 2
- Nit: 0

### Major 1 - Ready-to-draft lead and generated draft retain encoded HTML entities

Observed:
Daily Scan exposed a source-linked lead as `Ready to draft` while the title/body contained encoded `&#8211;` text:

`Longmont official city website: Independence Weekend Free Concert Friday, July 3 • 6 pm &#8211; 10 pm...`

After clicking `Generate Draft`, the generated story body still contained `&#8211;` repeatedly, including event times such as:

`Friday, July 3 • 6 pm &#8211; 10 pm`

Expected:
Malformed encoded text must be normalized, suppressed, or kept out of ready-to-draft/public copy. Directive step 13 specifically calls out encoded `&#8211;` text as malformed content that should not be draftable as strong news.

Impact:
This directly fails the reader-facing quality gate. A user can generate an article from a malformed ready lead, and the generated copy retains encoded marker text.

Repro:
Install v0.3.2 `feb5878`, run/inspect Daily Scan, open the `LONGMONT OFFICIAL CITY WEBSITE / PRIMARY RECORD` lead with `Ready to draft`, click `Open in Workbench`, then `Generate Draft`.

### Major 2 - Generated draft copies a broad multi-item event listing into article body

Observed:
The generated article body included a long concatenated event listing with repeated items, including `LIBRARY CLOSED`, `Free Fitness in the Park`, `July 4th Longmont Symphony Concert`, `Independence Weekend Festival`, `Lotería Mexicana`, `Bilingual Storytime`, and other items. The same event-listing sequence appeared duplicated in the draft body.

Expected:
Multi-item event calendar concatenations should be suppressed, downgraded, or transformed into a focused, sourced brief rather than copied into a reader-facing draft.

Impact:
This fails directive steps 12, 13, and 15. The draft is not clean public copy and risks publishing markup-debris/calendar-rollup content.

Repro:
Same as Major 1. Inspect the generated `Article Body (Markdown)` field.

### Minor 1 - Clean reinstall still skipped a true first-run onboarding path

Observed:
After uninstalling, removing identified Civic Desk app-data folders, and reinstalling, the app launched directly into the newsroom shell with Longmont state and starter sources rather than a fresh city/onboarding flow. It displayed a setup task saying starter sources had been added and then moved to Daily Scan.

Expected:
A clean profile should make first-run state explicit and natural. If the app intentionally seeds Longmont automatically for this build, the setup should make that state transition easy to audit.

Impact:
This reduces confidence in clean-profile first-run proof, although the app did launch and core navigation was accessible.

Repro:
Uninstall prior app, remove `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\The Civic Desk`, and `%APPDATA%\com.scottconverse.civicdesk`, install the release asset, then launch `%LOCALAPPDATA%\The Civic Desk\civicnews.exe`.

### Minor 2 - Sources list showed duplicated starter sources after setup

Observed:
Sources listed duplicated Longmont starter entries after setup, including duplicate `Longmont official city website`, `Longmont agendas and minutes`, and `Longmont public notices search` rows.

Expected:
Source discovery/import should avoid duplicate starter sources or clearly explain duplicate provenance.

Impact:
Duplicate watched sources can lead to duplicate scan evidence and duplicate/rollup story candidates.

Repro:
Open Sources after the clean reinstall/setup task completes.

## Request For Coder

Fix the Daily Scan/drafting quality gate before another release-pass attempt:

- Decode or strip HTML entities before lead ranking and draft generation.
- Prevent malformed `&#8211;` and `-->` leads from being `Ready to draft`.
- Suppress or downgrade broad multi-item event-calendar concatenations so they cannot become reader-facing drafts.
- Ensure source-linked generated drafts are focused, non-duplicative, and free of raw source/markup debris.
- Clarify whether the feb5878 build is expected to auto-seed Longmont after clean profile removal, or provide a more auditable first-run setup state.
