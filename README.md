# CivicNewspaper / The Civic Desk

> **Project name:** CivicNewspaper. **Installed app name:** The Civic Desk.
>
> **Current source/tag:** v0.3.0 public beta. Published GitHub Release installers may lag the source tag. Installers are unsigned. This is not a stable production release.

The Civic Desk is a local-first desktop newsroom tool for small local publishers, civic reporters, and community editors. It helps one person or a small team monitor public local sources, discover story leads, draft articles with a local AI model, review risks and evidence, and publish a static local-news issue.

The app is built for the reality that many towns no longer have enough reporters doing daily civic coverage. It does not replace editorial judgment. It surfaces leads, warnings, verification paths, and publishing packages; the human editor decides what to investigate, edit, hold, kill, approve, or publish.

## What It Does Today

- Runs as a Tauri desktop app with a React frontend and Rust backend.
- Stores sources, evidence, leads, drafts, settings, subscribers, publish history, civic entities, dark signals, and verification tasks in a local SQLite database.
- Bundles an Ollama sidecar for local AI. First-run setup selects a model based on machine capability and guides the user through model download.
- Watches official records, agenda pages, public notices, local media, and public community/social sources.
- Imports source lists from CSV, TXT, XLSX, DOCX, and text-backed PDF files. Image-only PDFs currently receive readable-text/OCR guidance rather than silent failure.
- Runs Daily Scan from watched sources, deterministic detectors, source diffs, civic entities, dark signals, verification tasks, and optional local-model summarization/ranking.
- Provides a Story Queue, Dark Signals feed, Verification Queue, and Workbench.
- Lets editors generate drafts, rewrite in plain language, run an optional press-freedom/legal-risk advisor, apply guardrail warnings, and approve stories for publication.
- Never uses software guardrails or AI advice as a publish veto. Warnings and advisor notes are for the editor.
- Compiles approved drafts into a static website with article pages, homepage, RSS, about/ethics/how-we-report/corrections pages, print CSS, newsletter markdown, Substack-ready markdown, social/community post copy, short-link copy, manifest, and ZIP.
- Publishes through here.now, GitHub Pages, Cloudflare Pages, Netlify, WordPress, or assisted/manual URL recording. Substack is assisted: the app prepares copy, then the editor publishes in Substack and records the URL.
- Includes a browser-extension pairing workflow for clipping pages into the local desk.

## What It Is Not Yet

- Not a signed stable release. Windows SmartScreen and macOS Gatekeeper warnings are expected.
- Not a multi-user newsroom server.
- Not legal advice. The press-freedom/legal-risk advisor is a risk-spotting tool, not a lawyer.
- Not a replacement for reporting. Dark signals and community/social leads need verification before publication.
- Not a guarantee of complete source discovery. Search engines and public websites change; the editor must review discovered sources.

## Download

Download installers from the [GitHub Releases page](https://github.com/scottconverse/CivicNewspaper/releases). Use the newest release that actually includes an installer for your platform.

- **Windows:** use the `.exe` or `.msi` installer when present. Because the installer is unsigned, choose **More info** then **Run anyway** when SmartScreen appears.
- **macOS:** use the `.dmg` release artifact when present. Because signing/notarization is incomplete, right-click the app and choose **Open**, or use **System Settings > Privacy & Security > Open Anyway**.
- **Linux:** use the `.deb` package when present. Linux packaging is currently Debian/Ubuntu oriented.

See [docs/install.md](docs/install.md) for checksum verification and OS-specific details.

## First Run

1. Enter publication identity: publication name, editor name, organization type, city, and state.
2. Let the app check the machine and local AI runtime.
3. Download the recommended local model. The current default path favors `qwen2.5:7b` on ordinary 8 GB+ machines, with `llama3.2:3b` as a lighter fallback.
4. Add or discover sources for your city.
5. Run Daily Scan, review leads, generate drafts, approve stories, compile the issue, and publish.

The app can use the internet for source fetching, model download, and publishing. Drafting and review happen locally once the model and source material are available.

## Core Workflow

```text
Sources -> Fetch/Scrape -> Evidence -> Detectors/Entities/Diffs
        -> Daily Scan Leads -> Story Queue -> Workbench
        -> Human Review -> Static Issue -> ZIP / Website / Share Package
```

## Repository Layout

```text
README.md
docs/
  prd-local-llm-newsroom-v1.md
  implementation-plan-v0.2.9-to-v1.0.0.md
  install.md
  user_manual.md
  architecture.md
  publishing-connectors.md
  release-readiness.md
  discussion_seeds.md
  index.html
src/
  React/TypeScript frontend
src-tauri/
  Rust backend, migrations, prompts, Tauri config
templates/
  Static website templates used by the compiler
browser-extension/chromium/
  Browser clipping extension
scripts/
  Release, smoke-test, and build helpers
```

## Build From Source

Prerequisites:

- Node.js 18+
- Rust stable toolchain
- Platform prerequisites for Tauri v2
- Bash for the sidecar-fetch script

```bash
git clone https://github.com/scottconverse/CivicNewspaper.git
cd CivicNewspaper
npm install
bash scripts/fetch-ollama-binaries.sh
npm run tauri dev
```

Build installers:

```bash
npm run tauri build
```

The sidecar binary is not committed to the repo. The fetch script downloads the pinned Ollama binary and verifies its SHA256 before local builds.

## Current Documentation

- [V1 PRD: local LLM newsroom](docs/prd-local-llm-newsroom-v1.md) - product requirements for the local-intelligence newsroom roadmap.
- [Implementation plan: v0.2.9 to v1.0.0](docs/implementation-plan-v0.2.9-to-v1.0.0.md) - phased roadmap with technology integrations and release gates.
- [User manual](docs/user_manual.md) - plain-English operator guide plus technical appendix.
- [Architecture](docs/architecture.md) - system design, schema, security, publishing, and AI boundaries.
- [Install guide](docs/install.md) - unsigned installer and checksum instructions.
- [Publishing connectors](docs/publishing-connectors.md) - supported publishing paths.
- [Release readiness](docs/release-readiness.md) - beta, release-candidate, and stable gates.
- [Discussion seeds](docs/discussion_seeds.md) - launch posts for GitHub Discussions or community forums.

## Release Status

v0.3.0 is public beta source. Cleanroom testing has proven a full Longmont issue can be generated, exported, zipped, and anonymously published to here.now on Windows. This release improves story-quality metadata, recurring-topic memory, lead novelty warnings, and editor workflow controls, but stable release still requires signed installers, cross-platform clean-machine proof, matching published release artifacts, and credentialed live verification for external publishing providers.

## License

MIT. See [LICENSE](LICENSE).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Useful contribution areas include source discovery adapters, import fixtures, accessibility checks, publishing connector hardening, release smoke tests, and better local model evaluation.
