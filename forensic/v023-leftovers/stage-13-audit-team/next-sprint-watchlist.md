# Next-Sprint Watchlist — CivicNewspaper

**Audit date:** 2026-05-26

Forward-looking items. These are findings that do not belong in the current sprint — usually because they require cross-team coordination, architectural thinking, or product/leadership input — but must be on the team's radar for the next planning cycle.

---

## Structural / architectural

| # | ID | Role | What to consider | Trigger to act |
|---|---|---|---|---|
| 1 | [ENG-003] | Engineering | Implement batch transactions for scraped items to optimize SQLite lock performance. | Before scraping >100 feed sources. |
| 2 | [ENG-005] | Engineering | Replace regex-based HTML clean parsing with a robust HTML sanitization library (like `ammonia`). | Before ingestion of nested layout tables. |
| 3 | [ENG-011] | Engineering | Clean up dead/unreachable bypass code in Axum middleware and apply properly to root router. | Prior to exposing additional endpoints. |

## Design debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | [UX-008] | UX | Add "About Page" text editor configurations inside the application SettingsPanel. |
| 2 | [UX-021] | UX | Redefine slate gray colors on landing page cards to ensure AA standard contrast. |

## Documentation debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | [DOC-007] | Docs | Update legacy single-page React component description in `CONTRIBUTING.md`. |
| 2 | [UX-018] | Docs | Align user manual's references of pairing tokens and detectors with code reality. |

## Test-culture debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | [TEST-003] | Test | Write integration tests at the IPC boundary to validate frontend-backend schema serialization. |
| 2 | [TEST-004] | Test | Write unit tests for core Rust scraping, paragraph chunking, and metadata parsing. |
| 3 | [TEST-013] | Test | Fix prompt loading tests to call the actual Tauri path resolvers instead of direct fs reads. |

## Performance and scaling

| # | ID | Role | What to consider | Trigger to act |
|---|---|---|---|---|
| 1 | [ENG-004] | Engineering | Enforce download payload size limits during feed ingestion to prevent OOM errors. | Prior to expanding to public feeds. |
| 2 | [ENG-006] | Engineering | Implement paragraph/sentence splitting boundaries inside scraper text chunker. | Prior to loading transcript documents. |

## Dependency and supply chain

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | [ENG-016] | Engineering | Clean up Tauri command stubs (`pull_model` vs `pull_ollama_model`) and payload schema overlaps. |

## Decisions needing product/leadership input

- **[QA-005] RSS Feed Absolute URLs**: RSS specification requires absolute links for feed entries, but we do not store the base deployment URL in profile settings. PM needs to decide if a site domain field is a required profile setup setting.
- **[ENG-010] Localhost Rate Limiting**: Limiters keyed purely on `127.0.0.1` create a loopback-wide denial of service risk. PM/Tech Lead needs to balance security requirements against multi-integration usability.
- **[UX-016] Extension Distribution**: Standard app bundles do not include the raw extension source tree. Product team needs to decide if the browser extension should be hosted on the official stores or if the Tauri app needs an "Export Extension Code" utility.
