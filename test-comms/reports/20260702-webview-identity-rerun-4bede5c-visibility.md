# Tester Visibility Report - WebView Identity Rerun 4bede5c

Date: 2026-07-02T06:34:45Z
Tester machine: MSI / civic
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
Product branch: main
Product commit represented by installer: 4bede5c6773189e24c8aa05a105e503b93111fca
Directive: test-comms/directives/20260702-webview-identity-rerun-4bede5c.md

## Result

BLOCKED.

The packaged Windows app launches and remains visible on first-run Identity, but ordinary identity typing/paste still does not remain visible in the fields. The Longmont starter profile also did not populate visible field values. The app did not advance to AI Service Setup, and the database did not save `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, or `onboarding.step`.

## Environment

- Windows: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 17179869184 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on C: 375225561088 bytes at capture time
- Tester account: civic

Full profile: test-comms/evidence/20260702-webview-identity-rerun-4bede5c/machine-profile.txt

## Installer Verification

- Installer: test-comms/artifacts/20260702-webview-identity-rerun-4bede5c/The Civic Desk_0.3.1_x64-setup.exe
- Expected SHA256: 4A40482D29B2C601CF28A9CAB7E1904A15BDD0653F99E26D250F037BF98662AD
- Actual SHA256: 4A40482D29B2C601CF28A9CAB7E1904A15BDD0653F99E26D250F037BF98662AD
- Expected size: 5653840
- Actual size: 5653840

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` with `git pull --ff-only`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
3. Stopped stale `civicnews` process `10888`.
4. Ran `C:\Users\civic\AppData\Local\The Civic Desk\uninstall.exe`.
5. Removed product state paths:
   - `%APPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%USERPROFILE%\.ollama` was already absent.
6. Installed only from the directive NSIS artifact.
7. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
8. Attempted ordinary typed/pasted identity entry:
   - Publication: `Longmont WebView Identity Desk`
   - Editor: `Cleanroom Tester`
   - City: `Longmont`
   - State: `CO`
9. Repositioned the app window to expose more of the form and confirm whether values remained visible.
10. Tried the Longmont starter profile fallback.
11. Tried direct Unicode keyboard entry.
12. Inspected the database after the attempts.

## Visibility / Persistence Checks

- App remained visible: yes.
- Ordinary identity typing/paste remained visible before Next: no.
- Starter profile selection remained visible: no.
- App advanced from Identity to AI Service Setup: no.
- `identity.newsroom_name` saved: no.
- `identity.editor_name` saved: no.
- `identity.city` saved: no.
- `identity.state` saved: no.
- `onboarding.step` saved: no.
- Product-owned runtime/model setup result: not reached.
- Dashboard local AI ready: not reached.

Final database settings at capture:

```text
model.selected = phi4-mini:latest
```

No `identity.*`, `onboarding.*`, or `setup.*` settings were present.

## Evidence

- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/install-clean-launch.log
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/machine-profile.txt
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/blocked-final-db-window-snapshot.json
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-01-after-launch.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-02-identity-values-before-next.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-03-window-tall-values-visible.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-04-longmont-starter-after-click.png
- test-comms/evidence/20260702-webview-identity-rerun-4bede5c/screenshot-05-unicode-entry-attempt.png

## Request For Coder

The packaged WebView Identity step is still blocked. Please fix or instrument why ordinary keyboard/clipboard entry and the Longmont starter button do not leave values visible or save `identity.*` settings in the installed app.
