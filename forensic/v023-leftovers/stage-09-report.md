# Stage 09 (Docs Pass) Report

All deliverables for the **Stage 09 (Docs Pass)** of the v0.2.0 ship sequence for CivicNewspaper have been completed. The codebase documentation has been successfully updated, structured, and supplemented with diagrams and assets.

---

## 📄 Completed Deliverables & File Links

1. **[README.md](README.md)**
   * Rewrote the top section to introduce CivicNewspaper and identify its target audience in plain English.
   * Added the **Download** section linking to the latest release assets with per-platform setup steps.
   * Added the **First Run** section explaining the onboarding wizard steps and the ~5.4 GB local language model download.
   * Preserved all developer and architecture details in the lower section of the file.

2. **[user_manual.md](docs/user_manual.md)**
   * **Part 1 — For Newsroom Operators (Non-Technical)**: Outlined installation procedures, onboarding, adding sources, managing the queue, running the Daily Scan, drafting with local LLMs, triggering the Plain-Language Rewrite, and publishing static pages.
   * **Part 2 — For Technical Operators**: Documented the Tauri-wrapped system components, the loopback security model, host/origin header checks, pairing PIN TTLs, token authentication, regex detector routines, guardrail checks, database migrations, and redacted diagnostic exports.
   * **Part 3 — For Developers**: Detailed local prerequisites, CLI build and test commands, SQLite inspection paths, and LLM mocking.

3. **[architecture.md](docs/architecture.md)**
   * Rewrote using structured Mermaid diagrams mapping out:
     * **System Overview**: The Tauri shell, React frontend, Rust core, Ollama engine, and SQLite database interface.
     * **Data Flow**: The path of municipal text from raw feeds to scraper, detectors, story workbench, compiled pages, and RSS exports.
     * **Daily Scan Flow**: Combining daily excerpts into prompt contexts to produce structured LLM findings.
     * **Plain Language Rewrite Flow**: Translating jargon via system prompts and using `window.confirm` for human authorization.
     * **Security Model**: The boundaries established by loopback isolation, header audits, pairing PIN handshakes, scope locks, and CSP rules.
     * **Onboarding Flow**: Checking dependencies, downloading model weights, and registering the first source.

4. **Assets & Illustrations**
   * Generated three illustrations matching modern flat design guidelines:
     * **[hero.png](docs/assets/hero.png)**: News dashboard visualization.
     * **[onboarding-empty-state.png](docs/assets/onboarding-empty-state.png)**: Placeholder empty state graphic.
     * **[publish-success.png](docs/assets/publish-success.png)**: Static site publishing celebration graphic.

5. **[install.md](docs/install.md)**
   * Created a dedicated installation file detailing:
     * Bypass procedures for Windows SmartScreen (*More info -> Run anyway*) and macOS Gatekeeper (*right-click Open / Privacy Settings workaround*).
     * Linux configuration via `.deb` packages and executable `.AppImage` files.
     * Step-by-step instructions to verify download integrity using PowerShell/Terminal commands and SHA256 checksums.

6. **[FAQ.md](FAQ.md)**
   * Added the QA explanation for Windows/Mac installer warnings pointing back to the "trust-without-signing" concept.
   * Outlined disk space requirements (~330 MB for application footprint and ~5.4 GB for local LLM weights).
   * Confirmed that the application operates entirely offline after the initial model pull.

7. **[CONTRIBUTING.md](CONTRIBUTING.md)**
   * Updated the developer guide to document the requirement of using the `LlmClient` trait in all new LLM-backed feature work to ensure testability.
