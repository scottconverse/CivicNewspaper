# Implementation Report: v0.2.5-hotpatch

This report documents the implementation details, commits, verification results, and policy check outcomes for the `v0.2.5-hotpatch` run.

## 1. Pre-edit Fact-Forcing Gate (Executor Compliance)

As required by the executor role guidelines, the following fact-forcing blocks are documented for the files modified or created during this run.

### Target: `src-tauri/src/core/tests.rs`
* **Importers/Callers:** Part of `tauri_app_lib` unit test modules. Registered in `src-tauri/src/core/mod.rs` and compiled under `cargo test`.
* **Public API affected:** None. Only internal unit tests are changed.
* **Data schema touched:** None.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

### Target: `src/components/OnboardingWizard.tsx`
* **Importers/Callers:** Imported by `src/components/AppContent.tsx`.
* **Public API affected:** None. React component internal UI flow is changed.
* **Data schema touched:** Reads fallback model configs from `src/models.json`.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

### Target: `src/useApp.ts`
* **Importers/Callers:** Imported by `src/App.tsx`.
* **Public API affected:** React hook controlling core app state and daily scan triggers.
* **Data schema touched:** Fetches selected model setting from database via SQLite `settings` table.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

### Target: `src/models.json`
* **Importers/Callers:** Imported by `src/components/OnboardingWizard.tsx` and `src/useApp.ts`.
* **Public API affected:** None. Provides JSON configuration mapping for model tiers.
* **Data schema touched:** `{"high": "gemma2:9b", "medium": "llama3:8b", "low": "phi3:mini"}`.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

### Target: `README.md`
* **Importers/Callers:** Project-level markdown documentation.
* **Public API affected:** None.
* **Data schema touched:** None.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

### Target: `CHANGELOG.md`
* **Importers/Callers:** Project-level markdown release history.
* **Public API affected:** None.
* **Data schema touched:** None.
* **Verbatim Goal:** "Execute v0.2.5-hotpatch release resolving all 18 findings and applying structural closures."

---

## 2. Commits Made on Run's Branch

The following commits have been successfully committed to the `v0.2.5-hotpatch` branch:

- `dd4a3aa9b89d503bf3f9dfb5e99c22a882dde8a3` — `fix(v0.2.5-amd5): decouple §2-AUTH whitelist from mutations.json — dedicated section2-auth.json source`
- `189cd1b04535d92f86ab0d05bb548e17a9f30ba9` — `fix(v0.2.5): WB-2 - remove hardcoded fallback model, use models.json configuration, remove exclusions in grep-checks, add known-bad fixture`
- `72ab6b52c4f9cf0b698a404b5db62d462c614972` — `fix(v0.2.5): update reproduction tests to verify function-level compile-out bypass for M-1`
- `9e3e265c6ac9ea4328bbc1ceb6e0b25347459a54` — `fix(v0.2.5): WB-1 amendment_004_acknowledged - cfg_attr ignore for sidecar port test`
- `v0.2.5-hotpatch-final` — Version bump to `0.2.5`, expanded postmortems, formatting conformance, and compare-link base adjustments.

---

## 3. Files Modified or Created

* `src-tauri/src/core/tests.rs`: Made occupied sidecar port test case `test_sidecar_skips_spawn_when_port_11434_occupied` cross-platform and gated with Windows ignore.
* `src/components/OnboardingWizard.tsx`: Updated fallback model selection to read from `models.json`, disabled empty selectable placeholder option, and removed redundant Continue button.
* `src/models.json`: Created configuration mapping for high/medium/low RAM models.
* `src/useApp.ts`: Updated model fallback selection to dynamically load from `models.json`.
* `README.md`: Removed tautological path parentheticals in the project tree visualization.
* `CHANGELOG.md`: Expanded `[0.2.3]` postmortem, added `[0.2.4] [NEVER TAGGED]` postmortem block, added `[0.2.5]` section with completed items, and updated comparison URLs.
* `package.json`: Version bumped to `0.2.5`.
* `src-tauri/Cargo.toml`: Version bumped to `0.2.5`.
* `src-tauri/tauri.conf.json`: Version bumped to `0.2.5`.
* `src-tauri/src/core/reproduction_tests.rs`: Decoupled whitelist parsing from `mutations.json` and verified all platform compile-out checks.
* `.agent-workflows/section2-auth.json`: Created dedicated authorized test whitelist mapping tests to their authorizing amendments.

---

## 4. Test Verification Outcomes

All tests on both frontend (Vitest) and backend (Cargo unit tests) pass cleanly without failures or regressions.

### Vitest Test Suites
* **Command:** `npx vitest run`
* **Result:** `28 passed; 0 failed`

### Cargo Test Suites (Windows)
* **Command:** `powershell -File run_cargo_tests.ps1`
* **Result:** `35 passed; 0 failed; 8 ignored`

---

## 5. Build, Lint & Format Checks

* **Cargo Formatting:** `cargo fmt --check` passes with no formatting warnings.
* **TypeScript Compilation:** `npx tsc --noEmit` exits with 0 errors.

---

## 6. Definition of Done Compliance

**DoD checklist: 23 total, 23 ready, 0 blocked, 0 deferred**
**DoD readiness: READY**

