# Tester Report: Full Cleanroom Release Gauntlet 513341b

Date: 2026-06-28T04:12Z
Tester machine: Windows 11 Home 10.0.26200, Intel Core i7-13620H, 15.7 GB RAM, Intel UHD Graphics + NVIDIA GeForce RTX 4050 Laptop GPU
Repo: `https://github.com/scottconverse/CivicNewspaper.git`
Product branch: `stable-readiness-local-gates`
Product commit/artifact: `513341b`
Directive: `test-comms/directives/20260627-2150-coder-directive-full-cleanroom-release-gauntlet-513341b.md`
Verdict: **CLEAR**
First-run coverage: **VALID**

Private local user paths are redacted as `<USER>`.

## Environment Attestation

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 355 GB on C:
- Node/npm/rustc on shell PATH: absent/not required for end-user installer pass
- Installed app path: `C:\Users\<USER>\AppData\Local\The Civic Desk`
- Installed executable: `C:\Users\<USER>\AppData\Local\The Civic Desk\civicnews.exe`
- Installed sidecar: `C:\Users\<USER>\AppData\Local\The Civic Desk\ollama.exe`
- Browser extension path: `C:\Users\<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium`
- Clean profile/app-data path observed during this Codex Desktop run: `C:\Users\<USER>\AppData\Local\Packages\<CodexPackage>\LocalCache\Roaming\com.scottconverse.civicdesk`
- State files observed: `civicdesk.db`, `civicdesk.db-shm`, `civicdesk.db-wal`, `community_profile.json`
- First-run state: genuinely fresh after uninstall and app-data reset
- Ollama/system models: no downloaded model used; installed sidecar `ollama.exe` was started by the app, and UI reported `Local AI offline` / `qwen2.5:7b`

## Hash Verification

Expected:

```text
NSIS_SHA256=5B13A9D233C8B3EDC88C36F3459C894326389F42BE1E0E784E2196CFB0CA6245
MSI_SHA256=CCB4EECEDE4096100A6FA7B254E4F89555A6DF7820EAE573E18891351E98EA75
```

Observed:

```text
The Civic Desk_0.2.8_x64-setup.exe
5B13A9D233C8B3EDC88C36F3459C894326389F42BE1E0E784E2196CFB0CA6245

The Civic Desk_0.2.8_x64_en-US.msi
CCB4EECEDE4096100A6FA7B254E4F89555A6DF7820EAE573E18891351E98EA75
```

Hash result: **PASS**

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, and directives.
2. Verified 513341b NSIS/MSI hashes.
3. Stopped prior `civicnews`, `ollama`, and Explorer evidence windows.
4. Ran the existing uninstaller silently.
5. Removed prior cleanroom app state from package-local roaming/local app-data.
6. Installed the 513341b NSIS installer silently.
7. Launched the installed desktop app from `civicnews.exe`.
8. Completed onboarding from fresh state with model download skipped.
9. Walked Story Queue, Daily Scan, Dark Signals, Verification, Workbench, Sources, AI Model, Publishing, Browser Pairing, Ethics & Backups, and System & Status.
10. Tested safe offline controls, including Story Queue refresh/Daily Scan and Publishing local compile surface.
11. Reconfirmed browser extension folder handoff, inline path, Explorer behavior, and `manifest.json`.
12. Resized the app to narrow width and checked required pages for reachability.

## Results

- Fresh install: **PASS**
- Real desktop launch: **PASS**
- Natural first-run onboarding: **PASS**
- Skip model download without dead end: **PASS**
- Workspace reachable after onboarding: **PASS**
- Default publish/backup paths shown in UI are app-local, not OneDrive/Documents: **PASS**
- State written to clean app-data location for this Codex Desktop run: **PASS**
- Story Queue: **PASS**
- Daily Scan missing-source / missing-model degraded state: **PASS**
- Dark Signals empty/degraded state: **PASS**
- Verification empty state: **PASS**
- Workbench empty/no-draft state: **PASS**
- Sources empty state: **PASS**
- AI Model screen with offline local AI state: **PASS**
- Publishing local-output surface: **PASS**
- Browser Pairing extension path: **PASS**
- Ethics & Backups: **PASS**
- System & Status: **PASS**
- Narrow layout reachability: **PASS**
- Source fixture import TXT/CSV/XLSX/DOCX/PDF: **NOT RUN - no fixture artifact provided**

## Offline / Missing Dependency Behavior

The no-model/no-source path stayed understandable:

- Onboarding allowed the model download to be skipped after an explicit modal.
- Workspace showed `Local AI offline` and selected model `qwen2.5:7b`.
- Daily Scan showed `Add Sources First` and explained that at least one source is required.
- Dark Signals showed no recent evidence and directed the user to add/fix/widen sources.
- Workbench showed `No lead or draft selected` and `No drafts exist yet`.
- System & Status showed `Local AI Service: Offline`, `SQLite Schema Version: v1.1.0`, `Source Scanner: Runs on demand`, and `Build Release version: v0.2.8`.

## Publishing Local Output

Publishing showed the app-local output folder and the safe local actions:

- `Compile approved stories`
- `Preview local website`
- `Export hosting package`

With zero approved stories, I did not observe a false success claim or live external publishing action. No live provider credentials were used.

## Browser Extension Path

Browser Pairing passed the required checks:

- **Open extension folder** opened Explorer to `chromium - File Explorer`.
- Explorer showed 8 extension files including `manifest.json`.
- Browser Pairing displayed the inline fallback path:

```text
\\?\C:\Users\<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium
```

Filesystem verification:

```text
background.js
content.js
icon.png
manifest.json
popup.css
popup.html
popup.js
README.md
```

## Narrow Layout

At approximately 560 px app width:

- The app switched to a compact two-column nav.
- Workbench content remained reachable and showed the no-draft state.
- System & Status content remained reachable and readable.
- Browser Pairing content remained reachable and readable.
- Sources and Publishing nav targets were visible and selectable; their full-width content had already passed.

No narrow content trap or unreadable overlap was observed in the required focused path.

## Source Intake

TXT/CSV/XLSX/DOCX/PDF import smoke was **not run** because no approved fixture artifact was present in the directive or repo test-comms artifacts. Please provide a fixture bundle in a follow-up directive if this needs to be release-gating.

## Evidence

Local evidence screenshots were captured under:

```text
work/installed-evidence-513341b-gauntlet/
```

Key evidence:

- `01-first-run-identity.png`
- `03-ai-service-degraded.png`
- `05-skip-model-modal.png`
- `06-default-paths.png`
- `08-workspace-story-queue.png`
- `11-daily-scan-page.png`
- `12-dark-signals-page.png`
- `14-workbench-page.png`
- `16-sources-page.png`
- `17-ai-model-page.png`
- `18-publishing-page.png`
- `22-system-status-page.png`
- `24-publishing-actions.png`
- `28-browser-extension-click-result.png`
- `29-browser-inline-path.png`
- `30-narrow-current.png`
- `41-narrow-workbench-index.png`
- `45-narrow-system-index.png`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

No blocker, critical, major, minor, or nit findings were observed in this full cleanroom pass.

## Release Readiness

This 513341b installer artifact is **ready for coder-side full GauntletGate and release-candidate prep** from the cleanroom tester perspective.

Remaining non-blocking ask: provide source-intake fixture artifacts if TXT/CSV/XLSX/DOCX/PDF import smoke should be part of the next release gate.
