# Directive: Rerun Prebuilt Installer Cleanroom Test For 67994ae

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit represented by artifact: `67994ae`  
Artifact branch: `test-comms/cleanroom-coder-tester`

## What Changed Since c2ed188

Coder addressed the three findings from:

`test-comms/reports/20260628-0222-tester-report-prebuilt-installer-c2ed188.md`

Fixes in product commit `67994ae`:

1. Narrow Workbench visibility:
   - mobile/narrow sidebar is no longer sticky at <=560px;
   - Workbench empty-state card gets explicit mobile sizing/padding.
2. Extension folder opener:
   - Windows opener now calls Explorer with an explicit folder-open argument.
3. Default publish/backup paths:
   - onboarding and app bootstrap defaults now use app-local data paths, not Documents/OneDrive.

## Artifact To Test

Download/extract this file from the `test-comms/cleanroom-coder-tester` branch:

`test-comms/artifacts/67994ae/The-Civic-Desk-0.2.8-67994ae-windows-x64-cleanroom.zip`

It contains:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`
- `SHA256SUMS.txt`

Expected hashes:

```text
NSIS_SHA256=8BF4D50772584F1C0640D16BF73B0315AD9ED47E89AB0E5FB156B3384AA49D05
MSI_SHA256=217B7B8DD8B11B76564F3D92C5B2D5EB0CC868A7BBC2EA749CBFC3D8D814C57F
```

Prefer NSIS first.

## Required Rerun

Run the same cleanroom installer validation as the prior directive, with special attention to the previous findings:

1. Verify hashes.
2. Uninstall the prior app if present.
3. Clear only the test app's cleanroom state/profile data as needed to get a fresh first-run.
4. Install and launch the prebuilt app, not source/Vite.
5. Complete onboarding.
6. Confirm default Publish Path no longer points under OneDrive/Documents.
7. Verify missing Ollama and missing selected model states are understandable and non-dead-end.
8. Verify Workbench at narrow width:
   - about 508px wide;
   - select Workbench;
   - scroll if needed;
   - confirm the Workbench empty-state content is visibly reachable, not just present in accessibility.
9. Verify Open extension folder:
   - click the app button;
   - confirm Explorer opens or, if automation cannot target it, confirm any visible system handoff and record what happened;
   - verify installed resource folder still contains `manifest.json`.
10. Check Sources, Publishing, and System narrow-width behavior again.

## Report Requirements

Write a new report under `test-comms/reports/` with:

- pass/fail for the three prior findings;
- full first-run environment attestation;
- screenshots/log paths;
- any new findings with severity;
- whether this build is ready for a full GauntletGate rerun.

Do not modify product code on the tester machine.
