# CivicNewspaper FAQ

Honest answers. If something here contradicts the marketing copy on the landing page, this file is right and the marketing copy hasn't been updated yet.

## Project status

### Is this production software?

No. CivicNewspaper is pre-alpha. There are no signed installers, no security review, and no formal QA. Use it for experimentation. Do not stake legal or journalistic claims on it without independent verification of every output.

### Should I use it to publish public-records reporting today?

You can — the compiler works, the citation-link convention is sound, the local-LLM workflow does produce drafts. But you are responsible for every word. The "guardrails" check is a keyword lint, not a fact-checker; the "detector engine" is a set of regular expressions that will miss things your city's clerk phrases unusually. Treat its output as a tip line, not a finished story.

## Local AI / Ollama

### Why local AI instead of ChatGPT or Claude APIs?

Three reasons:
1. **Privacy.** Local records, drafts, and watchlists never leave your device.
2. **Cost.** No API bills, no surprise rate limits.
3. **Offline.** You can draft and review without internet.

The tradeoff is real: local models in the 3B–9B range are significantly weaker than frontier models. They will hallucinate. You must verify.

### What hardware do I need?

The Onboarding wizard inspects your system memory and suggests a model. Rough guide:
- **16 GB RAM or more**: `gemma2:9b` (recommended for the standard workflow).
- **8 GB RAM**: `llama3:8b` or `qwen2.5:3b`.
- **4 GB RAM**: `qwen2.5:1.5b` or `tinyllama` — drafting quality will be noticeably worse.

CPU inference works but is slow. A modern Apple Silicon Mac or any machine with a GPU that Ollama can use will be much faster.

### How do I change models after onboarding?

Open the Settings tab. Type any Ollama model name you have already pulled (e.g. `mistral`, `llama3.2`). Save.

### The LLM hallucinated a fact that wasn't in the evidence. Whose fault is that?

Yours, if you publish it. The guardrail check does not verify factual correspondence between the draft and the evidence — it checks for the presence of citation-link syntax and the absence of certain accusatory words. Read every draft against the linked source. This is not optional.

## Setup and installation

### Where do I download the app?

Pre-compiled installers for Windows, macOS, and Linux are available on the [latest GitHub Releases page](https://github.com/scottconverse/CivicNewspaper/releases/latest). See the [docs/install.md](docs/install.md) file for step-by-step setup guides.

### Why does Windows/Mac warn me about this app?

Windows SmartScreen and macOS Gatekeeper warn you because the installers are not digitally signed with a (paid, recurring) Microsoft/Apple developer certificate. CivicNewspaper is an open-source, community-led public beta, so the installers are unsigned and these warnings are expected. You can confirm a download matches the file published on the GitHub Release page by computing its SHA256 checksum and comparing it to the `SHA256SUMS` manifest there — that verifies the file wasn't corrupted or altered in transit. Note the limit: a checksum is not a substitute for code signing and doesn't prove who built the binary; for an end-to-end-trustworthy build, build from source. For step-by-step instructions to proceed past the warnings and verify the files, see the [Installation Guide](docs/install.md).

### How much disk space do I need?

You will need:
- **Application space**: Around 330 MB for the installed application.
- **AI Model space**: Around 5.4 GB for the default `gemma2:9b` offline writing model.
- **Database space**: The SQLite database starts at less than 1 MB, but will grow depending on how many sources you monitor and how many text excerpts you scrape. Typically, a year of municipal monitoring uses under 100 MB.

Total recommended free space: **6 GB to 10 GB**.

### Does this work offline?

Yes. Once you have completed the onboarding setup and downloaded the language model (which requires an active internet connection), all text processing, signal detection, AI drafting, guardrail checking, database storage, and site compilation happen locally on your computer. You can use CivicNewspaper entirely offline. You only need the internet to scrape new online feeds or to upload your compiled site to a hosting provider.

### How do I install the Chromium browser extension?

In Chrome: `chrome://extensions/` → enable Developer Mode → "Load Unpacked" → select the `browser-extension/chromium/` directory. Then pair it from CivicNewspaper's Browser Pairing tab using the pairing token.

## Publishing

### How do I actually publish the compiled site to GitHub Pages / Netlify / Vercel?

The "Static Compilation & Publishing Wizard" in CivicNewspaper compiles the site to a folder on your computer and opens that folder in Explorer/Finder. From there:
- **GitHub Pages**: commit the folder's contents to a `gh-pages` branch (or to `docs/` on `main` and point Pages at it). Push.
- **Netlify**: drag the folder onto [Netlify Drop](https://app.netlify.com/drop).
- **Vercel**: run `vercel deploy` inside the folder, or import the folder via the dashboard.
- **Any other host**: it's just a folder of HTML/CSS/RSS. Upload via FTP/SFTP if you must.

There is no integrated upload. The "wizard" is text instructions plus an open-folder button.

### What gets published?

Whatever drafts you've moved to the "Approved for Static Publish" state (status `ready_to_publish` or `published` / `corrected`). The compiler produces:
- `index.html` (newsroom home, listing approved posts).
- One HTML page per approved post.
- `styles.css`, `print.css`.
- `feed.xml` (RSS).
- An evidence section per post with anchor links matching `[Label](evidence:123)` markdown citations.

## Detectors and guardrails

### What does "Factual Guardrail Inspector" actually check?

Four things — the first three keyword-based, the fourth sequence-based:
1. **Citation coverage**: every paragraph longer than 30 characters that isn't a heading or code block must contain the literal substring `evidence:` (e.g. `[Source](evidence:12)`).
2. **Accusatory language**: if a paragraph contains any of ~20 words (`corrupt`, `stole`, `fraud`, `embezzle`, etc.), it must also contain a citation. If it doesn't, the UI flags a warning.
3. **Presumption of innocence**: if a paragraph contains arrest-related words (`arrested`, `charged`, `indicted`, `convicted`, `prosecuted`), it must also contain a modifier like `alleged` / `allegedly` / `suspected` / `accused` nearby.
4. **Verbatim overlap**: warns when a paragraph copies a sequence of 7+ words verbatim from a linked evidence excerpt. Rewrite it in your own words or format it as a blockquote.

It is a lint rule. It is not an inspector in any AI/NLP sense.

### What does the "OSINT Detector Engine" actually do?

It runs eight regular expressions over each new evidence excerpt and creates a "lead" record when a regex matches. The eight:
- **Source went quiet** — fires if a source hasn't successfully scraped in 7+ days.
- **New primary record** — fires whenever a new doc arrives from a `primary_record` or `official_comm` source.
- **Money threshold** — finds `$NNN,NNN` patterns, parses the amount, fires if it exceeds your configured threshold (default $250,000).
- **Decision / vote** — fires on `unanimously|voted|approved|resolved|passed|carried|denied|motion|adopted|rejected`.
- **Personnel change** — fires on `appoint|resign|retire|terminate|hire|employ|vacancy|...`.
- **Public meeting scheduled** — fires on `public hearing|special meeting|...|council chamber|town hall|public meeting`.
- **Deadline** — fires on `deadline|submit by|due date|public comment period|rfp|bid due|applications close`.
- **Watchlist hit** — case-insensitive word-boundary match against your custom term list.

Lead-deduplication is exact-string match on the `why` field. Punctuation drift will produce duplicate leads.

### Why is there no NLP?

Cost (compute), latency (must run on a 4-8 GB-RAM target machine), and complexity (NLP makes regressions invisible). The team takes "boring, transparent regex" as a deliberate choice for v0.1.

## Data and backups

### Where is my data stored?

In a single SQLite file in the OS app-data directory (Windows: `%APPDATA%\org.civicnews.app\`; macOS: `~/Library/Application Support/org.civicnews.app/`; Linux: `~/.local/share/org.civicnews.app/`). Exact path depends on the `identifier` in `tauri.conf.json`.

### How do I back up?

Settings tab → "Save Backup". Pick a destination (USB drive, Dropbox folder, anywhere). The backup is a `.db` SQLite file. To restore: Settings → "Restore Backup".

### Can I sync between two machines?

No. Single-machine by design. If you put the SQLite file in Dropbox/iCloud/etc. and use only one machine at a time you can fake it, but two simultaneous edits will corrupt the database.

## Privacy

### Does the app phone home?

Outbound HTTP from the Rust backend goes to:
1. Whatever feed URLs you configured (RSS, HTML pages).
2. `127.0.0.1:11434` (your local Ollama).

There is no auto-updater. The Tauri updater plugin was removed entirely in v0.2.6 (see CHANGELOG ENG-001), so the app performs no automatic update checks and contacts no update server. To update, manually download the newer installer from the GitHub Releases page.
