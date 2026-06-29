# Tester Report - Draft Action Above Fold e2ac517

Date: 2026-06-29T01:41Z-02:03Z
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: e2ac5176b99e8adac13b9655c10bfc0130c77838
Directive: test-comms/directives/20260628-rerun-draft-action-above-fold-e2ac517.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.4 GB on C:
- Node: not manually used for product runtime
- Rust: not manually used for product runtime
- npm: not manually used for product runtime
- Ollama installed/running: installed/started by app during setup; process observed running
- Models present: qwen2.5:7b shown by app as Local AI ready

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, `test-comms/ACTIVE_DIRECTIVE.md`, and `test-comms/directives/20260628-rerun-draft-action-above-fold-e2ac517.md`.
3. Confirmed product branch pointer:
   - `origin/stable-readiness-local-gates = e2ac5176b99e8adac13b9655c10bfc0130c77838`
4. Verified installer hashes:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `F1A958082A220BC3E25562CD03BABAF57274B4CE19D9434F5845055D128013A6`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `9FB4ADBFFE432B2616D55BE081E25FC8FA777A06CF88B220888C3AD134C49A44`
5. Stopped prior app/runtime processes and removed product/Ollama app-data/install state.
6. Installed only the directive NSIS installer with `/S`.
7. Launched `civicnews.exe` and set the app window to 1280x720.
8. Let first-run setup/model/source recovery proceed without manually installing dependencies or editing the DB.
9. Confirmed Story Queue state and Local AI ready in the UI.
10. Opened a visible lead through the app UI.
11. Confirmed the draft wizard now shows a fully visible `Generate Draft` button above `Article Format`.
12. Activated `Generate Draft` through normal keyboard focus and waited for local model generation to finish.
13. Confirmed one draft was created in the editor.
14. Returned to Story Queue and attempted to continue generating more drafts through visible UI. Stopped when follow-up UI activation/focus became unreliable and ordinary interactions no longer opened the next draft wizard.
15. Collected final read-only SQLite counts with bundled Python only; no database edits were made.

## Results

- Cleanroom wipe/reinstall: Pass.
- App-launched setup/model recovery: Pass.
- Source intake / Longmont queue recovery: Pass.
- Story Queue after Daily Scan: Pass.
- Draft wizard above-fold action placement: Pass for first opened lead.
- Generate at least five drafts through UI/local model: Fail/blocked after 1 draft.
- Approve/hold/send-back/cut workflows: Not reached.
- Export static publication package: Not reached.
- here.now anonymous publish: Not reached.
- 12-hour soak: Not started because the release workflow did not reach publication.

## Evidence

Artifacts are under:

`test-comms/artifacts/20260628-draft-action-above-fold-rerun-e2ac517/`

Key files:

- `01-launch-state.png`: clean launch/setup state.
- `monitor-log.json`: timed app/runtime monitor.
- `monitor-300.png`: app recovered to main UI with Story Queue selected and Local AI ready.
- `04-after-alt1-story-queue.png`: Story Queue with 18 leads, 0 drafts, latest scan.
- `05-lead-cards-visible.png`: visible lead cards and Draft buttons.
- `08-draft-activation-click-tab-enter.png`: draft wizard with fully visible `Generate Draft` button above Article Format.
- `10-after-keyboard-generate-20s.png`: button active as `Generating Draft...`.
- `generate-wait-180.png`: generated draft editor visible.
- `db-counts-after-first-draft.json`: read-only count after first draft.
- `db-readonly-final-counts.json`: final read-only DB counts and sanitized draft row.

Final read-only DB counts:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 18
- `daily_scan_leads`: 10
- `daily_scan_runs`: 1
- `drafts`: 1
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 3

Created draft:

- Draft id 1, lead id 16, format `watch`, status `draft_generated`
- Title begins: `Draft: Vision Zero Initiative Updates...`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Run still cannot reach five drafts/editor/publish

Observed: The `e2ac517` layout fix does move the primary `Generate Draft` action above the fold for the opened lead, and the first local-model draft generation completed successfully. However, after returning to Story Queue, follow-up ordinary UI interactions became unreliable. A visible second lead `Draft` button was shown, but subsequent click/keyboard activation did not open the next draft wizard; focus instead moved around the navigation grid. A bad click attempt briefly escaped to Chrome; that contaminated screenshot was deleted and is not committed.

Expected: After one successful draft, tester should be able to continue opening additional produced leads and generating at least five drafts using ordinary visible app UI controls.

Impact: The previous bottom-clipped Generate Draft blocker appears fixed for a single opened lead, but the full release path is still blocked before the required five drafts. Editor approval/hold/send-back/cut, publication export, here.now publish, and soak were not reached.

Repro:

1. Cleanroom install `e2ac517` from the directive installer.
2. Let app complete setup/model/source recovery.
3. Open Story Queue at 1280x720.
4. Open a visible lead and verify `Generate Draft` appears above `Article Format`.
5. Activate `Generate Draft`; wait for draft editor.
6. Return to Story Queue.
7. Try to open another visible lead and continue; interaction/focus does not reliably open the next draft wizard.

## Request For Coder

The above-fold primary action placement is materially improved and worked for one draft. Please clarify or fix the post-draft return-to-queue/open-next-lead flow so the tester can reliably generate five drafts through visible UI, then continue into editor controls and publishing.
