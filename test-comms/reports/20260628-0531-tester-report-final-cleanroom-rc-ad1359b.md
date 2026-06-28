# Tester Report: Final Cleanroom RC for ad1359b

Directive: `test-comms/directives/20260628-0521-coder-directive-final-cleanroom-rc-ad1359b.md`

Role: tester

Product branch under test: `stable-readiness-local-gates`

Product commit under test: `ad1359b11cf52ca20fb553c3f8c3c06dfd3e1747`

Artifact under test: `test-comms/artifacts/ad1359b/The-Civic-Desk-0.2.8-ad1359b-windows-x64-cleanroom.zip`

## Artifact and Hash Verification

Verified the cleanroom ZIP was extracted and the expected installer hashes matched:

| File | Expected SHA256 | Observed SHA256 | Result |
|---|---|---|---|
| `The Civic Desk_0.2.8_x64-setup.exe` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` | PASS |

Installed with the NSIS installer and launched from the installed app path. No source/Vite app was used for this pass.

## Environment and Profile Reset

- OS: Windows 10 Home / Windows 11-class install (`WindowsVersion 2009`, HAL `10.0.26100.1`).
- CPU: 13th Gen Intel Core i7-13620H.
- RAM: about 15.7 GiB.
- GPU: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU.
- Free disk on `C:` before run: about 354.9 GiB.
- `ollama` CLI: not available on PATH.

Fresh profile was practical. I reset the app-owned package-local profile data for `com.scottconverse.civicdesk` by moving `civicdesk.db`, WAL/SHM, and `community_profile.json` into a timestamped backup folder under package-local app data. No non-app user data was removed.

## Required Final RC Scope Results

### 1. Install / First Launch

Result: PASS

- First launch showed onboarding instead of a blank screen or dead end.
- Completed onboarding with local-only values:
  - Publication: `The Brighton Gazette`
  - Editor: `Cleanroom Tester`
  - City/state: `Brighton, CO`
- AI setup showed the local service starting/offline path and allowed skipping setup after confirmation.
- Default publish and backup paths were app-local under `...\AppData\Roaming\com.scottconverse.civicdesk\...`, not OneDrive/Documents.
- Workspace opened successfully with `Local AI offline`.

### 2. Whole-App Navigation and Controls

Result: PASS

Visited every sidebar item in the installed app:

- Story Queue
- Daily Scan
- Dark Signals
- Verification
- Workbench
- Sources
- AI Model
- Publishing
- Browser Pairing
- Ethics & Backups
- System & Status

Core safe controls and modal exits were exercised where available. Empty states were reachable and explained the next step instead of trapping the user. No blank pages, missing bundled extension state, broken modal exits, or sidebar trap were observed.

### 3. Source Intake Smoke

Result: PASS

- Sources page opened from the clean profile and showed no feeds initially.
- Bulk import modal opened and accepted the edge-case XLSX fixture.
- Review result: `18 importable, 1 duplicate, 2 skipped. Selected: 6.`
- Previous source-intake fixes looked good at a glance:
  - Trailing punctuation was trimmed from the Denver Legistar URL.
  - `http` / `https` variants were normalized/deduped with only the selected safe row imported.
  - Review-needed, malformed, and non-official/social rows stayed unchecked.
- Import completed with `Bulk imported 6 source(s) successfully.`
- Sources table showed imported rows afterward.

### 4. Daily Scan / Local Model Degraded Mode

Result: PASS

- Local AI status remained clearly `Local AI offline` / `qwen2.5:7b`.
- Daily Scan reflected `6` sources watched after import.
- Running Daily Scan did not hang. It displayed `Running the daily scan across your collected evidence...`, then completed with `Daily Scan complete (Scan ID: 1).`
- The page returned to normal controls with `0` open leads.
- The app did not misleadingly claim only that a model was missing while the runtime was offline.

### 5. Workbench / Editor Path

Result: PASS WITH CLEAN-PROFILE LIMITATION

- Workbench was reachable and showed a clear empty state: no lead/draft selected and no drafts yet.
- The empty state pointed back to Story Queue rather than trapping the user.
- Because this clean local-only pass produced no lead/draft/article, I could not complete a full story approval path or trigger the editor attestation gate on a real draft.
- Ethics & Backups remained reachable and the publication identity/legal policy area stayed advisory. Publishing identity review also showed `Advisor: On` and explicitly stated blank footer/legal note is allowed and nothing will be invented.

No blocker/critical/major issue is assigned for the unexercised draft approval branch because the clean profile had no draft-producing content and all reachable editor paths explained the next step.

### 6. Publishing / Output Path

Result: PASS WITH CLEAN-PROFILE LIMITATION

- Publishing page opened and showed the publication identity review.
- Output folder defaulted to app-local roaming data under `...\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
- Compile/export controls were visible. Clicking `Compile approved stories` from a clean profile with no approved stories did not perform a live publish and did not trap the app.
- No homepage/article/RSS/share package was generated because no approved article existed in the clean profile.
- No live external publishing was attempted.

### 7. Browser Extension Path

Result: PASS

- Browser Pairing page opened and showed the local bridge instructions.
- `Open extension folder` opened the packaged `chromium` extension folder in File Explorer.
- Verified `manifest.json` exists at the installed app's browser-extension `chromium` folder.
- The app also displayed inline fallback text after the handoff: if File Explorer did not come forward, use the displayed path.
- Chrome/Edge extension loading was not performed because the directive only required it if practical, and the installed app already verified the packaged folder handoff without needing browser extension setup.

### 8. Narrow / Mobile Window Check

Result: PASS

- Resized the installed app to about 760 px wide using the Windows window-position API after manual drag did not resize the maximized window.
- Navigation collapsed into a compact multi-column layout.
- System & Status remained readable.
- Sources remained reachable; scrolling showed the source actions and imported rows without a sidebar trap or obvious text overlap.

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

No release-blocking findings were observed.

## Evidence

Screenshots captured under:

`work/installed-evidence-final-rc-ad1359b/`

Key screenshots:

- `02-onboarding-default-paths.png`
- `03-workspace-open.png`
- `08-bulk-import-edge-loaded.png`
- `09-bulk-import-edge-review-rows.png`
- `13-bulk-import-bottom-button-visible.png`
- `14-sources-imported.png`
- `16-daily-scan-with-sources.png`
- `18-daily-scan-result.png`
- `20-publishing-checklist-visible.png`
- `21-publishing-compile-approved-blocked.png`
- `22-browser-pairing-page.png`
- `23-browser-pairing-after-open-folder.png`
- `26-system-status.png`
- `28-narrow-system-status-winapi.png`
- `30-narrow-sources-content.png`

No separate application log file was needed for this pass; all observed issues were UI-state/evidence based and there were no crashes.

RC verdict: CLEAR
