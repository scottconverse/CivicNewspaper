# CivicNewspaper cleanroom report - beat memory rerun 77ece86

Status: FAIL

UTC run window: 2026-06-29T22:18Z to 2026-06-29T22:45Z

Machine identity: tester machine as `civic`

Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Coordination branch: `test-comms/cleanroom-coder-tester`

Directive: `test-comms/directives/20260629-beat-memory-rerun-77ece86.md`

Product commit under test: `77ece863db668df9889828587416696f3a39b6cc`

Evidence folder: `test-comms/reports/20260629-beat-memory-rerun-77ece86-evidence/`

## Installer

Preferred NSIS installer:

`test-comms/artifacts/20260629-beat-memory-rerun-77ece86/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`FBAA8AB176A0AB256A0D710B781472DEC15216F99250C30D787D99D430DC85F0`

Observed SHA256:

`FBAA8AB176A0AB256A0D710B781472DEC15216F99250C30D787D99D430DC85F0`

Install result:

- Method: NSIS
- Exit code: 0
- Installed executable: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

## AI Setup

The app reached the main Civic Desk interface after first-run setup.

Observed AI status in the app header:

- `Local AI ready`
- Selected model: `qwen2.5:7b`
- Hardware summary shown during setup: `Local Ram: 15 GB`
- The setup flow displayed an app message saying the setup screen did not receive input events, so the app continued with a starter Longmont profile.
- Manual tester installation of Ollama, models, prerequisites, or developer tools was not performed.

After draft generation failed, I checked the app-managed Ollama runtime without starting or installing anything manually:

- Process: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Endpoint `http://127.0.0.1:11434/api/tags`: reachable, HTTP 200
- Model visible from endpoint: `qwen2.5:7b`

Despite that, the app's draft workflow repeatedly reported that it could not reach the local AI model service.

## Source List

The app added 6 starter Longmont sources:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit
- Longmont Colorado subreddit

The source set included official municipal/public-record sources, an evergreen official agenda/council source, and public readable community/social sources.

## First Scan

The first recovered/source-intake scan completed and produced:

- Leads: 10
- Drafts: 0
- High priority: 0
- Sources: 6

Observed lead types included:

- `daily_scan`
- `Decision / Vote`
- `Public Meeting Scheduled`
- `New Primary Record`

The scan did not hide recurring/background material. The queue remained visible and draft buttons were available.

## Beat-Memory Rerun

A second overlapping `Scrape & Detect` run was executed.

Beat-memory behavior passed the visibility/context portion of the directive. The recurring source-evidence lead remained visible and included this editor-facing context:

`Beat memory: similar topic 'Review new source evidence from source #1' was first seen 2026-06-29T22:21:31.524842800+00:00, last seen 2026-06-29T22:21:31.524842800+00:00, and has appeared 1 previous time(s). Treat this as recurring/background unless the source shows a new vote, deadline, dollar amount, filing, outage, meeting item, or public impact. Suggested next step: Open the source, decide whether it contains a reportable civic action, then save or dismiss it from the queue. Compare against beat memory before drafting; write a story only if there is a new reportable fact.`

This satisfies the beat-memory expectations that recurring/background material remains visible, includes clear editor context, mentions a similar topic seen before, and warns that new reportable facts are required before drafting.

## Editor Workflow

I attempted to generate a draft from the recurring/background lead that displayed beat-memory context.

The draft dialog opened and showed:

- The selected recurring/background lead
- Beat-memory/editor context
- Article format options
- Linked sources
- `Generate Draft`
- `Cancel`

Clicking `Generate Draft` failed with:

`Draft generation failed: Couldn't reach the local AI model service (Ollama). Make sure it's installed and running, then try again.`

I retried once after verifying the app-managed Ollama endpoint was reachable and the model was listed. The retry failed with the same error.

Because no draft was created, I could not complete the required Hold / Resume Editing / Send Back for More Work checks in this run.

Observed held-draft controls:

- Hold: not reached
- Resume Editing: not reached
- Send Back for More Work: not reached

## Output Package

No ZIP export or here.now publish was performed.

Reason: the directive says not to approve fake or evergreen story copy as news, and draft generation failed before an editor-approved current story could be produced.

## Evidence

Key evidence files:

- `installer-hashes.json`
- `install-result.json`
- `clean-wipe-log.json`
- `launch-result.json`
- `setup-states.json`
- `05-daily-disabled-state.txt`
- `05-buttons.json`
- `first-scan-states.json`
- `second-scan-states.json`
- `beat-memory-matches.json`
- `editor-workflow-result.json`
- `ollama-endpoint-check.json`
- `editor-workflow-retry-result.json`
- `draft-retry-states.json`
- Screenshots `01-launch.png` through `29-after-resume-retry.png` where generated

## Conclusion

FAIL.

The installer, first-run setup, source intake, first scan, and beat-memory visibility/context checks worked. The run failed at draft generation: the app UI reported `Local AI ready` with `qwen2.5:7b`, and the app-managed Ollama endpoint was reachable with that model listed, but the draft workflow still failed twice with `Couldn't reach the local AI model service (Ollama)`.

This blocks the required editor workflow checks for Hold, Resume Editing, Send Back for More Work, and any ZIP/here.now output package.

## Reproduction Steps

1. Install preferred NSIS artifact `The Civic Desk_0.2.9_x64-setup.exe` from `test-comms/artifacts/20260629-beat-memory-rerun-77ece86/`.
2. Launch the app.
3. Complete app-guided Longmont setup using detected existing model `qwen2.5:7b`.
4. Observe header `Local AI ready`.
5. Let starter Longmont sources load.
6. Run or observe Daily Scan / Story Queue with 10 leads.
7. Run `Scrape & Detect` again.
8. Select the recurring/background lead with beat-memory context.
9. Click `Draft`.
10. Click `Generate Draft`.
11. Observe failure: `Draft generation failed: Couldn't reach the local AI model service (Ollama). Make sure it's installed and running, then try again.`
12. Verify `http://127.0.0.1:11434/api/tags` is reachable and lists `qwen2.5:7b`.
13. Retry `Generate Draft`.
14. Observe the same failure.
