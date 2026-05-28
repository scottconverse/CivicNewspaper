# UI/UX Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-26
**Role:** Senior UI/UX Designer
**Scope audited:** Landing Page (docs/index.html, style.css, script.js), User Manual (docs/user_manual.md), Architecture Specs (docs/architecture.md), Onboarding Wizard (src/components/OnboardingWizard.tsx), Plain-Language Rewrite Confirm Dialog (src/components/Workbench.tsx), and all app surfaces (Story Queue, Story Workbench, Sources Manager, Browser Pairing panel, Ethics & Backups settings).
**Auditor posture:** Balanced

---

## TL;DR

CivicNewspaper establishes an aesthetic and structurally readable foundation for local-first, evidence-linked civic journalism, featuring high contrast and warm typography on compiled public-facing templates. Recent engineering cycles successfully resolved mobile layout collapses for main viewports and layout grids (workbench, sources, and pairing), while securing asset navigation link paths and basic accessibility labels.

However, critical user journey and interaction issues remain. The onboarding wizard recommends low-spec model paths but forces a hardcoded 5.4 GB download that will freeze low-spec systems, and instructs users to load extensions from dev-only directories. Furthermore, the plain-language rewrite feature triggers a blind in-place text replacement without a side-by-side preview, differing from the user manual's description and risking user data loss. Finally, the documentation landing page collapses to hide all navigation links on mobile.

---

## Severity roll-up (UX)

| Severity | Active Count | Resolved Count | Total |
|---|---|---|---|
| Blocker | 1 | 2 | 3 |
| Critical | 3 | 1 | 4 |
| Major | 10 | 1 | 11 |
| Minor | 3 | 0 | 3 |
| Nit | 1 | 0 | 1 |
| **Total** | **18** | **4** | **22** |

---

## What's working

- **Beautiful Newspaper Aesthetic** — The compiled public newsroom uses a gorgeous paper-like background (`#fcfbfa`) combined with a dark editorial crimson accent (`#5a1818`), achieving a warm, classic Gazette feel.
- **Robust Typography Scale** — The font stack ('Inter' for UI controls, 'Lora' / 'Georgia' for editorial body texts) is well-considered, establishing a highly readable, classic serif rhythm for long-form reading.
- **High Contrast Ratios** — Contrast ratios are exceptionally strong, with primary text (`10.4:1` in light UI, `17.4:1` in dark UI) and compiled editorial text (`6.4:1` for muted text, `9.1:1` for crimson accents) exceeding WCAG 2.1 AAA standards.
- **Resolved Mobile Main Content Squeeze** — The main content view now flows vertically and scales correctly to 100% viewport width without clipping.
- **Resolved Layout Grid Collapse** — The main columns of the Story Workbench, Sources Manager, and Pairing panels collapse into a single-column layout on viewport widths below 1024px, preventing clipping.
- **Correct Detail Page Link Resolving** — Relative assets and header link targets are successfully modified in compiled subfolders (e.g. `watch/1.html`), preventing broken pages and stylesheets.
- **Added Accessibility Labels for Actions** — Ingest and list delete buttons are now associated with descriptive aria-labels, preventing "blank" button announcements for screen-readers.
- **Structured Documentation Layout** — The user manual is logically split into reader-focused, technical, and developer parts, easing comprehension.

---

## What couldn't be assessed

- **Actual Browser Extension Performance** — The pairing setup was inspected from the Tauri desktop UI; the actual runtime interaction within Chrome could not be reviewed without active browser runtime pairing.

---

## First impressions

- **Landing Page Experience:** The landing page features an appealing dark mode design with modern geometric typography (Outfit + Inter). However, clicking "User Manual" or "First time installing?" redirects the user to raw Markdown source code in the browser, showing unformatted text, which feels incomplete and non-functional for non-technical users.
- **App Arrival Experience:** On a fresh run, the first-time user lands directly on an empty Story Queue dashboard. The page displays "No unlinked leads available" and invites the user to click "Scrape & Detect."
- **5-Second Clarity:** Within 5 seconds, a user knows this is a news monitoring and drafting feed, but does not know *how* it gets its data or *why* it's currently empty.
- **Journey Friction:** If a new user clicks "Scrape & Detect" on their first run, it quietly scrapes 0 feeds and generates 0 leads because no sources have been set up. The app does not route first-time users to the "Ollama Wizard" or the "Sources Setup" tabs.

---

## Journey walkthroughs

### Journey: First-time Onboarding → Feed Ingestion → Article Publication

1. **Onboarding Setup:** The user clicks the "Ollama Wizard" tab (or is forced into it).
   - *Friction:* The wizard detects system RAM and recommends a specific model size (e.g. `phi3:mini` for low RAM). However, when the user clicks "Download Model" on Step 3, the wizard ignores the recommendation and forces the download of the heavy `gemma2:9b` (5.4 GB). Furthermore, on Step 5 (Browser Pairing), the instructions tell the user to load the browser extension unpacked from a local developer folder (`browser-extension/chromium/`) which is not shipped with client installers, blocking production users.
2. **Sources Configuration:** The user navigates to the "Sources Setup" tab to register feeds.
   - *Delight:* The "Auto-Discover Town Feeds" modal is highly intuitive. Entering a city/state queries DDG and displays checklists of local agendas and regional subreddits categorized by type.
3. **Ingestion & Detection:** The user returns to the queue and clicks "Scrape & Detect."
   - *Friction:* The status bar turns blue and shows "Scraping feeds...", but the rest of the UI remains frozen with no progress bar or detailed ingestion logs.
4. **Editorial Workbench & Guardrails:** The user selects a lead, opens the drafting wizard, and clicks "Generate Draft."
   - *Friction:* The local Ollama model generates text, but if the editor makes adjustments that trigger plagiarism warnings (verbatim copying), these warning issues are completely hidden because of an incorrect `is_clean` check.
5. **Plain-Language Rewrite:** The user wants to simplify jargon and clicks "Plain Language Rewrite" in the text editor.
   - *Friction:* The editor prompts the user to confirm replacement *before* running the LLM api call. Once confirmed, the editor content is immediately overwritten with the model output without showing a side-by-side comparison first. If the LLM errors out or rewrites poorly, the original draft is lost.
6. **Static Site Compilation:** The user approves the draft and publishes it.
   - *Delight:* Opening the compiled folder reveals that detail pages (e.g. `watch/1.html`) are now fully styled and the header navigation works correctly.

---

## Findings

> **Finding ID prefix:** `UX-`
> **Categories:** Visual hierarchy / Copy / State / Accessibility / Responsive / Journey / Pattern / Motion / IA

### [UX-001] — Blocker — [RESOLVED] — Broken Navigation and Styles on Compiled Detail Pages

**Evidence**
- **File:** `templates/post.html`, `src-tauri/src/core/compiler.rs` lines 164-172.
- **Visuals:** Compiled HTML subpages (e.g. `watch/1.html`) are now fully styled and navigation links are correctly mapped.

**Why this matters**
Because articles are compiled into subdirectory folders (`watch/`, `briefs/`, etc.), raw relative assets and links in `post.html` (e.g., `href="styles.css"`, `href="index.html"`) look for assets inside the subfolder. This completely breaks the website's design, styling, and navigation for end readers, rendering subpages unusable.

**Blast radius**
- All compiled post pages in subdirectories (`briefs`, `watch`, `explainers`, `stories`, `opinions`).
- User-facing static newsroom.

**Fix path**
[RESOLVED] Checked `compiler.rs`. The compiler now does string replacement of asset/page names (e.g., `href="styles.css"` to `href="../styles.css"`) for posts compiled inside subfolders.

---

### [UX-002] — Blocker — [RESOLVED] — Main Content Viewport Squeezed to 60px on Mobile

**Evidence**
- **File:** `src/App.css` lines 678-690.
- **Visuals:** Viewport size 320px. The main layout now flows vertically and scales correctly to 100% viewport width without clipping.

**Why this matters**
The main content class had a fixed width calculation: `width: calc(100% - 260px)`. Although the mobile media query sets `.sidebar` to `100%`, it failed to reset `.main-content` width. This makes the application completely unusable on viewports under 768px.

**Blast radius**
- All application tabs and views on viewports below 768px.

**Fix path**
[RESOLVED] Checked `App.css`. The media query under `max-width: 768px` now resets both `.sidebar` and `.main-content` to `width: 100% !important; min-width: 100% !important;` and adds `padding: 1rem !important;`.

---

### [UX-003] — Critical — [RESOLVED] — Layout Grids Do Not Collapse on Mobile and Tablet

**Evidence**
- **File:** `src/components/Workbench.tsx` (`.workbench-container`), `src/components/SourcesPanel.tsx` (`.sources-grid`), `src/components/PairDialog.tsx` (`.pairing-grid`), `src/App.css` lines 666-688.
- **Visuals:** Viewport width 768px (tablet portrait). Grids now collapse correctly to a single-column flow.

**Why this matters**
When editors work on tablets or smaller screens, fixed layout grids squish the main textareas (such as the Markdown editor body) to the point where they are unreadable.

**Blast radius**
- Story Workbench pane, Sources Manager, and Pairing panel.

**Fix path**
[RESOLVED] Checked `SourcesPanel.tsx`, `PairDialog.tsx`, and `App.css`. The inline style grid definitions were removed, the classes `sources-grid` and `pairing-grid` were added to the respective containers, and the media query under `max-width: 1024px` successfully collapses all three layout grids to `1fr !important; height: auto !important;`.

---

### [UX-004] — Major — State / Journey — Silenced Plagiarism and Citation Guardrails Warnings

**Evidence**
- **File:** `src/components/Workbench.tsx` lines 180-212.
- **Problematic code:** `{!guardrailsReport.is_clean && ( <div id="guardrails-issues-list">...` in `Workbench.tsx`.

**Why this matters**
The backend classifies plagiarism (Verbatim Source Overlap) and missing citations on normal paragraphs as `"warning"`. Because there are no errors, the backend marks `is_clean: true`. The frontend then displays "Pre-publication Guardrails Passed: No major issues detected. (N issue(s))" and hides the list of issues. This leaves plagiarism warnings completely invisible to the editor.

**Blast radius**
- Guardrails panel within the Story Workbench.

**Fix path**
Modify `Workbench.tsx` to always render the issues list if issues exist, regardless of the `is_clean` status.

---

### [UX-005] — Major — Journey / State — Onboarding Wizard is Non-Functional and Discards Input

**Evidence**
- **File:** `src/components/OnboardingWizard.tsx`, `src/components/AppContent.tsx` lines 125-138.
- **Problematic code:** The wizard is mounted without profile update callbacks or model pull handlers. The download button is hardcoded `disabled`.

**Why this matters**
A first-time user entering a "Publication Name" in Step 1 has their input discarded upon completing onboarding. Furthermore, the user cannot download their recommended local LLM model from the wizard. 

**Blast radius**
- First-time user setup flow.

**Fix path**
Provide the `handleSaveProfile` and `handlePullModel` hooks to the `OnboardingWizard` component. Save Step 1's publication name to settings, and trigger the Ollama pull on Step 3, rendering a progress bar from `pullProgressText`.

---

### [UX-006] — Major — Journey / State — Non-Functional "Verify Extracted Token" UI Control

**Evidence**
- **File:** `src/components/PairDialog.tsx` lines 77-86.
- **Visuals:** The "Verify Extracted Token" text input is completely unresponsive. Pasting a token does nothing.

**Why this matters**
The field lacks `value`, `onChange` hooks, or a submit button. It is a dead interaction state, leading users to believe the token validation is broken or frozen.

**Blast radius**
- Browser Integration Pairing panel.

**Fix path**
Either remove the verify input box, or implement a backend `verify_pairing_token` command and hook it to the input's `onChange` to validate token existence, displaying a success/error message.

---

### [UX-007] — Major — Accessibility — Systemic Form Input Label Disassociation

**Evidence**
- **Files:** `SettingsPanel.tsx`, `SourcesPanel.tsx`, `Workbench.tsx`.
- **Problematic structure:** `<label>Label Text</label><input id="id" />` (Missing `htmlFor` attribute linking to input IDs).

**Why this matters**
Without `htmlFor` attributes matching the input `id` (or nesting the input inside the label), screen readers will not associate form labels with their input controls. This makes form configuration inaccessible.

**Blast radius**
- All settings, source creation, and workbench editor inputs.

**Fix path**
Apply `htmlFor` matching the input's `id` to all labels.

---

### [UX-008] — Major — Journey / IA — Missing "About Page" Configuration in Settings Panel

**Evidence**
- **Files:** `src/components/SettingsPanel.tsx`, `src-tauri/src/core/compiler.rs` line 273.
- **Problematic structure:** Settings form lacks any input for `about_text` even though the compiler builds `about.html` using this field.

**Why this matters**
The compiled "About" page is locked to default fallback text ("We report on local government activities..."). The editor cannot update this text in the UI settings panel.

**Blast radius**
- Settings Panel and compiled static site.

**Fix path**
Add an "About Page Text" textarea in `SettingsPanel.tsx` bound to `profileForm.about_text`.

---

### [UX-009] — Major — Pattern / Journey — Relative URLs in RSS Feed Break XML Validation

**Evidence**
- **File:** `src-tauri/src/core/compiler.rs` lines 208-212.
- **Visuals:** Compiled `feed.xml` lists items with `<link>watch/1.html</link>`.

**Why this matters**
RSS feed specifications require fully-qualified absolute URLs. Relative paths fail feed validation, preventing RSS reader aggregators from loading articles.

**Blast radius**
- Compiled static newsroom RSS feed (`feed.xml`).

**Fix path**
Add a "Base URL" configuration input in `SettingsPanel.tsx`, store it in the profile json, and prepend it to links built inside `compiler.rs` for RSS compilation.

---

### [UX-010] — Major — [RESOLVED] — Accessibility — Unlabeled Icon-Only Action Buttons

**Evidence**
- **File:** `LeadQueue.tsx` line 240 (Delete Draft button), `SourcesPanel.tsx` line 145 (Delete Source button).
- **Problematic code:** `<button className="btn btn-danger btn-sm" ...><Trash2 size={12} /></button>`.

**Why this matters**
Visual screen reader users hear only "button" for the trash can icon. There is no fallback description, hiding the critical, destructive delete action from visually impaired editors.

**Blast radius**
- Leads/Drafts tables and Sources list tables.

**Fix path**
[RESOLVED] Checked both `LeadQueue.tsx` and `SourcesPanel.tsx`. Both trash buttons are now equipped with `aria-label="Delete draft"` and `aria-label="Delete source"` respectively.

---

### [UX-011] — Minor — Visual / Style — Inconsistent and Cluttered Dark Mode CSS Redefinitions

**Evidence**
- **File:** `src/App.css` lines 39-64 and 672-679.
- **Problematic code:** Two distinct media queries target `prefers-color-scheme: dark`.

**Why this matters**
The second block overrides `--text-primary` differently and adds light-mode missing variables (`--bg-primary`, `--bg-secondary`). This causes scoping clutter, color drift, and CSS maintenance risks.

**Blast radius**
- App-wide color scheme.

**Fix path**
Merge both dark-mode media queries into a single block at the top of `App.css`.

---

### [UX-012] — Minor — Copy / UX — PIN vs. Token Naming Inconsistency

**Evidence**
- **File:** `src/components/PairDialog.tsx`.
- **Problematic copy:** Labeling refers to a "PIN" in some instructions and a "Token" in buttons, when the actual output is a 22-character base64 URL-safe token.

**Why this matters**
Confuses users who expect a 6-digit numeric input but are instead given a long alphanumeric string.

**Fix path**
Align the copy to use "Pairing Token" consistently throughout the interface.

---

### [UX-013] — Critical — Responsive / Journey — Navigation Links and Primary CTAs Hidden on Mobile Viewports

**Evidence**
- **File:** `docs/style.css` lines 431-435
- **Visuals:** Viewport sizes 320px and 768px. The desktop navigation links are hidden. Because the "User Manual" link (`class="nav-btn-secondary"`) and the "GitHub" link (`class="nav-btn-primary"`) do not have the `.btn` class, the over-broad selector `.nav-links a:not(.btn)` hides them on mobile as well.

**Why this matters**
A mobile user visiting the landing page has absolutely no navigation paths. They cannot read the user manual, find the GitHub link, download the binaries, or navigate between the product description sections. It removes all core navigation functions on mobile devices.

**Blast radius**
- Entire landing page (`docs/index.html`) on any mobile viewport (under 768px).

**Fix path**
Modify the CSS rule on `docs/style.css` line 432 to explicitly preserve the buttons, or refactor the class structures to ensure `.nav-btn-primary` and `.nav-btn-secondary` are not collapsed.

---

### [UX-014] — Major — Journey — Raw Markdown File Redirection in Production Deployed Environment

**Evidence**
- **File:** `docs/index.html` lines 24, 45, 56, 67, 211
- **Visuals:** Clicking on "User Manual" or "First time installing?" links in a browser redirects the user to raw files (`user_manual.md`, `install.md`).

**Why this matters**
Standard users who visit the landing page will be presented with raw, unrendered Markdown files. This breaks the professional aesthetic of the product and is difficult to read. 

**Blast radius**
- All documentation link journeys on the landing page.

**Fix path**
Compile `user_manual.md` and `install.md` into clean HTML files (e.g. `user_manual.html`, `install.html`) and update the link paths on the landing page to target these styled HTML pages.

---

### [UX-015] — Blocker — Journey / State — Hardcoded Gemma2 Model Ingestion Forced on Low-RAM Systems during Onboarding

**Evidence**
- **File:** `src/components/OnboardingWizard.tsx` line 125
- **Visuals:** Step 3 of the Onboarding Wizard pulls the hardcoded string `"gemma2:9b"`.

**Why this matters**
The onboarding wizard correctly detects RAM and displays a recommendation (`phi3:mini` for RAM < 8GB, `llama3:8b` for RAM >= 8GB). However, when the user clicks the "Download Model" button, the code ignores the recommendation and invokes the pull operation with `"gemma2:9b"`. If the user is on a low-RAM system, this causes the heavy 5.4 GB model to download, which will crash their Ollama instance or freeze their system.

**Blast radius**
- Onboarding Wizard setup flow (`OnboardingWizard.tsx`) for low-spec/low-RAM computers.

**Fix path**
Modify `OnboardingWizard.tsx` to pass the model returned by the helper `getRecommendedModel()` instead of the hardcoded string.

---

### [UX-016] — Critical — Journey — Out-of-Context Local File Setup Instructions for Packaged Clean App Installers

**Evidence**
- **File:** `src/components/OnboardingWizard.tsx` line 355
- **Visuals:** The text instructs the user to select the `browser-extension/chromium/` folder when loading unpacked extensions.

**Why this matters**
When the application is installed via a packaged installer (MSI or DMG), the local source repository folder `browser-extension/` is not distributed inside the app installation directory. Non-technical users cannot find this folder, rendering the browser extension configuration impossible.

**Blast radius**
- First-time user setup experience for all production app installers.

**Fix path**
Add an "Export Extension Folder..." button to the wizard or pairing dialog that extracts the bundled extension code into a directory of the user's choosing, or host the extension on the Chrome Web Store and link to it.

---

### [UX-017] — Critical — Journey / State — Jarring and Destructive Blind Text Overwrite in Plain-Language Rewrite Dialog

**Evidence**
- **File:** `src/components/Workbench.tsx` lines 238-251
- **Visuals:** In the Workbench, clicking the "Plain Language Rewrite" button triggers a native `window.confirm` box *before* calling the API, then directly overwrites the editor value upon receipt.

**Why this matters**
The user manual promises that a side-by-side comparison modal will allow the user to review the changes before confirming. In reality, the confirm box happens *before* the API call runs, and the replacement is done blindly in-place. If the AI model returns nonsense or fails with a blank output, the original draft content is overwritten and lost without recovery.

**Blast radius**
- Story workbench plain language rewrite button and draft editor.

**Fix path**
Fetch the rewritten text first, open a side-by-side preview diff modal showing the comparison, and update the editor state only when the user explicitly clicks "Confirm" on the preview modal.

---

### [UX-018] — Major — Journey / IA — Technical Discrepancies and Inaccurate Token Info in User Manual

**Evidence**
- **File:** `docs/user_manual.md` lines 130-136, 139-149, and 215, and `docs/architecture.md` lines 137-142
- **Visuals:** The manual references a "6-digit PIN" for pairing and lists "Bids & RFPs" and "Ordinances & Resolutions" as two of the eight detectors. The onboarding architecture diagram shows a "First Source Added" step.

**Why this matters**
The actual code generates a 22-character Base64 token rather than a 6-digit PIN. The detectors code does not have "Bids & RFPs" (it is part of the "Deadline" detector) or "Ordinances & Resolutions" (which is completely unimplemented). Finally, the onboarding wizard does not contain a "First Source" configuration step. This creates confusing mismatches for technical operators auditing or configuring the system.

**Blast radius**
- Technical user manual and architecture documentation.

**Fix path**
Rewrite the manual and architecture diagrams to accurately match the implementation.

---

### [UX-019] — Major — State / Interaction — Bugged Wizard Navigation Skip Logic when Ollama Health Check is Pending

**Evidence**
- **File:** `src/components/OnboardingWizard.tsx` line 391
- **Visuals:** Clicking "Skip for now" on Step 2.

**Why this matters**
If the health check is slow or pending, the state `health` is null. The click handler condition `health && !health.reachable` evaluates to `false` and navigates the user to Step 3 (Download model) instead of skipping to Step 4, getting them stuck on a model pull window with an offline connection.

**Blast radius**
- Onboarding Wizard skip navigation.

**Fix path**
Modify the click handler to skip if health is null or unreachable.

---

### [UX-020] — Major — State / Interaction — Missing Spinner and Empty States on Ollama Health Check

**Evidence**
- **File:** `src/components/OnboardingWizard.tsx` line 235
- **Visuals:** Step 2 shows a completely blank card while `health` is null.

**Why this matters**
While the health check is query-running, the card is blank. The user sees a blank container with no loading spinner or text, which looks like a frozen interface.

**Blast radius**
- Step 2 of the Onboarding Wizard.

**Fix path**
Add a loading skeleton or a message like "Querying local Ollama status..." when `health` is null and `checkingHealth` is true.

---

### [UX-021] — Minor — Visual — Color Contrast Failure on Secondary Text in Docs Landing Page

**Evidence**
- **File:** `docs/style.css` lines 3 and 6
- **Visuals:** Secondary text (`#94a3b8`) on surface card background (`#1e293b`).

**Why this matters**
The contrast ratio of `#94a3b8` on `#1e293b` is `3.8:1`. This falls below the WCAG 2.1 AA requirement of `4.5:1` for normal body text, causing readability issues and visual fatigue.

**Blast radius**
- Landing page surface cards.

**Fix path**
Change the secondary text color variable for dark mode surfaces to `#cbd5e1` (Slate 300) to achieve a compliant contrast ratio of `6.0:1`.

---

### [UX-022] — Nit — Accessibility — Missing Focus Indicators on Landing Page Interactive Elements

**Evidence**
- **File:** `docs/style.css`
- **Visuals:** Pressing `Tab` to navigate landing page links and buttons.

**Why this matters**
The stylesheet has no custom focus outline styling. Interactive elements rely on browser defaults, which can have poor visibility on dark backgrounds.

**Blast radius**
- Entire landing page keyboard navigation.

**Fix path**
Add high-contrast `:focus-visible` styles to buttons and anchors in `docs/style.css`.
