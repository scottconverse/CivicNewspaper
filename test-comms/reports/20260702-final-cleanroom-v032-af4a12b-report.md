# Tester Report - Final Cleanroom v0.3.2 af4a12b

Date: 2026-07-03T02:45:00Z
Tester machine: MSI Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: af4a12b0689dd8de64ce6af707b0c305a9cdaba0
Directive: test-comms/directives/20260702-final-cleanroom-v032-af4a12b.md

## Verdict

PASS. The Improve for Publication rewrite artifact guard passed the named regression check: no `[Evidence 21]`, `[Evidence 22]`, `[Evidence 23]`, `unlinked-evidence-`, `https://www.longmondchamber.org`, or unsupported external URL appeared in the editor after Improve. The full cleanroom release path completed through install, setup, source discovery, drafting, no-source verification blocking, clean approval, compile, ZIP export, here.now publish, and public/ZIP/share artifact scan.

Public URL:

- https://olive-gorge-cgsr.here.now

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 334.6 GB
- Node/npm/Rust: not available in tester shell
- Ollama installed/running: app-guided setup completed; ollama process observed
- Model selected: phi4-mini:latest
- Installed app: C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
- Installed app identity: native window title was The Civic Desk

## Installer

- Path: test-comms/artifacts/20260702-final-cleanroom-v032-af4a12b/The Civic Desk_0.3.2_x64-setup.exe
- SHA256 observed: AB598EC26F658BB2B0735827F15DC787162D372A0C3FF0A3A18B6ADE48ABE241
- Size observed: 5229719 bytes

## Steps Run

1. Pulled test-comms/cleanroom-coder-tester and reread ACTIVE_DIRECTIVE.md, README.md, protocol.md, tester prompt, and the active af4a12b directive.
2. Wrote the required visibility report before install.
3. Stopped old civicnews/ollama processes, ran the prior uninstaller, removed app data, local install folders, and test-only .ollama state.
4. Installed only the af4a12b NSIS artifact and launched the installed app.
5. Completed first-run Longmont setup with publication Longmont Cleanroom AF4 Desk and editor Cleanroom Tester.
6. Used app-guided AI setup until AI Status was Ready.
7. Ran Daily Scan and waited for completion.
8. Generated two linked-source drafts from different leads and one no-source verification assignment.
9. Ran Improve for Publication on the Longmont Area Chamber draft with linked Colorado General Assembly evidence.
10. Checked improved editor text for the named bad artifacts and unsupported URL typo.
11. Approved only cleaned, source-grounded, linked-evidence copy.
12. Compiled the static site, verified ZIP output, published to here.now, and fetched public pages.
13. Scanned local output, extracted ZIP contents, RSS/share artifacts, and public here.now pages for forbidden markers.

## Results

- Clean wipe/install: PASS.
- Longmont setup: PASS.
- AI setup: PASS, phi4-mini:latest selected.
- Source discovery/Daily Scan: PASS. Latest scan completed with 9 sources, 22 daily_scan_leads, 25 leads, 26 evidence_items, 13 lead_evidence links, and 57 verification_tasks.
- Two linked drafts: PASS. Draft 1 used the Longmont Area Chamber lead; draft 2 used the St. Vrain Valley Schools lead.
- No-source path: PASS. Draft 3 remained needs_verification with missing_evidence_notes and no attestation.
- Improve for Publication artifact guard: PASS. Improved text used linked citation `[Source](evidence:19)` and did not contain the named bad artifacts.
- Clean approval: PASS. Only manually cleaned, source-grounded copy was approved.
- Static compile: PASS. Output path C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default, 18 files, 1 article, 0 skipped.
- ZIP export: PASS. ZIP path C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip.
- here.now publish: PASS. Publish run provider here_now with URL https://olive-gorge-cgsr.here.now.
- Public/ZIP/share scan: PASS. No forbidden markers, unlinked evidence citations, bracketed evidence labels, city hallucinations, duplicate-topic output, mojibake markers, or reporter-note scaffolding found.

## Evidence

- Installer verification: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/installer-verify.txt
- Clean install/launch log: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/install-clean-launch.log
- AI setup proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-ai-ready.txt
- Daily Scan proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-scan-wait150.txt
- Linked leads: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/linked-leads-before-drafts.jsonl
- Draft workflow: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/drafts-after-generation.jsonl
- Improve before/after: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/workbench-before-improve-edit-values.txt and workbench-after-improve-edit-values.txt
- Bad artifact scan: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/improve-bad-artifact-scan.json
- Approval proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-approval.jsonl
- Publish proof: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/db-after-here-now.jsonl
- Publish folder listing: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/publish-folder-listing.txt
- ZIP listing: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/zip-listing.txt
- Local/ZIP audit: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/local-public-output-audit.json
- here.now audit: test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/here-now-public-audit.json
- Screenshots: screenshot-01-launch.png through screenshot-08-after-here-now.png

## Publish Details

- Approved draft ID: 1
- Published post path: briefs/1.html
- Publish run issue_id: issue-20260703-024010-059256900
- Files written: styles.css, print.css, briefs/1.html, index.html, about.html, ethics.html, how-we-report.html, corrections.html, feed.xml, newsletter.md, substack.md, share-package.md, facebook-post.txt, subreddit-post.md, nextdoor-post.txt, short-link-blurb.txt, publish-manifest.json, site-package.zip
- here.now deployment: slug=olive-gorge-cgsr;version=01KWJXJCHHXDCAS806T9T8C80P;created_slug=olive-gorge-cgsr

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

No blocker, critical, major, minor, or nit findings were filed for this run.

## Request For Coder

No release-blocking request. af4a12b satisfies the rewrite artifact guard and full publication proof in this cleanroom run.
