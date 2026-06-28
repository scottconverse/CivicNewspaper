# CivicNewspaper / "The Civic Desk" — Beta-Readiness Assessment & Improvement Plan

_Generated 2026-06-25 against commit `29dca4b` (v0.2.7). Based on a local build, full
test run, and a 40-agent adversarial code review (30 of 33 findings confirmed)._

## Verdict

**Close to a defensible public beta, but not quite there.** The security *plumbing* is
genuinely strong for a pre-alpha — the failure modes that usually sink local-first apps
are hardened. The remaining blockers are all at the **editorial trust boundary**, which
is exactly where a journalism tool's reputation lives. Fix three things and this is a
beta you can stand behind.

### Verified strengths
- Parameterized SQL throughout; constant-time token/PIN comparison.
- Layered loopback auth: Host-header anti-DNS-rebind + Origin allowlist + mandatory
  bearer token on every non-pair route + per-IP PIN rate limiting.
- DNS-pinned SSRF defense with per-hop redirect re-validation and a redirect cap.
- SHA-pinned Ollama sidecar that coexists with a user's own Ollama; sidecar never kills
  a process it didn't spawn.
- **165 tests pass** (75 frontend / 90 backend); security fixes are regression-tested.

## Build & test results (local, this machine)

| Step | Result |
|------|--------|
| `npm ci` | ✅ 144 pkgs, 0 vulnerabilities |
| Frontend tests (`vitest`) | ✅ 75/75 |
| Frontend build (`tsc && vite build`) | ✅ clean |
| Design-token lint gate | ✅ pass |
| Backend `cargo test --all` | ✅ 90/90 |
| Full app bundle (`tauri build`) | ⚠️ not attempted — needs the 1.4 GB Ollama sidecar |

> Note: a fresh checkout cannot even `cargo test` without first fetching the ~hundreds-of-MB
> sidecar, because `tauri_build::build()` hard-fails on the missing `externalBin` — even
> though `build.rs`'s own comment says it intends `cargo test` to stay usable. That
> intent/behavior gap is itself worth fixing (see P3 / dev-experience).

---

## BETA BLOCKERS (fix before a public beta)

> Each was independently verified as a real, high-confidence **P1**; the synthesis
> elevates them to beta-gating because they sit on the public-facing artifact / legal
> surface.

### B1 — Stored XSS in published sites via markdown URI schemes  · effort: S
`src-tauri/src/core/compiler.rs:42-53` (`render_markdown`), used at `:122`
`render_markdown` filters raw-HTML events but **not** markdown-link/image URI schemes, so
`[click](javascript:alert(document.domain))` survives into the published static HTML as a
live `<a href="javascript:...">`. Draft content originates from the LLM (summarizing
attacker-controllable scraped RSS/HTML) and from the loopback `POST /api/drafts` endpoint.
The v0.2.6 XSS fix and its passing tests only covered *profile fields* — this vector is
uncovered. (Caveat: the link vector requires a victim click; `<img src=javascript:>` does
not auto-execute. Still a stored XSS on the public site.)
- Allowlist link/image destination schemes (`http`, `https`, `mailto`, `evidence:`) in the
  existing `Parser` filter; strip/rewrite anything else.
- Add a unit test: `[x](javascript:alert(1))` and `![x](data:text/html,...)` render inert.
- Add a CSP meta tag (`default-src 'self'; script-src 'none'`) to `templates/post.html`
  and `index.html` as defense-in-depth (the site has none).

### B2 - SUPERSEDED: guardrails are advisory, never a publish veto  ? effort: M

This older recommendation is intentionally superseded. Guardrails may warn, rank,
request editor notes, and preserve review concerns in compiler output, but they
must never veto the editor's decision to approve or publish.

### B3 - SUPERSEDED: no mandatory publication disclosure gate  ? effort: M

This older recommendation is intentionally superseded. Human review reminders,
AI disclosures, sourcing notes, ownership language, ad policy, and footer text
belong to publisher-controlled configuration. The app must not invent those
claims or require them as a publication precondition.

---

## P1 — fix early in beta

### P1a — Add a test gate to the release pipeline  · effort: M
`release.yml` builds & publishes installers with **no** `cargo test`/`clippy`/`npm test`
and no `needs:` on a test job; `ci.yml` branch filters never match tag pushes, so
`git push origin v0.2.7` ships installers with no CI on that commit. Only convention
(merge→main→tag) protects this.
- Add a test job to `release.yml` that the release job `needs:`, or `releaseDraft: true`.
- Require green status checks on release tags via branch protection.
- `verify-release.sh`: hard-fail on zero artifacts in CI mode; fix bundle search paths to
  `src-tauri/target/<triple>/release/bundle` (currently a silent no-op).

### P1b — Improve guardrail accuracy so the now-blocking checks have signal  · effort: M
`src-tauri/src/core/guardrails.rs:50-71,116-137`
Keyword + paragraph-wide matching: "alleged" anywhere in the paragraph satisfies the rule;
benign "combat fraud" misfires; synonyms/inflections missed; citation check is substring
presence only. Once these block (B2), accuracy becomes load-bearing.
- Sentence/clause-level proximity for the presumption-of-innocence rule.
- Exclude direct quotes/blockquotes; expand the lexicon; document limits in-UI.

---

## P2 — later in beta
- **Close the highest-value test-coverage gaps** (M): 5/8 detectors untested (incl.
  high-risk elevation); HTTP business endpoints tested only as auth probes; backup
  version-gating + verbatim-overlap failure paths untested. Add table-driven detector
  tests, a mock-`LlmClient` injection for `POST /api/drafts`, and one e2e happy-path.
- **Plan code signing / notarization for pre-1.0** (L): no Authenticode/Apple
  notarization; integrity depends on users hand-checking SHA256SUMS. Acceptable for beta
  (documented), but real install friction.
- Detector quality vs. real municipal data (regex misses `$1.2M`/`250k`; no PDF ingestion,
  and municipal records are overwhelmingly PDF → recall on the dominant format ≈ 0).
- Beta/onboarding warnings cover install/technical but omit editorial-safety expectations.

## P3 — defense-in-depth & hygiene (batch when touching the files)
- Add NAT64/6to4 embedded-IPv4 unwrapping to `is_blocked_ip` (IPv6 arm).
- `record_paired_client_use`: update by `id` not `token` (consistency w/ constant-time path).
- `save_community_profile`: atomic temp-write+rename (mirror `backups.rs`).
- Startup reconciliation: mark stale `in_progress` daily-scan runs as failed.
- `chunk_text`: hard-split oversized single paragraphs on char boundaries.
- `--locked` on cargo in CI + tauri-action; DoD hash-integrity gates fail (not skip) on
  missing sidecar files for `v*` branches.
- Associate onboarding form labels with inputs (`htmlFor`/`id`) — accessibility.
- Surface app version in the running UI (`SystemStatus` is currently dead code); add a
  privacy-preserving "check for updates" nudge.

---

## Quick wins (high value / low effort)
1. **B1 markdown URI sanitizer** — ~10-line allowlist in the existing filter closes the XSS blocker.
2. CSP meta tag on `post.html` + `index.html` — one line per template.
3. Editorial-responsibility acknowledgment in onboarding/BetaNotice — copy-only (addresses B3 partially).
4. `update_source_status`: set `last_scraped` on every attempt — one-line SQL that resurrects a dead detector.
5. `record_paired_client_use` update-by-id — trivial.
6. `save_community_profile` atomic write — reuse the `backups.rs` pattern.
7. `verify-release.sh` zero-artifact fail + path fix — restores real per-platform release verification.
8. `htmlFor`/`id` on onboarding inputs + model `<select>` — mirror `PublishPanel`.

## Strategic bets (1.0)
1. **Treat the editorial trust boundary as the product's true core** — build one coherent
   human-in-the-loop publishing gate (blocking guardrails + verification attestation +
   reader AI disclosure), not three separate patches.
2. **Factor LLM/guardrails/draft orchestration into shared core functions** called by both
   the Tauri command and the Axum handler, so enforcement can't drift between the UI and
   the loopback HTTP path. Document the loopback server in an ADR.
3. **Detector quality + PDF ingestion** validated against a real municipal corpus.
4. **A real release-trust story**: enforced test gates on tags, signing/notarization, and a
   privacy-preserving update-awareness nudge so security fixes don't ship silently.
