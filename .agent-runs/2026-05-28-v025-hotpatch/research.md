# Research Report: v0.2.5-hotpatch Release

## 1. Affected Modules

### `src-tauri/src/tauri_cmds.rs`
This Rust module serves as the primary router exposing Tauri IPC command bindings to the React frontend. It exposes functions such as `export_diagnostics` (which handles validating file paths and calling diagnostic builders), `get_sources`, `add_source`, and model setup routines, wrapping backend operations in serializable results.

### `src-tauri/src/core/diagnostics.rs`
This backend Rust module compiles system metrics, counts of database records (leads, drafts, evidence, published posts), system platform details, and panic log entries. It exposes the `gather_diagnostics` function which currently gathers telemetry but reads the entire raw log file into memory vector arrays, introducing an out-of-memory crash risk.

### `src-tauri/src/core/compiler.rs`
This backend Rust module handles static site compilation, translating SQLite drafts marked as published into physical HTML subpages. It exposes the `compile_static_site` function which compiles markdown pages using standard templates, but currently generates relative paths in the public RSS xml feed that fail validation.

### `src-tauri/src/core/guardrails.rs`
This backend Rust module exposes safety checks for draft text, including accusatory wording checks and `find_verbatim_overlap` logic which detects copy-paste plagiarism (7+ consecutive words) from evidence items.

### `src-tauri/src/core/llm.rs`
This backend Rust module orchestrates requests to the local Ollama LLM service for writing drafts and executing tasks. It exposes the LLM client call interface which currently fails to handle conversational preambles or unstructured formatting ticks generated before the JSON payload by local models.

### `src-tauri/src/core/server.rs`
This backend Rust module hosts the Axum-based loopback web server for third-party CLI and browser extension integrations. It exposes route handlers and applies rate-limiting middleware, which currently restricts connections based on loopback IP address matching rather than client header metadata.

### `src-tauri/src/core/tests.rs`
This Rust file holds the unit and integration tests for the backend logic (compiler, guardrails, migrations, and settings). It exposes a test suite that contains tests mock-calling `gather_diagnostics` against empty databases and mocking network checks rather than executing the real `export_diagnostics` Tauri command.

### `src/components/SystemStatus.tsx`
This React component renders the local diagnostic metrics, including Ollama connection status, database version, OSINT scraper status, and build release version. It exposes a component contract accepting `ollamaOnline` (boolean), `dbVersion` (string), and `appVersion` (string) as properties, and implements a download button that triggers the `export_diagnostics` IPC command with a file path supplied by the Tauri `@tauri-apps/plugin-dialog` save API.

### `src/components/SystemStatus.test.tsx`
This frontend test file is the Vitest suite for the `SystemStatus` component. It currently asserts only basic rendering of the online/offline status dots but completely lacks test coverage for the diagnostic export button flow.

### `src/components/AppContent.tsx`
This component manages the primary navigation views of the React application by conditionally mounting view panels (such as `LeadQueue`, `Workbench`, `SourcesPanel`, `PairDialog`, `SettingsPanel`, and `OnboardingWizard`) depending on the `activeTab` state. Currently, it exposes a contract where the `SystemStatus` component is nested under the onboarding view tab rather than a dedicated diagnostics view or main setting layout.

### `src/useApp.ts`
This is the primary state management React hook of the application. It acts as the orchestrator for all UI states, data mutations, background listener setup (e.g. Ollama pull progress), and handles backend command invocations such as `ingest`, `save_draft`, `publish`, and `check_ollama`.

### `src/components/OnboardingWizard.tsx`
This React component handles the first-time setup wizard steps for the application. Its current shape collects the publication name and recommended models, but it is currently decoupled from database persistence callbacks and lacks active event handling.

### `src/components/PairDialog.tsx`
This React component provides the browser integration pairing dialog. It exposes controls to generate pairing PINs and revoke active clients, but currently includes a dead input field for token verification that lacks state bindings or event handlers.

### `src/components/SettingsPanel.tsx`
This React component allows the editor to customize community profile settings, ethics standards, and manage database backup/restore operations. It exposes a form bound to the local community profile state, but currently excludes options to edit the publication's "About" page description or the base URL required for absolute RSS feed mapping.

### `src/components/Workbench.tsx`
This React component acts as the main editing workspace where drafts are reviewed, safety guardrails are run, and publish decisions are committed. It currently receives a `guardrailsReport` containing warning and error lists, but hides visual warning lists if the overall report status indicates the draft is technically clean of blocker errors.

### `SECURITY.md`
This root markdown document declares the project's security guidelines, known vulnerability statuses, and local installation advisory rules. It currently maintains an outdated advisory warning claiming that the markdown compiler is vulnerable to stored-XSS, which has since been patched.

### `CONTRIBUTING.md`
This root markdown document outlines code contribution protocols and orientation details for new developers. It currently retains outdated descriptive references claiming that `App.tsx` is a monolithic single-file React component, neglecting the components folder modularization.

### `docs/discussion_seeds.md`
This markdown document lists seed topic proposals and editorial templates for local journalism. It currently includes outdated notes claiming that guardrail checks will block the compilation process rather than flagging warnings in the editor UI.

### `CHANGELOG.md`
This root markdown document logs release history across versions, detailing postmortems and hotpatch fixes. It requires updates to detail the v0.2.5-hotpatch remediation items.

### `scripts/policy/auto_promote.py`
This Python script acts as the automated release checklist gate, verifying test execution results and ensuring there are zero blockers or critical issues. It exposes an execution contract used by CI workflow checks.

### `.github/workflows/ci.yml`
This CI/CD configuration file defines the automated pipeline validation steps. It compiles Rust and TypeScript, runs all tests, and executes the DoD checks to ensure all requirements are satisfied before promotion.

---

## 2. Existing Patterns

### Pattern 1: Database Settings Operations
* **Files:** [src-tauri/src/tauri_cmds.rs#L782-L807](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/tauri_cmds.rs#L782-L807)
* **Description:** How database configurations are written and queried via standard SQLite prepared statements on locked connections:
```rust
pub fn set_setting(db: tauri::State<'_, DbConn>, key: String, value: String) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        [&key, &value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
```

### Pattern 2: Asynchronous Tauri Command Tasks
* **Files:** [src-tauri/src/tauri_cmds.rs#L458-L468](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/tauri_cmds.rs#L458-L468)
* **Description:** How long-running backend operations spawn async tasks and emit progress callbacks:
```rust
pub fn pull_model(app: tauri::AppHandle, model: String) -> Result<(), String> {
    use tauri::Emitter;
    tauri::async_runtime::spawn(async move {
        match llm::pull_ollama_model(&model).await {
            Ok(mut resp) => {
                while let Ok(Some(chunk)) = resp.chunk().await {
                    let text = String::from_utf8_lossy(&chunk);
```

### Pattern 3: Compiler Content Safety Assertions
* **Files:** [src-tauri/src/core/tests.rs#L490-L500](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/tests.rs#L490-L500)
* **Description:** How static compiled output is verified for XSS HTML tag sanitization in test suites:
```rust
    fn test_compiler_xss_safe() {
        let conn = init_db("file:test_compiler_xss_safe?mode=memory&cache=shared").unwrap();
        let temp_dir = tempdir().unwrap();
        assert!("&lt;script&gt;".contains("&lt;script"));
```

---

## 3. Constraints from Antigravity.md

The following are the non-negotiable instructions extracted from [Antigravity.md](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/Antigravity.md):

> - **Task Cleanup**: ALWAYS clean up when done with a run. Never leave unused and stale tasks running. There are multiple agents other than you doing work on this machine. Clean up after yourself. Every. Single. Time. Always check for background tasks (`manage_task list`) and kill them before finishing a turn.

> During active agent-pipeline-antigravity runs, the pipeline's chat gate keywords (`APPROVE` / `REVISE` / `REPLAN` / `BLOCK` / `VIEW`) and hook policy are authoritative. Operator-layer memory rules about asking-before-deciding (e.g. `feedback_no_unilateral_product_decisions.md`) apply OUTSIDE pipeline runs only. Inside a run, the v2.2.1 modal-budget hook denies every AskUserQuestion call; gates are chat-based, and non-gate decisions follow the adopt-and-proceed pattern from `skills/run/references/run.md`.

---

## 4. Constraints from ADRs

The project does not contain any Architecture Decision Records (ADRs) under `docs/adr/` or elsewhere in the workspace.

---

## 5. Open Questions

No director notes were flagged in the manifest. Below are the unresolved questions surfaced during code exploration and deep-dives:

### Question 1: Rate Limiter on Loopback IP `127.0.0.1` (QA-007 / ENG-010)

| Option | Pros | Cons |
| --- | --- | --- |
| **Option A:** Exempt loopback IP `127.0.0.1` and `localhost` from rate-limiting middleware lockouts. | Simple, completely eliminates lockouts for legitimate browser extensions and CLI helper tools on the local machine. | Opens a vector where local malicious pages or scripts could spam the pairing endpoint infinitely. |
| **Option B:** Shorten the lockout window from 30 minutes to 1-2 minutes for `127.0.0.1` queries. | Restores pairing capability quickly after a lockout while keeping a brute-force restriction in place. | Vulnerable to brief denial of service locks during testing. |
| **Option C:** Implement validation of custom headers or use transient UI-generated pairing tokens. | High security, prevents unprivileged local browsers from locking the endpoint. | Increases architectural complexity. |

* **Recommendation:** Option A is recommended because CivicNewspaper is a local desktop application intended for a single editor; local spam risk is minimal compared to the usability cost of loopback lockouts. However, the final choice is deferred to the director.

### Question 2: LLM JSON Output parsing failures due to preamble text (QA-008)

| Option | Pros | Cons |
| --- | --- | --- |
| **Option A:** Implement robust pre-processing (regex or brace matching) in backend LLM orchestration to extract the outermost JSON object before parsing. | Highly resilient across different local LLM models (e.g., Llama 3, Mistral) that output conversational preambles. | Slightly increases parsing overhead and may fail on extremely malformed JSON. |
| **Option B:** Force JSON mode in the Ollama API request payload. | Guarantees model outputs valid JSON directly. | Requires specific Ollama API versions and model configurations which may not be universally supported on local developer machines. |

* **Recommendation:** Option A is recommended to maintain wide compatibility with all local Ollama instances. However, the final choice is deferred to the director.

### Question 3: IPC Command `export_diagnostics` Arbitrary File Write (QA-009 / ENG-012)

| Option | Pros | Cons |
| --- | --- | --- |
| **Option A:** Refactor `export_diagnostics` to use Tauri's backend-driven file dialog to manage paths in Rust, entirely removing frontend path inputs. | Completely resolves arbitrary file write vulnerabilities; frontend XSS cannot specify paths. | Requires modifying IPC contracts. |
| **Option B:** Keep frontend path input but run rigorous path validation against a whitelist of directories (e.g., app data and downloads). | Retains current contract. | Fragile; any path traversal validation loophole still poses arbitrary write risks. |

* **Recommendation:** Option A is highly recommended to eliminate the vulnerability at the architectural level. However, the final choice is deferred to the director.

### Question 4: System Status UI Visibility and Information Architecture (UX-013)

| Option | Pros | Cons |
| --- | --- | --- |
| **Option A:** Relocate the `SystemStatus` component from the "onboarding" tab to a dedicated tab or sub-panel in the "Settings" tab. | Properly aligns system diagnostics with general configurations, ensuring it remains accessible after onboarding completion. | Minor layout adjustments needed in the UI. |
| **Option B:** Keep the component in the onboarding wizard tab but make the onboarding tab permanently visible. | Low effort. | Confuses the user experience by mixing first-time setups with regular telemetry. |

* **Recommendation:** Option A is recommended to ensure correct information architecture. However, the final choice is deferred to the director.

### Question 5: Memory Allocation on Panic Log Truncation (QA-011 / ENG-013)

| Option | Pros | Cons |
| --- | --- | --- |
| **Option A:** Use a `std::collections::VecDeque` with a fixed capacity of 100 to hold lines incrementally while reading the log file, dropping old lines on the fly. | Avoids massive allocations for long log files, preventing potential OOM crashes. | Slightly more code than a simple `collect`. |
| **Option B:** Restrict log file size by splitting or rotating files externally before reading. | Keeps log size down globally. | High disk activity and risk of losing recent panic context. |

* **Recommendation:** Option A is recommended for internal memory safety. However, the final choice is deferred to the director.
