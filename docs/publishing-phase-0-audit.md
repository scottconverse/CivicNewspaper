# Publishing Phase 0 Audit

Run date: 2026-06-26

## Scope

Audited the current local publishing output in `C:\Users\instynct\Documents\civicnews-site` and the current app database at `C:\Users\instynct\AppData\Roaming\com.scottconverse.civicdesk\civicdesk.db`.

Evidence generated during this audit:

- `.agent-runs/phase0-publish-desktop.png`
- `.agent-runs/phase0-publish-mobile.png`
- `.agent-runs/phase0-publish-print.png`
- `.agent-runs/phase0-seeded-publish-site/`
- `.agent-runs/phase0-seeded-article-desktop.png`
- `.agent-runs/phase0-seeded-article-mobile.png`
- `.agent-runs/phase0-seeded-article-print.png`

## Current Data State

The local database currently has:

- 26 sources
- 3 leads
- 0 drafts
- 0 published posts

Because there are no drafts, a real publish from the current local database cannot exercise article pages, article evidence blocks, correction banners on article pages, or per-story share copy. This is a product/audit blocker: publishing needs a seeded publish-preview path or a test fixture that can produce one approved, attested story without relying on a full Daily Scan and draft-generation run.

Status: addressed for regression testing by `test_seeded_publish_fixture_generates_article_evidence_and_correction_package`, which creates one source, evidence item, lead, attested corrected draft, and local export package through the real compiler.

## Output Audited

The existing output folder contains:

- `index.html`
- `about.html`
- `ethics.html`
- `how-we-report.html`
- `corrections.html`
- `feed.xml`
- `styles.css`
- `print.css`

No article subfolders or article pages are present.

Seeded audit output now contains:

- `watch/1.html`
- `index.html`
- `about.html`
- `ethics.html`
- `how-we-report.html`
- `corrections.html`
- `feed.xml`
- `newsletter.md`
- `substack.md`
- `share-package.md`
- `facebook-post.txt`
- `subreddit-post.md`
- `nextdoor-post.txt`
- `short-link-blurb.txt`
- `publish-manifest.json`
- `site-package.zip`

## Findings

### P0 - No Publish-Ready Local Drafts To Audit Article Output

The current app data has no drafts. The generated site is therefore only a shell with "No observation records published yet."

Impact: the publishing workflow cannot be fully validated from real current data. Article page layout, source/evidence display, corrections on articles, RSS item content, and shareability of real stories remain unaudited.

Recommended fix: add a local "create sample publish package" audit/test lane or a seeded publish fixture that creates one lead, evidence item, attested draft, and optional correction, then compiles it through the real compiler.

Status: addressed in Phase 1. The seeded fixture compiles a corrected watch article titled "Council Approves Library Roof Contract" with a linked source record, evidence excerpt, correction banner, RSS item, manifest, ZIP, and share artifacts.

### P1 - Empty Site Is Technically Valid But Not Product-Helpful

The empty homepage and RSS feed render without broken links, but the operator gets no useful next step beyond "No observation records published yet."

Recommended fix: the Publishing screen should explain when zero articles were included and route the editor back to Workbench to approve an attested story.

Status: partially addressed by the compile receipt and zero-article message added in Phase 1.

### P1 - Meta Pages Show Evidence UI That Does Not Apply

About and corrections pages render the article template's "Evidence & Sources Check" block even when there are no citations or when the page is a meta page.

Impact: readers may think the About, Ethics, or Corrections pages are missing evidence rather than understanding they are policy/meta pages.

Recommended fix: use a dedicated meta-page template or hide the evidence block for `about.html`, `ethics.html`, `how-we-report.html`, and `corrections.html`.

Status: addressed. Meta pages now suppress the article evidence section.

### P1 - Shareability Was Missing From Baseline Output

The baseline output folder had no newsletter draft, Substack draft, social copy, short-link copy, manifest, or ZIP package.

Status: addressed in Phase 1 foundation by adding `newsletter.md`, `substack.md`, `share-package.md`, `facebook-post.txt`, `subreddit-post.md`, `nextdoor-post.txt`, `short-link-blurb.txt`, `publish-manifest.json`, and `site-package.zip`.

### P2 - Links Pass Basic Local Existence Check

All local links in the shell pages resolve to existing files. No missing local navigation targets were found in the empty-site output.

### P2 - Mobile And Print Need Human Review With Real Articles

The empty homepage renders without obvious catastrophic layout failure in desktop, mobile, and print screenshots. This does not validate article typography, evidence lists, long titles, source links, correction banners, or share copy.

Recommended fix: rerun this audit after the seeded publish fixture exists.

Status: rerun against the seeded article output. Desktop, mobile, and print screenshots were generated. The article page displayed the correction banner, headline, body, evidence/source block, and AI disclosure. The mobile nav wraps on narrow width but remains usable and non-overlapping.

## Seeded Article Audit Results

The seeded article package passed the core Phase 0 checks:

- Article page generated at `watch/1.html`.
- Homepage links to the article.
- RSS includes the article title and link.
- About, Ethics, How We Report, and Corrections pages exist.
- Local link audit found no missing local targets.
- Evidence/source display includes a source-record anchor, original document link, and excerpt.
- Correction banner and public corrections ledger both include the correction note.
- Share artifacts were generated for newsletter, Substack, Facebook, subreddit, Nextdoor, and short-link copy.
- `publish-manifest.json` records issue ID, provider, output path, generated files, article count, skipped count, files written, timestamp, and article metadata.
- `site-package.zip` was generated.
- Publish run metadata is persisted in SQLite via `publish_runs`, not only in the output manifest.

## Phase 1 Punchlist

1. Add a seeded publish fixture/audit lane for one complete article. Done.
2. Split meta pages from article pages so policy pages do not show the evidence block. Done.
3. Keep the export package and compile receipt. Done.
4. Add a Preview state that opens `index.html` directly after compile. Done.
5. Add provider metadata placeholders now, then wire real Netlify/GitHub Pages/Cloudflare connectors later. Placeholder metadata done in Phase 1; Phase 2 added guided destination capture for Netlify, Cloudflare Pages, GitHub Pages, Substack, WordPress, and other hosts. The app now validates the public URL, records provider/deployment metadata, updates `publish_runs`, rewrites the manifest/share artifacts with the live URL, and rebuilds the ZIP package.
6. Persist completed publish metadata to app history, not only the output manifest. Done via `publish_runs`.
7. Rerun full publishing audit with a real article, evidence citation, correction, mobile view, and print view. Done for the seeded package.
