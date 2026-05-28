# CivicNewspaper

> Pre-alpha. Not production software. No security review. APIs and database schema may break without notice.

CivicNewspaper is a local-first, privacy-focused desktop application designed for local journalists, newsroom operators, and community organizers who want to monitor public municipal feeds without relying on complex, cloud-based software. It automatically scans local RSS feeds and websites for public records and meeting notices, identifies important civic signals, and helps draft plain-language summaries using a private, offline language model running directly on your computer. By storing all database records and drafts locally, CivicNewspaper provides a secure, independent toolkit to help you track municipal actions and compile a static website ready for publishing.

## Download

You can download the latest pre-compiled installers for your platform from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).

### Installation Instructions
* **Windows**: Download the `.msi` or `.exe` installer. When launching, Windows SmartScreen will display a warning because the installer is currently unsigned. Click **"More info"** and then **"Run anyway"** to proceed.
* **macOS**: Download the `.dmg` or `.app` file. Because the app is not signed with an Apple developer certificate, macOS Gatekeeper will block execution. Right-click the application icon, select **Open**, and then confirm the prompt. Alternatively, go to **System Settings > Privacy & Security** and scroll down to click **"Open Anyway"** for CivicNewspaper.
* **Linux**: Download the `.deb` package (for Debian/Ubuntu) or the `.AppImage`. Install the `.deb` via your package manager (e.g., `sudo dpkg -i civicnewspaper_*.deb`) or make the `.AppImage` executable (`chmod +x CivicNewspaper.AppImage`) and run it.

## First Run

When you open CivicNewspaper for the first time, you will be guided through an onboarding wizard. 

1. **Identity**: Enter your Publication Name, Editor Name, City, and State.
2. **AI Service Setup**: The wizard verifies connection to the bundled offline AI sidecar engine.
3. **Download AI Model**: Pull the recommended AI model (such as `gemma2:9b`, `llama3:8b`, or `phi3:mini` depending on your computer's RAM) for offline draft generation.
4. **Defaults**: Configure your Publish Path (where static sites are compiled) and Backup Path (for database backups).
5. **Done**: Complete the setup to enter your workspace.

No accounts or internet connections are required after this initial setup.

## What this is, and what it isn't

**What it is, today:**
- A Tauri v2 desktop app with a React 19 frontend structured as modular components (`src/components/`) and state management (`src/useApp.ts`).
- A Rust core (`src-tauri/src/core/`) that:
  - Scrapes RSS/HTML feeds (`scraper.rs`).
  - Stores everything in a local SQLite database in WAL mode (`db.rs`).
  - Runs **eight hand-written regex detectors** against scraped text — for money amounts, vote/decision keywords, personnel-change keywords, meeting/deadline keywords, watchlist hits, and a "source went quiet" timer (`detectors.rs`). This is regular expressions in a loop.
  - Runs **pre-publication checks** on drafts — alerts on accusatory terms, checks for the literal substring `evidence:` in paragraphs, and flags missing presumption-of-innocence modifiers near arrest-related words (`guardrails.rs`). Note: this is a lint-like helper in the UI, not a compilation block.
  - Calls a local Ollama instance (`llm.rs`) for draft generation. Output quality depends on your local model configuration.
  - Compiles approved drafts into a flat HTML site using `pulldown-cmark` and templates in `templates/` (`compiler.rs`).
  - Exposes a localhost-only Axum HTTP server on `127.0.0.1:12053` for browser-extension and assistant-skill pairing (`server.rs`, `auth.rs`).

**What it isn't:**
- A finished product. Installers are currently unsigned.
- An NLP system. The "detectors" match keywords rather than named entities or numeric contexts.
- A multi-user newsroom. It is single-editor, single-machine.
- A polished publishing host. The "wizard" compiles the site to a folder on your computer, allowing you to drag-and-drop it into hosting providers like Netlify or GitHub Pages.

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
│   ├── tauri.conf.json         # Tauri configs, includes Updater plugin (dormant — see FAQ)
│   ├── capabilities/
│   ├── icons/
│   ├── migrations/
│   │   └── 0001_init.sql
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
├── scripts/                    # Release & policy tools
│   ├── policy/                 # Hard-fail by construction auto-promote scripts
│   └── audit/                  # Audit verification tools
├── assistant-skill/            # SKILL.md + client.js for CLI/IDE integrations
└── docs/
    ├── architecture.md
    ├── user_manual.md
    ├── discussion_seeds.md
    ├── index.html              # GitHub Pages landing
    ├── script.js
    └── style.css
```

## Building from source

You can build the application locally for development or packaging.

**Prerequisites (all OSes):**
- Rust toolchain — install via [rustup.rs](https://rustup.rs/).
- Node.js 18+ and npm — [nodejs.org](https://nodejs.org/).
- Ollama — No separate installation is required; Ollama is bundled as a sidecar inside the application.

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
npm run tauri dev        # dev mode with hot reload
# or
npm run tauri build      # produces a platform installer in src-tauri/target/release/bundle/
```

If `npm run tauri` errors with "tauri: command not found", install the Tauri CLI as a dev dependency: `npm install --save-dev @tauri-apps/cli`.

## Status, license, contributing

- **Status:** pre-alpha. The eight detectors and the guardrails check are usable but unsophisticated.
- **License:** MIT. See [LICENSE](LICENSE).
- **Contributing:** see [CONTRIBUTING.md](CONTRIBUTING.md). The detector regexes in `detectors.rs` are an easy, valuable place to start — every municipality uses slightly different boilerplate, and broader regex coverage directly improves the tool.
- **Security:** see [SECURITY.md](SECURITY.md). The app opens a localhost HTTP server; please report any issues that bypass the host-header / origin / bearer-token checks.
- **Changes:** see [CHANGELOG.md](CHANGELOG.md).

## Further reading

- [docs/user_manual.md](docs/user_manual.md) — for non-technical editors.
- [docs/architecture.md](docs/architecture.md) — for developers and reviewers.
- [docs/discussion_seeds.md](docs/discussion_seeds.md) — templates for GitHub Discussions.
- [FAQ.md](FAQ.md).

## Known TODOs visible in the manifest

- GitHub Actions / CI is configured under `.github/workflows/ci.yml` for continuous testing on Linux, macOS, and Windows.
- No signed installers; macOS Gatekeeper and Windows SmartScreen will warn users.
