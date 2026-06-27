# CivicNewspaper

> **The installed app ships as "The Civic Desk."** "CivicNewspaper" is the project/repository name; the desktop window, installer, and Start-menu shortcut are branded **The Civic Desk** (renamed in v0.2.7).
>
> Public beta — unsigned. Not production software. No security review. APIs and database schema may break without notice.

CivicNewspaper is a local-first, privacy-focused desktop application designed for local journalists, newsroom operators, and community organizers who want to monitor public municipal feeds without relying on complex, cloud-based software. It automatically scans local RSS feeds and websites for public records and meeting notices, identifies important civic signals, and helps draft plain-language summaries using a private, offline language model running directly on your computer. By storing all database records and drafts locally, CivicNewspaper provides a secure, independent toolkit to help you track municipal actions and compile a static website ready for publishing.

> **Public beta — unsigned.** This is a public beta. The installers are not code-signed, so Windows SmartScreen and macOS Gatekeeper will warn you on first launch. Those warnings are expected; see [Installation Instructions](#installation-instructions) for the exact steps to proceed.

## Download

You can download the latest pre-compiled installers for your platform from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).

### Installation Instructions

Because this is an unsigned public beta, your operating system will warn you the first time you run it. This is expected for unsigned software and does not mean the file is unsafe. To proceed:

* **Windows**: Download the `.msi` or `.exe` installer. When launching, Windows SmartScreen will display a "Windows protected your PC" warning because the installer is unsigned. Click **"More info"**, then click **"Run anyway"** to proceed.
* **macOS**: Download the `.dmg` or `.app` file. Because the app is not signed with an Apple developer certificate, macOS Gatekeeper will block execution. Right-click the application icon, select **Open**, and then confirm the prompt. Alternatively, go to **System Settings > Privacy & Security** and scroll down to click **"Open Anyway"** for The Civic Desk.
* **Linux**: Download the `.deb` package (for Debian/Ubuntu) and install it via your package manager (e.g., `sudo dpkg -i ./*.deb` on the downloaded file). Linux builds are deb-only.

## First Run

When you open CivicNewspaper for the first time, you will be guided through an onboarding wizard. 

1. **Identity**: Enter your Publication Name, Editor Name, City, and State.
2. **AI Service Setup**: The wizard verifies connection to the bundled offline AI sidecar engine.
3. **Download AI Model**: Pull the recommended AI model for offline draft generation and Daily Scan. The wizard recommends `qwen2.5:7b` (≈4.7 GB download) for 8 GB RAM or more, with `llama3.2:3b` (≈2 GB) as the low-RAM fallback. CivicNewspaper uses a scan-tested model because the Daily Scan depends on reliable JSON output, not just fluent prose.
4. **Defaults**: Configure your Publish Path (where static sites are compiled) and Backup Path (for database backups).
5. **Done**: Complete the setup to enter your workspace.

No accounts or internet connections are required after this initial setup.

## What this is, and what it isn't

**What it is, today:**
- A Tauri v2 desktop app with a React 19 frontend structured as modular components (`src/components/`) and state management (`src/useApp.ts`).
- A Rust core (`src-tauri/src/core/`) that:
  - Scrapes RSS/HTML feeds (`scraper.rs`).
  - Stores everything in a local SQLite database in WAL mode (`db.rs`).
  - Runs **eight hand-written regex detectors** against scraped text — for money amounts, vote/decision keywords, personnel-change keywords, meeting/deadline keywords, watchlist hits, and a "source went quiet" timer (`detectors.rs`). Each detector is a regular expression run over scraped text.
  - Runs **pre-publication checks** on drafts — alerts on accusatory terms, checks for the literal substring `evidence:` in paragraphs, flags missing presumption-of-innocence modifiers near arrest-related words, and warns when a paragraph copies a 7+-word sequence verbatim from a linked evidence excerpt (`guardrails.rs`). Note: this is a lint-like helper in the UI, not a compilation block.
  - Calls a local Ollama instance (`llm.rs`) for draft generation. Output quality depends on your local model configuration.
  - Compiles approved drafts into a flat HTML site using `pulldown-cmark` and templates in `templates/` (`compiler.rs`).
  - Exposes a localhost-only Axum HTTP server on `127.0.0.1:12053` for browser-extension and assistant-skill pairing (`server.rs`, `auth.rs`).

**What it isn't:**
- A finished product. Installers are currently unsigned.
- An NLP system. The "detectors" match keywords rather than named entities or numeric contexts.
- A multi-user newsroom. It is single-editor, single-machine.
- A full publishing platform. The wizard now compiles the site locally and can publish instantly with here.now, use GitHub Pages as a durable repository-backed archive, or use technical hosts such as Cloudflare Pages and Netlify.

## Architecture (one paragraph)

A Tauri-wrapped React frontend talks to a Rust backend via Tauri IPC. The Rust backend also runs an Axum HTTP server bound strictly to `127.0.0.1:12053` so that browser extensions and IDE-side assistant skills can pair (via short-lived 22-char token) and exchange bearer tokens. All persistent state lives in a single SQLite file (WAL mode). Draft generation routes to a local Ollama instance at `127.0.0.1:11434`. The static-site compiler reads approved drafts from SQLite and writes a folder of HTML + CSS + RSS to a user-chosen output path.

For details: [docs/architecture.md](docs/architecture.md).

## Project structure (verified)

```
.
├── README.md
├── package.json                # Vite + React frontend config
├── tsconfig.json
├── vite.config.ts
├── public/                     # Vite public assets
├── src/                        # React frontend
│   ├── App.tsx                 # Core entry point
│   ├── App.css
│   ├── useApp.ts               # React state logic & IPC handler
│   ├── ipc.ts                  # Tauri command bindings
│   ├── main.tsx
│   ├── vite-env.d.ts
│   ├── components/             # UI Components split by tab
│   │   ├── Workbench.tsx       # Story draft editor & guardrails
│   │   ├── PublishPanel.tsx    # Static site compilation panel
│   │   ├── SourcesPanel.tsx    # Source scraper settings
│   │   ├── SettingsPanel.tsx   # Backup, identity & model selection
│   │   ├── OnboardingWizard.tsx # Multi-step offline setup wizard
│   │   └── DailyScanResults.tsx # Extracted intelligence lead results
│   └── assets/
├── src-tauri/                  # Tauri Rust backend
│   ├── Cargo.toml              # Rust crate config (named "civicnews")
│   ├── build.rs
│   ├── tauri.conf.json         # Tauri configs (no updater plugin — updates are manual; see CHANGELOG ENG-001)
│   ├── capabilities/
│   ├── icons/
│   ├── migrations/             # schema migrations 0001, 0003–0010 (eleven tables + publish metadata)
│   │   ├── 0001_init.sql
│   │   ├── 0003_settings.sql
│   │   ├── 0004_source_tier.sql
│   │   ├── 0005_daily_scans.sql
│   │   ├── 0006_daily_scan_lead_source_nullable.sql
│   │   ├── 0007_source_tier_check.sql
│   │   ├── 0008_draft_publish_gate.sql
│   │   ├── 0009_daily_scan_lead_context.sql
│   │   └── 0010_publish_runs.sql
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── tauri_cmds.rs       # Tauri IPC command handlers
│       └── core/
│           ├── mod.rs
│           ├── auth.rs         # Host-header + Origin + PIN/token middleware
│           ├── backups.rs      # SQLite backup/restore
│           ├── compiler.rs     # Markdown -> flat HTML compiler
│           ├── db.rs           # Schema + CRUD
│           ├── detectors.rs    # 8 regex detectors
│           ├── discovery.rs    # Feed-discovery helpers
│           ├── guardrails.rs   # Pre-publish keyword checks
│           ├── llm.rs          # Ollama HTTP client
│           ├── migrations.rs   # Migration runner
│           ├── scraper.rs      # RSS / HTML feed parser
│           ├── server.rs       # Axum loopback server
│           └── tests.rs        # Backend tests
├── templates/                  # Static-site templates (read by compiler.rs)
│   ├── index.html
│   ├── post.html
│   ├── styles.css
│   └── print.css
├── scripts/                    # Release & build tools
│   ├── policy/                 # legacy self-grading promotion pipeline (retired — see CHANGELOG 0.2.6 Note)
│   └── audit/                  # Audit verification tools
├── assistant-skill/            # SKILL.md + client.js for CLI/IDE integrations
└── docs/
    ├── architecture.md
    ├── api.md                  # Loopback HTTP API route reference
    ├── user_manual.md
    ├── install.md              # Download / checksum / install steps
    ├── manual-smoke.md         # Manual smoke-test checklist
    ├── discussion_seeds.md
    ├── index.html              # GitHub Pages landing
    ├── script.js
    ├── style.css
    └── spec/
        └── v0.2-phase-4.md     # Canonical phase-4 spec
```

## Building from source

You can build the application locally for development or packaging.

**Prerequisites (all OSes):**
- Rust toolchain — install via [rustup.rs](https://rustup.rs/).
- Node.js 18+ and npm — [nodejs.org](https://nodejs.org/).
- Ollama — no separate Ollama install is needed for normal use. For developers building from source, the build bundles Ollama as a sidecar binary that is **not** committed to the repo: a fetch script downloads it and verifies it against a pinned SHA256 (see the required fetch step below), and you must run that script before building.
- A bash shell — the fetch script (`scripts/fetch-ollama-binaries.sh`) is bash-only and calls `python`/`python3`. On Windows, run it from Git Bash or WSL (it will not run in PowerShell or `cmd`).

**Platform prerequisites for Tauri v2:**
- **Windows**: Microsoft Edge WebView2, plus the C++ Build Tools (`Desktop development with C++` workload).
- **macOS**: Xcode Command Line Tools (`xcode-select --install`).
- **Linux**: WebKitGTK and dev dependencies. On Debian/Ubuntu:
  ```bash
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```
  Other distros: see the [Tauri prereqs guide](https://v2.tauri.app/start/prerequisites/).

**Build steps:**
```bash
git clone https://github.com/scottconverse/CivicNewspaper.git
cd CivicNewspaper
npm install

# REQUIRED before any dev/build run: fetch + SHA-verify the Ollama sidecar binary.
# This downloads the pinned Ollama release into src-tauri/binaries/ and aborts on a
# checksum mismatch. The binary is not committed, so the build fails without this step
# ("external-bin not found: binaries/ollama-..."). Bash-only — use Git Bash or WSL on Windows.
bash scripts/fetch-ollama-binaries.sh

npm run tauri dev        # dev mode with hot reload
# or
npm run tauri build      # produces a platform installer in src-tauri/target/release/bundle/
```

If `npm run tauri` errors with "tauri: command not found", install the Tauri CLI as a dev dependency: `npm install --save-dev @tauri-apps/cli`.

## Status, license, contributing

- **Status:** public beta (unsigned). The eight detectors and the guardrails check are usable but unsophisticated.
- **License:** MIT. See [LICENSE](LICENSE).
- **Contributing:** see [CONTRIBUTING.md](CONTRIBUTING.md). The detector regexes in `detectors.rs` are an easy, valuable place to start — every municipality uses slightly different boilerplate, and broader regex coverage directly improves the tool.
- **Security:** see [SECURITY.md](SECURITY.md). The app opens a localhost HTTP server; please report any issues that bypass the host-header / origin / bearer-token checks.
- **Changes:** see [CHANGELOG.md](CHANGELOG.md).

## Further reading

- [docs/user_manual.md](docs/user_manual.md) — for non-technical editors.
- [docs/architecture.md](docs/architecture.md) — for developers and reviewers.
- [docs/api.md](docs/api.md) — loopback HTTP API route reference (for building a second client).
- [docs/discussion_seeds.md](docs/discussion_seeds.md) — templates for GitHub Discussions.
- [FAQ.md](FAQ.md).

## Status and known TODOs

**Status:**
- GitHub Actions / CI is configured under `.github/workflows/ci.yml` for continuous testing on Linux, macOS, and Windows.

**Known TODOs:**
- No signed installers; macOS Gatekeeper and Windows SmartScreen will warn users.
