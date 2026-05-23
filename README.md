# CivicNewspaper

> Pre-alpha. Not production software. No security review. APIs and database schema may break without notice.

A local-first, single-editor desktop app for monitoring municipal feeds, flagging public-record signals, drafting reports with a local LLM, and compiling a static HTML newsroom. Built on Tauri v2, React, SQLite, and Ollama. Runs entirely on your computer — there is no cloud component.

## What this is, and what it isn't

**What it is, today:**
- A Tauri v2 desktop app with a single-page React UI (`src/App.tsx`).
- A Rust core (`src-tauri/src/core/`) that:
  - Scrapes RSS/HTML feeds (`scraper.rs`).
  - Stores everything in a local SQLite database in WAL mode (`db.rs`).
  - Runs **eight hand-written regex detectors** against scraped text — for money amounts, vote/decision keywords, personnel-change keywords, meeting/deadline keywords, watchlist hits, and a "source went quiet" timer (`detectors.rs`). This is not NLP. It is regular expressions in a loop.
  - Runs **keyword-based pre-publication checks** on drafts — looks for a hard-coded list of accusatory terms, looks for the literal substring `evidence:` in each paragraph, requires presumption-of-innocence modifiers near arrest-related words (`guardrails.rs`). This is a lint rule, not an editor.
  - Calls a local Ollama instance (`llm.rs`) for draft generation. Output quality is whatever your local model produces.
  - Compiles approved drafts into a flat HTML site using `pulldown-cmark` and four templates in `templates/` (`compiler.rs`).
  - Exposes a localhost-only Axum HTTP server on `127.0.0.1:12053` for browser-extension and assistant-skill pairing (`server.rs`, `auth.rs`).

**What it isn't:**
- A finished product. There are no signed installers and no GitHub releases yet.
- An NLP system. The "detectors" cannot resolve composite events, named entities, or numeric context. They match keywords.
- A multi-user newsroom. It is single-editor, single-machine.
- A polished publishing host. The "wizard" for GitHub Pages / Netlify / Vercel is a button that opens your output folder in Explorer/Finder so you can drag-and-drop it into your hosting provider's web UI.


## Architecture (one paragraph)

A Tauri-wrapped React frontend talks to a Rust backend via Tauri IPC. The Rust backend also runs an Axum HTTP server bound strictly to `127.0.0.1:12053` so that browser extensions and IDE-side assistant skills can pair (via short-lived 22-char token) and exchange bearer tokens. All persistent state lives in a single SQLite file (WAL mode). Draft generation routes to a local Ollama instance at `127.0.0.1:11434`. The static-site compiler reads approved drafts from SQLite and writes a folder of HTML + CSS + RSS to a user-chosen output path.

For details: [docs/architecture.md](docs/architecture.md).

## Project structure (verified)

```
.
├── README.md
├── package.json                # Vite + React frontend
├── tsconfig.json
├── vite.config.ts
├── public/                     # Vite public assets
├── src/                        # React frontend — single file today
│   ├── App.tsx                 # 1,918-line single-page UI
│   ├── App.css
│   ├── ipc.ts                  # Tauri command bindings
│   ├── main.tsx
│   ├── vite-env.d.ts
│   └── assets/
├── src-tauri/                  # Tauri Rust backend
│   ├── Cargo.toml              # NOTE: name still "tauri-app", authors ["you"] — TODO rebrand
│   ├── build.rs
│   ├── tauri.conf.json
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
├── browser-extension/
│   ├── chromium/               # Manifest v3 extension (background.js, content.js, manifest.json, icon.png)

├── assistant-skill/            # SKILL.md + skill.json + client.js for AI editors
└── docs/
    ├── architecture.md
    ├── user_manual.md
    ├── discussion_seeds.md
    ├── index.html              # GitHub Pages landing
    ├── script.js
    └── style.css
```

## Building from source

There are no prebuilt installers. You must build locally.

**Prerequisites (all OSes):**
- Rust toolchain — install via [rustup.rs](https://rustup.rs/).
- Node.js 18+ and npm — [nodejs.org](https://nodejs.org/).
- Ollama running locally — [ollama.com](https://ollama.com/). Pull at least one model: `ollama pull gemma2:9b` (or smaller).

**Platform prerequisites for Tauri v2:**
- **Windows**: Microsoft Edge WebView2 (preinstalled on Windows 11; installer on Windows 10), plus the C++ Build Tools (`Desktop development with C++` workload).
- **macOS**: Xcode Command Line Tools (`xcode-select --install`).
- **Linux**: WebKitGTK and a small graph of dev libraries. On Debian/Ubuntu:
  ```
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

- **Status:** pre-alpha. The eight detectors and the guardrails check are usable but unsophisticated. No release has been cut.
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

- `src-tauri/Cargo.toml`: package is still named `tauri-app`, description `"A Tauri App"`, authors `["you"]`. Rebrand before any release.
- `package.json`: `"name": "tauri-app"`. Same.
- No GitHub Actions / CI configured.
- No signed installers; macOS Gatekeeper and Windows SmartScreen will warn users.
