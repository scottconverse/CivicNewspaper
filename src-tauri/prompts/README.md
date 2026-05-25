# CivicNewspaper Bundled Prompt Library

These are the bundled LLM prompts the CivicNewspaper desktop app ships with.
They drive the Daily Scan aggregator, the Plain Language rewrite, and other
local-LLM workflows.

## Provenance

Imported from [`scottconverse/civic-transparency-toolkit`](https://github.com/scottconverse/civic-transparency-toolkit)'s
`prompts/` directory on 2026-05-25. That repo is the upstream operator-authored
prompt set. CivicNewspaper organizes the same prompts into category
subdirectories.

## Categories and inventory

| Category | File | Drives |
|---|---|---|
| `aggregator` | `01-daily-scan.md` | Daily Scan run_daily_scan command |
| `story` | `02-story-expansion.md` | Future story-expansion flows |
| `audit` | `03-black-desk.md` | Future adversarial audit flows |
| `audit` | `04-dark-signal-desk.md` | Future dark-signal detection |
| `audit` | `05-integrity-checker.md` | Phase 5 optional integrity audit (audit_draft_integrity) |
| `legal` | `06-first-amendment-counsel.md` | Future legal-review flows |
| `story` | `07-plain-language.md` | Phase 4 Plain Language rewrite (plain_language_rewrite) |
| `utility` | `08-civic-grounding.md` | Utility prompt for civic grounding |
| `story` | `09-story-research-writing.md` | Future story research flow |

Nine prompts total: 1 aggregator + 3 story + 3 audit + 1 utility + 1 legal.

## How the app loads them

The Rust backend reads these files via `tauri::api::path::resolve_resource`,
which requires `prompts/**/*` to be declared in `src-tauri/tauri.conf.json`'s
`bundle.resources` array. Phase 4 implementation wires this up.

The Tauri commands that consume the library are:

- `list_prompts() -> Vec<PromptMeta>` — enumerates the directory, returns
  `[{ id, category, title, path, description }]` for each.
- `get_prompt(id: String) -> Result<String, String>` — loads the raw markdown
  body for a single prompt. Input id MUST be validated against the
  enumerated list returned by `list_prompts()` (no path traversal).

## Updating prompts

These files are operator-owned. The Phase 4 manifest's `allowed_paths`
includes `src-tauri/prompts/` only for the initial bundle wiring (so the dev
can verify the tauri.conf.json bundle.resources entry resolves correctly).
Updates to prompt content after Phase 4 land via a separate operator-side
setup PR — not as part of feature pipeline runs.

When updating: keep the upstream-equivalent prompt in
`scottconverse/civic-transparency-toolkit/prompts/` in sync. If the two
diverge, document the reason here.
