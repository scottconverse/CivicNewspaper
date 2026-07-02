# 20260702 City News Chrome Rerun 302e9bf - Visibility Report

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Directive: `test-comms/directives/20260702-city-news-chrome-rerun-302e9bf.md`

## Installer Verification

- Installer: `test-comms/artifacts/20260702-city-news-chrome-rerun-302e9bf/The Civic Desk_0.3.1_x64-setup.exe`
- Expected SHA256: `821CCEC384B35FFF8E1C01A602CA3FFB45AF29FA417DA0022FA1631230CD37C4`
- Actual SHA256: `821CCEC384B35FFF8E1C01A602CA3FFB45AF29FA417DA0022FA1631230CD37C4`
- Expected size: `5647473`
- Actual size: `5647473`
- Product commit represented by installer: `302e9bf414dd1fb366229743998432a13d2e3644`

## Cleanroom Install

Prior state was wiped before install:

- stopped stale `civicnews`
- stopped product-owned `ollama`
- ran prior `The Civic Desk` uninstaller
- removed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\The Civic Desk`
- removed `C:\Users\civic\.ollama`

The new installer exited `0`, installed `civicnews.exe`, and initially launched a visible desktop app window titled `The Civic Desk`.

## Product-Owned Runtime And Model

BLOCKED.

The app reached the AI Setup identity screen after clean launch, but it did not reach product-owned local AI setup or a ready dashboard. After entering an identity and clicking `Next`, the app window disappeared from the visible desktop while the `civicnews.exe` process remained alive. Subsequent relaunches returned to the same AI Setup identity screen. Re-entering identity and clicking `Next` reproduced the disappearance.

At the failure point:

- `civicnews.exe` process remained running.
- Window state snapshot reported a `The Civic Desk` window at visible coordinates, but screenshots showed only the desktop.
- No product-owned `ollama` process was running during the 120-second startup watch.
- Database existed and had `model.selected = phi4-mini:latest`, but no identity settings, sources, evidence, leads, drafts, publish runs, or published posts.

## Evidence

- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/install-clean-launch.log`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/model-watch.txt`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/machine-profile.txt`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/blocked-setup-visibility-snapshot.json`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-10s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-30s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-60s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-120s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-relaunched-after-hidden.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-after-unicode-identity-next.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-window-restored-after-next.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-fresh-relaunch-visibility-check.png`

## Notes

This is a visibility/setup blocker before Daily Scan. I did not proceed with source discovery, Story Queue, draft, export, or here.now publish because the installed app could not complete the required setup flow as an end user.
