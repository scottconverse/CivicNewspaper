# CivicNewspaper FAQ

CivicNewspaper is the repository. The installed app is The Civic Desk.

This FAQ is intentionally plain about what works, what is still beta, and what the software must not decide for you.

## Project Status

### Is this production software?

Not yet. The Civic Desk is a public beta. It has working source intake, local AI paths, editorial workflow, static-site generation, share-package export, and publishing connectors, but the installers are unsigned and stable-grade cross-platform clean-machine proof is still in progress.

Use it with informed caution. A human editor remains responsible for every published word.

### Is this only for nonprofits or public-records-only publications?

No. The app should support different publisher identities: single-person projects, private organizations, nonprofits, for-profit publishers, and other local publishing models.

The software should not invent your business model, claim you run no ads, claim every story is public-record-backed, or publish AI disclosures you did not choose. Those are publisher/editorial decisions.

### Can the app block a story?

No. The app can warn, rank, label, suggest verification tasks, and show advisor output. It must not veto the editor.

## Local AI

### Why local AI?

Local AI keeps drafts and newsroom context on the user's machine by default and avoids API bills for routine drafting and summarization.

The tradeoff is real: local models can be slower and weaker than frontier cloud models. They can hallucinate. Treat AI output as assistance, not as fact.

### What hardware do I need?

The setup flow should inspect the machine and recommend a model. A rough guide:

- 8 GB RAM or more: `phi4-mini:latest` is the current conservative default because the latest local bakeoff showed valid structured output on both civic-signal and empty/noise cases.
- Other installed models, including `qwen2.5:7b`, `gemma4:e4b`, and `llama3.2:3b`, remain useful comparison options, but the app should not silently switch you to one without showing the choice.

Model downloads are large. The app should explain size, expected time, and degraded-mode behavior before downloading.

### What happens if Ollama or the model is missing?

The app should degrade gracefully. Source fetching, imports, editing, backup, export, and publishing should remain usable. AI-assisted summarization, drafting, and advisor features may be unavailable until the runtime/model is installed.

### Can the AI provide legal review?

It can provide a press-freedom and legal-risk advisory pass. It can look for likely risk areas such as attribution gaps, public/private figure concerns, privacy issues, defamation risk, missing verification, and fair-report questions.

It is not a lawyer. It is not legal advice. It should create warnings and verification tasks, not block publication.

## Sources And Discovery

### What sources can I add?

You can add official sources, feeds, agenda pages, local media, document pages, and public social/community sources. Browser pairing can also send pages into the app while you read.

### Does source discovery depend on search engines?

Search should be a fallback, not the primary path. The app should first use deterministic discovery: city/county domains, feeds, sitemaps, agenda portals, known civic platforms, public-notice pages, imported seed lists, and public social/community sources. Search can still help catch what those paths miss.

### What file types can bulk import handle?

Current bulk import targets:

- CSV
- TXT
- XLSX
- DOCX
- text-readable PDF

Image-only scanned PDFs need OCR support before text or URLs can be extracted.

## Dark Signals

### What is the Dark Signal Desk?

It is a review area for early, weak, unusual, or socially surfaced signals. These can be useful because local stories often start as small irregularities, rumors, public comments, or changes in obscure records.

The desk should rank signals and explain why they may matter, but it must not hide them from the editor.

### Are dark signals published?

Not automatically. Low-confidence material should become verification work first. It can inform reporting, but it should not become published evidence without human review and adequate sourcing.

## Workbench And Guardrails

### What does the Workbench do?

The Workbench is where drafts are written, edited, held, approved, returned for more work, or prepared for publishing.

### What do guardrails check?

Guardrails can flag risk patterns such as unsupported claims, loaded terms, missing citations, presumption-of-innocence problems, or excessive copying from source text.

They are not a fact-checker. They are not a legal decision. They warn the editor.

## Publishing

### What does the app publish?

The app compiles a static issue package with:

- homepage
- article pages
- RSS feed
- about, ethics/reporting, and corrections pages
- ZIP export
- newsletter markdown
- Substack-ready markdown
- social/community share copy

### Where can I publish?

Supported or assisted paths include:

- here.now anonymous preview for the live-verified public-beta publishing path
- GitHub Pages for durable public archive publishing when you connect and verify your own repo
- Cloudflare Pages
- Netlify
- WordPress
- Substack-assisted copy/paste workflow
- Other host/manual URL recording

GitHub Pages, Cloudflare Pages, Netlify, WordPress, and permanent here.now publishing require user-owned accounts/credentials and release-specific live proof before treating them as stable publishing paths.

### Is Substack automatic?

No. Substack does not currently provide a stable public publishing API for third-party post creation. The app generates Substack-ready copy, opens the editor workflow, and lets the user record the final public URL after posting.

## Data And Privacy

### Where is my data stored?

The app uses a local SQLite database in the operating system's app-data directory. The exact path depends on platform and app identifier.

### Does the app phone home?

There is no automatic update service in the app. Network access happens when the user:

- fetches configured sources
- runs discovery/search
- opens web pages
- downloads local AI runtime/model assets
- publishes to an external provider
- uses provider test-connection or connector features

Your local database and drafts are not uploaded unless you choose an export, share, or publishing workflow that sends them somewhere.

## Backups

### How do I back up?

Use the Ethics & Backups/System areas in the app to create local backups. Store backups somewhere you control, such as an external drive or private cloud folder.

### Can I sync two editors at the same time?

No. The current app is a single-machine desktop tool, not a multi-user server. Do not put the live SQLite database into a shared folder and edit from two machines at once.
