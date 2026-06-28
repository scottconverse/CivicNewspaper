# Directive: Verify Extension Path Resolver For 513341b

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit represented by artifact: `513341b`

## What Changed Since 4df344f

Coder addressed the failed extension-handoff report:

`test-comms/reports/20260628-0309-tester-report-extension-handoff-4df344f.md`

Fixes in product commit `513341b`:

- Extension path resolver now checks multiple installed package layouts:
  - `resource_dir/browser-extension/chromium`
  - `resource_dir/_up_/browser-extension/chromium`
  - executable directory equivalents
  - development checkout fallback
- Resolver now requires `manifest.json`, not just folder existence.
- Browser Pairing preserves and displays the resolved path even if Windows does not confirm the Explorer handoff.
- Windows opener no longer uses the brittle `/e,` Explorer argument.

## Artifact To Test

Download/extract:

`test-comms/artifacts/513341b/The-Civic-Desk-0.2.8-513341b-windows-x64-cleanroom.zip`

Expected hashes:

```text
NSIS_SHA256=5B13A9D233C8B3EDC88C36F3459C894326389F42BE1E0E784E2196CFB0CA6245
MSI_SHA256=CCB4EECEDE4096100A6FA7B254E4F89555A6DF7820EAE573E18891351E98EA75
```

## Required Test

1. Verify hash.
2. Install and launch the app from the prebuilt artifact.
3. Reach Browser Pairing.
4. Click **Open extension folder**.
5. Verify the page shows an inline status and the resolved folder path.
6. Verify the displayed path exists and contains `manifest.json`.
7. Record whether Explorer opens, but do not fail solely because automation cannot target Explorer if the in-app path is correct and usable.
8. Confirm no blocker/critical/major appears in this focused path.

## Report Requirements

Write a report under `test-comms/reports/` with:

- pass/fail for the previous extension-handoff major;
- screenshot/log evidence path;
- displayed path;
- manifest verification result;
- severity counts;
- explicit statement whether the cleanroom installer loop is ready for full GauntletGate rerun.

Do not modify product code on the tester machine.
