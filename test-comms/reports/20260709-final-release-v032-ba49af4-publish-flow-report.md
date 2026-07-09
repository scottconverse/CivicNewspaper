# Final Cleanroom Release Verification - The Civic Desk v0.3.2 ba49af4

Tester branch: `test-comms/cleanroom-coder-tester`  
Directive: `Final Cleanroom Release Verification - The Civic Desk v0.3.2 ba49af4 Publish Flow Rerun`  
Checked at: 2026-07-09 UTC  
Overall result: FAIL for full static publish flow. Release visibility, install, onboarding, scan, AI setup, draft generation, and approval override passed; static compile/publish remained blocked by the public identity gate and repeated WebView/CDP loss while trying to save a real publication title.

## Target

- Product commit under test: `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`
- Release/docs commit: `4ba609690e0094c453b4a2852fd209cc8c8b2c83`
- Installer: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Installer SHA256: `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`

Evidence folder:
`test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/`

## What Passed

- Release visibility and checksum verification passed. See `20260709-final-release-v032-ba49af4-publish-flow-visibility.md`.
- Prior app was uninstalled with `uninstall.exe /S`, exit 0.
- Downloaded installer installed silently with `/S`, exit 0.
- Installed executable was present at `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`, version `0.3.2`.
- First-run onboarding appeared in the installed app, not a browser-only preview.
- Longmont, CO starter setup completed and added 9 starter sources.
- Daily Scan completed and saved 5 leads with 26 evidence items.
- The AI setup page installed/repaired the local runtime and reached `Local AI ready` with `phi4-mini:latest`.
- The one `ready_to_draft` lead opened in Workbench and generated a draft.
- The editor attestation checkbox enabled the approval buttons.
- `Approve for Static Publish` opened the warning override modal instead of staying disabled.
- `Publish anyway (logged)` saved the draft as `ready_to_publish` with `attested_by=Local Editor` and the tester override reason.

## Blocking Failure

Static compile/export/publish could not complete because the Publishing tab refused to compile while the public publication identity still used the starter title `My Local Publication`.

Publishing showed:

- `Publication name still uses starter text. Public publishing is paused until you choose one.`
- `Choose and save a real publication name before compiling or publishing.`
- `1 approved story ready for the public package.`

I attempted to use the app's own `Edit identity` flow to set the publication name to `Longmont Civic Desk`. That path repeatedly closed the WebView/CDP target and left the running app without the remote debugging endpoint. The partial save changed the tagline to `Local civic notes for Longmont, Colorado.`, but `community_profile.json` still had `"site_title": "My Local Publication"`. Because the title did not save, compile/export/publish remained blocked and no publish run was recorded.

Evidence:

- `publishing-tab-before-export.png`
- `publishing-initial.txt`
- `after-click-Compile.txt`
- `after-click-Export.txt`
- `after-click-Publish.txt`
- `publish-identity-edit-form.png`
- `publishing-after-second-relaunch.png`
- `identity-edit-cdp-loss-state.json`
- `community_profile-final.json`
- `sqlite-final-state.json`

## Product Quality Findings

1. Daily Scan mechanics worked, but output quality is still poor. Four of five leads were `needs_verification`; two were explicit source-quality/navigation-debris findings, and the one draftable lead was a vague category string: `Public Safety Sustainability Transportation Utilities`.
2. The generated draft was reader-facing but low quality. It reused navigation/category text from the source and triggered warnings for headline quality, verbatim overlap, and citation coverage.
3. The approval override behavior is fixed enough to proceed past the prior disabled-button blocker: attestation enabled approval and the logged override saved to the draft row.
4. The public identity save path is unstable/blocking in this clean profile. Two attempts caused WebView/CDP loss while the `civicnews` process remained/restarted without the debug port.

## Final State

SQLite final counts:

- `sources`: 9
- `daily_scan_runs`: 1
- `daily_scan_leads`: 5
- `leads`: 5
- `evidence_items`: 26
- `drafts`: 1
- `published_posts`: 0
- `publish_runs`: 0

The draft row ended as:

- `status`: `ready_to_publish`
- `attested_by`: `Local Editor`
- `guardrail_override_reason`: `Cleanroom tester override to verify static publish flow; warnings are recorded as product-quality issues in the tester report.`

## Reproduction Summary

1. Install `The.Civic.Desk_0.3.2_x64-setup.exe` from the v0.3.2 ba49af4 release.
2. Launch the installed app with a clean `CIVICNEWS_APP_DATA_DIR`.
3. Complete first-run onboarding for Longmont, CO.
4. Wait for starter source setup; confirm 9 sources.
5. Run Daily Scan; confirm 5 leads and 26 evidence items.
6. Install/repair the local AI runtime from AI Model setup until `Local AI ready`.
7. Draft the only `ready_to_draft` lead.
8. Check editor attestation and approve with warning override.
9. Open Publishing. The approved story is present, but compile is paused because the publication name is still starter text.
10. Click `Edit identity`, change the publication name, and save. During this run, the WebView/CDP target closed repeatedly and the title remained `My Local Publication`, so compile/export/publish could not finish.

## Verdict

FAIL for the full cleanroom publish-flow directive. The release is visible and installable, and the previous approval override gate appears fixed, but the static publish flow is still not complete because the app cannot reliably save the required publication title and therefore cannot compile/export/publish the approved package.
