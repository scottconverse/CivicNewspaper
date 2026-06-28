# Tester Report: 4df344f Browser Extension Folder Handoff

From: `tester`
To: `coder`
Directive: `test-comms/directives/20260627-2055-coder-directive-extension-handoff-4df344f.md`
Product branch under test: `stable-readiness-local-gates`
Product commit/artifact under test: `4df344f`
Result: **FAIL - previous extension-handoff issue is not resolved**

## Summary

The prebuilt 4df344f installer hash-verified, installed, launched, and reached Browser Pairing after onboarding. The installed browser extension directory exists on disk and contains `manifest.json`.

However, after clicking **Open extension folder**, the Browser Pairing page displayed an inline failure instead of the required resolved extension path:

```text
Couldn't open the browser extension folder: The requested item couldn't be found. It may have been moved or deleted.
```

No targetable File Explorer window opened during the click check. Because the directive requires inline status text that includes the resolved extension path, and that path must then be verified, the previous extension-handoff minor remains failing in this artifact.

## Environment

- Tester role: `tester`
- Comms branch: `test-comms/cleanroom-coder-tester`
- Product branch: `stable-readiness-local-gates`
- Product commit: `4df344f`
- Artifact tested: `test-comms/artifacts/4df344f/The-Civic-Desk-0.2.8-4df344f-windows-x64-cleanroom.zip`
- Installer used: `The Civic Desk_0.2.8_x64-setup.exe`
- Install scope: per-user Windows install

Private local user paths are redacted below as `<USER>`.

## Hash Verification

Expected hashes from directive:

```text
NSIS_SHA256=7C9C00D535D804A454695A07B7AF97EBA14F8B92AC39494C6476FC925035D988
MSI_SHA256=CD9E6A1BAA5BB15EC42F14D67BDE5DC298736908E3604D66ED4C13908A18DE96
```

Observed hashes:

```text
The Civic Desk_0.2.8_x64-setup.exe
7C9C00D535D804A454695A07B7AF97EBA14F8B92AC39494C6476FC925035D988

The Civic Desk_0.2.8_x64_en-US.msi
CD9E6A1BAA5BB15EC42F14D67BDE5DC298736908E3604D66ED4C13908A18DE96
```

Hash result: **PASS**

## Install And Launch

Cleanroom reset steps:

- Stopped `civicnews` and `ollama` processes if present.
- Ran the existing `The Civic Desk` uninstaller silently.
- Removed app test state under the package-local `Roaming\com.scottconverse.civicdesk` and `Local\com.scottconverse.civicdesk` folders.

Install result:

```text
Install exit code: 0
DisplayName: The Civic Desk
DisplayVersion: 0.2.8
InstallLocation: C:\Users\<USER>\AppData\Local\The Civic Desk
```

Launch result: **PASS**

## Onboarding Path

Onboarding completed/bypassed sufficiently to reach the main workspace and Browser Pairing:

- Entered cleanroom publication metadata.
- Local AI service step was allowed to initialize/check.
- Model download was skipped with the expected limited-mode modal.
- Default generated paths were app-local, not OneDrive:

```text
Publish Path: C:\Users\<USER>\AppData\Roaming\com.scottconverse.civicdesk\sites\default
Backup Path: C:\Users\<USER>\AppData\Roaming\com.scottconverse.civicdesk\backups
```

Onboarding result: **PASS**

## Browser Extension Handoff Check

Clicked **Open extension folder** on Browser Pairing.

Expected per directive:

- Browser Pairing shows persistent inline status after click.
- Inline status includes the resolved browser extension folder path.
- The resolved path exists and contains `manifest.json`.

Observed:

- Inline status appears: **PASS**
- Inline status includes resolved extension path: **FAIL**
- Inline status reports folder-open failure: **FAIL**
- File Explorer target window observed: **FAIL / not observed**
- Installed extension folder exists independently on disk: **PASS**
- Installed extension folder contains `manifest.json`: **PASS**

Observed inline status:

```text
Couldn't open the browser extension folder: The requested item couldn't be found. It may have been moved or deleted.
```

Filesystem verification of installed extension payload:

```text
C:\Users\<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium

background.js
content.js
icon.png
manifest.json
popup.css
popup.html
popup.js
README.md
```

Explorer process check after the click found `explorer.exe` running but with no targetable main-window title/path for the opened extension folder.

Extension handoff result: **FAIL**

## Evidence

Local evidence screenshots/logs were captured under:

```text
work/installed-evidence-4df344f/
```

Key screenshots:

- `14-browser-pairing-top-0.png`
- `15-open-folder-button-visible-0.png`
- `16-after-open-folder-click-0.png`

`16-after-open-folder-click-0.png` shows the inline failure text after clicking **Open extension folder**.

## Severity Counts

- Blocker: 0
- Critical: 0
- Major: 1
- Minor: 0

Major:

1. Browser Pairing extension-folder handoff does not provide the required resolved path proof and instead reports that the requested item could not be found, even though the installed extension directory exists and contains `manifest.json`.

## Readiness Statement

The cleanroom installer loop is **not ready for full GauntletGate rerun** based on this focused directive. The remaining extension handoff issue is now observable, but the observed status is a failure and does not include the resolved extension path required by the directive.

