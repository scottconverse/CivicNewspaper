# Tester Report - WebView Identity Rerun 4bede5c

Date: 2026-07-02T06:34:45Z
Tester machine: MSI / civic
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit: 4bede5c6773189e24c8aa05a105e503b93111fca, represented by directive installer artifact
Directive: test-comms/directives/20260702-webview-identity-rerun-4bede5c.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 17179869184 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 375225561088 bytes on C: at capture time
- Node: not used for this packaged-installer run
- Rust: not used for this packaged-installer run
- npm: not used for this packaged-installer run
- Ollama installed/running: no user `.ollama` state present after wipe; product-owned runtime setup was not reached
- Models present: not reached; DB retained only `model.selected = phi4-mini:latest`

Full profile: test-comms/evidence/20260702-webview-identity-rerun-4bede5c/machine-profile.txt

## Overall Result

BLOCKED.

The 4bede5c packaged installer passes hash/size verification and launches a visible real Tauri desktop app, but first-run Identity setup still cannot be completed. Ordinary keyboard/clipboard entry did not remain visible, the Longmont starter did not populate visible values, direct Unicode keyboard entry did not persist, and no identity settings were written to the app database. The app stayed on Identity and never reached AI Service Setup.

Because setup did not advance past Identity, the required local AI setup, dashboard, source discovery, Daily Scan, Story Queue audits, draft/workbench workflow, export, and here.now publish path were not reachable in this cleanroom run.

## Steps Run

1. `git -C C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms pull --ff-only`
2. Reread:
   - `test-comms/ACTIVE_DIRECTIVE.md`
   - `test-comms/README.md`
   - `test-comms/protocol.md`
   - `test-comms/prompts/tester-codex-desktop-prompt.md`
   - `test-comms/directives/20260702-webview-identity-rerun-4bede5c.md`
3. Verified installer:
   - SHA256 `4A40482D29B2C601CF28A9CAB7E1904A15BDD0653F99E26D250F037BF98662AD`
   - Size `5653840`
4. Clean-wiped product state:
   - stopped stale `civicnews` PID `10888`
   - ran product uninstaller
   - removed `%APPDATA%\com.scottconverse.civicdesk`
   - removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - confirmed `%LOCALAPPDATA%\The Civic Desk` absent after uninstall
   - confirmed `%USERPROFILE%\.ollama` absent
5. Installed only from `test-comms/artifacts/20260702-webview-identity-rerun-4bede5c/The Civic Desk_0.3.1_x64-setup.exe`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Observed visible app window titled `The Civic Desk` on AI Setup / Identity.
8. Attempted ordinary typed/pasted Longmont identity:
   - Publication: `Longmont WebView Identity Desk`
   - Editor: `Cleanroom Tester`
   - City: `Longmont`
   - State: `CO`
9. Repositioned the window to show more form content and verify whether values remained visible.
10. Tried the Longmont starter button.
11. Tried direct Unicode keyboard entry.
12. Captured screenshots and inspected the database.

## Results

- Installer hash/size: PASS.
- Clean wipe/install/launch: PASS.
- Real installed desktop app visible: PASS.
- Ordinary identity typing/paste remained visible before Next: FAIL/BLOCKED.
- Starter profile selection remained visible: FAIL/BLOCKED.
- Identity Next remained visible and advanced: FAIL/BLOCKED; setup remained on Identity.
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
- Chrome/navigation/source-grounding prior blocker audits: NOT REACHED.
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

- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/install-clean-launch.log
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/machine-profile.txt
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/blocked-final-db-window-snapshot.json
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-01-after-launch.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-02-identity-values-before-next.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-03-window-tall-values-visible.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-04-longmont-starter-after-click.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-05-unicode-entry-attempt.png

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Packaged Identity Inputs And Starter Profile Still Do Not Persist

Observed: In the real installed Tauri app, the Identity page is visible and the Publication Name field shows focus/caret, but ordinary typed/pasted values do not remain visible after attempted entry and window reposition. The Longmont starter button did not visibly populate the form. Direct Unicode keyboard entry also did not persist. The database still contains only `model.selected = phi4-mini:latest`.

Expected: Ordinary native field entry should remain visible before Next. Longmont starter selection should populate visible identity fields if tested. Clicking Next should advance to AI Service Setup and save `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, and `onboarding.step`.

Impact: First-run setup remains blocked for the packaged Windows app, so cleanroom users cannot reach product-owned local AI setup, dashboard, or the Longmont E2E publication flow.

Repro:

1. Clean wipe product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from the 4bede5c directive artifact.
3. Launch normally.
4. On Identity, attempt ordinary typed/pasted Longmont publication identity.
5. Try the Longmont starter profile.
6. Observe the app remains on Identity and the database does not save `identity.*` or `onboarding.step`.

## Request For Coder

Please fix or instrument the packaged WebView Identity input path further. Build 4bede5c did not resolve the cleanroom blocker on this Windows tester machine: ordinary entry, starter profile selection, and direct Unicode key entry all failed to persist visible identity values or advance setup.
