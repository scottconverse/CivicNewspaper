# Directive: Cleanroom First-Run And Dependency-Absent Validation

From: `coder`  
To: `tester`  
Branch: `test-comms/cleanroom-coder-tester`  
Product branch to test: `stable-readiness-local-gates`  
Repo: `https://github.com/scottconverse/CivicNewspaper.git`

## Goal

Prove or disprove CivicNewspaper's real Windows desktop first-run behavior on a clean machine/profile. Browser-forced first-run is not enough.

## Target Machines

Use one of these:

- Windows 11, Intel CPU, 16 GB RAM, 8 GB VRAM GPU.
- Windows 11, Ryzen 7/9, integrated GPU, 32 GB RAM.

Record exact CPU, RAM, GPU, Windows version, disk free space, and whether Ollama is already installed.

## Required Tests

1. Clone the repo and check out `stable-readiness-local-gates`.
2. Build or run the desktop app from the real Tauri path, not just Vite browser preview.
3. Use a genuinely clean app-data state:
   - Prefer a fresh Windows user profile or VM snapshot.
   - If not available, redirect `APPDATA` and `LOCALAPPDATA` to an isolated temp directory and prove where the app writes its database/settings.
4. Confirm first-run state is not forced by query string or dev-only URL.
5. Exercise onboarding from first launch through the workspace.
6. Test dependency-absent behavior:
   - no Ollama running;
   - Ollama running but selected model missing;
   - model available, if practical.
7. Exercise Daily Scan degraded behavior without a model.
8. Exercise Draft/Workbench degraded behavior without a model.
9. Capture screenshots and logs for every state.
10. Write a report under `test-comms/reports/`.

## Pass Criteria

The report must prove:

- Real Tauri app launched.
- Clean first-run state was used.
- App-data/database/settings location is known and isolated.
- Missing Ollama/model messages are accurate and user-understandable.
- User can complete onboarding and reach the workspace.
- No dead end prevents a new user from continuing.

## Blocked Criteria

If any required tool is missing, install it if reasonable. If a hard boundary remains, write a blocked report with:

- the exact missing tool or permission;
- what was attempted;
- the smallest human action needed.
