# Tester Report - Draft Controls Rerun b1aebf4

Date: 2026-06-29T01:05Z-01:34Z
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: b1aebf4957b0f423da259d80d19145bd8bc3ea2a
Directive: test-comms/directives/20260628-rerun-draft-controls-b1aebf4.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.5 GB on C:
- Node: not manually used for product runtime
- Rust: not manually used for product runtime
- npm: not manually used for product runtime
- Ollama installed/running: installed/started by app during setup; model available by monitor checkpoint
- Models present: qwen2.5:7b observed by app/runtime monitor

## Steps Run

1. Pulled coordination branch:
   - `git pull --ff-only origin test-comms/cleanroom-coder-tester`
2. Reread:
   - `test-comms/README.md`
   - `test-comms/protocol.md`
   - `test-comms/prompts/tester-codex-desktop-prompt.md`
   - `test-comms/ACTIVE_DIRECTIVE.md`
   - `test-comms/directives/20260628-rerun-draft-controls-b1aebf4.md`
3. Confirmed product branch pointer:
   - `origin/stable-readiness-local-gates = b1aebf4957b0f423da259d80d19145bd8bc3ea2a`
4. Verified installer hashes:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `DCBE98E3056AC06D98F4A795F8D80C3362D9F26861201AABD7624911E7862A61`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `84C15F4C56EB24D203199ED497EF745323119ACFEBE277393757999867B8159E`
5. Performed cleanroom reset by stopping product/runtime processes and removing product/Ollama app-data and install directories.
6. Installed only the directive NSIS installer with `/S`.
7. Launched `civicnews.exe`, set the app window to 1280x720, and let setup/model/source-intake recovery complete without manually installing Ollama, model, Node, npm, Rust, or editing the database.
8. Monitored setup at timed checkpoints through 420 seconds.
9. Confirmed app landed on Story Queue and showed recovered Longmont queue state.
10. Opened a lead through the visible Draft button.
11. Exercised the draft wizard at 1280x720 by keyboard scroll, mouse wheel, and one direct click attempt on the visible top edge of the bottom action area.
12. Stopped at the exact blocker required by the directive: the draft-generation action area remains clipped/unreachable.
13. Collected read-only SQLite counts with bundled Python only; no database edits were made.

## Results

- Cleanroom wipe/reinstall: Pass.
- App-launched setup/model recovery: Pass.
- Source intake / Longmont queue recovery: Pass.
- Story Queue after Daily Scan: Pass.
- Open at least five leads: Blocked after first lead because draft generation cannot proceed.
- Generate at least five drafts through UI/local model: Fail, blocked by clipped/unreachable draft action bar.
- Approve/hold/send-back/cut workflows: Not reached.
- Export static publication package: Not reached.
- here.now anonymous publish: Not reached.
- 12-hour soak: Not started because the release workflow did not reach publication.

## Evidence

Artifacts are under:

`test-comms/artifacts/20260628-draft-controls-rerun-b1aebf4/`

Key files:

- `monitor-log.json`: setup/runtime monitor checkpoints.
- `02-app-after-overlay-close.png`: clean app state after setup, Story Queue visible.
- `03-story-queue-top-clean.png`: Story Queue top at 1280x720.
- `04-story-queue-scrolled-to-summary.png`: queue summary visible.
- `08-draft-form-action-area.png`: draft wizard opened.
- `13-after-second-draft-click.png`: draft wizard format/guideline area.
- `14-live-draft-form-state.png`: live wizard state at 1280x720.
- `16-after-wheel-down-draft-actionbar.png`: lower wizard content visible, action bar still clipped at bottom.
- `17-after-wheel-to-bottom-actionbar.png`: bottom reachable content; only the top of the action area is visible.
- `18-clean-blocker-actionbar-clipped.png`: clean blocker screenshot showing the action area still clipped at the bottom edge.
- `db-readonly-counts.json`: read-only DB counts and sample sanitized rows.

Read-only DB counts at blocker:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 18
- `daily_scan_leads`: 10
- `daily_scan_runs`: 1 completed
- `drafts`: 0
- `publish_runs`: 0
- `published_posts`: 0

Important UI observation:

After scrolling to the bottom of the draft wizard, only the top edge of the bottom action control area is visible inside the 1280x720 app window. A direct click on the visible edge did not activate draft generation and instead escaped the app window, opening Windows Search. That contaminated screenshot was deleted and is not committed; `18-clean-blocker-actionbar-clipped.png` is the clean post-overlay blocker capture.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Draft generation action bar remains clipped at 1280x720

Observed: At 1280x720, the draft wizard opens and can be scrolled down to linked sources, but the bottom draft-generation action area remains mostly below the app viewport. Only a thin top slice of the rounded button/control area is visible. Keyboard PageDown did not reveal it. Mouse wheel reached the bottom content, but the action bar was still clipped. A direct click on the visible top edge escaped the app and opened Windows Search instead of starting draft generation.

Expected: The sticky draft action bar should be fully visible and clickable at 1280x720 so tester can generate drafts through the normal UI.

Impact: The product still cannot complete the required release path. No drafts were generated, editor workflows could not be exercised, publication/export could not be reached, and the 12-hour soak could not begin.

Repro:

1. Cleanroom install `b1aebf4` with the directive NSIS installer.
2. Launch at 1280x720 and allow setup/source recovery to complete.
3. Open Story Queue.
4. Scroll to a lead and click `Draft`.
5. Scroll the draft wizard to the bottom.
6. Observe only the top edge of the bottom action area is visible; draft generation cannot be activated.

## Request For Coder

The `b1aebf4` reachability fix is not sufficient on this cleanroom Windows 11 machine at 1280x720. Please make the draft-generation primary action fully visible and clickable at 1280x720 without relying on off-window bottom-edge clicks. A shorter wizard layout, smaller sticky footer, or moving the action above the fold would unblock the next rerun.
