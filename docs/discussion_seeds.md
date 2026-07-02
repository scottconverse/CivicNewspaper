# GitHub Discussion Seed Posts

These are copy-ready starter posts for the CivicNewspaper GitHub Discussions area. The repository is CivicNewspaper; the installed app is The Civic Desk.

## 1. Welcome To The Civic Desk Public Beta

**Category:** General

**Title:** Welcome to The Civic Desk public beta

**Body:**

Welcome to the CivicNewspaper community.

The Civic Desk is a local-first desktop newsroom for people trying to cover local government with limited time and limited staff. It helps a small publisher add sources, scan for leads, review evidence, draft stories, verify claims, and publish a static local paper.

This is a public beta, not a signed stable release. We want bug reports, usability reports, clean-machine install results, real source intake examples, and honest feedback from people who might actually use this to cover a town, city, county, school board, water district, or local agency.

Helpful reports include:

- Your operating system and app version.
- What city or jurisdiction you tested.
- What source type you used.
- What worked.
- What broke.
- Whether the output was useful to a real editor.

The guiding rule is simple: the software can assist, warn, rank, and organize. The human editor decides.

## 2. Local AI And Hardware

**Category:** Q&A

**Title:** Local AI setup, model choice, and hardware notes

**Body:**

The Civic Desk uses Ollama for local AI features when available. Local AI can help with Daily Scan summaries, draft assistance, and optional press-freedom/legal-risk review.

The app should continue to function in degraded mode if Ollama is missing, slow, offline, or no model is installed. Deterministic source fetching, import review, Workbench editing, backup, export, and publishing should still be usable.

Useful things to share here:

- Machine RAM, CPU, and GPU if known.
- Which model the app recommended.
- Whether model download progress was clear.
- Whether Daily Scan and drafting felt fast enough.
- What happened when the model was missing or unavailable.

Please do not paste private sources, credentials, or unpublished sensitive drafts into a public discussion.

## 3. Source Discovery And Bulk Import

**Category:** Ideas

**Title:** Source discovery and bulk import examples

**Body:**

Source intake is one of the most important parts of the product. The app supports manual sources, discovery, and imports from CSV, TXT, XLSX, and DOCX files. PDF import is disabled in the public beta until hardened parsing is available; convert PDF source lists to a supported format or paste the URLs directly.

The sources we especially want to test:

- City council agendas and minutes.
- Planning and zoning pages.
- School board agendas.
- County records and public notices.
- Police, fire, and emergency service feeds.
- Local news sites and calendars.
- Public social/community pages that do not require login.
- YouTube meeting videos and transcript sources.

Good bug reports include the file type, a short description of the layout, how many URLs you expected, how many the app found, and whether duplicates or split-cell URLs were handled correctly.

## 4. Dark Signals And Verification

**Category:** Editorial Workflow

**Title:** Dark Signal Desk: how should weak leads be ranked and verified?

**Body:**

The Dark Signal Desk is for early, weak, unusual, or socially surfaced signals. These may come from public forums, public social posts, local video transcripts, comments, or community discussion.

The app should rank and explain signals, but it must never hide information from the editor. A low-confidence signal should remain visible and clearly labeled so a human can decide whether it deserves more reporting.

Useful discussion points:

- What sources produce early local signals in your community?
- What labels make uncertainty clear without burying the lead?
- What verification steps should the app suggest first?
- How should the app distinguish publishable evidence from editor-only leads?

Low-confidence material should not become published evidence automatically. It should become a verification task.

## 5. Publishing Options

**Category:** Q&A

**Title:** Publishing stack: here.now, GitHub Pages, Netlify, WordPress, manual Cloudflare, Substack

**Body:**

The Civic Desk produces static website output first. That output can be zipped, reviewed, archived, and published.

Recommended publishing paths:

- here.now for simple temporary publishing and fast testing.
- GitHub Pages for a durable public archive.
- Netlify for technical users with an existing site/token.
- Cloudflare Pages as an assisted/manual hosting path: export the folder or ZIP, deploy in Cloudflare, then save the public URL.
- WordPress for users with an existing site.
- Substack and newsletters as distribution channels, not the only canonical archive.

If you test a provider, please share:

- Provider name.
- Whether setup was clear.
- Whether dry-run/test connection worked.
- Whether the published URL loaded.
- What failed and how recovery behaved.

Do not post provider tokens, access keys, or private repo credentials.

## 6. Guardrails And Press-Freedom Advisor

**Category:** Editorial Workflow

**Title:** Guardrails and advisor output should help, not override editors

**Body:**

The Civic Desk includes advisory guardrails and an optional press-freedom/legal-risk advisor. These tools can flag sourcing gaps, attribution issues, privacy concerns, defamation risk, public/private figure questions, and verification tasks.

They are not legal advice. They are not censorship. They must not veto the editor.

Please share examples where:

- The warning was useful.
- The warning was confusing.
- The language sounded too legalistic.
- The tool missed an obvious risk.
- The tool overreacted to ordinary reporting language.

The goal is to help under-resourced local publishers slow down at the right moments without taking editorial judgment away from them.
