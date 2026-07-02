# Tester Report - Native Identity Rerun dfc3c22

Date: 2026-07-02T05:53:20Z
Tester machine: MSI / civic
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit: dfc3c22789a388dbede422f5c3ac1750efa707d9, represented by directive installer artifact
Directive: test-comms/directives/20260702-native-identity-rerun-dfc3c22.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 17179869184 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376374489088 bytes on C: at capture time
- Node: not used for this packaged-installer run
- Rust: not used for this packaged-installer run
- npm: not used for this packaged-installer run
- Ollama installed/running: no user `.ollama` state present after wipe; product-owned runtime setup was not reached
- Models present: not reached; DB retained only `model.selected = phi4-mini:latest`

Full profile: test-comms/evidence/20260702-native-identity-rerun-dfc3c22/machine-profile.txt

## Overall Result

BLOCKED.

The dfc3c22 packaged installer passes hash/size verification and launches a visible real Tauri desktop app, but first-run Identity setup still cannot be completed. The visible input fields focus, but entered/pasted/native-keyed identity values do not remain visible and no identity settings are written to the app database. The app stays on Identity and never reaches AI Service Setup.

Because setup did not advance past Identity, the required local AI setup, dashboard, source discovery, Daily Scan, Story Queue audits, draft/workbench workflow, export, and here.now publish path were not reachable in this cleanroom run.

## Steps Run

1. `git -C C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms pull --ff-only`
2. Reread:
   - `test-comms/ACTIVE_DIRECTIVE.md`
   - `test-comms/README.md`
   - `test-comms/protocol.md`
   - `test-comms/prompts/tester-codex-desktop-prompt.md`
   - `test-comms/directives/20260702-native-identity-rerun-dfc3c22.md`
3. Verified installer:
   - SHA256 `2F2B89F973630BDF8AA5310726E30F45D7228C286BD151B97B4BEC63F5BCC9B3`
   - Size `5659065`
4. Clean-wiped product state:
   - stopped stale `civicnews` PIDs `11488` and `24520`
   - ran product uninstaller
   - removed `%APPDATA%\com.scottconverse.civicdesk`
   - removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - confirmed `%LOCALAPPDATA%\The Civic Desk` absent after uninstall
   - confirmed `%USERPROFILE%\.ollama` absent
5. Installed only from `test-comms/artifacts/20260702-native-identity-rerun-dfc3c22/The Civic Desk_0.3.1_x64-setup.exe`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Observed visible app window titled `The Civic Desk` on AI Setup / Identity.
8. Attempted the required Longmont identity through the packaged app UI:
   - Publication: `Longmont Native Identity Desk`
   - Editor: `Cleanroom Tester`
   - City: `Longmont`
   - State: `CO`
9. Attempted direct native UI entry with:
   - field focus and typed input
   - clipboard paste
   - Longmont starter profile button
   - resized/tall window placement to expose more of the form
   - native Windows SendKeys text entry
   - Enter key path
   - visible lower-right click area after scroll attempts
10. Captured screenshots and inspected the database.

## Results

- Installer hash/size: PASS.
- Clean wipe/install/launch: PASS.
- Real installed desktop app visible: PASS.
- Identity Next remained visible and advanced: FAIL/BLOCKED; the app remained on Identity.
- `identity.newsroom_name`: missing.
- `identity.editor_name`: missing.
- `identity.city`: missing.
- `identity.state`: missing.
- `onboarding.step`: missing.
- Product-owned runtime/model setup: NOT REACHED.
- Dashboard local AI ready: NOT REACHED.
- Source discovery / source intake: NOT REACHED.
- Daily Scan: NOT REACHED.
- Story Queue evidence-linkage audit: NOT REACHED.
- City news chrome rescue audit: NOT REACHED.
- Visit Longmont tourism/calendar audit: NOT REACHED.
- City-site navigation audit: NOT REACHED.
- Summer Reading semantic grounding audit: NOT REACHED.
- Workbench/editor workflow: NOT REACHED.
- Export ZIP/package: NOT REACHED.
- here.now publish: NOT REACHED.
- Output quality audit: NOT REACHED.

## Database Counts

From `blocked-final-db-window-snapshot.json`:

- sources: 0
- daily_scan_runs: 0
- daily_scan_leads: 0
- leads: 0
- evidence_items: 0
- lead_evidence: 0
- drafts: 0
- publish_runs: 0
- published_posts: 0

Settings present:

```text
model.selected = phi4-mini:latest
```

Settings absent:

```text
identity.newsroom_name
identity.editor_name
identity.city
identity.state
onboarding.step
```

## Evidence

- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/install-clean-launch.log
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/machine-profile.txt
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/blocked-final-db-window-snapshot.json
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-01-after-launch.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-02-pub-editor-filled-scrolled.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-03-paste-pub-editor.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-04-longmont-starter-scrolled.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-05-window-tall-up.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-06-identity-entry-after-enter.png
- test-comms/evidence/20260702-native-identity-rerun-dfc3c22/screenshot-07-identity-entry-sendkeys.png

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Packaged Identity Inputs Still Do Not Persist Or Advance

Observed: In the real installed Tauri app, the Identity page is visible and the Publication Name field shows focus/caret, but typed/pasted/native-keyed values do not remain visible. The Longmont starter profile button did not visibly populate the form. Pressing Enter and clicking the lower-right action area did not advance setup. The database still contains only `model.selected = phi4-mini:latest`.

Expected: The tester should be able to enter a Longmont publication identity, click Next, remain in a visible app window, advance to AI Service Setup, and see `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, and `onboarding.step` saved.

Impact: First-run setup remains blocked for the packaged Windows app, so cleanroom users cannot reach product-owned local AI setup or the dashboard. All downstream Longmont E2E validation is blocked.

Repro:

1. Clean wipe product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from the dfc3c22 directive artifact.
3. Launch normally.
4. On Identity, attempt to enter a Longmont publication identity through the visible UI.
5. Observe the app remains on Identity and the database does not save `identity.*` or `onboarding.step`.

## Request For Coder

Please fix or instrument the packaged WebView Identity input path. The dfc3c22 native-first handoff did not resolve the cleanroom blocker on this Windows tester machine. The next build should prove that ordinary field entry, starter profile selection, and Next/Enter all produce persisted `identity.*` settings and advance to AI Service Setup in the installed app.
