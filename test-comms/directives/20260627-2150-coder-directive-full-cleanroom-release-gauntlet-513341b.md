# Directive: Full Cleanroom Release Gauntlet For 513341b

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit represented by artifact: `513341b`  
Artifact branch: `test-comms/cleanroom-coder-tester`

## Why This Directive Exists

The focused cleanroom loop has resolved the prior install/runtime blockers:

- source-build blocker was bypassed with a prebuilt end-user installer;
- first-run onboarding from installer passed;
- no-Ollama / missing-model degraded states passed;
- narrow Workbench and other narrow pages passed;
- default publish/backup paths no longer point into OneDrive/Documents;
- installed browser-extension resources are bundled and reachable;
- Open extension folder now opens Explorer and shows an in-app resolved path.

Now run a full cleanroom release gauntlet against the installed 513341b artifact.

## Artifact To Test

Use the already-published artifact:

`test-comms/artifacts/513341b/The-Civic-Desk-0.2.8-513341b-windows-x64-cleanroom.zip`

Expected hashes:

```text
NSIS_SHA256=5B13A9D233C8B3EDC88C36F3459C894326389F42BE1E0E784E2196CFB0CA6245
MSI_SHA256=CCB4EECEDE4096100A6FA7B254E4F89555A6DF7820EAE573E18891351E98EA75
```

Prefer NSIS.

## Required Scope

Run this as an end-user cleanroom pass, not a source-build pass.

### 1. Environment Attestation

Record:

- Windows version, CPU, RAM, GPU, disk free.
- Install path.
- App-data/database/settings path.
- Whether Ollama/system models were absent, running, or bundled sidecar only.
- Whether first-run state was genuinely fresh.
- Whether the app wrote state to the expected clean profile/app-data location.

### 2. First-Run / Onboarding

From clean state:

- install;
- launch;
- complete onboarding;
- skip model download;
- verify no dead end;
- verify user reaches the workspace.

### 3. Core Product Walkthrough

Click through every nav item and core visible control that is safe offline:

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

Do not perform live external publishing unless separately directed. Local compile/export is allowed if safe.

### 4. Offline / Missing Dependency Behavior

Verify with no downloaded model:

- Daily Scan degraded copy and controls;
- draft/workbench degraded behavior;
- AI Model setup screen;
- System & Status;
- any diagnostics/export path that is safe.

### 5. Source Intake

If local fixture files are available on tester, run file import smoke across at least TXT/CSV/XLSX/DOCX/PDF. If not available, mark this as not run and ask coder to provide a fixture artifact in the next directive.

### 6. Publishing Local Output

If the app has enough data to compile a site, run local compile/export. If no approved story exists, verify the empty/blocked state is understandable and does not claim success. Do not use live provider credentials.

### 7. Browser Extension Path

Reconfirm:

- Open extension folder button;
- inline path;
- `manifest.json`;
- Explorer behavior.

### 8. Narrow Layout

At narrow app width, verify content remains reachable for:

- Sources
- Publishing
- Workbench
- System & Status
- Browser Pairing

### 9. Report

Write a new report under `test-comms/reports/`.

Include:

- verdict: CLEAR / DO NOT ADVANCE / BLOCKED;
- first-run coverage VALID or INVALID with evidence;
- severity counts;
- every finding with observed/expected/impact/repro;
- what passed;
- whether this is ready for coder-side full GauntletGate and release-candidate prep.

Do not modify product code on tester.
