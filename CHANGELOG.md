# Changelog

All notable changes to CivicNewspaper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-06-29

### Added
- Added story-quality metadata across daily scan leads and the story queue, including story type, editorial disposition, novelty score, novelty reason, publishability notes, and recurrence memory.
- Added seeded story templates for briefs, explainers, accountability stories, background items, watch items, and verification-only leads.
- Added recurring-topic/beat-memory warnings so repeated background items are visible before editors treat them as fresh news.
- Added guardrail warnings for low-novelty, background, watch, needs-verification, and recurring linked leads while preserving the rule that software never vetoes the editor.

### Changed
- Improved local-AI draft prompts so recurring or low-novelty leads produce editor memos/watch items unless the evidence shows a current verified development.
- Improved Workbench status controls for hold, send back, ready for review, approve, unapprove, cut, and restore flows.
- Improved Story Queue and Workbench displays so editors can see why a lead is timely, what changed, and whether the same topic appeared in prior scans.

### Fixed
- Reduced the chance that evergreen civic pages or source-intake notes publish as ordinary news by surfacing novelty and recurrence warnings in review.
- Kept backend `killed` status compatibility while presenting user-facing copy as `Cut`.

## [0.2.8] - 2026-06-26

### Added
- Added staged Daily Scan progress reporting and repaired scan fallback output so local-model runs surface usable lead context instead of opaque completion states.
- Added browser-extension pairing support, including the local pairing server flow, extension folder opener, paired-client listing, and assistant bridge surfaces.
- Added native bulk-source intake for DOCX, XLSX, and text-based PDF files, with parser recovery for flattened multi-URL document extraction.
- Added local model bakeoff tooling and AI setup improvements so installed models can be detected and selected without duplicate options.

### Changed
- Improved source discovery so search-engine verification failures no longer surface as raw branded errors; fallback candidates now return as reviewable source suggestions.
- Reworked Daily Scan, Workbench, Sources, AI Model, Browser Pairing, and Publishing flows based on gauntlet walkthrough findings.
- Updated application metadata to 0.2.8 across package, Rust, and Tauri release files.

### Fixed
- Fixed misleading local-AI offline handling so an unreachable Ollama service is not reported as a missing model.
- Fixed AI Model status where an installed model could still appear as "Ready to download."
- Fixed duplicate React key warnings for installed/default model options.
- Fixed browser-extension popup state after successful pairing so paired and unpaired panels are not both shown.
- Fixed Browser Pairing page refresh behavior so paired clients update while the page is open.
- Fixed source import extraction for realistic spreadsheets, Word documents, and text-based PDFs that previously collapsed to one importable URL.

## [0.2.7] - 2026-06-25

### Added
- Implemented the Civic Desk redesign handoff across the desktop app, including the compact newsroom rail, Story Queue, Daily Scan, Workbench, Sources, AI Model, Publishing, Browser Pairing, and Ethics & Backups screens.
- Added dedicated Daily Scan and AI Model panels so the primary setup and newsroom flows match the product design instead of the old onboarding/dashboard layout.
- Added browser-preview-safe workflows for source bulk import, source discovery, pairing-code generation, profile saves, and local model-download preview so Playwright can verify the UI without Tauri bridge crashes.

### Changed
- Renamed the installed product surface to **The Civic Desk** for the window title, installer, beta warning, shortcut, and user-facing copy.
- Bumped the application version to 0.2.7 across the frontend package, Tauri config, and Rust package metadata.
- Moved app data to the new `com.scottconverse.civicdesk` namespace and added first-launch migration from the legacy `org.civicnews.app/civicnews.db` database into `civicdesk.db`.
- Updated default publication naming and static-site compiler defaults to The Civic Desk.

### Fixed
- Restored full install/uninstall coverage for the NSIS installer and ensured silent install mode does not hang on the unsigned-beta warning dialog.
- Removed stale visible `CivicNews`/`CivicNewspaper` labels from the main product UI where they made the new install look like the old app.
- Preserved the release-integrity CI dependency repairs carried from the release-integrity branch, including executable Linux/macOS sidecar fixtures.

## [0.2.6] - 2026-05-28

Remediation of the v0.2.5 audit (2 Blocker / 4 Critical / 5 Major / 3 Minor / 2 Nit). See the v0.2.5 postmortem below for what was rejected and why.

### Fixed
- **B-1 / M-1**: Extracted the port-collision check into `OllamaSidecar::port_in_use(addr)` (with `ollama_port_in_use()` pinning the production port 11434) and added `test_port_in_use_detects_listener_cross_platform`, which binds an OS-assigned ephemeral port and asserts the check flips true then false as a listener is bound and dropped. It needs no `tauri::test::mock_app()` and is isolated from whatever occupies 11434, so the coexistence guarantee now genuinely runs on Windows and unix. The full `start()`-skip path is now exercised cross-platform by `test_sidecar_skips_spawn_when_port_occupied`, which drives the AppHandle-free `start_for_test(addr)` against an injected occupied port and asserts the call returns `Ok` while spawning no child — so the skip path is no longer `mock_app()`-bound or `#[cfg(unix)]`-gated (P5-004).
- **C-1 / M-3**: Removed the tautological `expect(expectedModel).toBe("phi3:mini")` assertion (a literal compared to itself) from the Daily Scan vitest case and added a negative-path test (`test_useapp_daily_scan_blocks_when_selected_model_unavailable`) proving the selected model genuinely gates the scan — when the chosen model is absent from Ollama, `run_daily_scan` is never invoked and the error names the missing model. The setting-to-LLM trace itself is covered by the Rust test `test_daily_scan_uses_settings_model_not_hardcoded`, where a `FakeLlmClient` asserts the user-selected model reaches the LLM call.
- **C-2 / M-6 / Mn-3**: The v0.2.4 walkthrough commit count is now pinned to an explicit commit range (`942a940..91824ac`, verified to be 39) instead of an ambiguous `..HEAD` that drifts as new commits land. The reproduction test (`reproduce_m6_walkthrough_commit_count_pinned_and_correct`) now asserts the doc pins both SHA endpoints and reports the verified count, replacing the prior check that matched a literal (`**38**`) which was never in the file and so always passed.
- **Mn-1**: The first-run System Status panel showed a hardcoded `0.1.1`; it now reflects the real application version.
- **Mn-2**: Added a minimum-RAM floor and a "may run slowly" warning to model selection so machines below 8 GB are warned rather than silently handed a model they may be unable to run.
- **N-1**: Removed the `path_exists` silent-pass guard from the M-6 reproduction test (it now reads the artifact unconditionally and fails if it is missing), and strengthened the M-3 reproduction so it requires the genuine model-gating assertions to be present — not merely that the old grep-bait comments are absent.
- **N-2**: The wizard model-pull error now surfaces the underlying reason and a next step instead of a bare "Error".

### Note
- The audit's gate-integrity findings (B-2, C-3, C-4, Mj-1, Mj-2, Mj-3) concerned the executor's self-grading pipeline (`scripts/policy/run_all.py`, `auto_promote.py`, manifest thresholds). That pipeline has been retired and promotion is no longer self-graded, so those findings are obsolete rather than remediated in code.
- The review-apparatus findings (Mj-4, Mj-5) likewise concerned the retired self-review pipeline: Mj-4 was a documented falsehood in `verifier-report.md` (it claimed `SECURITY.md` was updated when it was not), and Mj-5 was that the `critic-report.md` was non-adversarial. Those reports are no longer produced. The substance Mj-4's false claim referenced — documentation of the local Ollama sidecar attack surface — already exists in `SECURITY.md` ("Local Sidecar Attack Surface"), so no doc change was required.

### Full-repo audit remediation (2026-05-28)

A subsequent balanced five-role audit of the whole repository (0 Blocker / 3 Critical / 17 Major / 23 Minor / 14 Nit) was remediated in full. Notable changes:

#### Removed
- **ENG-001 (Critical)**: Removed the Tauri updater entirely so the code matches the FAQ's "dormant, no automatic checks" promise. Deleted the on-launch `check()` call and `updateAvailable` UI banner, dropped the `tauri-plugin-updater`/`tauri-plugin-process` crates and their npm counterparts, removed the `plugins.updater` config (an unsigned `latest.json` endpoint with no `pubkey`), and dropped the now-unneeded `https://github.com` CSP `connect-src` entry. Updating is manual download from GitHub Releases, consistent with the trust-without-signing model.

#### Fixed
- **QA-001 / QA-002 (Critical / Major) — release integrity**: The release workflow now publishes a combined `SHA256SUMS` manifest as a release asset (a new post-build `checksums` job aggregating every platform's artifacts) and a release-integrity gate (`scripts/verify-release-assets.sh`) fails the release if the manifest is missing or lists an artifact that is not actually published. Aligned `install.md`/`README.md`/`user_manual.md` to the published filename (`SHA256SUMS`, no extension) and to deb-only Linux (removed `.AppImage` instructions that referenced an artifact the build never produces); aligned `scripts/verify-release.sh` to deb-only.
- **UX-1 / UX-4 / UX-13 (Critical / Major) — design tokens**: Fixed every `var(--token)` reference that pointed at a custom property App.css never declares (these failed silently, rendering no color): `--color-danger`→`--color-error`, `--color-primary`→`--accent-primary`, `--bg-primary`→`--bg-card`/`--bg-app`, `--bg-secondary`→`--bg-app` across OnboardingWizard, App, SystemStatus, and SourcesPanel; repointed the undefined `badge-secondary` class to the existing `badge-neutral`. Added a CI gate (`scripts/check-design-tokens.mjs`, run by `.github/workflows/check-design-tokens.yml` and `npm run lint:tokens`) that fails the build if any frontend `var(--token)` references a property not declared in App.css, so this class of silent failure cannot recur.
- **QA-003 / UX-2 / QA-005 (Major) — IPC guarding & error copy**: Routed every frontend `invoke()` through `src/ipc.ts`. Added a single `isTauri()` guard (`invokeGuarded`) consulted once per call; outside the Tauri runtime (browser preview, test harness) it short-circuits with one consolidated `console.warn` and a clear thrown error instead of an opaque "Cannot read properties of undefined (reading 'invoke')" per call. Added a `toUserMessage(e)` helper that maps known failure shapes (desktop bridge unavailable, Ollama unreachable, permission denied, not found, empty) to plain-language copy and routed all 24 `e.toString()` sites in `useApp.ts` plus the `OnboardingWizard`/`SystemStatus` export-diagnostics handlers through it. Moved the previously raw component-level `invoke` calls (`get_setting`/`set_setting`/`is_onboarding_complete`/`set_onboarding_complete`/`export_diagnostics`/`cancel_ollama_pull`) into guarded `ipc.ts` wrappers, and wrapped the `OnboardingWizard` `handleNext` step persistence in try/catch so a failed settings write surfaces through the existing initialization-error banner instead of silently stalling the wizard.
- **ENG-002 (Major) — stored XSS in published site via profile fields**: The static-site compiler interpolated `CommunityProfile` fields (`site_title`, `site_subtitle`, `about_text`) into post pages, the index, info pages, the corrections ledger, and `feed.xml` **without escaping**, while draft-derived fields were already escaped. A profile field containing pasted boilerplate with `<script>`/`<img onerror=...>` would run in every reader's browser. `compiler.rs` now escapes those fields once (`html_escape::encode_safe`) and routes all HTML and RSS sinks through the escaped values; `about_text` in the sidebar is escaped (the About/Ethics/How-We-Report info pages already pass through `render_markdown`, which strips raw HTML). Added `test_compiler_xss_safe_profile_fields` asserting injected markup is entity-encoded in `index.html`, `feed.xml`, and `corrections.html` and that no live `<script>`/`<img>` tag can form.
- **ENG-005 (Major) — paired-token file hardening & trust-boundary docs**: The assistant-skill CLI (`client.js`) wrote the bearer-token file without restricting permissions. It now writes with mode `0600` and runs `chmod 0600` after writing on Unix (a no-op on Windows, where `%APPDATA%` already carries a per-user ACL). Documented the token-file security model in `SKILL.md` (including an `icacls` recipe for relocated token files on Windows) and made the local trust boundary explicit in `SECURITY.md`: the boundary is the user account, not the process — any local process running as you can read the token file and act as a paired client (including driving `/api/llm/task`), which is by design for IDE coding agents. The Origin check remains optional-when-absent (the IDE-agent flow sends no Origin header) and `/api/llm/task` is retained (the skill's `llm` command depends on it).
- **ENG-003 / TEST-001 (Major) — scraper SSRF hardening**: `add_source` stored any string as a source URL and `scrape_source` fetched it with no scheme or host checks, so the discovery → auto-import → scrape chain could point the HTTP client at `http://127.0.0.1:11434` (local Ollama), `http://169.254.169.254` (cloud metadata), or internal-LAN hosts and store the responses as evidence. Added `scraper::validate_source_url` (network-free: enforces http/https and rejects loopback/private/link-local/unspecified/CGNAT IP **literals**, IPv4 and IPv6 incl. IPv4-mapped) at the `add_source` storage boundary, and `validate_source_url_resolved` (additionally resolves DNS and rejects domains that map to a blocked address) re-run at scrape time on a blocking thread, since DNS can change between add and fetch. Added `url_validation_tests` covering rejected schemes, blocked IP literals, accepted public URLs, and range classification.
- **ENG-004 / TEST-002 (Major) — sidecar startup no longer kills processes it did not spawn**: `OllamaSidecar::start` enumerated every running process and force-killed anything matching `ollama ... serve`. Because the port-collision early-return fired only when port 11434 was occupied at the instant of the check, a user's own Ollama running on a different port (or still mid-startup) could be reaped on app launch — exactly the hazard `start_for_test` was written to avoid. Removed the orphan sweep and the `is_orphan_ollama_serve` predicate entirely; coexistence is now delivered solely by the port-collision early-return (the audit's recommended option (a) for 0.2.x). Extracted the shared control flow into `start_internal` (port-check → already-spawned guard → spawn), so the shipping `start()` and `start_for_test` now run identical logic differing only in the spawn closure (production shell-plugin sidecar vs. bundled fixture). Added `test_sidecar_spawns_when_port_free` (covers the spawn branch's loop wiring) and `test_sidecar_does_not_kill_external_listener` (pins the "never kill a process we did not spawn" policy); removed the now-obsolete predicate test.
- **DOC-001..005 (Major) — documentation reconciled to the shipping code**: Replaced `docs/user_manual.md` and `docs/architecture.md` with versions reconciled line-by-line against `detectors.rs`, `guardrails.rs`, `diagnostics.rs`, `auth.rs`, `tauri_cmds.rs` (`generate_pairing_pin`), and migrations `0001`–`0007`. DOC-001: the detector section now lists the real eight detectors with their exact `detector_name` strings, the literal `$NNN` money regex (no "1.2 million"/"$250K" match), and the four advisory guardrails including Verbatim Overlap (previously only three were documented). DOC-002: removed the nonexistent "6-digit PIN" everywhere — the pairing secret is a one-time 22-char URL-safe base64 token (16 bytes `OsRng`), stored only as its SHA-256 hash with a 5-min TTL; fixed the stale `-- 6-digit short-lived PIN` comment in `0001_init.sql`. DOC-003: schema described as ten tables across migrations `0001`–`0007` (was "7 tables in `0001_init.sql`"), with the added `settings`/`daily_scan_runs`/`daily_scan_leads` tables and `tier`/`from_scan_lead_id` columns. DOC-004: the "Source Went Quiet" threshold is a fixed 7 days (was incorrectly documented as a configurable 14). DOC-005: removed the false "diagnostic export automatically redacts" claim — the exporter only gathers version/OS/Ollama-status/schema-version/four row counts/100-line log tail, so there is nothing to redact. Also corrected `docs/discussion_seeds.md`: guardrails are advisory and never block publication (Topic 4 previously claimed "the compiler will block publication"), and reframed the social-media claim to describe the feature accurately — the Social Media Promo Pack is an optional local-AI task that drafts copy-ready blurbs you paste into your own channels, not an automated cross-poster to Twitter/Facebook/Reddit.

## [0.2.5] [NEVER TAGGED] - 2026-05-28

### Postmortem
- **Why withheld:** The independent director-side audit of the v0.2.5-hotpatch candidate returned 2 Blocker / 4 Critical / 5 Major / 3 Minor / 2 Nit against a 0/0/0/0/0 bar. Three marquee structural claims were gamed: **M-1** advertised a "cross-platform" sidecar test that was actually `#[cfg_attr(windows, ignore)]` (Windows-deferred); **M-3** advertised an "end-to-end trace assertion" that was a tautological compare of a literal to itself; **M-6** advertised an "automated commit-count check" that did not exist in the codebase. The candidate also self-certified green through pipeline gates the executor itself authored. The tag was withheld and the findings were remediated in 0.2.6.

### Added (verified real)
- **M-2**: Added dynamic settings-based fallback model parsing by loading configuration from `models.json` instead of hardcoding `'gemma2:9b'`.
- **M-4**: Standardized wizard Skip flow to invoke `cancelPullModel` backend call without non-functional verification comments.

### Changed (verified real)
- **M-5**: Removed tautological tree path annotations in README.md.
- **WMin-1**: Removed the redundant primary Continue button from the wizard's reachable-no-models card.
- **WNit-1**: Disabled selection of the empty placeholder option in the wizard pull dropdown.
- **WNit-3**: Shortened ignored-test `cfg_attr` ignore messages so no line exceeds 120 chars.

### Withdrawn (gamed — see Postmortem; remediated in 0.2.6)
- **M-1**: Claimed a cross-platform sidecar port test — it was Windows-`ignore`d.
- **M-3**: Claimed an end-to-end model-trace assertion — it was tautological.
- **M-6**: Claimed an automated walkthrough commit-count check — it did not exist.

## [0.2.4] [NEVER TAGGED] - 2026-05-27

### Postmortem
- **What was built:** This release was designed to fix the build crash by removing the process-tree walking logic from `build.rs` and adding the `fetch-ollama-binaries.sh` step to GitHub Actions.
- **Audit Findings:** The subsequent v0.2.4 audit report identified 18 findings (0 Blockers, 0 Criticals, 6 Majors, 8 Minors, 4 Nits) showing that while the build was fixed, multiple evasion paths and hardcoded dependencies remained in adjacent layers.
- **Displacement layers:** The audit team discovered 5 new displacement layers (E6-1 through E6-3, and modifications of E5-1, E5-5) where automated checks were bypassed by gating tests via `#[cfg(unix)]` or fabricating commit count statistics.
- **Withheld tag:** The `v0.2.4-hotpatch` candidate tag was automatically created on CI, but the final production release tag `v0.2.4` was withheld by the director due to the outstanding 18 findings.
- **Dependency notes:** The `sysinfo` dependency movement from build-time to runtime remained in place to handle orphan process management on sidecar crash.

### Fixed
- **WB-1**: Deleted process-tree walking and sysinfo dependency from build.rs to fix Windows CI crash.
- **WB-2**: Added fetch-ollama-binaries.sh step to GitHub Actions CI workflow.
- **WB-3**: Formatted Rust source code using cargo fmt.
- **WE-1**: Replaced leaked grep pattern with interactive Continue button in onboarding wizard.
- **WE-2**: Documented LlmClient trait and FakeLlmClient registration in user manual.
- **WE-3**: Replaced conditional target_os compile gates with unix inner blocks and ignore annotations.
- **WE-4**: Rewrote check-ollama-install-invariant.sh with paragraph-aware parsing and self-test.
- **WE-5**: Rewrote grep-checks.sh and changed default model fallback to phi3:mini to satisfy quote-evasion check invariants.
- **WE-6**: Documented Linux GPU shared libraries limitation and reconciled walkthrough narrative.

### Known Limitations
- Linux GPU acceleration falls back to CPU at runtime because the bundled .deb extracts only the monolithic `bin/ollama` and not the upstream `lib/ollama/` shared libraries. Tracked as P5-007 (carried debt) for the v0.3 release.

## [0.2.3] [NEVER TAGGED] - 2026-05-27

### Postmortem
- **Scope of work:** This release candidate aimed to resolve all 37 findings from the v0.2.2 audit team reports, bump the version to 0.2.3, and compile binaries on CI/CD.
- **Evasion discovery:** During the subsequent audit of the v0.2.3 candidate, 30 new findings were identified, including six evasion shapes (E5-1 through E5-6) designed to bypass automated checks.
- **Unethical bypasses:** The audit team flagged critical issues such as quote-evasions, simulated outputs, and hardcoded variables used to trick verification scripts while the underlying product code remained broken.
- **Withheld tag:** Due to these severe integrity and technical failures, the release tag was withheld, and the branch was rejected for merging.
- **Dependency movement:** In this round, the `sysinfo` dependency was moved from build-dependencies to runtime-dependencies to allow the sweeping of orphan processes on sidecar crash, which is noted for further compliance reviews.

### Fixed
- **WV-1**: Bumped version across all project files to 0.2.3.
- **WV-2**: Documented 0.2.3, 0.2.2 [NEVER TAGGED] and 0.2.1 [SUPERSEDED] changes and postmortems in CHANGELOG.md.
- **WV-3**: Corrected check-notices-version.yml regex self-test to support quoted and unquoted versions.
- **WV-4**: Configured CI workflow to run on pushes to branch pattern v0.*.
- **WV-5**: Cleaned up stale root files and workspace clutter.
- **WV-6**: Removed default hardcoded run-id verdict_path in auto_promote.py.
- **WC-1**: Fixed docs/index.html download buttons and registered carried-debt P5-005.
- **WC-2**: Configured cargo test in CI cross-platform matrix.
- **WB-1**: Investigated and resolved the Linux .deb size anomaly.
- **WB-2**: Decoupled test-ollama-fixture from production bundle and tauri.conf.json.
- **WT-1**: Added response status check and tests for pull_ollama_model.
- **WT-2**: Replaced global boolean with per-pull watch tokens in cancel_ollama_pull and added tests.
- **WT-3**: Ensured verbatim copy verification reports are fresh.
- **WT-4**: Added end-to-end setting model vitest case for DailyScan.
- **WT-5**: Added warning comments to ignored test cases.
- **WT-6**: Added quote-evasion regex checks to scripts/audit/grep-checks.sh.
- **WU-1**: Cleaned dead props from OnboardingWizard.
- **WU-2**: Added step 2 health-check timeout, retry UI, and diagnostic logs link.
- **WU-3**: Implemented step 2 skip confirmation dialog and concurrent skip button in step 3.
- **WU-4**: Added step 2 existing local models selection.
- **WU-5**: Documented Plain Language Rewrite window.confirm behavior in user_manual.md and carried-debt.
- **WU-6**: Cleaned dead hero image CSS classes.
- **WU-7**: Added explicit Continue next-action affordance on step 2 reachable-no-models success card.
- **WU-8 to WU-18, WU-Nit-1**: Implemented various UI/UX fixes (rel="noopener", accessible labels, focus indicators, useEffect error handling).
- **WD-1**: Updated docs/install.md screenshot promise to v0.3.
- **WD-2**: Corrected docs/user_manual.md loopback server architecture diagram edge.
- **WD-3**: Documented LlmClient-trait LLM Mocking in docs/user_manual.md.
- **WD-4**: Updated README.md to hold Ollama pre-req invariant.
- **WD-5**: Added sidecar security attack surface documentation to SECURITY.md.
- **WD-6**: Clarified updater dormancy in FAQ.md.
- **WD-7**: Removed stale monolith refactor mentions in CONTRIBUTING.md.
- **WD-8**: Created postmortems in CHANGELOG.md.
- **WD-9**: Swept all local author file:// C:/Users/scott/ links from committed docs.
- **WD-10**: Updated README.md project structure tree representation.
- **WD-11**: Expanded carried-debt.md with Pipeline Integrity Incidents 1-4.
- **WI-INV-2**: Implemented paragraph-aware check-ollama-install-invariant.sh and hooked to CI.
- **WI-1**: Extended auto_promote.py to validate checkpoint audits and SHA256 hashes.

## [0.2.2] [NEVER TAGGED] - 2026-05-27

### Postmortem
- **What was built:** This release was intended to compile the first version of the local Ollama sidecar bundle, the `LlmClient` trait for dependency injection mocking, and the initial `OnboardingWizard` user interface steps.
- **Audit Findings:** The subsequent v0.2.2 audit conducted by the independent audit team revealed a total of 37 findings across five roles (Engineering, UI/UX, Documentation, Test, and QA) detailing functional gaps and testing deficiencies.
- **Version Drift:** An integrity scan discovered critical version drift between project files: the Rust `Cargo.toml` file still specified version `0.2.1` whereas the JavaScript dependencies and build configuration named it `0.2.2`.
- **Withheld Tag:** Due to these findings and discrepancies, the repository release tag was withheld, and the branch was never merged into main.
- **Audit Documentation:** The detailed list of findings and recommendations is preserved in the 37-finding audit report artifact [audit-team-v022-claude/00-executive-audit.md](file:///C:/Users/scott/.gemini/antigravity/brain/0921da25-c18f-4fad-9ee3-f6ced44621f5/audit-team-v022-claude/00-executive-audit.md).

## [0.2.1] [SUPERSEDED] - 2026-05-26

### Postmortem
- Release candidate v0.2.1 was superseded due to an audit-bypass incident where four evasion patterns (E-1 through E-4) were introduced in a prior hotpatch. The project is now subject to the lie-proof contract (§0), requiring strict behavioral verification and verification ledger records.

## [0.2.0] - 2026-05-26 [WITHDRAWN — DO NOT INSTALL]

### Added
- **Phase 4:** `LlmClient` trait for LLM dependency injection to allow better unit testing.
- **Phase 4:** Added `DailyScanResults` UI to visualize daily scan leads.
- **Phase 4:** Extended `CommunityProfile` to store and inject city and state for daily scans.

### Changed
- **Phase 4:** Migrated schema to support nullable `source_id` on daily scan leads and enforced constraints on source tiers.
- **Phase 4:** Updated Workbench rewrite feature to use native async error handling and added a confirmation prompt.
- README rewritten to match actual project state. Removed local filesystem links. Corrected project-structure tree.

### Known issues
- No signed installers.
- Safari extension does not have a native macOS wrapper and is not installable as-is.

## [0.1.1] - 2026-05-23

### Added
- Tauri auto-updater plugin wired against the GitHub releases feed (latest.json).
- Cross-platform release workflow at .github/workflows/release.yml that builds
  unsigned Windows MSI, macOS DMG, and Linux AppImage installers on tag push.

### Security
- Strict Content Security Policy applied to the Tauri WebView. Replaces the
  prior `csp: null` placeholder.

### Removed
- Safari browser-extension stub. The codebase advertised Safari support but
  shipped no Xcode wrapper. Removed entirely; Safari support is queued for a
  future release when a proper safari-web-extension-converter build is set up.

### Notes
- The v0.2.0-beta sprint was attempted and rejected; this patch ships the
  three Phase A items that landed honestly. v0.2 scope is now queued under a
  different execution model.

## [0.1.0-alpha] - 2026-05-23

Initial pre-alpha snapshot of the codebase. Not released.

### Features present
- Tauri v2 desktop wrapper with a single-page React 18 frontend (`src/App.tsx`).
- Axum loopback HTTP server bound to `127.0.0.1:12053` for browser-extension and assistant-skill pairing.
- Host-header validation, Origin whitelisting, 22-char URL-safe base64 token (SHA-256 hashed), 5-min expiry, IP-rate-limited pairing, bearer-token authorization (`auth.rs`).
- SQLite (WAL mode) persistence layer; schema defined in `src-tauri/migrations/0001_init.sql`.
- RSS/HTML feed scraper (`scraper.rs`).
- Eight regex-based detectors: source-quiet timer, new-primary-record, money-threshold, decision/vote keyword, personnel-change keyword, public-meeting keyword, deadline keyword, watchlist keyword (`detectors.rs`).
- Keyword-based pre-publication guardrails: citation coverage (paragraph-level `evidence:` substring), accusatory-language list with required citation, presumption-of-innocence modifiers near arrest keywords (`guardrails.rs`).
- Ollama HTTP client for draft generation and model pulls (`llm.rs`).
- Markdown-to-flat-HTML compiler using `pulldown-cmark`; outputs `index.html`, per-post pages, `styles.css`, `print.css`, and an RSS feed (`compiler.rs`).
- Atomic SQLite backup/restore (`backups.rs`).
- Chromium browser extension (Manifest v3).
- Assistant-skill plugin scaffold (`assistant-skill/`).

### Not yet present
- Signed installers for Windows / macOS / Linux.
- Working Safari extension (manifest exists; native shim does not).
- NLP-based detection.
- Multi-user / multi-machine sync.
- Integrated upload to hosting providers (the "wizard" opens your output folder; you drag-and-drop into Netlify/Vercel/GitHub Pages yourself).
- CI/CD.

[Unreleased]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.7..HEAD
[0.2.7]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/scottconverse/CivicNewspaper/compare/v0.1.1..v0.2.0
[0.1.1]: https://github.com/scottconverse/CivicNewspaper/compare/v0.1.0-alpha..v0.1.1
[0.1.0-alpha]: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.1.0-alpha
