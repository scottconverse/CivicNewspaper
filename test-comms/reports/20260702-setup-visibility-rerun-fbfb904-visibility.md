# 20260702 Setup Visibility Rerun fbfb904 - Visibility Report

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Directive: `test-comms/directives/20260702-setup-visibility-rerun-fbfb904.md`

## Installer Verification

- Installer: `test-comms/artifacts/20260702-setup-visibility-rerun-fbfb904/The Civic Desk_0.3.1_x64-setup.exe`
- Expected SHA256: `D6ABA5A6D17D46AD466BB745D02D9DB3EB3AA5986150A5CC4B17248905A93BF4`
- Actual SHA256: `D6ABA5A6D17D46AD466BB745D02D9DB3EB3AA5986150A5CC4B17248905A93BF4`
- Expected size: `5661647`
- Actual size: `5661647`
- Product commit represented by installer: `fbfb90464590d0f643cdb8189ee0a44ef5597a5b`

## Cleanroom Install

Prior state was wiped before install:

- stopped stale `civicnews`
- stopped product-owned `ollama`
- ran prior `The Civic Desk` uninstaller
- removed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\The Civic Desk`

The new installer exited successfully, installed `civicnews.exe`, and launched a visible desktop app window titled `The Civic Desk`.

## AI Setup Identity Next

BLOCKED.

The app was visible on AI Setup identity. I entered a Longmont cleanroom identity through the UI:

- publication name: intended `Longmont Chrome Gate Desk`
- editor name: intended `Cleanroom Tester`
- city: `Longmont`
- state: `CO`

Because of the current viewport, I had to scroll/resize/reposition the setup window to expose all fields and the `Next` button. Once the visible city/state fields were filled and `Next` was clicked, the app process exited/disappeared. The app did not advance visibly to AI Service Setup.

## Relaunch Recovery

BLOCKED.

Per directive, I relaunched the app normally one time after the disappearance. Relaunch succeeded visually, but it returned to AI Setup step 1 with blank/restarter identity state. Database settings still contained only:

- `model.selected = phi4-mini:latest`

No `identity.*` settings and no `onboarding.step` were present, so identity and setup step did not restore.

## Product-Owned Runtime And Model

BLOCKED.

The product-owned local AI setup did not complete and the dashboard did not reach local AI ready. No product-owned `ollama` process was confirmed during this run.

## Evidence

- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/install-clean-launch.log`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/machine-profile.txt`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/blocked-final-db-window-snapshot.json`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-01-after-launch.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-11-city-state-next-visible.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-23-bottom-next-shifted-left.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-24-after-click-next.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-25-after-city-state-next.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-26-one-relaunch-recovery.png`

## Notes

This remains a first-run setup blocker. I did not proceed to source discovery, Daily Scan, Story Queue, draft, export, or here.now publish because the installed app could not advance from AI Setup identity to AI Service Setup as a normal end-user flow.
