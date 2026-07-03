# Tester Report - Final Cleanroom v0.3.2 bdd0a40

Date: 2026-07-03T01:22:00Z
Tester machine: MSI Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: bdd0a40e0af46701c8a8eb1b815178bf830caae9
Directive: test-comms/directives/20260702-final-cleanroom-v032-bdd0a40.md

## Result

PASS WITH FINDINGS. The installed app completed cleanroom setup, generated source-linked drafts, blocked no-source/unlinked-evidence approval, compiled the static site, wrote site-package.zip, recorded a publish run, published to here.now, and the public pages passed the banned-marker/mojibake scan.

Public URL:

- https://hearty-clover-fzqa.here.now

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 347.2 GB
- Ollama installed/running: app-guided install completed; ollama process observed after setup
- Models present/selected: phi4-mini:latest
- Installed app: C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
- Native window title observed: The Civic Desk

## Steps Run

1. Pulled test-comms/cleanroom-coder-tester and reread ACTIVE_DIRECTIVE.md, README.md, protocol.md, tester prompt, and the active bdd0a40 directive.
2. Verified the NSIS installer hash and byte size.
3. Stopped existing civicnews and ollama processes, ran the prior uninstaller when present, removed app data and local install folders, removed test-only .ollama state, installed only the bdd0a40 NSIS artifact, and launched the installed app.
4. Completed first-run identity setup for Longmont, Colorado with publication name Longmont Cleanroom Beta Desk and editor Cleanroom Tester.
5. Used the app-guided local AI setup until the database showed model.selected phi4-mini:latest and onboarding_complete 1.
6. Waited for Daily Scan completion and checked the database.
7. Generated two linked-source drafts from different leads.
8. Created one no-source verification assignment and confirmed it stayed needs_verification without approval.
9. Exercised Workbench draft picker, top action strip, Improve for Publication, manual editor cleanup, approval attestation, unlinked-evidence blocking, Publishing output folder open, compile checklist, Compile site, ZIP/package outputs, here.now publish, and public page inspection.

## Results

- Installer verification: PASS.
- Clean wipe/install: PASS.
- Native app identity: PASS, title was The Civic Desk.
- Longmont first-run setup: PASS.
- App-guided AI setup: PASS.
- Source discovery/Daily Scan: PASS. Latest scan completed with 9 sources, 1 scan run, 24 scan leads, 27 total leads, 26 evidence items, and 11 lead-evidence links.
- Two linked drafts: PASS. Drafts created for St. Vrain Valley Schools and Longmont Area Chamber leads.
- No-source lead handling: PASS. Lead 24 became draft 3, status needs_verification, with missing_evidence_notes and no attestation.
- Improve for Publication: MIXED. The editor normalized citation syntax to linked [Source](evidence:19), but it also changed "Colorado General Assembly" to "California Legislature" in the editable text. I corrected the draft manually before approval.
- Unlinked evidence approval blocking: PASS. A probe draft citing [Source](evidence:999) showed "Fix before static publish approval" and specifically named evidence ID 999 as not linked to the lead; it was not approved.
- Static compile: PASS. Output path C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default, 18 files, 1 article, 0 skipped.
- here.now publish: PASS. Publish run provider changed to here_now with deployment slug hearty-clover-fzqa.
- Public output scan: PASS. Fetched index, briefs/2.html, and feed.xml from here.now; banned markers, unlinked evidence markers, and mojibake markers were not found.

## Evidence

- Installer verification: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/installer-verify.txt
- Clean install/launch log: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/install-clean-launch.log
- AI setup DB proof: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/db-after-ai-ready.txt
- Daily Scan DB proof: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/db-after-scan-wait120.txt
- Draft/approval DB proof: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/db-after-approval.jsonl
- Compile DB proof: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/db-after-compile.jsonl
- here.now DB proof: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/db-after-here-now.jsonl
- Publish folder listing: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/publish-folder-listing.txt
- ZIP listing: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/zip-listing.txt
- Local public audit: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/local-public-output-audit.json
- here.now public audit: test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/here-now-public-audit.json
- Screenshots: screenshot-01-launch.png through screenshot-13-unlinked-evidence-blocked.png in the evidence folder

## Publish Details

- Approved draft ID: 2
- Published post path: briefs/2.html
- Publish run issue_id: issue-20260703-011137-759360100
- Files written: styles.css, print.css, briefs/2.html, index.html, about.html, ethics.html, how-we-report.html, corrections.html, feed.xml, newsletter.md, substack.md, share-package.md, facebook-post.txt, subreddit-post.md, nextdoor-post.txt, short-link-blurb.txt, publish-manifest.json, site-package.zip
- ZIP path: C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip
- here.now deployment: slug=hearty-clover-fzqa;version=01KWJRJ2V5MBBEKXAGBQND9VHN;created_slug=hearty-clover-fzqa

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Major - Improve for Publication introduced an inaccurate state legislature reference

Observed: On the Longmont Area Chamber linked-source draft, Improve for Publication rewrote the source sentence as "California Legislature" even though the linked source says "Colorado General Assembly." The citation remained normalized as [Source](evidence:19).

Expected: Improve for Publication should preserve the jurisdiction/source meaning while normalizing linked citations.

Impact: An editor who trusts the rewrite could publish a materially wrong state reference.

Repro: Open draft 2, click Improve for Publication, inspect test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/workbench-linked-after-improve-edit-values.txt.

## Request For Coder

Please investigate the Improve for Publication rewrite drift that changed Colorado to California. The release mechanics and public-output gates passed after manual editor cleanup.
