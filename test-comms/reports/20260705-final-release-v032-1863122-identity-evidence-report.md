# Tester Report - Civic Desk v0.3.2 1863122 Identity And Evidence

Date: 2026-07-05T04:26:53Z
Tester machine: `MSI\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product/release: GitHub release `v0.3.2`
Product build commit under test: `186312209b743824ae33bd48777c90b0e6a545ec`
Directive: `test-comms/ACTIVE_DIRECTIVE.md` / `20260705-final-release-v032-1863122-identity-evidence.md`

## Result

FAIL.

The rerun confirms several targeted fixes worked: the release visibility checks passed, the isolated first-run profile opened onboarding instead of inherited Longmont state, onboarding persisted `Longmont` / `CO` into `community_profile.json`, Daily Scan ran without manually repairing Settings, and source discovery returned results for both `Longmont` / `Colorado` and `Longmont` / `CO`.

The release still does not pass because Daily Scan/Story Queue continues to expose draft paths for weak or unsupported items, including high-priority/no-linked-evidence items, and the generated draft from a ready linked-evidence lead was not publication-quality.

## Environment

- Windows: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 15.71 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on C: 341 GB
- Ollama: running from `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- App launch path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Clean profile override: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-1863122-identity-evidence`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read the active directive plus protocol files.
2. Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release.
3. Verified installer size `5250373` and SHA256 `6CD5B8C6D3565AFAE8A39357DEAEC1CE53ADEDADB8316BEB6C44DCB86C87EE74`.
4. Verified release/docs visibility requirements; see the separate visibility report.
5. Recorded the directive-listed state paths before install.
6. Uninstalled the prior app instance, installed the downloaded GitHub release installer, and launched only the installed EXE with `CIVICNEWS_APP_DATA_DIR`.
7. Completed first-run setup for Longmont, Colorado using app controls and the detected local model `phi4-mini:latest`.
8. Went directly to Daily Scan after onboarding, without repairing Settings.
9. Captured `community_profile.json` after onboarding.
10. Ran source discovery with `Longmont` / `Colorado` and `Longmont` / `CO`.
11. Ran Daily Scan, inspected Story Queue, summarized SQLite lead/evidence linkage, and generated one draft from a ready linked-evidence lead.
12. Stopped before export/publish because major release-quality findings remained.

## Results

- Visibility: PASS.
- Installer download/hash/size: PASS.
- Installed-app launch from GitHub release installer: PASS.
- `CIVICNEWS_APP_DATA_DIR` isolated profile: PASS.
- First-run onboarding instead of inherited state: PASS.
- Onboarding city/state persistence: PASS. `community_profile.json` contained `"city": "Longmont"` and `"state": "CO"` before any Settings repair.
- Daily Scan immediately after onboarding: PASS mechanically. The prior `Choose your publication city and state in Settings before running Daily Scan.` blocker did not recur.
- Source discovery with full state name `Colorado`: PASS. It returned Longmont discovery candidates.
- Source discovery with state abbreviation `CO`: PASS. It returned Longmont discovery candidates.
- Daily Scan lead quality: FAIL.
- Workbench/draft quality: FAIL.
- Export/publish: NOT RUN because the generated draft and lead queue did not meet release quality.

## Evidence

- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/visibility-receipt.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cleanprofile-launch.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/community_profile-after-onboarding-before-settings.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-run-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/sqlite-lead-evidence-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/sqlite-drafts-publish-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-11-story-queue-after-scan.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-12-ready-lead-draft-start.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-13-generated-draft.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/environment-final.json`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 2
- Minor: 2
- Nit: 0

### Major 1 - Story Queue still exposes draft paths for weak/no-linked-evidence leads

Observed: SQLite summary showed 20 Daily Scan leads, with 15 leads having zero linked evidence. Story Queue showed `Draft anyway` on several weak/background/watch items. It also showed a high-risk/high-priority `City Clerk Office Enhances Accessibility and Community Services` item with no linked evidence and a `Draft anyway` path. Another high-priority `Review community signal from Longmont city events` lead had no linked evidence.

Expected: Leads with no linked `lead_evidence`, broad/navigation/index content, or weak community signals should be clearly verification-only, suppressed, or downgraded without a normal-looking draft path.

Impact: The release still invites editors toward reader-facing drafts from weak or unsupported material, which violates the directive's evidence-first gate.

Repro: Complete onboarding, run Daily Scan, open Story Queue, and inspect `sqlite-lead-evidence-summary.json`.

### Major 2 - Generated draft from a ready linked-evidence lead is not publication-quality

Observed: A normal `Draft` path on `'Academic Excellence By Design' Initiative at St. Vrain Valley Schools` opened a ready-to-draft lead with three linked sources. The generated draft had malformed title text: `Academic Excellence By Design' Initiative at St. Vrain Valley Schools: The district's new`. The body was a thin fragment: `According to the linked source, August 1, 2026 - - Aug 12 Board of Education Regular Meeting...`, followed by a generic watch-brief paragraph. Guardrails warned that the public title read like a summary, wording was too close to the source, and factual claims needed source links.

Expected: A ready linked-evidence lead should generate clean reader-facing copy with a real headline, coherent body, and no source-fragment lead sentence.

Impact: The test could not proceed to approval/export/publish because the draft did not meet public copy quality.

Repro: In Story Queue, click the first `Draft` button for the ready St. Vrain lead, then click `Generate Draft`.

### Minor 1 - Source discovery/import duplicates candidates across repeated discovery attempts

Observed: The existing starter/imported sources remained while both `Colorado` and `CO` discovery modals showed selected candidate counts. Repeated discovery produced overlapping Longmont candidates and imported state was not very clear.

Expected: Repeated discovery should make duplicate/import status obvious and avoid confusing selected counts.

Impact: Tester/editor friction, not a release blocker by itself.

### Minor 2 - Format selector defaulted to `watch` for a lead labeled Brief

Observed: The ready-to-draft St. Vrain lead displayed `Brief` in Workbench, but the `Article Format` select value was `watch`. The generated content was also a watch brief.

Expected: The format selector should match the displayed suggested treatment unless the editor changes it.

Impact: Contributed to weak generated copy and confusing editor workflow.

## Request For Coder

1. Remove or strongly quarantine draft paths for no-linked-evidence and weak/background/watch leads.
2. Fix draft generation so ready linked-evidence leads produce a real headline and coherent source-backed body.
3. Align Workbench article-format defaults with the lead's displayed suggested treatment.
