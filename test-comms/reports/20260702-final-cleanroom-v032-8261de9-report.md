# Tester Report - Final Cleanroom v0.3.2 8261de9

Date: 2026-07-03T01:53:00Z
Tester machine: MSI Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 8261de957b37beeda07944c8b12ab758494d1796
Directive: test-comms/directives/20260702-final-cleanroom-v032-8261de9.md

## Result

PASS WITH FINDINGS. The bdd0a40 Colorado-to-California jurisdiction drift did not reproduce. Improve for Publication preserved "Colorado General Assembly" in the rewritten headline/body. It did, however, introduce disabled/unlinked evidence citations and unsupported extra detail; the app blocked static approval until I manually cleaned the draft. After cleanup, compile, ZIP export, here.now publish, and public-output scans passed.

Public URL:

- https://pearly-finch-t8rj.here.now

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 341.6 GB
- Node/npm/Rust: not available in tester shell
- Ollama installed/running: app-guided setup completed; ollama process observed
- Model selected: phi4-mini:latest
- Installed app: C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
- Native window title observed: The Civic Desk

## Steps Run

1. Pulled test-comms/cleanroom-coder-tester and reread ACTIVE_DIRECTIVE.md, README.md, protocol.md, tester prompt, and the active 8261de9 directive.
2. Verified installer hash and size before install.
3. Stopped old civicnews/ollama processes, ran the prior uninstaller, removed app data, local install folders, and test-only .ollama state.
4. Installed only the 8261de9 NSIS artifact and launched the installed app.
5. Completed first-run Longmont identity setup.
6. Used app-guided AI setup until AI Status was Ready.
7. Waited for Daily Scan completion.
8. Generated two linked-source drafts from different leads and one no-source verification assignment.
9. Ran Improve for Publication on the Longmont Area Chamber draft whose evidence contained "Colorado General Assembly."
10. Verified unlinked/disabled citation approval blocking, manually cleaned the draft, approved only the clean linked-source draft, compiled the static site, published to here.now, and fetched public pages for inspection.

## Results

- Installer verification: PASS.
  - Path: test-comms/artifacts/20260702-final-cleanroom-v032-8261de9/The Civic Desk_0.3.2_x64-setup.exe
  - SHA256 observed: 7A08193C2BBA216C4E16291EB8EC45F89B6161B07BBF59D0A169D7DD590960D8
  - Size observed: 5225374 bytes
- Clean wipe/install: PASS.
- Native app identity: PASS, title was The Civic Desk.
- Longmont setup: PASS, publication Longmont Cleanroom 8261 Desk, editor Cleanroom Tester, city Longmont, state CO.
- AI setup: PASS, phi4-mini:latest selected and AI Ready.
- Daily Scan: PASS. Latest row completed with 9 sources, 20 daily_scan_leads, 23 leads, 26 evidence_items, 13 lead_evidence links, and 57 verification_tasks.
- Two linked drafts: PASS. Draft 1 was Longmont Area Chamber lead 22; draft 2 was St. Vrain Valley Schools lead 23.
- No-source verification assignment: PASS. Draft 3/lead 21 remained needs_verification with missing_evidence_notes and no attestation.
- Jurisdiction drift recheck: PASS for the named fix. Improve kept the Colorado jurisdiction and did not change it to California or another unsupported state.
- Citation shorthand normalization/blocking: PASS WITH FINDINGS. Improve used valid [Source](evidence:19) for the linked source, but also emitted [Evidence 21], [Evidence 22], [Evidence 23], [Source](unlinked-evidence-20), and [Source](unlinked-evidence-24). Static approval was blocked with "The draft has disabled or unlinked evidence citations. Link the correct source before approval."
- Static compile: PASS. Output path C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default, 18 files, 1 article, 0 skipped.
- here.now publish: PASS. Publish run provider here_now with URL https://pearly-finch-t8rj.here.now.
- Public output scan: PASS. Fetched index, briefs/1.html, and feed.xml; no banned markers, unlinked evidence markers, mojibake markers, or California jurisdiction drift appeared.

## Evidence

- Installer verification: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/installer-verify.txt
- Clean install/launch log: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/install-clean-launch.log
- AI/setup DB proof: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/db-after-scan-wait150.txt
- Linked leads: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/linked-leads-before-drafts.jsonl
- Drafts after linked generation: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/drafts-after-two-linked.jsonl
- No-source assignment: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/drafts-after-no-source.jsonl
- Improve output: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/workbench-after-improve-edit-values.txt
- Improve approval block UI: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/improved-unlinked-approval-block-uia.txt
- Approval DB proof: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/db-after-approval.jsonl
- Publish DB proof: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/db-after-here-now.jsonl
- Publish folder listing: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/publish-folder-listing.txt
- ZIP listing: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/zip-listing.txt
- Local output audit: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/local-public-output-audit.json
- here.now output audit: test-comms/evidence/20260702-final-cleanroom-v032-8261de9/here-now-public-audit.json
- Screenshots: screenshot-01-launch.png through screenshot-10-after-here-now.png

## Publish Details

- Approved draft ID: 1
- Published post path: briefs/1.html
- Publish run issue_id: issue-20260703-015128-513733200
- Files written: styles.css, print.css, briefs/1.html, index.html, about.html, ethics.html, how-we-report.html, corrections.html, feed.xml, newsletter.md, substack.md, share-package.md, facebook-post.txt, subreddit-post.md, nextdoor-post.txt, short-link-blurb.txt, publish-manifest.json, site-package.zip
- ZIP path: C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip
- here.now deployment: slug=pearly-finch-t8rj;version=01KWJTS7J74CRCNC0VDTCZFN7E;created_slug=pearly-finch-t8rj

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Major - Improve for Publication still introduces unsupported/unlinked source material, but approval blocking catches it

Observed: Improve for Publication preserved Colorado, but expanded the draft with unsupported extra details, a typo URL, bracketed evidence references not linked to the lead, and disabled unlinked-evidence citations.

Expected: Improve should either produce publishable linked-source copy or leave questionable claims out, especially when the source excerpt is narrow.

Impact: The approval guardrail prevented static publishing of the bad text, so public output stayed clean. Still, the editor would need to manually repair the improved draft before publication.

Repro: Open draft 1, run Improve for Publication, inspect workbench-after-improve-edit-values.txt, then attempt Approve for Static Publish. The UI shows the disabled/unlinked evidence blocker.

## Request For Coder

The targeted Colorado-to-California fix looks good. Please consider tightening Improve for Publication so it does not introduce disabled/unlinked evidence citations or unsupported extra source detail in the first place.
