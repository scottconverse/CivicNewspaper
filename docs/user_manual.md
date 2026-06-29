# The Civic Desk User Manual

This manual is written for people running a small local publication. You do not need to be a developer to use the app.

The Civic Desk helps you find leads, draft stories, review risk, and publish a local issue. It does not decide what is true, legal, fair, or newsworthy. The editor does.

## 1. Install The App

Download the latest release from:

<https://github.com/scottconverse/CivicNewspaper/releases/latest>

The installers are unsigned during public beta. That means Windows or macOS may warn you before opening the app. This is expected for an unsigned open-source beta. See [install.md](install.md) for detailed steps and checksum verification.

## 2. First-Run Setup

When you open the app for the first time, the setup wizard asks for:

- Publication name
- Editor name
- Organization type: single person, private organization, nonprofit, for-profit, or other
- City and state
- Local AI setup
- Default backup and publish folders

The app checks your computer and recommends a local AI model. On ordinary 8 GB+ machines, the current default is `qwen2.5:7b`. On smaller machines, the app may suggest a lighter model such as `llama3.2:3b`.

Model download can take a while. The setup screen should show progress so it does not look stuck. If you skip model setup, deterministic source fetching and review paths still work, but drafting and AI-assisted scan summaries are limited.

## 3. Add Sources

Open **Sources** to tell the app what to watch.

Source types include:

- **Primary record:** official agendas, minutes, ordinances, budget records, court or public-notice records.
- **Official communication:** city press releases, public information pages, agency notices.
- **News reporting:** local news sites or independent reporting.
- **Community signal:** public social/community sources such as public Reddit pages, public meeting-video pages, or public forums.

You can add sources manually, run city discovery, or bulk import files. Bulk import supports:

- CSV
- TXT
- XLSX
- DOCX
- Text-backed PDF

Image-only scanned PDFs are detected and should return guidance rather than silently pretending nothing was found. OCR support is a future improvement.

Every imported or discovered source should be reviewed before relying on it.

## 4. Daily Scan

Open **Daily Scan** and run a scan after sources are configured.

Daily Scan does several things:

1. Checks watched sources.
2. Stores new evidence.
3. Runs deterministic detectors and change checks.
4. Extracts civic entities such as people, agencies, companies, addresses, parcels, vendors, and organizations.
5. Creates observations and source performance scores.
6. Produces leads, dark signals, and verification tasks.
7. Uses the local model for targeted summarization/ranking when available.

If the local AI model is unavailable, the app should explain that clearly and continue the deterministic parts where possible.

## 5. Review The Story Queue

The **Story Queue** is where leads become stories.

From each lead, you can:

- Open linked evidence.
- Generate a draft.
- Move into the Workbench.
- Leave the lead for later.

The app may surface low-confidence or unverified leads. That is intentional. It should rank and explain them, not hide them.

## 6. Dark Signals

The **Dark Signals** tab is for early, messy civic signals that may matter but are not ready to publish.

Examples:

- Public discussion about a possible land deal.
- A recurring complaint pattern.
- A new entity appearing across documents.
- A public social/community post that points toward a verifiable issue.

Dark signals are for editor review. They are not automatically publishable evidence. The app should show:

- Why it might matter
- Origin
- Risk level
- Related entities
- Verification path
- Publication status

The system should never hide a signal from the editor solely because it is messy. It should rank it and explain why.

## 7. Verification Queue

The **Verification Queue** turns leads and dark signals into reporting tasks.

Tasks can be:

- Suggested
- Auto-checked
- Needs human
- Blocked
- Resolved

Use this queue to decide what can be checked quickly and what requires calls, records requests, meeting review, or human reporting.

## 8. Workbench

The **Workbench** is the editor.

You can:

- Generate a draft from a lead.
- Write or edit manually.
- Choose article format: brief, watch, explainer, investigation, opinion, or custom.
- Link source evidence.
- Run a plain-language rewrite.
- Run the optional press-freedom/legal-risk advisor.
- Put a story on hold.
- Send it back for more work.
- Kill it.
- Approve it for publishing.

Approval requires a human attestation. The app records that a person reviewed and accepted responsibility for publishing.

## 9. Guardrails And Advisor

The app has two kinds of review help:

### Story Guardrails

Guardrails warn about issues such as:

- Unsupported factual claims
- Accusatory wording
- Legal/charge language without careful attribution
- Long verbatim overlap with source text

Words and high-concern terms are configurable in **Ethics & Backups**.

Guardrails do not veto publication. High-concern terms may ask for an editor note, but the editor decides.

### Press-Freedom / Legal-Risk Advisor

The advisor is optional and invoked from the Workbench. It can ask the local AI model for:

- Risk notes
- Verification paths
- Public/private figure considerations
- Defamation/privacy/prior-restraint style issue spotting
- Wording options
- Questions to resolve before publication

It is not a lawyer and not a publish/kill decision. It is a newsroom risk memo.

## 10. Publishing

Publishing starts from approved drafts.

Open **Publishing** and follow the flow:

1. **Compile** the static issue into a folder.
2. **Preview** and inspect the output.
3. **Export** the ZIP and share files.
4. **Publish** through a connector or record a manual URL.
5. **Share** using generated newsletter/social/community copy.

The generated package includes:

- `index.html`
- Article pages
- `feed.xml`
- About, ethics, how-we-report, and corrections pages
- `site-package.zip`
- `publish-manifest.json`
- `newsletter.md`
- `substack.md`
- `share-package.md`
- Facebook, subreddit, Nextdoor, and short-link copy

Supported publishing paths:

- **here.now:** recommended default. Anonymous preview publishing works without an account and expires after about 24 hours. Account/API-key publishing can be permanent.
- **GitHub Pages:** durable public archive in a repository.
- **Cloudflare Pages:** technical-user connector.
- **Netlify:** technical-user connector.
- **WordPress:** creates an issue page and article pages through the WordPress REST API.
- **Substack:** assisted. The app prepares copy; you paste into Substack and record the public URL.
- **Other/manual:** record an existing public URL and update share artifacts.

Connector secrets are stored in the operating system credential store. The SQLite database stores non-secret connector metadata.

## 11. Browser Extension Pairing

The browser extension lets you send public pages into the app while you read.

Pairing works only on your computer:

1. Open **Browser Pairing**.
2. Generate a pairing token.
3. Paste it into the browser extension.
4. Confirm the paired device appears in the app.

The loopback server is bound to `127.0.0.1:12053` and uses bearer-token authentication after pairing.

## 12. Backups And Diagnostics

Open **Ethics & Backups** to configure:

- Publication identity
- Organization type
- Editorial/ethics text
- Logo image
- Story guardrail terms
- Backup path
- Diagnostic export

Diagnostics are manual. Review any diagnostic package before sharing it.

## 13. Current Public-Beta Limits

- Installers are unsigned.
- Cleanroom testing has focused on Windows.
- macOS notarization/signing is not complete.
- OCR for scanned PDFs is not implemented.
- External publishing providers require real credentials for stable-grade live proof.
- Local AI quality depends on the model and hardware.
- Daily Scan and discovery are useful but still require editor review.

## 14. Technical Appendix

### Data Location

Windows:

```text
%APPDATA%\com.scottconverse.civicdesk\civicdesk.db
```

macOS:

```text
~/Library/Application Support/com.scottconverse.civicdesk/civicdesk.db
```

Linux:

```text
~/.local/share/com.scottconverse.civicdesk/civicdesk.db
```

Older builds used `org.civicnews.app/civicnews.db`; current builds migrate that data into the new app location on first launch.

### Current Tables

As of v0.2.9, migrations run through `0013_verification_queue`. The live schema includes:

- `sources`
- `evidence_items`
- `leads`
- `lead_evidence`
- `drafts`
- `published_posts`
- `paired_clients`
- `settings`
- `daily_scan_runs`
- `daily_scan_leads`
- `publish_runs`
- `subscribers`
- `civic_observations`
- `civic_entities`
- `civic_observation_entities`
- `source_performance_scores`
- `dark_signals`
- `verification_tasks`

Schema version is tracked with SQLite `PRAGMA user_version`, not a migrations table.

### Developer Commands

```bash
npm install
bash scripts/fetch-ollama-binaries.sh
npm run tauri dev
npm run tauri build
npm test -- --run
cd src-tauri && cargo test --lib
```
