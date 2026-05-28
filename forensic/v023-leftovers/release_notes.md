# CivicNewspaper v0.2.0

## Features & Improvements in v0.2.0:
- **Ollama Sidecar Integration**: Bundled Ollama as a native Tauri sidecar binary to streamline first-run setup without requiring a separate manual installation of Ollama.
- **Onboarding Model-Pull Wizard**: Added a clean progress wizard to download and set up the default local AI model (`gemma2:9b`) upon first app launch, with real-time progress bars and skip capabilities.
- **Phase 4 Hardening**: Resolved key architecture findings, introduced `LlmClient` trait for mockable LLM interactions, added `DailyScanResults` lead visualization, and verified schema integrity.
- **Updated Documentation**: Full rewrite of user manual (in three dedicated parts for operators, technical operators, and developers), inline Mermaid system and dataflow diagrams, and platform-specific installation guides for unsigned binaries.
- **Landing Page Refresh**: Renewed GitHub Pages landing page with v0.2.0 smart download links, mobile responsiveness, and dynamic architecture diagrams.

## Changes:
### Added
- LICENSE (MIT).
- CONTRIBUTING.md, SECURITY.md, CHANGELOG.md, FAQ.md.
- `LlmClient` trait for LLM dependency injection to allow better unit testing.
- Added `DailyScanResults` UI to visualize daily scan leads.
- Extended `CommunityProfile` to store and inject city and state for daily scans.

### Changed
- Migrated schema to support nullable `source_id` on daily scan leads and enforced constraints on source tiers.
- Updated Workbench rewrite feature to use native async error handling and added a confirmation prompt.
- README rewritten to match actual project state.

### Known issues
- `src-tauri/Cargo.toml` package name is still `tauri-app`, authors `["you"]`. Pending rebrand.
- `package.json` name is still `tauri-app`. Pending rebrand.
- No signed installers (see `docs/install.md` for trust-bypass guides).
