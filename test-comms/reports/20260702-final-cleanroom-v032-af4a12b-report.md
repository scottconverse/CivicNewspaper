# Tester Report - Final Cleanroom v0.3.2 af4a12b

Date: 2026-07-03T02:45:00Z
Tester machine: MSI Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: af4a12b0689dd8de64ce6af707b0c305a9cdaba0
Directive: test-comms/directives/20260702-final-cleanroom-v032-af4a12b.md

## Result

PASS. The af4a12b build completed the cleanroom path and passed the specific rewrite-artifact guard recheck. Improve for Publication rewrote the linked Chamber draft without introducing the prior bad artifacts: no `[Evidence 21]`, `[Evidence 22]`, `[Evidence 23]`, `unlinked-evidence-`, `https://www.longmondchamber.org`, unsupported external URL, or California jurisdiction drift was found after Improve. The cleaned linked-source draft was approved, compiled, exported to ZIP, published to here.now, and public-output scans passed.

Public URL:

- https://olive-gorge-cgsr.here.now

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU
- Installed app: C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
- Native app identity observed: The Civic Desk
- Model selected: phi4-mini:latest
- Publication identity: Longmont Cleanroom AF4 Desk, Longmont, CO

## Installer

- Installer path: test-comms/artifacts/20260702-final-cleanroom-v032-af4a12b/The Civic Desk_0.3.2_x64-setup.exe
- Observed SHA256: AB598EC26F658BB2B0735827F15DC787162D372A0C3FF0A3A18B6ADE48ABE241
- Observed size: 5229719 bytes
- Evidence: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/installer-verify.txt

## Steps Run

1. Pulled test-comms/cleanroom-coder-tester and reread ACTIVE_DIRECTIVE.md plus the active af4a12b directive.
2. Wrote the required visibility report before install/run continuation.
3. Verified installer hash and size.
4. Clean-wiped prior The Civic Desk/CivicNewspaper app state and installed only the af4a12b NSIS artifact.
5. Launched the installed Windows app, completed Longmont setup, and used app-guided local AI setup until AI was ready.
6. Ran source discovery/Daily Scan and waited for completion.
7. Generated two linked-source drafts from real Longmont leads.
8. Generated one no-source verification assignment and confirmed it remained `needs_verification`.
9. Opened the linked Longmont Area Chamber draft in Workbench and exercised Improve for Publication.
10. Approved only the clean source-linked Chamber draft.
11. Opened Publishing, compiled the site, exported ZIP, published to here.now, fetched public pages, and scanned local/ZIP/public outputs.

## Results

- Installer verification: PASS.
- Clean wipe/install: PASS.
- Native app identity: PASS, title was The Civic Desk.
- AI setup: PASS, `phi4-mini:latest` selected and app-guided setup completed.
- Source discovery/Daily Scan: PASS. Latest run completed with 9 sources, 25 leads, 26 evidence items, 13 lead-evidence links, and 1 completed daily scan run.
- Linked drafts: PASS. Drafts were generated for Longmont Area Chamber of Commerce and St. Vrain Valley Schools leads.
- No-source assignment: PASS. Draft 3 / lead 21 stayed `needs_verification`, with missing evidence notes and no attestation.
- Improve for Publication rewrite-artifact guard: PASS. The improved Chamber draft used only linked `[Source](evidence:19)` and did not introduce the prior bracketed evidence labels, disabled/unlinked citations, unsupported longmondchamber typo URL, unsupported external URL, or California jurisdiction drift.
- Approval: PASS. Draft 1 was approved as `ready_to_publish`, attested by Cleanroom Tester, with one logged guardrail warning.
- Static compile: PASS. Output path `C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default`; 18 generated files, 1 article, 0 skipped.
- ZIP export: PASS. `site-package.zip` was written in the output folder.
- here.now publish: PASS. Publish run provider `here_now`, URL `https://olive-gorge-cgsr.here.now`, deployment `slug=olive-gorge-cgsr;version=01KWJXJCHHXDCAS806T9T8C80P;created_slug=olive-gorge-cgsr`.
- Public output scan: PASS. Local output, ZIP contents, and here.now pages had zero hits for unsupported source material, unlinked evidence citations, bracketed evidence labels, city-specific hallucination markers, duplicate-topic output markers, mojibake marker code points, or reporter-note scaffolding.

## Evidence

- Visibility report: test-comms/reports/20260702-final-cleanroom-v032-af4a12b-visibility.md
- Installer verification: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/installer-verify.txt
- Clean install/launch log: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/install-clean-launch.log
- AI setup DB proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-ai-ready.txt
- Daily Scan DB proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-scan-wait150.txt
- Linked leads: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/linked-leads-before-drafts.jsonl
- Draft generation: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/drafts-after-generation.jsonl
- Drafts before Improve: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/drafts-before-improve.jsonl
- Drafts after Improve: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/drafts-after-improve.jsonl
- Improve artifact scan: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/improve-bad-artifact-scan.json
- Approval DB proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-approval.jsonl
- Publish DB proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-here-now.jsonl
- Final DB summary: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/final-db-summary.json
- Publish folder listing: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/publish-folder-listing.txt
- ZIP listing: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/zip-listing.txt
- here.now fetch log: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/here-now-fetch-log.json
- Local output audit: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/local-public-output-audit.json
- ZIP output audit: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/zip-public-output-audit.json
- here.now output audit: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/here-now-public-audit.json
- Public page captures: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/here-now-index.html, here-now-briefs-1.html, here-now-feed.xml
- Screenshots: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/screenshot-01-launch.png through screenshot-08-after-here-now.png

## Published Output

- Approved draft ID: 1
- Published article path: briefs/1.html
- Publish run issue_id: issue-20260703-024010-059256900
- ZIP path: C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip
- here.now URL: https://olive-gorge-cgsr.here.now

## Verdict

af4a12b passes the directive. The prior major rewrite-artifact issue was not reproduced, the final output had no prohibited public markers, and there are no blockers, critical findings, or major findings in this run.
