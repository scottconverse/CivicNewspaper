# Tester report: full E2E rerun after runtime install thread fix

- Tester: Codex desktop cleanroom tester
- Report time: 2026-06-28T09:04:58Z
- Comms branch: `test-comms/cleanroom-coder-tester`
- Comms commit tested from: `f070f660e0d8db97938385d8a1da6a67530fc7cd`
- Directive: `test-comms/directives/20260628-rerun-full-e2e-after-runtime-install-thread-fix-26d461d.md`
- Product branch: `stable-readiness-local-gates`
- Product commit: `26d461dd3507aead46d7bfba3c5310e8d4a7c54d`
- Product subject: `Run local AI runtime installer on large-stack thread`
- Result: **FAIL / BLOCKED at draft generation persistence**

## Summary

The specific runtime-install thread fix passed. From a clean product state, the installed NSIS build launched, onboarding completed, `Install local AI runtime` no longer crashed, the app-local runtime downloaded/started, and the recommended `qwen2.5:7b` model downloaded successfully.

The full Longmont publication workflow could not complete. Source discovery/import, Daily Scan, and Scrape & Detect progressed, but the first attempted draft failed after local generation with:

```text
Draft generation failed: Something went wrong: invalid args `draft` for command `save_draft`: missing field `created_at`
```

Because no draft could be saved, I could not proceed to editor decisions, guardrail review, export ZIP, or here.now publishing.

## Artifact Verification

Preferred installer used:

```text
test-comms/artifacts/26d461d-runtime-install-thread-fix/The Civic Desk_0.2.8_x64-setup.exe
Expected SHA256: EFB2C97B8F5863C0FACFFCD1D94049A9BD59F3DC55BEE9966CBC1F21BA93066D
Observed SHA256: EFB2C97B8F5863C0FACFFCD1D94049A9BD59F3DC55BEE9966CBC1F21BA93066D
```

Fallback MSI hash was also checked:

```text
test-comms/artifacts/26d461d-runtime-install-thread-fix/The Civic Desk_0.2.8_x64_en-US.msi
Expected SHA256: A62208B3874E6425EC65B4F34F21C5911CCE6C387A3B4F95A9F87D68351CC8D3
Observed SHA256: A62208B3874E6425EC65B4F34F21C5911CCE6C387A3B4F95A9F87D68351CC8D3
```

## Environment

- OS: Windows 11 Home, version `10.0.26200`
- CPU: 13th Gen Intel Core i7-13620H, 10 cores / 16 logical processors
- RAM: about 16 GB physical; app detected 15 GB local RAM
- GPUs: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU
- Pre-run wipe: prior app install/profile and user Ollama stores removed; no `civicnews` or `ollama` process was running before the clean install.

## Steps and Observations

1. Installed the preferred NSIS artifact silently from the directive.
2. Launched installed app.
3. Confirmed first-run onboarding reached identity setup.
4. Entered Longmont identity values and completed onboarding.
5. AI service setup initially showed `Couldn't reach the AI service` and offered `Install local AI runtime`.
6. Clicked app-provided `Install local AI runtime`.
7. Runtime installer showed progress instead of crashing.
8. App-local runtime started from the app profile runtime directory and the app reported the local AI service ready.
9. Downloaded the recommended `qwen2.5:7b` model; it reached `Download complete! 100.0%` and `Model pulled successfully.`
10. Finished onboarding into the workspace; sidebar showed `LONGMONT · CO`, `Local AI ready`, and `qwen2.5:7b`.
11. Ran source discovery for `Longmont, CO`.
12. Imported 6 discovered sources:
    - Longmont official city website
    - Longmont agendas and minutes
    - Boulder County public notices
    - r/Longmont
    - Times-Call Longmont news
    - Longmont Library
13. Ran Daily Scan.
    - It completed with `Model: qwen2.5:7b. Evidence: 20. Saved leads: 17.`
    - Story Queue still showed `Leads 0` immediately afterward.
14. Ran Scrape & Detect from Story Queue.
    - It completed and populated `Leads 4`, `Drafts 0`.
15. Opened the first lead's Draft flow.
16. Clicked `Generate Draft`.
17. Generation switched to `Generating Draft...`, then failed with the `save_draft` `created_at` validation error above.

## Evidence

Screenshots are under:

```text
test-comms/artifacts/20260628-full-e2e-longmont-publication-26d461d/
```

Key screenshots:

- `01-first-run-identity.png` - clean first-run onboarding reached identity.
- `04-after-click-install-runtime-3s.png` - runtime download/install progress after clicking `Install local AI runtime`.
- `06-runtime-service-green.png` - local AI service ready after app-local runtime install.
- `09-model-download-progress-2min.png` - model download complete.
- `13-workspace-after-onboarding.png` - workspace after onboarding with local AI ready.
- `21-sources-after-import.png` - six sources imported.
- `26-daily-scan-check-3.png` - Daily Scan completed with 20 evidence items and 17 saved leads.
- `29-scrape-detect-check-1.png` - Story Queue populated with 4 leads.
- `33-generate-draft-result.png` - draft generation/persistence failure state.

I intentionally omitted the defaults screenshot that exposed a raw local profile path.

## Product Issues Found

### P1: Draft generation cannot save generated draft

After clicking `Generate Draft` on the first detected lead, the app generated long enough to enter the disabled `Generating Draft...` state, then surfaced:

```text
Draft generation failed: Something went wrong: invalid args `draft` for command `save_draft`: missing field `created_at`
```

Impact: this blocks all downstream publication workflow: drafts cannot be created, so editor decisions, export ZIP, and here.now publishing cannot be tested.

### P2: Daily Scan saved lead count does not populate Story Queue leads

Daily Scan completed with:

```text
Model: qwen2.5:7b. Evidence: 20. Saved leads: 17.
Complete
```

Immediately after opening Story Queue, the queue still showed `Leads 0`, `Drafts 0`, and the empty-state prompt to run `Scrape & Detect`. Running `Scrape & Detect` later populated 4 leads.

Impact: either Daily Scan's saved leads are not the same queue leads, or the queue is not reflecting them. This is confusing and reduced the available draftable leads below the directive's target.

### P3: Source discovery import UX only imported 6 of 11 candidates in this run

The discovery modal found 11 candidates, but the modal's visible scrolling and offscreen checkbox behavior made bulk selection fragile under desktop automation. I imported 6 trusted candidates and continued. This may be automation friction rather than a product defect, but it limited the subsequent lead pool.

## Pass/Fail Gate Status

- Launch reaches onboarding: **PASS**
- Identity setup succeeds: **PASS**
- `Install local AI runtime` avoids previous crash: **PASS**
- Runtime install progress appears: **PASS**
- App-local runtime starts and local AI becomes ready: **PASS**
- Recommended model download succeeds: **PASS**
- Longmont sources discovered/imported: **PARTIAL PASS** (6 sources imported)
- Daily Scan runs: **PASS with queue mismatch**
- Scrape & Detect runs: **PASS** (4 leads)
- Draft generation: **FAIL**
- Editor decisions: **NOT RUN, blocked by draft failure**
- Export ZIP: **NOT RUN, blocked by draft failure**
- here.now publish: **NOT RUN, blocked by draft failure**

## Tester Notes

I did not manually install Ollama, manually pull models, edit PATH, repair prerequisites, or create any article content outside the product. All runtime/model actions were initiated from the app UI.

No crash occurred in this run. The blocker is an in-app command validation/persistence failure during draft save.

Watcher remains armed for new directives.
