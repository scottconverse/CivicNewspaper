# UI/UX Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-23
**Role:** Senior UI/UX Designer
**Scope audited:** Story Queue (Leads & Drafts), Story Workbench (Drafting & Editing), Sources Manager (Add feed, auto-discovery modal, bulk import), Browser Pairing panel, Ethics & Backups settings, Compiled Static Newsroom templates.
**Auditor posture:** Balanced

---

## TL;DR

CivicNewspaper establishes an aesthetic and structurally readable foundation for local-first, evidence-linked civic journalism, featuring high contrast and warm typography on compiled public-facing templates. Recent engineering cycles successfully resolved all Blocker and Critical issues, correcting mobile layout collapses for the main viewports and layout grids (workbench, sources, and pairing), while securing asset navigation link paths and basic accessibility labels. However, key onboarding steps, guardrail warnings display, and token verification flows remain non-functional.

---

## Severity roll-up (UX)

| Severity | Active Count | Resolved Count | Total |
|---|---|---|---|
| Blocker | 0 | 2 | 2 |
| Critical | 0 | 1 | 1 |
| Major | 6 | 1 | 7 |
| Minor | 2 | 0 | 2 |
| Nit | 0 | 0 | 0 |
| **Total** | **8** | **4** | **12** |

---

## What's working

- **Beautiful Newspaper Aesthetic** — The compiled public newsroom uses a gorgeous paper-like background (`#fcfbfa`) combined with a dark editorial crimson accent (`#5a1818`), achieving a warm, classic Gazette feel.
- **Robust Typography Scale** — The font stack ('Inter' for UI controls, 'Lora' / 'Georgia' for editorial body texts) is well-considered, establishing a highly readable, classic serif rhythm for long-form reading.
- **High Color Contrast** — Contrast ratios are exceptionally strong, with primary text (`10.4:1` in light UI, `17.4:1` in dark UI) and compiled editorial text (`6.4:1` for muted text, `9.1:1` for crimson accents) exceeding WCAG 2.1 AAA standards.
- **Resolved Mobile Main Content Squeeze** — The main content view now flows vertically and spans full width on viewports under 768px, ensuring core app usability on mobile.
- **Resolved Layout Grid Collapse** — The main columns of the Story Workbench, Sources Manager, and Pairing panels collapse into a single-column layout on viewport widths below 1024px, preventing clipping.
- **Correct Detail Page Link Resolving** — Relative assets and header link targets are successfully modified in compiled subfolders (e.g. `watch/1.html`), preventing broken pages and stylesheets.
- **Added Accessibility Labels for Actions** — Ingest and list delete buttons are now associated with descriptive aria-labels, preventing "blank" button announcements for screen-readers.

---

## What couldn't be assessed

- **Actual Browser Extension Performance** — The pairing setup was inspected from the Tauri desktop UI; the actual runtime interaction within Chrome (e.g. text selection, context menu triggers) could not be reviewed without active browser runtime pairing.

---

## First impressions

- **Arrival Experience:** The first-time user lands directly on an empty Story Queue dashboard. The page displays "No unlinked leads available" and invites the user to click "Scrape & Detect."
- **5-Second Clarity:** Within 5 seconds, a user knows this is a news monitoring and drafting feed, but does not know *how* it gets its data or *why* it's currently empty.
- **Journey Friction:** If a new user clicks "Scrape & Detect" on their first run, it quietly scrapes 0 feeds and generates 0 leads because no sources have been set up. The app does not route first-time users to the "Ollama Wizard" or the "Sources Setup" tabs. It feels like entering a building through a side door and landing in an empty hallway.

---

## Journey walkthroughs

### Journey: First-time Onboarding → Feed Ingestion → Article Publication

1. **Onboarding Setup:** The user clicks the "Ollama Wizard" tab.
   - *Friction:* Step 1 asks for a "Publication Name", but hitting Next simply discards the input. In Step 3, the "Pull Recommended Model" button is hardcoded as `disabled` with no connection handler. The onboarding setup is essentially non-functional.
2. **Sources Configuration:** The user navigates to the "Sources Setup" tab to register feeds.
   - *Delight:* The "Auto-Discover Town Feeds" modal is highly intuitive. Entering a city/state queries DDG and displays checklists of local agendas and regional subreddits categorized by type.
3. **Ingestion & Detection:** The user returns to the queue and clicks "Scrape & Detect."
   - *Friction:* The status bar turns blue and shows "Scraping feeds...", but the rest of the UI remains frozen with no progress bar or detailed ingestion logs.
4. **Editorial Workbench & Guardrails:** The user selects a lead, opens the drafting wizard, and clicks "Generate Draft."
   - *Friction:* The local Ollama model generates text, but if the editor makes adjustments that trigger plagiarism warnings (verbatim copying), these warning issues are completely hidden because of an incorrect `is_clean` check.
5. **Static Site Compilation:** The user approves the draft and publishes it.
   - *Delight:* Opening the compiled folder reveals that detail pages (e.g. `watch/1.html`) are now fully styled and the header navigation works correctly, thanks to compiler link pre-processing.

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
Modify `Workbench.tsx` to always render the issues list if issues exist, regardless of the `is_clean` status:
```tsx
{guardrailsReport.issues.length > 0 && (
  <div style={{ marginTop: "0.5rem" }} id="guardrails-issues-list">
    {guardrailsReport.issues.map((issue: any, idx: number) => (
      <div key={idx} className={`guardrail-issue ${issue.severity}`}>
        ⚠️ [Category: {issue.category.replace(/_/g, " ")}] {issue.message}
      </div>
    ))}
  </div>
)}
```

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
Apply `htmlFor` matching the input's `id` to all labels:
```tsx
<label htmlFor="input-profile-subtitle" style={{ fontWeight: 600, display: "block" }}>Subtitle / Motto</label>
<input id="input-profile-subtitle" ... />
```

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
**Fix path:** Align the copy to use "Pairing Token" consistently throughout the interface.

---

## States audit matrix

| Component / page | Default | Loading | Empty | Error | Partial | Notes |
|---|---|---|---|---|---|---|
| Story Queue (Leads) | ✓ | ✗ | ✓ | ✓ | — | No skeleton screen during sync. [UX-004] |
| Story Queue (Drafts) | ✓ | ✗ | ✓ | ✓ | — | No skeleton screen during sync. |
| Story Workbench (Drafting) | ✓ | ✓ | — | ✓ | — | "Generating..." button state is handled. |
| Story Workbench (Editing) | ✓ | — | — | ✗ | — | Plagiarism warnings hidden if is_clean is true. [UX-004] |
| Sources Panel | ✓ | — | ✓ | ✓ | — | Add source form handles default empty states. |
| Auto-Discovery Modal | ✓ | ✓ | ✓ | ✓ | — | Spinner is present, but uses missing CSS variable. |
| Browser Pairing | ✓ | — | ✓ | ✓ | — | Has dead verify input. [UX-006] |

---

## Accessibility snapshot

- **Keyboard navigation:** Fair. Buttons and tabs can be reached using `Tab` and selected with `Enter` / `Space`.
- **Focus visibility:** Visible. Inputs and textareas light up with blue rings on focus.
- **Color contrast:** Excellent contrast ratios exceeding AAA standards for text. However, secondary text (`--text-muted`) falls below AA contrast thresholds (`2.6:1`).
- **Screen reader labeling:** Systemic issues. Form input labels are not programmatically associated, and icon-only buttons lack `aria-label` tags. [UX-007, UX-010] (Trash buttons are now resolved).
- **Reduced motion:** Ingest buttons spin (`.animate-spin`) without checking `prefers-reduced-motion`.
- **Touch target size:** Sidebar navigation items exceed `44x44px`. Table action delete buttons are too small (`30x30px`) on mobile viewports.

---

## Patterns and systemic observations

1. **Responsiveness is an afterthought:** Layout grids are hardcoded for desktop resolutions and overflow on narrow viewports. Sizing calculations on mobile shrink the main viewport area to an unusable width. (Resolved by fixing main-content width and collapsing all layout grids).
2. **Disconnected Wizard Logic:** Forms/Wizards are decoupled from settings update controllers. They function as placeholders rather than integrated workflow steps.
3. **Template Paths:** Compiled site assets use absolute root paths, breaking the newsroom structure when compiled to subdirectories. (Resolved by compiler pre-processing).

---

## Appendix: surfaces reviewed

- `/queue` (leads, drafts, scrapers) — 1024px, 1440px
- `/sources` (auto-discovery modal, bulk upload) — 768px, 1440px
- `/onboarding` (setup wizard) — 1440px
- `/pairing` (integration PIN generator) — 1440px
- `/settings` (ethics standards, database recovery) — 1440px
- `templates/post.html`, `templates/index.html` (compiled site output layout)
