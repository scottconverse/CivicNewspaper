# Tester Visibility Report - Native Identity Rerun dfc3c22

Date: 2026-07-02T05:53:20Z
Tester machine: MSI / civic
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
Product branch: main
Product commit represented by installer: dfc3c22789a388dbede422f5c3ac1750efa707d9
Directive: test-comms/directives/20260702-native-identity-rerun-dfc3c22.md

## Result

BLOCKED.

The packaged Windows app launches and remains visible on the first-run Identity step, but identity input still does not persist through native field entry attempts. The app did not advance to AI Service Setup, and the database did not save `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, or `onboarding.step`.

## Environment

- Windows: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 17179869184 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on C: 376374489088 bytes at capture time
- Tester account: civic

Full profile: test-comms/evidence/20260702-native-identity-rerun-dfc3c22/machine-profile.txt

## Installer Verification

- Installer: test-comms/artifacts/20260702-native-identity-rerun-dfc3c22/The Civic Desk_0.3.1_x64-setup.exe
- Expected SHA256: 2F2B89F973630BDF8AA5310726E30F45D7228C286BD151B97B4BEC63F5BCC9B3
- Actual SHA256: 2F2B89F973630BDF8AA5310726E30F45D7228C286BD151B97B4BEC63F5BCC9B3
- Expected size: 5659065
- Actual size: 5659065

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` with `git pull --ff-only`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
3. Stopped stale `civicnews` processes `11488` and `24520`.
4. Ran `C:\Users\civic\AppData\Local\The Civic Desk\uninstall.exe`.
5. Removed product state paths:
   - `%APPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%USERPROFILE%\.ollama` was already absent.
6. Installed from the directive NSIS artifact only.
7. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
8. Attempted to enter the required Longmont identity using visible packaged-app UI fields:
   - Publication: `Longmont Native Identity Desk`
   - Editor: `Cleanroom Tester`
   - City: `Longmont`
   - State: `CO`
9. Tried direct field typing, clipboard paste, the Longmont starter profile button, enlarged/tall window placement, native Windows SendKeys text entry, Enter, and a visible lower-right click area.
10. Inspected the database after the attempts.

## Visibility / Persistence Checks

- App remained visible: yes.
- App advanced from Identity to AI Service Setup: no.
- `identity.newsroom_name` saved: no.
- `identity.editor_name` saved: no.
- `identity.city` saved: no.
- `identity.state` saved: no.
- `onboarding.step` saved: no.
- Relaunch recovery needed: no app disappearance occurred; the blocker is failure to enter/persist identity and advance.
- Product-owned runtime/model setup result: not reached.
- Dashboard local AI ready: not reached.

Final database settings at capture:

```text
model.selected = phi4-mini:latest
```

No `identity.*`, `onboarding.*`, or `setup.*` settings were present.

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

## Request For Coder

The identity setup path is still blocked in the packaged Windows app. Please fix or instrument why the real WebView Identity fields show focus but do not retain entered values or advance the setup handoff.
