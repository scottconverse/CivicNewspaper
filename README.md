# CivicNewspaper / The Civic Desk

> **Project name:** CivicNewspaper. **Installed app name:** The Civic Desk.
>
> **Current release line:** v0.3.2 Windows-only public beta. Windows release candidates are Authenticode-signed, timestamped, and fail the release workflow unless the installer, installed application, and uninstaller all validate. The GitHub release asset has not yet been replaced with the current candidate. This is not a stable production release.

The Civic Desk is a local-first desktop newsroom tool for small local publishers, civic reporters, and community editors. It helps one person or a small team monitor public local sources, discover story leads, draft articles with a local AI model, review risks and evidence, and publish a static local-news issue.

The app is built for the reality that many towns no longer have enough reporters doing daily civic coverage. It does not replace editorial judgment. It surfaces leads, warnings, verification paths, and publishing packages; the human editor decides what to investigate, edit, hold, cut, approve, or publish.

## What It Does Today

- Runs as a Tauri desktop app with a React frontend and Rust backend.
- Stores sources, evidence, leads, drafts, settings, subscribers, publish history, civic entities, dark signals, and verification tasks in a local SQLite database.
- On the Windows public-beta path, manages a local Ollama runtime for local AI. First-run setup checks the machine, can install the pinned Windows runtime when needed, and currently guides the user toward the public-beta default `phi4-mini:latest` model.
- Watches official records, agenda pages, public notices, local media, and public community/social sources.
- Imports source lists from CSV, TXT, XLSX, and DOCX files. PDF source-list import is disabled in this public beta until hardened PDF parsing is available; convert PDFs to TXT/CSV/DOCX/XLSX or paste the URLs directly.
- Runs Daily Scan from watched sources, deterministic detectors, source diffs, civic entities, dark signals, verification tasks, and optional local-model summarization/ranking.
- Provides a Story Queue, Dark Signals feed, Verification Queue, and Workbench.
- Lets editors generate drafts, rewrite in plain language, run an optional press-freedom/legal-risk advisor, apply guardrail warnings, and approve stories for publication.
- Never uses software guardrails or AI advice as a publish veto. Warnings and advisor notes are for the editor.
- Compiles approved drafts into a static website with article pages, homepage, RSS, about/ethics/how-we-report/corrections pages, print CSS, newsletter markdown, Substack-ready markdown, social/community post copy, short-link copy, manifest, and ZIP.
- Publishes through here.now, GitHub Pages, Netlify, or assisted/manual URL recording. Anonymous here.now preview is the live-verified public-beta path. GitHub Pages, Netlify, permanent here.now publishing, and assisted/manual URL recording require your own accounts/credentials or release-specific proof before treating them as stable publishing paths. WordPress direct API publishing is disabled in this beta until draft-first publishing, rollback, and live connector proof are complete; export the ZIP/static folder or record a manually published URL instead. Cloudflare Pages is assisted/manual in this beta: export the folder or ZIP, deploy in Cloudflare, then record the public URL. Substack is assisted: the app prepares copy, then the editor publishes in Substack and records the URL.
- Includes a browser-extension pairing workflow for clipping pages into the local desk.

## What It Is Not Yet

- Public-beta release; installer signing is verified as part of the release artifact gate.
- Not a multi-user newsroom server.
- Not legal advice. The press-freedom/legal-risk advisor is a risk-spotting tool, not a lawyer.
- Not a replacement for reporting. Dark signals and community/social leads need verification before publication.
- Not a guarantee of complete source discovery. Search engines and public websites change; the editor must review discovered sources.

## Download

The [v0.3.2 release page](https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2) still serves the earlier asset. **Do not install it as the signed candidate.** Wait until the release page identifies the replacement candidate, then verify `The.Civic.Desk_0.3.2_x64-setup.exe` against `SHA256SUMS` and confirm Authenticode reports `Status: Valid` with a signer containing `Scott Converse` before opening it.

- **Windows:** after the signed replacement is published, use `The.Civic.Desk_0.3.2_x64-setup.exe` only when both its published checksum and signer identity pass the checks above.
- **macOS and Linux:** backlog/proof-needed for this release line. Do not treat macOS or Linux download cards, package configs, or historical artifacts as supported public-beta installers until a clean-machine proof is recorded.

Initial installation requires an internet connection on Windows systems where Microsoft Edge WebView2 Runtime is not already installed. First-run local AI setup also needs internet access to download the app-managed Ollama runtime and selected model. After those components and source material are available, drafting and review run locally.

See [docs/install.md](docs/install.md) for checksum verification and OS-specific details.

## First Run

1. Enter publication identity: publication name, editor name, organization type, city, and state.
2. Let the app check the machine and local AI runtime.
3. Download the recommended local model, or skip/defer local AI setup and continue with source review, manual editing, static export, ZIP review, and here.now preview publishing. The current default path favors `phi4-mini:latest` because the latest local bakeoff showed it produced valid JSON for both real civic signals and empty/noise input.
4. Add or discover sources for your city.
5. Run Daily Scan, review leads, generate drafts, approve stories, compile the issue, and publish.

The app can use the internet for installer prerequisites, source fetching, model download, and publishing. Drafting and review happen locally once the model and source material are available.
For the editor desk workflow, see the manual's Workbench section: improve weak drafts, send stories back with assignment notes, hold stories with reasons, cut stories from an issue, then approve only the exact saved draft you reviewed.

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
  implementation-plan-v0.3.0-to-v1.0.0.md
  install.md
  troubleshooting.md
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
- An internet connection for model download. The app-managed runtime download is Windows-only in this public-beta line; other platforms need an existing/manual Ollama install until platform proof exists.

```bash
git clone https://github.com/scottconverse/CivicNewspaper.git
cd CivicNewspaper
npm install
npm run tauri dev
```

Build installers:

```bash
npm run tauri build
```

The Ollama runtime binary is not committed to the repo. The app-managed first-run installer downloads the pinned Windows runtime and verifies its SHA256 before use. The legacy `scripts/fetch-ollama-binaries.sh` helper is not part of the v0.3.x release verifier.

## Current Documentation

- [V1 PRD: local LLM newsroom](docs/prd-local-llm-newsroom-v1.md) - product requirements for the local-intelligence newsroom roadmap.
- [Implementation plan: v0.3.0 to v1.0.0](docs/implementation-plan-v0.3.0-to-v1.0.0.md) - phased roadmap with technology integrations and release gates.
- [User manual](docs/user_manual.md) - plain-English operator guide plus technical appendix.
- [Architecture](docs/architecture.md) - system design, schema, security, publishing, and AI boundaries.
- [Install guide](docs/install.md) - official installer and checksum instructions.
- [Troubleshooting](docs/troubleshooting.md) - installer provenance, model download, local AI, weak output, here.now preview, ZIP, and import help.
- [Publishing connectors](docs/publishing-connectors.md) - supported publishing paths.
- [Browser Bridge extension](browser-extension/chromium/README.md) - install and pair the Chromium extension.
- [Authenticated localhost API](docs/api.md) - loopback-only routes, headers, authentication, and response contract for the Browser Bridge and local tools.
- [Release readiness](docs/release-readiness.md) - beta, release-candidate, and stable gates.
- [Discussion seeds](docs/discussion_seeds.md) - launch posts for GitHub Discussions or community forums.

## Release Status

v0.3.2 is a Windows public beta. The Windows release workflow Authenticode-signs and timestamps the installer, installed application, and uninstaller, then fails closed if any signature check fails. The downloadable GitHub asset has not yet been replaced with the candidate currently under evaluation, so the repository does not present an older installer hash as current proof. The [local isolated-package report](docs/release-evidence/v0.3.2-local-isolated-package-report.md) and [hosted evidence JSON](docs/release-evidence/v0.3.2.json) are preserved as historical evidence for an earlier candidate. Final beta publication requires one exact identity chain across the merged commit, signed installer, cleanroom report, checksum manifest, and hosted release asset. Stable release additionally requires cross-platform clean-machine proof for every advertised platform and credentialed live verification for external publishing providers.

## Backlog: Mac And Linux Installer Proof

Mac and Linux installer work is intentionally out of the v0.3.2 Windows public-beta release candidate. Before the public docs can advertise those platforms, the project needs real build artifacts, first-run local-AI setup proof, clean-machine install notes, and platform-specific signing/notarization guidance for each OS.

## License

MIT. See [LICENSE](LICENSE).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Useful contribution areas include source discovery adapters, import fixtures, accessibility checks, publishing connector hardening, release smoke tests, and better local model evaluation.
