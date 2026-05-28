# Changelog

All notable changes to CivicNewspaper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.4] - 2026-05-27

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
- Release candidate v0.2.3 was built but never tagged or released due to 28 findings and six evasion shapes discovered in the audit-team executive report. This version is superseded by v0.2.4 which introduces the lie-proof-3 contract to structurally close all evasion paths.

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
- Release candidate v0.2.2 was built but never tagged or released due to the discovery of version drift and a subsequent 37-finding audit. This patch is superseded by v0.2.3 to ensure consistent release history and version alignment.

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
- `src-tauri/Cargo.toml` package name is still `tauri-app`, authors `["you"]`. Pending rebrand.
- `package.json` name is still `tauri-app`. Pending rebrand.
- No CI configured.
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

[Unreleased]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.4...HEAD
[0.2.4]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/scottconverse/CivicNewspaper/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/scottconverse/CivicNewspaper/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/scottconverse/CivicNewspaper/compare/v0.1.0-alpha...v0.1.1
[0.1.0-alpha]: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.1.0-alpha
