# UI/UX Deep-Dive — CivicNewspaper (CivicNews)

**Audit date:** 2026-05-28
**Role:** Senior UI/UX Designer
**Scope audited:** The two UI surfaces touched by the uncommitted working tree on branch `v0.2.5-hotpatch`:
(A) the new plain-language-rewrite diff modal in `src/components/Workbench.tsx` + `src/App.css` (+ `src/components/Workbench.test.tsx`); (B) the per-platform download buttons on the landing page driven by `docs/script.js` (HTML in `docs/index.html`, styles in `docs/style.css`).
**Auditor posture:** Balanced

---

## TL;DR

The rewrite-to-diff-modal change is a genuine UX upgrade — replacing a destructive, irreversible `window.confirm` overwrite with a side-by-side preview that the editor can accept or reject is exactly the right move for a newsroom tool where draft content is the product. The modal is clean, themed consistently, and has a labelled, scannable two-pane layout. The weakest dimension is **accessibility and modal mechanics**: the dialog announces `role="dialog" aria-modal="true"` but has no `aria-labelledby`, no Esc-to-close, no focus trap, and no initial focus — it is keyboard-incomplete. The diff also relies on a low-alpha background tint as the **only** signal for added/removed lines, which fails colorblind and low-contrast users. On the landing page, the per-platform asset-resolution JS is well-engineered and live-verified working (Windows→`.exe`, mac→`.dmg`, Linux→`.deb`, with the correct platform highlighted), but a pre-existing CSS source-order bug means the download cards **do not stack on mobile** and overflow the viewport — the recommended card is clipped off-screen. The single most important takeaway: the diff modal needs focus management + a non-color diff signal before it ships, and the mobile download layout is broken at the exact moment a mobile visitor is deciding to install.

## Severity roll-up (UX)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 1 |
| Major | 4 |
| Minor | 4 |
| Nit | 3 |

## What's working

- **The core interaction redesign is correct.** Previously the rewrite overwrote the draft after a one-way `window.confirm("...This cannot be undone.")` (`Workbench.tsx`, old line removed in diff). It now opens a non-destructive side-by-side preview (`Workbench.tsx:412-457`) where the editor reviews before applying. For a tool whose entire value is verified, citation-linked draft text, making an AI rewrite *reviewable and reversible* is the right product decision. Credit.
- **The diff modal copy is honest and instructive.** The helper line — *"Compare the original draft with the AI rewrite. Removed lines are marked on the left, new lines on the right. Accept to replace the draft, or reject to keep the original."* (`Workbench.tsx:418-420`) — tells the user exactly what they're looking at and what each button does. This is better microcopy than most diff UIs ship.
- **Action labels are verbs with object, not vague nouns.** "Accept Rewrite" and "Reject" (`Workbench.tsx:440-452`) read clearly; the primary/secondary button hierarchy (`btn-primary` for accept, `btn-secondary` for reject) and the `flex-between` split placement are conventional and correct.
- **Pane labelling is well-considered.** Each pane has an uppercase, letter-spaced "Original" / "Plain-Language Rewrite" header (`.diff-pane-header`, `App.css:614-621`) so the panes are self-describing without relying on left/right convention alone.
- **The download asset-resolution logic is robust and degrades safely.** `docs/script.js:73-155` resolves each button to the real installer from the GitHub latest release, prefers sensible asset types (`.exe`→`.msi`, `.appimage`→`.deb`), uses User-Agent Client Hints for Apple-Silicon detection, and — critically — *leaves the safe `releases/latest` fallback href in place* on any failure rather than rewriting to a broken link (`script.js:151-154`). Live-verified: Windows resolved to `CivicNewspaper_0.2.4_x64-setup.exe`, mac to `…x64.dmg`, Linux to `…amd64.deb`.
- **Platform highlight works and is honest.** Live on Windows, the Windows card carried the `highlighted` class and the "Recommended Platform" badge (`style.css:620-641`); the other two were neutral. The detection block (`script.js:10-28`) is unchanged by this diff and continues to function.

## What couldn't be assessed

- **Live runtime verification of the diff modal was NOT possible**, and this is a hard limitation worth stating plainly. The modal only renders after `plainLanguageRewrite(...)` resolves, and that IPC call requires the Tauri Rust backend plus a running Ollama sidecar to produce a real rewrite. A plain browser / static preview cannot exercise it. Everything in this report about the modal is assessed from the JSX (`Workbench.tsx:412-457`), the CSS (`App.css:590-636`), and the component test (`Workbench.test.tsx:154-205`) — **not** from observed runtime behavior. In particular, the modal's behavior with a genuinely long draft (scroll sync, performance of the O(n·m) LCS on a large article), with multi-paragraph wrapping, and the visual rendering of the red/green tints in real dark/light mode were reasoned about, not seen. A QA pass inside the running Tauri app is recommended to close this gap.
- **The landing page WAS verified live** via the `civicnewspaper-docs` static server (port 4406) at 1440px and at a clamped narrow viewport (`clientWidth` 320, `matchMedia('(max-width: 768px)')` → true). The preview tool clamps reported `innerWidth` to ~497 at DPR 2, so exact 320px pixel measurements are approximate, but the media-query state and the broken stacking were confirmed directly.

---

## First impressions

**Landing page:** Within five seconds the value proposition is clear — "The newsroom for the community observer," local-first, zero runtime, three platform download buttons front and center. The eye lands on the highlighted (blue, gradient) recommended-platform card, which is the right place for it to land. Strong first impression on desktop.

**Diff modal (from source):** A first-time editor clicks "Plain Language Rewrite," waits through a "Rewriting…" button state, and is presented with a clear two-column before/after. The intent is immediately legible. The friction only appears on inspection: there's no way to dismiss with Esc, keyboard focus isn't moved into the dialog, and the only thing distinguishing a removed line from a kept line is a faint red wash that a colorblind user may not perceive.

## Journey walkthroughs

### Journey: Editor refines a draft → AI rewrite → review → apply

1. Editor is in the Workbench with a draft loaded. Clicks **"Plain Language Rewrite"** (`Workbench.tsx:299-320`). Button disables and reads "Rewriting…". Good feedback.
2. On success, the diff modal appears (`Workbench.tsx:412`). On failure, the inline error span renders (`Workbench.tsx:294-298`) and no modal opens. The success path is clear; the failure path is *inline*, not in the modal, which is fine but means a mid-review failure has no in-context surface (the modal is gone or never appeared).
3. Editor reviews two panes, clicks **Accept Rewrite** (applies, closes) or **Reject** (discards, closes). Both verified by tests (`Workbench.test.tsx:172-205`).
4. **Gap:** there is no path to dismiss the modal with the keyboard (Esc), no click-outside-to-close, and focus is not trapped — a keyboard or screen-reader user can tab *out of* the modal into the page behind it. See UX-001.
5. **Gap:** if the editor's draft is empty, the button silently no-ops (`if (!selectedDraft.content) return;`, `Workbench.tsx:304`) with zero feedback — the user clicks and nothing happens. See UX-005.

### Journey: Visitor on a phone decides to install

1. Visitor lands on the hero on a phone. The three download cards are meant to stack into one column at ≤768px (`style.css:530-534`).
2. **They don't.** Live-verified: at narrow width the cards stay in a 3-column grid and overflow the viewport; the highlighted/recommended Windows card is pushed partly off the left edge with its badge clipped. The primary install CTA is degraded at the exact decision moment. See UX-002.

---

## Findings

> **Finding ID prefix:** `UX-`
> **Categories:** Visual hierarchy / Copy / State / Accessibility / Responsive / Journey / Pattern / Motion / IA

### [UX-001] — Critical — Accessibility — Diff modal has no focus management, no Esc-to-close, and no `aria-labelledby`

**Evidence**
`src/components/Workbench.tsx:415-455`. The dialog is declared:
```jsx
<div className="modal-overlay" id="rewrite-diff-modal" role="dialog" aria-modal="true">
  <div className="modal-content modal-content-wide">
    <h3 style={{ marginTop: 0 }}>Review Plain-Language Rewrite</h3>
```
The `<h3>` is not referenced by an `aria-labelledby` on the dialog, so the accessible name of the dialog is empty. There is no `onKeyDown`/Esc handler, no `ref`-based initial focus into the dialog, and no focus trap. Grep across `src/` confirms the only `.focus()` call in the component is the unrelated citation textarea (`Workbench.tsx:137`); no modal in the app has keyboard handling (`AppContent.tsx:220`, `SourcesPanel.tsx:224,277`). Assessed from source — live modal verification not possible (see "What couldn't be assessed").

**Why this matters**
`aria-modal="true"` is a *promise* to assistive tech that focus is constrained to the dialog and the rest of the page is inert. Here that promise is false: a keyboard or screen-reader user can Tab straight past the Accept/Reject buttons and out into the page behind the overlay, where they can interact with controls that are visually obscured and supposedly disabled. There is no Esc escape hatch — the muscle-memory dismissal for every modal on the web does nothing. And with no `aria-labelledby`, a screen reader announces an unnamed dialog, so the user doesn't know what just opened. For a desktop tool an editor uses all day, this is a recurring friction, and it is a WCAG 2.1 keyboard/focus failure on a brand-new component.

**Blast radius**
- Adjacent code: the same gap exists in *every* modal in the app — `AppContent.tsx:220`, `SourcesPanel.tsx:224` (bulk import), `SourcesPanel.tsx:277` (discovery). None has `role="dialog"`, focus management, or Esc. Fixing the diff modal in isolation leaves three siblings broken.
- Shared state: all four share the `.modal-overlay`/`.modal-content` CSS contract (`App.css:565-588`). A shared `<Modal>` wrapper that owns ARIA + focus trap + Esc would fix all four at once.
- User-facing: keyboard/SR users gain a working, dismissible, named dialog; mouse users are unaffected.
- Migration: none — additive behavior.
- Tests to update: add Esc-to-close and focus-trap assertions to `Workbench.test.tsx` (currently absent). Other modals have no modal-behavior tests to break.
- Related findings: UX-006 (no shared Modal component is a pattern gap), QA/Test deep-dives likely flag the missing keyboard tests.

**Fix path**
Extract a `<Modal>` component that: (1) renders the overlay, (2) sets `aria-labelledby` to the heading's `id`, (3) on mount moves focus to the first focusable element (or the dialog container with `tabIndex={-1}`), (4) installs a `keydown` listener for Esc → close and Tab → wrap focus within the dialog, (5) restores focus to the trigger on close. Minimal local fix if a wrapper is out of scope for the hotpatch: add `aria-labelledby="rewrite-diff-title"` + `id="rewrite-diff-title"` on the `<h3>`, an `onKeyDown` on the overlay that calls `setRewritePreview(null)` on `Escape`, and an `autoFocus` on the Reject button (the safe default — Reject preserves the user's work).

---

### [UX-002] — Major — Responsive — Download cards do not stack on mobile; recommended card overflows off-screen

**Evidence**
Live-verified on the running landing page (`civicnewspaper-docs`, port 4406). At a narrow viewport (`document.documentElement.clientWidth` = 320, `matchMedia('(max-width: 768px)').matches` = `true`), `getComputedStyle('.download-container').gridTemplateColumns` returned `"198.103px 198.103px 198.103px"` — i.e. still three columns — and the Windows card's bounding rect was `x: -164` (off the left edge). Screenshot confirms the three cards rendered side-by-side overflowing the viewport, with the highlighted Windows card's "Recommended Platform" badge clipped to "…RM" at the left edge.

Root cause: source order. The mobile override `.download-container { grid-template-columns: 1fr; }` lives inside `@media (max-width: 768px)` at `style.css:530-534`, but the desktop rule `.download-container { grid-template-columns: repeat(3, 1fr); }` at `style.css:566-568` appears **later in the file** with equal specificity. The later declaration wins even when the media query matches, so the cards never collapse to one column.

**Why this matters**
A mobile visitor — exactly the casual community-observer audience this product targets — arrives at the install decision and is shown three cards crammed off the side of the screen, with the *recommended* card (the one the highlight logic worked to surface) the most clipped. This is the primary conversion moment of the page, degraded. It also produces horizontal awkwardness and makes the "First time installing?" callouts hard to reach. This is pre-existing CSS (not introduced by the `script.js` diff), but it directly undermines the in-scope download surface the diff is investing in.

**Blast radius**
- Adjacent code: any other component placed *before* its own desktop rule in `style.css` could share the same ordering hazard; the `.download-container` is the confirmed instance. Audit `.features-grid`, `.arch-grid` ordering as a precaution (arch-grid override is at `style.css:517` and its base later — verify it actually collapses).
- Shared state: the `@media (max-width: 768px)` block at `style.css:440`.
- User-facing: mobile visitors get a clean single-column download stack; desktop unchanged.
- Migration: none.
- Tests to update: none exist (static page, no responsive snapshot tests).
- Related findings: UX-007 (Recommended Platform badge clipping is a symptom of the same overflow).

**Fix path**
Move the desktop `.download-container { grid-template-columns: repeat(3, 1fr); }` rule (lines 566-573) so it appears *before* the `@media (max-width: 768px)` block, OR scope the desktop rule in a `@media (min-width: 769px)` query, OR bump the override's specificity. The min-width approach is cleanest and removes the ordering fragility: wrap the 3-column rule in `@media (min-width: 769px)` and leave the mobile rule as the unconditional base. Verify live at 320/375/768.

---

### [UX-003] — Major — Accessibility — Diff color-coding uses background tint as the *only* signal (colorblind / low-contrast failure)

**Evidence**
`src/App.css:630-636`:
```css
.diff-line-removed { background-color: rgba(239, 68, 68, 0.18); }
.diff-line-added   { background-color: rgba(16, 185, 129, 0.18); }
```
Removed = red wash, added = green wash, both at 0.18 alpha. There is no `+`/`−` gutter marker, no icon, no border, no text-decoration. Red-vs-green is the single hardest pair for the most common form of color blindness (deuteranomaly/protanomaly). At 0.18 alpha the tint is also very faint over `--bg-app` in both themes, so even fully-sighted users may miss it. The pane headers ("Original" / "Plain-Language Rewrite") tell you which *side* is which, but within a pane there is nothing non-color to distinguish a changed line from an unchanged one. Assessed from CSS — live rendering not verified.

**Why this matters**
The entire purpose of this modal is to let the editor *see what changed* before accepting. If a red-green colorblind editor (≈8% of men) cannot perceive which lines were dropped, the diff is decorative, not functional — they're back to accepting blind, which is the exact risk the modal was built to remove. Faint tints compound the problem for everyone in bright-room/glare conditions.

**Blast radius**
- Adjacent code: only this modal uses `.diff-line-*` today, so the fix is contained to `App.css:623-636` + the span rendering in `Workbench.tsx:424-436`.
- User-facing: colorblind and low-vision editors gain a usable diff; others get a clearer, stronger signal.
- Migration: none.
- Tests to update: none assert on diff styling; consider adding a test that removed/added lines carry a non-color marker class.
- Related findings: UX-001 (both are a11y gaps on the same new component — fix together).

**Fix path**
Add a redundant non-color signal: render a leading gutter glyph on changed lines (`−` for removed, `+` for added) inside the `.diff-line` span, and/or a 3px left border (`border-left: 3px solid` in a darker red/green). Raise the tint alpha to ~0.30–0.35 so the fill is perceptible. Keep the color *and* add the glyph — color is fine as a secondary channel, just not the sole one. Suggested: prepend the row text with `row.type === "removed" ? "− " : row.type === "added" ? "+ " : "  "`.

---

### [UX-004] — Major — State — Diff modal has no error state and no loading state inside the dialog

**Evidence**
The rewrite call's loading state is only the trigger button text "Rewriting…" (`Workbench.tsx:319`); the error state is an inline span beside the button (`Workbench.tsx:294-298`) that uses `var(--color-danger)` — a variable that is **not defined** (the theme defines `--color-error`, `App.css:24/57`), so the error text falls back to the inherited color rather than the intended danger red. The diff modal itself (`Workbench.tsx:412-457`) renders only the success state; there is no "generating…" skeleton and no in-modal error surface. The component test (`Workbench.test.tsx`) covers open/accept/reject but has **no** test for the loading or error path.

**Why this matters**
A local Ollama rewrite can be slow or fail (model not loaded, sidecar down, timeout). On failure the modal never opens and the error appears as small inline text the user may not connect to the button they just clicked — and because of the `--color-danger` typo it isn't even rendered in the intended alarming red. The user's mental model ("I clicked rewrite, where's my diff?") isn't answered in context. On a slow rewrite, the only feedback is a four-character button label change, easy to miss.

**Blast radius**
- Adjacent code: `--color-danger` is referenced at `Workbench.tsx:295`; grep for other `--color-danger` uses before renaming — the danger color may be mis-referenced elsewhere. The canonical token is `--color-error`.
- User-facing: editors get visible, correctly-colored feedback on slow/failed rewrites.
- Migration: none.
- Tests to update: add loading-state and rejected-promise (error) tests to `Workbench.test.tsx`; both paths are currently untested.
- Related findings: UX-001 (same component), Test deep-dive (coverage gap on these states).

**Fix path**
(1) Fix the token: `var(--color-danger)` → `var(--color-error)` at `Workbench.tsx:295`. (2) Consider surfacing the error inside a small modal/toast rather than an easily-missed inline span, or at minimum keep the error span visually anchored to the button with an icon (it already has `<AlertTriangle>`). (3) For long rewrites, a more prominent in-flight indicator (e.g. a spinner next to "Rewriting…") would help. (4) Add the two missing state tests.

---

### [UX-005] — Minor — State — "Plain Language Rewrite" silently no-ops on an empty draft

**Evidence**
`Workbench.tsx:303-304`: `onClick={async () => { if (!selectedDraft.content) return; ... }`. The button is not disabled when `content` is empty; it simply returns with no feedback. (The button *is* correctly disabled while `isRewriting`.)

**Why this matters**
An editor with an empty draft clicks "Plain Language Rewrite" and absolutely nothing happens — no error, no tooltip, no disabled affordance. Silent no-ops read as "the app is broken." Low exposure (rewriting an empty draft is unusual) but the fix is trivial.

**Blast radius**
- User-facing: clearer affordance; no silent dead clicks.
- Related findings: UX-004 (state-handling theme on this control).

**Fix path**
Disable the button when `!selectedDraft.content` (`disabled={isRewriting || !selectedDraft.content}`) and add a `title`/tooltip "Add draft text to rewrite." This turns an invisible no-op into a self-explaining disabled state.

---

### [UX-006] — Minor — Pattern — No shared Modal component; ARIA/focus correctness will keep drifting per-modal

**Evidence**
Four hand-rolled modals share only the `.modal-overlay`/`.modal-content` CSS: `Workbench.tsx:415`, `AppContent.tsx:220`, `SourcesPanel.tsx:224`, `SourcesPanel.tsx:277`. Only the new diff modal has `role="dialog"`/`aria-modal`; the other three have neither. None has focus management or Esc.

**Why this matters**
Each new modal re-implements (or forgets) accessibility from scratch. The diff modal is actually *ahead* of its siblings (it at least declares the role) — which proves the inconsistency. Without a shared primitive, UX-001's fix won't propagate and the next modal will start from zero again.

**Blast radius**
- Adjacent code: all four modal sites; a `<Modal>` wrapper centralizes overlay, ARIA, focus trap, Esc, and scroll-lock.
- User-facing: consistent dismissal and screen-reader behavior across the whole app.
- Migration: mechanical refactor of four call sites.
- Related findings: UX-001 (the concrete a11y instance), UX-003 (same new component).

**Fix path**
Introduce `src/components/Modal.tsx` owning overlay + `role="dialog"` + `aria-labelledby` + focus trap + Esc + body scroll-lock, and migrate the four existing modals to it. Do this once and UX-001 is solved everywhere.

---

### [UX-007] — Minor — Responsive — "Recommended Platform" badge clips when the card overflows

**Evidence**
`style.css:628-641` positions the badge `::after` at `top: -11px`. Live at narrow width the highlighted card sat at `x: -164`, so the badge rendered as "…RM" clipped against the viewport's left edge (screenshot). This is a downstream symptom of UX-002 but also worth noting: the badge has no max-width/ellipsis handling and the absolute-positioned `::after` can clip even in valid layouts on very narrow cards.

**Why this matters**
A clipped "Recommended Platform" label looks broken and undercuts the trust signal it's meant to convey. Mostly resolves once UX-002 is fixed (single-column cards are full-width), but the badge's absolute positioning is fragile.

**Blast radius**
- Adjacent code: `.download-card.highlighted::after` (`style.css:628-641`); the mobile transform override (`style.css:536-538`) only reduces scale, doesn't reposition the badge.
- User-facing: clean recommended badge on mobile.
- Related findings: UX-002 (root cause).

**Fix path**
Fix UX-002 first (single-column = full-width cards, badge no longer clipped). Defensively, give the badge `white-space: nowrap` and ensure the card has enough top padding/`overflow: visible` so the `-11px` offset isn't cut by an ancestor.

---

### [UX-008] — Minor — Accessibility / IA — Three identical "First time installing?" links share one accessible name

**Evidence**
`docs/index.html:48-52, 59-63, 70-74`: each download card contains `<a href="install.md" class="callout-link">… First time installing?</a>`. All three have identical text and identical href. A screen-reader user listing links hears "First time installing?" three times with no way to tell which platform each belongs to.

**Why this matters**
Ambiguous repeated link text is a recognized a11y/IA smell. It's minor here because all three go to the same page, but it still degrades the screen-reader and link-navigation experience. (Note: a separate, larger concern — `install.md` is served as raw `text/markdown` and renders as unstyled markdown source in the browser — is pre-existing and outside this diff's scope; flagged here only because the in-scope cards link to it.)

**Blast radius**
- Adjacent code: the three `.callout-link` anchors in `index.html`.
- User-facing: clearer link navigation for SR users.
- Related findings: none in scope.

**Fix path**
Add a platform-specific `aria-label` to each callout link, e.g. `aria-label="First time installing on Windows?"`. Leave the visible text as-is. Separately, recommend (out of scope) rendering `install.md` as HTML so the "First time installing?" journey doesn't dead-end on raw markdown.

---

### [UX-009] — Nit — Copy — Feature name drift: "Plain Language Rewrite" (app) vs "Plain Language Summary & Rewrite" (site)

**Evidence**
App button: "Plain Language Rewrite" (`Workbench.tsx:319`). Landing page feature card: "Plain Language Summary & Rewrite" (`docs/index.html:102`). The modal title is "Review Plain-Language Rewrite" (hyphenated) while the button is "Plain Language Rewrite" (unhyphenated) — `Workbench.tsx:319` vs `:417`.

**Why this matters**
Minor naming/hyphenation drift between marketing and product, and within the product itself. Trivial, but consistent terminology builds trust.

**Fix path**
Pick one form (recommend hyphenated "Plain-Language Rewrite" everywhere, since it's a compound modifier) and align the button, modal title, and site copy.

---

### [UX-010] — Nit — Visual hierarchy — Diff modal action buttons are full-size while the trigger is `btn-sm`

**Evidence**
The trigger is `btn btn-secondary btn-sm` (`Workbench.tsx:300`); the modal's Accept/Reject are full-size `btn` (`Workbench.tsx:440,444`). Not wrong — modal actions deserve more weight — but worth a deliberate check that the larger buttons sit comfortably in the `flex-between` footer at 900px max-width.

**Fix path**
Confirm visually in the running app (couldn't verify live). If the footer feels unbalanced, the two buttons are fine as-is; this is a confirmation item, not a defect.

---

### [UX-011] — Nit — Accessibility — Download buttons rely on the default browser focus ring; only the nav hamburger has a custom `:focus-visible`

**Evidence**
`docs/style.css:727-731` defines `:focus-visible` outlines only for the nav toggle/hamburger. The `.download-btn` and other anchors have no custom focus style (live: `getComputedStyle(btn).outline` → `"none 0px"`), relying on UA defaults. No global `outline: none` reset exists (verified by grep), so default rings still appear — but they're not tuned to the dark glassmorphic theme.

**Why this matters**
Default focus rings can be low-contrast against the dark gradient buttons. Keyboard users can still see focus (no hard failure), but a tuned ring would be clearer and on-brand.

**Fix path**
Add `.download-btn:focus-visible, .callout-link:focus-visible, .nav-links a:focus-visible { outline: 2px solid var(--accent-primary); outline-offset: 3px; }` to match the existing hamburger treatment.

---

## States audit matrix

| Component / page | Default | Loading | Empty | Error | Partial | Notes |
|---|---|---|---|---|---|---|
| Rewrite trigger button | ✓ | ✓ ("Rewriting…") | ✗ silent no-op | ✓ inline (mis-colored) | — | UX-004, UX-005 |
| Diff modal | ✓ | ✗ (no in-modal loader) | ⚠ empty draft never opens it | ✗ (no in-modal error) | ✓ unequal pane lengths handled | UX-004; long-draft perf unverified |
| Download card | ✓ | n/a | n/a | ✓ safe fallback href | ✓ partial assets handled | script.js degrades well; UX-002 responsive |
| Platform highlight | ✓ | n/a | ✓ (no detection → no highlight) | n/a | — | Works live |

## Accessibility snapshot

- **Keyboard navigation:** Landing page tab order is logical (nav → hero downloads → callouts → arch → footer), verified live. Diff modal is keyboard-incomplete — no Esc, no focus trap, focus not moved into the dialog (UX-001).
- **Focus visibility:** Custom `:focus-visible` only on the nav hamburger (`style.css:727-731`); download buttons use UA defaults (UX-011). Diff modal has no managed focus (UX-001).
- **Color contrast:** Diff add/removed tints at 0.18 alpha are faint and red/green-only (UX-003). Highlighted download button is white-on-gradient (high contrast, good); neutral mac/linux buttons are `rgb(248,250,252)` on a near-transparent fill over the dark hero (adequate). Pane headers use `--text-secondary` — acceptable for uppercase labels.
- **Screen reader labeling:** Diff dialog has `role="dialog" aria-modal` but no accessible name (`aria-labelledby` missing) — UX-001. Three identical "First time installing?" links (UX-008). Lucide icons in buttons are decorative beside text labels, so acceptable.
- **Reduced motion:** Landing page uses `transition: all 0.4s cubic-bezier(...)` on cards and fade-up scroll animations; no `prefers-reduced-motion` guard observed (pre-existing, out of strict scope but noted). Diff modal has no animation.
- **Touch target size:** Download buttons ~78px tall (live-measured) — comfortably ≥44px. "First time installing?" callout link ~25px tall — below the 44px touch target minimum, a borderline concern on mobile (compounded by UX-002).

## Patterns and systemic observations

- **Pattern: modal accessibility is implemented ad hoc, not via a shared primitive** — UX-001, UX-003, UX-006 all stem from the absence of a `<Modal>` component. The new diff modal is the most-correct of the four (it at least declares `role="dialog"`), which paradoxically highlights the inconsistency. A single shared `<Modal>` owning ARIA + focus trap + Esc is the highest-leverage fix: it closes UX-001 here and pre-empts the same gap in `AppContent.tsx:220` and `SourcesPanel.tsx:224,277`.
- **Pattern: the diff modal's interaction redesign is right but its non-happy-path states are thin** — UX-003, UX-004, UX-005 are all "the success path is polished, the loading/error/empty/colorblind paths are not." One focused pass on states-and-signals finishes the component.
- **Pattern: CSS source-order fragility on the landing page** — UX-002/UX-007 come from a desktop rule overriding a media-query rule by appearing later. Worth a quick sweep of the other responsive overrides (`.arch-grid`, `.features-grid`) to confirm none share the same ordering hazard.

## Appendix: surfaces reviewed

- `src/components/Workbench.tsx:1-65` (computeLineDiff), `:296-330` (rewrite trigger), `:412-457` (diff modal JSX)
- `src/App.css:18-60` (theme tokens), `:504-518` (button classes), `:564-636` (modal + diff styles)
- `src/components/Workbench.test.tsx:1-205` (mock setup + 3 new modal tests)
- `docs/index.html:43-83` (download container/cards/callouts)
- `docs/script.js:10-28` (platform highlight, unchanged), `:73-155` (per-platform asset resolution, new)
- `docs/style.css:440-539` (768px media block), `:565-653` (download cards, buttons, highlighted badge), `:727-731` (focus-visible)
- Live: landing page via `civicnewspaper-docs` static server (port 4406) at 1440×900 and at narrow viewport (`clientWidth` 320, `matchMedia('(max-width:768px)')` = true). Verified: real GitHub asset hrefs resolved (`v0.2.4-hotpatch` .exe/.dmg/.deb), Windows card highlighted with "Recommended Platform" badge, 3-column download grid failing to collapse on mobile (cards overflow, recommended card clipped at `x:-164`).
- **Not** verifiable live: the diff modal (requires Tauri backend + Ollama to produce a rewrite) — assessed from source/CSS/tests only.
