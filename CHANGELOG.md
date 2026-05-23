# Changelog

All notable changes to CivicNewspaper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- LICENSE (MIT).
- CONTRIBUTING.md, SECURITY.md, CHANGELOG.md, FAQ.md.

### Changed
- README rewritten to match actual project state. Removed `file:///C:/...` links to author's local filesystem. Corrected project-structure tree (`src/components/` and `src-tauri/templates/` claims removed).

### Known issues
- `src-tauri/Cargo.toml` package name is still `tauri-app`, authors `["you"]`. Pending rebrand.
- `package.json` name is still `tauri-app`. Pending rebrand.
- No CI configured.
- No signed installers.
- Safari extension does not have a native macOS wrapper and is not installable as-is.

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

[Unreleased]: https://github.com/scottconverse/CivicNewspaper/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.1.0-alpha
