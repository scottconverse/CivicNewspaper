# The Civic Desk User Manual

The Civic Desk is a desktop newsroom for people trying to cover local civic life with limited time. It helps you collect sources, find leads, draft stories, verify claims, and publish a small local paper.

This manual is written for a new user, not a programmer. If you are a reporter, editor, town resident, nonprofit worker, blogger, or one-person local publisher, start at the beginning and follow the first-issue walkthrough. If you already know the app, use the later chapters as a reference.

The most important rule is simple: The Civic Desk assists you. It does not decide for you. It can warn, rank, summarize, suggest, and organize. You decide what is newsworthy, fair, legal, verified, and ready to publish.

## Before You Start

The Civic Desk is a signed Windows public beta. It is useful, but it is not a stable release yet.

What that means in practice:

- The public-beta installer is distributed through the official release page; verify its checksum before installing.
- The public release page should match the version you intend to install.
- Windows is the tested public-beta installer path for this release line.
- macOS and Linux installers are backlog/proof-needed until clean-machine proof is recorded.
- You should verify important output before publishing.

The app is local-first, not internet-free. Your database, drafts, settings, and output files are local by default. The app uses the internet when you fetch sources, run discovery/search, download a local AI model, or publish to an external provider.

## What The App Does Not Do

The Civic Desk is not a substitute for a publisher, reporter, editor, lawyer, or public-records clerk.

It does not:

- Decide what you are allowed to publish.
- Guarantee that a lead is true, current, fair, or newsworthy.
- Guarantee that discovery found every useful source in town.
- Turn rumors, comments, or public social posts into verified stories by itself.
- Provide legal advice.
- Replace proofreading, fact-checking, source review, or human judgment.
- Promise polished professional newspaper quality from every AI draft.

It can warn you, organize evidence, suggest reporting paths, improve drafts, and package an issue. You still decide what to report, what to verify, what to hold, what to cut, and what to publish.

## 1. Installing The App

Open the GitHub Releases page:

<https://github.com/scottconverse/CivicNewspaper/releases>

Choose the v0.3.2 Windows public beta release:

<https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2>

On Windows, use the `.exe` installer from the official release page. Confirm the filename, publisher, and published checksum before installing.

macOS and Linux are planned, but they are not supported public-beta installer paths in this release line yet. The project needs a real artifact, clean-machine install proof, first-run local AI proof, and honest platform-specific warning text before the manual should tell normal users to install those builds.

Before running an installer, you can compare its SHA256 hash with the release checksum. Checksums confirm that your download matches the release artifact. They are not a replacement for code signing. See [install.md](install.md) for exact commands.

If installation, local AI setup, publishing, source import, or output quality goes wrong, use [troubleshooting.md](troubleshooting.md). It has plain-English help for installer provenance, model downloads, local AI runtime states, weak story output, here.now previews, ZIP/static output, and source import problems.

## 2. First Launch And Setup

When you open The Civic Desk for the first time, it walks you through five essential setup steps and asks you to describe the publication you want to run.

You will choose or enter:

- Publication Name.
- Editor Name.
- Organization type, such as single person, nonprofit, for-profit, private organization, or other.
- City.
- State.
- Local AI setup.
- Publication folder under The Civic Desk app-data folder or your Downloads folder, where finished HTML sites and ZIP review packages are saved.
- Backup file: a complete `.db` path under The Civic Desk app-data folder or your Downloads folder.

Use real names if you intend to publish publicly. The app should not invent your copyright line, business model, ad policy, AI disclosure, or editorial policy. Those are your decisions.

### Local AI Setup

The app checks your machine and recommends a local model. For this public-beta line, `phi4-mini:latest` is the conservative default because the latest local bakeoff showed it returned valid structured results for both real civic signals and empty/noise input. You can still choose another installed model from the AI Model screen, but the app should not silently switch models without telling you.

Model downloads can be large and slow. This is normal. The setup screen should tell you what is being downloaded and show progress. If you skip local AI setup, you can still use many parts of the app: source review, manual writing in the editor, editing, backup, export, and publishing. AI-assisted summarization, drafting, ranking, improvement, social copy, and advisor features require a reachable local model.

If the app says local AI is unavailable, do not assume your work is lost. It usually means the Ollama runtime or selected model is not running or not installed. Use the AI Model screen to retry, download, or change the model.

## 3. Your First Issue: A Practical Walkthrough

This chapter walks through a small-town first issue. Do this before trying advanced workflows.

### Step 1: Add A Few Reliable Sources

Open **Sources**.

Start with official sources you already know:

- City council agenda page.
- City news or public information page.
- Planning or zoning page.
- School board agenda page.
- County public notices.

Add each source manually if you know the URL. If you do not, use **Discover for my city**. Discovery should return candidates for review. Do not blindly trust every candidate. Open questionable sources and decide whether they belong.

Source labels matter:

- **Primary record** means official records such as agendas, minutes, ordinances, public notices, court records, or budgets.
- **Official communication** means public information pages, press releases, and agency notices.
- **News reporting** means independent reporting from a news outlet.
- **Community signal** means public community/social sources, such as public Reddit pages, public forums, public meeting-video pages, or other public pages that may surface early signals.

Community signals are useful, but they are not automatically publishable evidence. Treat them as leads to verify.

### Step 2: Run Daily Scan

Open **Daily Scan** and click **Run Daily Scan**.

The scan should:

1. Fetch configured sources.
2. Store evidence.
3. Detect changes and civic observations.
4. Extract entities such as agencies, vendors, people, addresses, companies, parcels, and departments.
5. Create leads and dark signals.
6. Use local AI, if available, to summarize and rank what needs your attention.

If there are no sources yet, the app should send you back to Sources instead of running an empty scan.

### Step 3: Read The Story Queue Like An Assignment Editor

Open **Story Queue**.

The Story Queue is not a finished newspaper. It is an assignment list. A lead can be:

- A real story.
- A brief.
- A routine notice.
- A duplicate.
- A background item.
- A bad lead that should be ignored.

Open the linked source before drafting. Ask:

- Is something new?
- Does it affect residents?
- Is there a decision, deadline, cost, conflict, risk, or opportunity?
- Is there enough evidence to write now?
- Is this only a generic city web page?
- Would a resident reasonably care today?

If the answer is no, leave it alone, hold it, or turn it into a verification task. Do not publish filler just because the app surfaced it.

### Step 4: Use Dark Signals Carefully

Open **Dark Signals**.

Dark signals are early warnings. They may come from public social/community pages, comments, public video pages, local forums, or unusual changes in official records.

The point is not to publish rumors. The point is to notice what may need reporting.

For every dark signal, ask:

- Where did this come from?
- Is it public and legally accessible?
- What official record could confirm or disprove it?
- Who would know the answer?
- What harm could come from publishing too soon?

Turn promising dark signals into verification tasks. Keep low-confidence material out of published evidence until it is verified.

### Step 5: Create Verification Tasks

Open **Verification**.

Use this area to turn leads into reporting work. A verification task might say:

- Check the city agenda packet for the contract amount.
- Call the city clerk to confirm the hearing date.
- Find the parcel record for the address.
- Compare the current agenda with last month's agenda.
- Look for a second source confirming the claim.

Task states help you manage work:

- **Suggested:** the app or editor thinks this should be checked.
- **Auto-checked:** the app performed a simple check.
- **Needs human:** a person must call, email, attend, inspect, or decide.
- **Blocked:** you cannot proceed yet.
- **Resolved:** the task is done.

For a one-person publication, this queue is your notebook.

### Step 6: Draft A Story

When a lead is worth writing, click **Draft** or **Open draft**.

The Workbench opens with linked sources. Choose the format:

- **Brief:** short item, usually under 200 words.
- **Watch alert:** something residents should watch, such as a deadline, public hearing, or service disruption.
- **Explainer:** background on how a policy or process works.
- **Investigation:** deeper story following money, influence, records, or repeated signals.
- **Editorial/opinion:** opinion writing, if that fits your publication.

If local AI is available, click **Generate Draft**. Treat the draft as a starting point. AI drafts often sound like notes. They may include placeholders such as `[Source needed]` or `[Verification needed]`. Those are not acceptable in a finished story unless you intentionally publish them as editor's notes.

Before publishing, rewrite the story in your own editorial voice.

### Step 7: Edit Like A Publisher

A publishable local story should usually answer:

- What happened?
- When did it happen or when will it happen?
- Who is affected?
- Why does it matter locally?
- What evidence supports it?
- What is still unknown?
- What should residents do next, if anything?

Do not let the app turn evergreen background pages into fake news. For example, a page explaining that city council meetings have videos is not necessarily a current story unless something changed: a new archive launched, access was expanded, video was removed, captions were added, viewership data surfaced, or residents raised an issue about access.

If the draft is really a reporting note, keep it in draft, send it back for more work, or turn it into a verification task.

### Step 8: Run Advisor And Guardrails

The Workbench includes advisory checks. They can flag:

- Missing source support.
- Loaded or accusatory wording.
- Defamation or privacy risk.
- Public/private figure questions.
- Presumption-of-innocence issues.
- Claims that need verification.
- Excessive copying from source material.

These advisory tools do not make editorial decisions for you. They are there to slow you down at the right moments.

If the advisor raises a concern, decide what to do:

- Rewrite.
- Add attribution.
- Add evidence.
- Hold the story.
- Mark a verification task.
- Publish anyway with an editor note if you judge that appropriate.

There is one important exception: **static package-validity blockers** must be fixed before the app will approve a story for static publishing. These are not editorial vetoes. They are output-integrity checks that prevent a public package from containing broken or unsupported material.

Approval can be blocked when:

- A scanned-source lead has no linked source documents.
- A lead-based story has no inline evidence citation.
- A citation points to evidence that is disabled, missing, or not linked to that lead.
- The article topic does not match the cited evidence.
- The body is empty, too short, too large, or mostly metadata.
- Reporter notes, test notes, or editor-only scaffolding would leak into the public story.

If approval is blocked, read the on-screen **Fix before static publish approval** list, repair the draft or evidence, save again, and then approve. The editor still decides what is newsworthy and what to publish; the blocker only protects the exported public package from being structurally invalid.

### Step 8A: Use The Workbench Status Controls

The Workbench is where a draft becomes an editorial decision. Think of it like the editor's desk in a small newsroom.

If the draft is close but still reads like notes, use **Improve for Publication**. This asks the local model to clean up headline, structure, and reader-facing wording without adding facts. It loads the revised copy into the editor so you can inspect it. It does not publish the story for you. Review the text, check the sources, and save the draft if you want to keep the change.

If the story is too long or thin for a full article, use **Make this a brief**. This changes the draft format in the editor. Save the draft before moving on so the issue compiler uses the format you actually reviewed.

If a writer needs to do more reporting, click **Send Back for More Work**. Write a clear assignment note, such as "Confirm the dollar amount with the agenda packet" or "Find a second source before naming the contractor." The note stays with the draft so the next person knows what to fix.

If the story is real but not ready today, click **Hold**. Use the hold note to explain what you are waiting for: a meeting packet, a return phone call, an election result, a court filing, or a public document. Held stories are paused. Resume them when the missing piece arrives.

If the story should not be in this issue, click **Cut Story**. Cut does not mean the app censored the story. It means the editor decided not to run it now. You can restore it later if the newsroom changes its mind.

When a sent-back or held story is ready again, use **Resume Editing** or **Mark Ready for Review** first. The app will not let a paused draft jump straight to publication from the paused state; that is workflow discipline, not a content veto.

### Step 9: Approve For Publishing

Only approve a story when a human editor has reviewed it.

Approving means: this exact saved draft is ready to be part of the public issue. It does not mean the app certified it. It means you did. When you approve, the app saves the visible draft first, checks it again, records the human review, and then marks it ready for the issue.

The app records that a human review happened. It should never silently approve a story for you.

### Step 10: Compile, Export, Publish

Open **Publishing**.

The normal sequence is:

1. **Compile:** build the static issue locally.
2. **Preview:** open the output and check it.
3. **Export:** create the ZIP and share package.
4. **Publish:** send the site to a provider.
5. **Share:** use the generated newsletter and community posts.

The recommended default publisher is **here.now**. Anonymous here.now publishing can create a temporary preview without an account. Account/API-key publishing can support permanent sites. GitHub Pages is the durable public archive option. Netlify is for more technical users with an existing site and token. Cloudflare Pages and WordPress are assisted/manual in this beta: export the folder or ZIP, publish through that service, then record the public URL in the app. Substack is assisted: the app prepares the text, you paste it into Substack, publish there, and record the URL.

Always inspect the output before sharing the link.

## 4. Everyday Workflow After Setup

Once your sources are configured, the day-to-day workflow is simple:

1. Run Daily Scan.
2. Review new leads.
3. Open the source for anything interesting.
4. Create verification tasks.
5. Draft only the leads that are actually newsworthy.
6. Edit drafts into real stories or briefs.
7. Run advisor/guardrails when useful.
8. Approve publishable items.
9. Compile and preview the issue.
10. Publish and share.

You do not need to publish every lead. A good day may produce zero stories and several watch items. That is better than padding the paper with weak rewrites.

## 5. Understanding Lead Quality

The app currently finds more possible leads than polished stories. That is by design, but it means you must use judgment.

A strong lead usually has one or more of these:

- A new decision.
- A deadline.
- A public hearing.
- A contract, budget item, or vendor.
- A policy change.
- A changed document.
- A repeated pattern across sources.
- A credible dark signal with a verification path.
- A local consequence residents can understand.

A weak lead often looks like:

- A general department page.
- A page that has existed for years.
- A broad city services page.
- A source that was merely fetched for the first time.
- A generic summary of how government works.
- A duplicate of another lead.
- A story with no current hook.

Weak leads are not useless. They can help build background knowledge. But they should not automatically become published stories.

## 6. Source Discovery And Bulk Import

You can import source lists from:

- CSV
- TXT
- XLSX
- DOCX
- PDF files are not imported in the public beta. Convert PDF source lists to TXT, CSV, DOCX, or XLSX, or paste the URLs directly.

The app should split URLs into separate reviewable candidates. If a spreadsheet or document contains many URLs and the app finds only one, that is a bug.

PDF import is disabled in the public beta, including scanned and text PDFs. Convert PDFs to a supported format with your own trusted tool, or paste the source URLs directly. Hardened/sandboxed PDF parsing remains on the security backlog.

When importing a file, review:

- Source name.
- URL.
- Type.
- Tier.
- Duplicate status.
- Whether it is reachable.
- Whether it belongs to your jurisdiction.

Do not import every URL just because it appeared in a file.

## 7. Browser Extension

The browser extension lets you send public pages into The Civic Desk while you read.

Use it when you find a useful page outside the app:

1. Open **Browser Pairing** in The Civic Desk.
2. Generate a pairing code.
3. Open the extension.
4. Enter the code.
5. Confirm the app shows the paired device.
6. Send a public page to the app.

The pairing is local to your computer. It is not an internet service.

## 8. Publishing Output And Site Customization

The app can generate:

- Homepage.
- Article pages.
- RSS feed.
- About page.
- Ethics/reporting page.
- Corrections page.
- Print stylesheet.
- ZIP package.
- Newsletter markdown.
- Substack-ready markdown.
- Facebook post.
- Subreddit post.
- Nextdoor post.
- Short-link blurb.

You can configure:

- Publication name.
- Subtitle.
- Editor/publisher identity.
- Organization type.
- About text.
- Ethics text.
- How-we-report text.
- Footer text.
- Accent color.
- Layout style.
- Logo image.

The app should not invent these for you. If the output says "we run no ads" or "all stories are public-record backed," that should be because you wrote it, not because the software assumed it.

## 9. Backups And Restore

Use backups before major imports, big scans, or release work.

Create a backup from the app's backup/system area. For write safety, the app creates backups only under its app-data folder or your Downloads folder. After creation, you can use File Explorer to copy the backup to an external drive or private cloud folder you control.

Restore replaces the current local database with the backup. Treat restore as a serious action. If you are unsure, make a fresh backup first.

The current app is single-machine software. Do not open the same live SQLite database from two computers at once.

## 10. Troubleshooting

### Daily Scan Produces Weak Leads

This can happen when sources are broad pages rather than update feeds or agenda packets. Add more precise sources: agenda RSS, meeting packets, public notices, department news pages, board calendars, and document portals.

If a lead is not newsworthy, do not draft it. Hold it, ignore it, or create a verification task.

### Draft Looks Like Reporter Notes

That means the AI produced a working draft, not a finished story. Edit it. Remove "Headline:", "Nut graf:", "Reporting steps:", "[Source needed]", and similar internal scaffolding unless you intentionally want those in public copy.

### Local AI Is Slow

Large local models can be slow, especially on CPU. Try a shorter format, a smaller model, or manual drafting. The app should not freeze without progress.

### Model Missing Or AI Offline

Open **AI Model**. Confirm the runtime is running and the selected model is installed. If not, download the recommended model or switch to one that exists locally.

### Publishing Fails

First compile and preview locally. If local output works but provider publishing fails, check:

- Provider credentials.
- Target slug/repo/project.
- Network connection.
- Whether the provider requires an account.
- Whether the publish URL is temporary.

You can still export the ZIP and publish manually.

### Sources Import Poorly

If XLSX or DOCX imports flatten many URLs into one candidate, report it as a bug with the file type and an example. The expected behavior is separate reviewable URL candidates. PDF import is disabled in the public beta until hardened parsing is available.

## 11. Advanced Reference

### Main Navigation

- **Story Queue:** review leads and drafts.
- **Daily Scan:** run source checks and AI-assisted summaries.
- **Dark Signals:** review weak or public social/community signals.
- **Verification:** manage reporting tasks.
- **Workbench:** write, edit, advise, approve, hold, cut, or return stories.
- **Sources:** add, discover, import, and review sources.
- **AI Model:** install/check local model support.
- **Publishing:** compile, export, publish, and share.
- **Browser Pairing:** connect the local browser extension.
- **Ethics & Backups:** configure policy language and backup/restore.
- **System & Status:** diagnostics and app state.

### Publishing Providers

- **here.now:** recommended default. Temporary anonymous previews are useful for testing. API-key/account publishing can support permanent sites.
- **GitHub Pages:** best for durable public archives in a repository.
- **Cloudflare Pages:** assisted/manual hosting option in this public beta.
- **Netlify:** technical hosting option.
- **WordPress:** direct API publishing is disabled in the public beta until draft-first publishing, rollback, and live connector proof are complete. Export the ZIP/static folder or record the URL after publishing manually.
- **Substack:** assisted copy/paste workflow.
- **Other/manual:** records a public URL after you publish elsewhere.

### Current Public-Beta Limits

- Authenticode signature verification is required before a release installer is published.
- Latest source/tag and latest published installer release may not always be the same.
- Windows is the tested public-beta installer path.
- macOS and Linux installer proof is backlog/proof-needed.
- OCR for scanned PDFs and hardened/sandboxed PDF parsing are not complete.
- Fully polished newsroom-quality story selection still needs improvement.
- External provider verification depends on user-owned credentials.

### Developer Commands

These are for contributors, not normal newsroom use.

```bash
npm install
npm run tauri dev
npm run tauri build
npm test -- --run
cd src-tauri && cargo test --lib
```

The v0.3.x app uses app-managed local AI setup during first run. The legacy sidecar-fetch script is not part of the current release verification path.

### Data Model Summary

The local database stores sources, evidence, leads, lead/evidence links, drafts, publish runs, subscribers, observations, entities, source performance scores, dark signals, and verification tasks. Schema version is tracked with SQLite `PRAGMA user_version`.
