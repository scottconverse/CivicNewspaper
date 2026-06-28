# Directive: Verify Browser Extension Folder Handoff For 4df344f

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit represented by artifact: `4df344f`

## What Changed Since 67994ae

Coder addressed the remaining Minor from:

`test-comms/reports/20260628-0239-tester-report-prebuilt-installer-67994ae-rerun.md`

Fix in product commit `4df344f`:

- Browser Pairing now shows a persistent inline status after **Open extension folder** is clicked.
- The inline status includes the resolved browser extension folder path, so the handoff is observable even when Windows automation cannot target File Explorer.

## Artifact To Test

Download/extract:

`test-comms/artifacts/4df344f/The-Civic-Desk-0.2.8-4df344f-windows-x64-cleanroom.zip`

Expected hashes:

```text
NSIS_SHA256=7C9C00D535D804A454695A07B7AF97EBA14F8B92AC39494C6476FC925035D988
MSI_SHA256=CD9E6A1BAA5BB15EC42F14D67BDE5DC298736908E3604D66ED4C13908A18DE96
```

## Required Test

This is a focused rerun. You may reuse the same cleanroom machine and prior uninstall/reset process.

1. Verify the artifact hash.
2. Install and launch the app from the prebuilt artifact.
3. Complete or bypass onboarding as needed to reach Browser Pairing.
4. Click **Open extension folder**.
5. Verify the Browser Pairing page shows inline status text and the resolved extension path.
6. Verify that path exists and contains `manifest.json`.
7. If File Explorer opens, record that too, but the inline status/path is the required proof.
8. Confirm no new blocker/critical/major issue appears in this focused path.

## Report Requirements

Write a new report under `test-comms/reports/` with:

- pass/fail for the previous extension-handoff minor;
- screenshot/log evidence path;
- severity counts;
- explicit statement whether the cleanroom installer loop is now ready for full GauntletGate rerun.

Do not modify product code on the tester machine.
