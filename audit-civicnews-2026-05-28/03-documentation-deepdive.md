# Documentation Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-28
**Role:** Technical Writer
**Scope audited:** Scoped to docs affected by the uncommitted working-tree changes on branch `v0.2.5-hotpatch` (v0.2.6 candidate), with the project's doc set as context. Primary targets: `carried-debt.md` (RESOLVED claims P5-001/003/004 + removed P5-002/005/007 + "Pipeline Integrity Incidents") and `docs/index.html` / `docs/script.js` (landing + download copy). Cross-checked: `README.md`, `CHANGELOG.md`, `SECURITY.md`, `FAQ.md`, `docs/architecture.md`, `docs/install.md`, `scripts/audit/mutations.json`. Every doc claim verified against the actual source under `src/`, `src-tauri/src/`.
**Writer mode:** audit-only (no Blocker/Critical doc gap caused by this change set; no drafts produced — see "Drafts produced")
**Auditor posture:** Balanced

---

## TL;DR

The three `carried-debt.md` "RESOLVED" claims (P5-001 diff modal, P5-003 / P5-004 AppHandle-decoupled cross-platform tests) are **accurate** — they describe what the code now actually does, verified line-by-line against `Workbench.tsx`, `llm.rs`, `daily_scan.rs`, `tauri_cmds.rs`, and `tests.rs`. Given this repo's documented history of verification artifacts overstating reality, that is the headline good news: the change set's self-reported claims hold up. The real doc debt is **consistency drift created by the edits**: `CHANGELOG.md` now contradicts the code it ships with (it still says the sidecar-skip test "requires `mock_app()`" and is "`#[cfg(unix)]`-gated" under the old test name, when the code is now ungated, renamed, and mock-free), and three debt items (P5-002, P5-005, P5-007) were **deleted from `carried-debt.md` without a RESOLVED note**, leaving dangling references in `CHANGELOG.md`, `README.md`, and `FAQ.md`. No marketing copy materially overstates the product. README and SECURITY.md are genuinely strong, honest docs.

## Severity roll-up (documentation)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 0 |
| Major | 2 |
| Minor | 3 |
| Nit | 2 |

## What's working

- **The RESOLVED claims in `carried-debt.md` are honest.** P5-001 (`carried-debt.md:8-14`) precisely matches `src/components/Workbench.tsx`: `rewritePreview` state (`Workbench.tsx:112`), `#rewrite-diff-modal` (`:415`), LCS line-diff (`:11-57`), red/green highlighting (`:425`, `:433`), Accept applies via `onUpdateDraftContent` (`:447`), Reject discards (`:440`). P5-003/P5-004 (`carried-debt.md:16-38`) match `daily_scan.rs:43-50` (`run_daily_scan` takes `Arc<dyn LlmClient>` + prompt), `llm.rs:89-98` (`plain_language_rewrite` injected client), `llm.rs:154-258` (`run_ollama_pull` + `PullProgressSink`), `llm.rs:290-416` (`port_in_use` / `start_for_test`). No `mock_app()` remains in `tests.rs` (verified — all matches are comments); the platform-gate whitelist `.agent-workflows/section2-auth.json` is genuinely `[]`.
- **README is a model of honest pre-alpha framing.** The banner `> Pre-alpha. Not production software. No security review.` (`README.md:3`), the "What this is, and what it isn't" section (`README.md:28-45`), and the deliberately deflationary "**eight hand-written regex detectors** … This is regular expressions in a loop" (`README.md:35`) set correct expectations. This is exactly the posture the severity framework rewards.
- **SECURITY.md documents the sidecar attack surface.** `SECURITY.md:51-57` ("Local Sidecar Attack Surface") substantiates the `CHANGELOG.md:22` claim that the Mj-4 false-claim subject was already covered — it is.
- **The new `docs/script.js` download logic degrades safely.** Per-platform asset resolution (`script.js:137-153`) falls back to the HTML's `releases/latest` hrefs on API failure rather than rewriting to a broken link, and the download-card IDs (`download-win/mac/linux`) match `docs/index.html:44,55,66`. Honest engineering, honestly commented.
- **Version numbers are consistent.** `0.2.6` across `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml`.

## What couldn't be assessed

- **Runtime behavior of the macOS/Linux test fixtures.** `src-tauri/tests/fixtures/test-ollama-fixture-{aarch64,x86_64}-apple-darwin` and `…-linux-gnu` are 81-byte `#!/bin/sh` stub scripts; only the Windows fixture is a real PE binary. The carried-debt P5-003/P5-004 "runs on every platform including Windows" claim is verified *structurally* (no mock_app, no cfg-gates), but I cannot from docs/static review confirm the unix fixtures actually exec on a unix CI runner. This is a Test-Engineer verification item, flagged here only as a boundary on the docs claim.
- **Live GitHub Releases asset names.** `script.js` asset-matching (`.exe`/`.msi`/`.dmg`/`.appimage`/`.deb`) is correct in principle but I did not fetch the live releases feed to confirm published asset names match.

---

## Doc asset inventory

| Asset | Exists? | Status | Finding(s) |
|---|---|---|---|
| README.md | Yes | Strong | — (credited) |
| ARCHITECTURE (docs/architecture.md) | Yes | Adequate | — |
| User manual (docs/user_manual.md) | Yes | Adequate | — |
| API reference | N/A (desktop app) | — | — |
| FAQ.md | Yes | Adequate | DOC-002 (dangling P5-002 state) |
| CHANGELOG.md | Yes | Weak (stale vs shipped code) | DOC-001 |
| CONTRIBUTING.md | Yes | Adequate | — |
| SECURITY.md | Yes | Strong | — (credited) |
| LICENSE | Yes | Adequate | — |
| carried-debt.md | Yes | Adequate (accurate where it speaks; silent on removals) | DOC-002, DOC-004 |
| Landing / marketing (docs/index.html) | Yes | Adequate | DOC-003 (Minor), DOC-005 (Nit) |

---

## Persona walk-through

### First-time user
Lands on `docs/index.html` or README. The README answers "what is this / who is it for / how do I install / is it production-ready" inside the first screen, honestly. The landing page's "Interactive Preview Coming Soon" placeholder (`index.html:78-81`) is candid rather than faking a screenshot — acceptable. Download buttons resolve to the right per-platform asset or fall back to the releases page. First-time user succeeds.

### Returning user
Wants to know "did the rewrite-overwrite data-loss risk get fixed?" or "is auto-update on yet?" The rewrite question is answered correctly in `carried-debt.md` (P5-001 RESOLVED) and matches the app. The auto-update question is answered by `FAQ.md:142` ("dormant and inactive") — still correct for the product, but the debt item that tracked it (P5-002) was silently deleted from `carried-debt.md`, so a returning user cross-referencing the debt log finds the tracking gone with no explanation (DOC-002).

### New team member
Reads `CHANGELOG.md` to understand the v0.2.6 remediation, then opens `tests.rs` — and hits a contradiction: the changelog (`CHANGELOG.md:12`) says the sidecar-skip test "still requires `mock_app()`" and is "`#[cfg(unix)]`-gated", but the code has neither (DOC-001). In a repo whose own changelog documents a history of gamed verification claims, a new engineer cannot tell whether the changelog or the code is the truth without doing exactly the verification I just did. That erodes trust precisely where this project most needs it.

---

## Findings

> **Finding ID prefix:** `DOC-`
> **Categories:** Accuracy / Completeness / Onboarding / Architecture / API / FAQ / Marketing / Tone / Hygiene

### [DOC-001] — Major — Accuracy — CHANGELOG describes a `mock_app()`/`cfg(unix)`-gated sidecar test that no longer exists in the shipped code

**Evidence**
`CHANGELOG.md:12` (the B-1 / M-1 entry for `[0.2.6] - 2026-05-28`):
> "The full `start()`-skip path (`test_sidecar_skips_spawn_when_port_11434_occupied`) still requires `mock_app()` and is honestly `#[cfg(unix)]`-gated; the cross-platform claim it once carried is covered by the new test instead."

The shipped code contradicts every load-bearing word of that sentence:
- The test is renamed to `test_sidecar_skips_spawn_when_port_occupied` (`src-tauri/src/core/tests.rs:1187`). The name in the changelog (`…_11434_occupied`) no longer exists in `tests.rs`.
- It does **not** require `mock_app()` — it calls `sidecar.start_for_test(&addr)` (`tests.rs:1194`), and a repo-wide search confirms zero `mock_app()` constructs remain in `src-tauri/src` (all matches are comments).
- It is **not** `#[cfg(unix)]`-gated — `tests.rs:1186-1187` shows a bare `#[test]`, and no `cfg(unix)` / `cfg_attr(target_os…)` / `windows, ignore` gate exists anywhere in `tests.rs`.

`carried-debt.md:30-38` (P5-004, this change set) correctly describes the *current* state ("run cross-platform via `OllamaSidecar::start_for_test(probe_addr)` … No test constructs `mock_app()`"). So the change set advanced the code past the CHANGELOG, but the CHANGELOG entry was left describing an interim state.

**Why this matters**
The new-team-member persona reads the CHANGELOG as the authoritative record of what shipped. Here it asserts a known-bad pattern (Windows-gated test) that the project's own v0.2.5 postmortem (`CHANGELOG.md:27`, `:40`) holds up as a gamed verification claim. A doc that says "this test is still Windows-deferred" when the code made it cross-platform is an *inverse* overstatement — it under-claims the fix — but in an integrity-sensitive repo it reads as either a gamed claim or sloppy bookkeeping, and the reader cannot distinguish the two without independent verification. Accuracy framework: a CHANGELOG that does not match the code it ships with is a Major.

**Blast radius**
- Other docs that repeat the same error: `scripts/audit/mutations.json:51-54` still maps the mutation test to the old name `test_sidecar_skips_spawn_when_port_11434_occupied` against `pub fn start` in `llm.rs` — the renamed test (`…_when_port_occupied`) exercises `start_for_test`, so this mutation mapping is now stale and may silently not run / not match (cross-flag to Test Engineer).
- Adjacent code: `src-tauri/src/core/reproduction_tests.rs` was correctly updated this change set (`reproduction_tests.rs` diff: "the whitelist is empty — every formerly-gated test now runs on all platforms") — so the reproduction harness already reflects reality; only the CHANGELOG and mutations.json lag.
- User-facing: none (internal doc).
- Migration: none.
- Tests to update: `scripts/audit/mutations.json` entry (test name + target function).
- Related findings: cross-references Test-Engineer findings on `mutations.json` accuracy.

**Fix path**
Rewrite `CHANGELOG.md:12` to match shipped reality, e.g.: "Extracted the port-collision check into `OllamaSidecar::port_in_use(addr)` / `ollama_port_in_use()`, decoupled the sidecar spawn/skip path from `AppHandle` via `start_for_test(probe_addr)`, and made the full skip-path test (`test_sidecar_skips_spawn_when_port_occupied`) genuinely cross-platform — it constructs no `mock_app()` and carries no platform gate." Update `scripts/audit/mutations.json:52` to the new test name and the function it actually exercises.

---

### [DOC-002] — Major — Accuracy — Three debt items (P5-002, P5-005, P5-007) deleted from carried-debt.md without resolution notes, leaving dangling references in other docs

**Evidence**
`git diff HEAD -- carried-debt.md` shows P5-002 (Tauri auto-updater), P5-005 (per-platform smart download links), and P5-007 (Linux GPU shared libraries) were **removed entirely** — unlike P5-001/003/004, which were retained and marked `RESOLVED` with explanatory prose. The removals leave references elsewhere pointing at debt items that no longer exist:
- **P5-007:** `CHANGELOG.md:65` (the v0.2.4 "Known Limitations") still says: "Linux GPU acceleration falls back to CPU at runtime … **Tracked as P5-007 (carried debt) for the v0.3 release.**" — pointing at a now-deleted item. No RESOLVED note and no code evidence the GPU-lib bundling shipped.
- **P5-002:** `README.md:80` still annotates `tauri.conf.json … includes Updater plugin (dormant — see FAQ)` and `FAQ.md:142` still says "The Tauri updater is dormant and inactive." The product limitation P5-002 tracked is unchanged, but its tracking entry is gone.
- **P5-005:** `CHANGELOG.md:83` references "registered carried-debt P5-005." P5-005 was in fact *implemented* in this same change set (`docs/script.js:137-153` per-platform links) — but it was removed silently rather than marked RESOLVED like its siblings.

**Why this matters**
This is the highest-leverage honesty finding given the repo's integrity history. `carried-debt.md` is the project's deferred-work ledger. Deleting an *unresolved* limitation (P5-002 auto-updater still dormant; P5-007 Linux GPU still CPU-only per `CHANGELOG.md:65`) without a resolution note silently shrinks the documented limitation set — it makes the product look more complete than it is. The returning-user and new-team-member personas lose the trail: a `CHANGELOG`/`README`/`FAQ` reference points into a debt file where the item has vanished. P5-005 is the opposite problem — a genuinely-shipped feature was demoted to a silent deletion instead of a credited RESOLVED, so the change set under-documents real work it did. Either way, the ledger is no longer trustworthy as the source of truth.

**Blast radius**
- Other docs that repeat / depend on the error: `CHANGELOG.md:65` (P5-007), `CHANGELOG.md:83` (P5-005), `README.md:80` + `FAQ.md:142` (P5-002 state).
- User-facing: indirectly — `FAQ.md:142` is the answer a user gets for "is auto-update on?" It is still correct, but its debt anchor is gone.
- Migration: none.
- Tests to update: none.
- Related findings: DOC-001 (same root cause — docs not reconciled with the code/ledger after the refactor); DOC-004.

**Fix path**
Adopt one disposition rule and apply it uniformly: an item leaves `carried-debt.md` only when it is either (a) RESOLVED with a prose note + code pointer (the P5-001/003/004 pattern) or (b) explicitly WITHDRAWN/OBSOLETE with a one-line reason. Concretely: mark **P5-005 RESOLVED** (cite `docs/script.js:137-153`); **keep P5-002 and P5-007 as DEFERRED** (they are still real limitations per `CHANGELOG.md:65` and `FAQ.md:142`) or, if intentionally dropped, record why. Then fix the dangling pointer in `CHANGELOG.md:65` ("Tracked as P5-007").

---

### [DOC-003] — Minor — Marketing — Landing copy claims "100% locally" / "no external server needed" — true, but the bundled-Ollama "zero setup" framing slightly outruns the documented CPU-only Linux reality

**Evidence**
`docs/index.html:40`: "publish flat HTML sites—all running 100% locally." `index.html:114-116` (Bundled Ollama Sidecar feature): "Features zero separate setup. Run your intelligence model directly on your CPU out-of-the-box, no external server needed." The "100% locally" and "no external server" claims are accurate and well-supported by `README.md:49` and `SECURITY.md:51-57`. The mild gap: `CHANGELOG.md:65` documents that Linux GPU acceleration silently falls back to CPU because GPU shared libs are not bundled — the landing page's "out-of-the-box" CPU framing happens to be honest for CPU, but a Linux user expecting GPU acceleration gets no signal here.

**Why this matters**
First-time user on the landing page forms a performance expectation. "Run your intelligence model directly on your CPU out-of-the-box" is honest about *CPU* but the marketing surface never mentions that GPU is not available on Linux, while the limitation is documented deep in the CHANGELOG. This is a soft expectation gap, not a misrepresentation — hence Minor.

**Fix path**
No marketing rewrite required. Optionally add one honest line near the feature card or on `docs/install.md`: "Inference runs on CPU by default; GPU acceleration is not yet bundled on Linux." Keeps the landing page in lockstep with `CHANGELOG.md:65`.

---

### [DOC-004] — Minor — Hygiene — RESOLVED prose in carried-debt.md is accurate but uses inconsistent disposition vocabulary across items

**Evidence**
`carried-debt.md` mixes resolution vocabularies with no key: P5-001/003/004 use "RESOLVED" with prose; P5-006 (`:39`) uses "Resolved in v0.2.4."; the "Forensic Branch Reference" (`:15`) and P5-000 (`:7`) are open items with no status label; P5-002/005/007 are removed with no trace. There is no legend explaining what a bare bullet vs. "RESOLVED" vs. removal means.

**Why this matters**
A ledger that is the designated source of truth for an integrity-sensitive project should make item state unambiguous at a glance. The content is accurate; the convention is ad hoc. Low impact, worth logging.

**Fix path**
Add a one-line status convention at the top of `carried-debt.md` (e.g. `OPEN | RESOLVED (vX.Y, <pointer>) | WITHDRAWN (<reason>)`) and tag every item. Pairs naturally with the DOC-002 fix.

---

### [DOC-005] — Nit — Marketing — "Zero Runtime." hero pill is puffy and ambiguous

**Evidence**
`docs/index.html:37`: `<div class="pill">Local-First. Zero Runtime.</div>`. "Zero Runtime" is not defined anywhere; the app does ship a bundled Ollama sidecar runtime and a Tauri/WebView runtime. The intended meaning ("no separate runtime to install") is defensible but the phrase as written is a marketing abstraction a literal reader can poke.

**Fix path**
Optional. "No separate install" or "No cloud, no accounts" would be more concrete and less pokeable. Nit — flag once, do not belabor.

---

### [DOC-006] — Nit — Tone — "Pipeline Integrity Incidents" section reads as internal forensics inside a user-facing-adjacent debt file

**Evidence**
`carried-debt.md:41-49` enumerates five executor-gaming incidents (walkthrough hallucination, manager-decision fabrication, four-bypass pattern, etc.) with links to `forensic/v0.2-pipeline-integrity-failures.md`. This change set did not modify this section (it is below the diff hunk), so it is in scope only as context. The content is accurate and admirably transparent, but it sits in the same file as the deferred-work ledger and is written in internal pipeline jargon ("lie-proof-3 contract", "grep-pattern-as-product-string").

**Why this matters**
Not a defect — the transparency is a credit. Nit-level observation: mixing a forward-looking debt ledger with a backward-looking incident log in one file blurs the file's purpose. No action needed unless the team wants `carried-debt.md` to be externally publishable.

**Fix path**
Optional: split incidents into `forensic/` (where the linked detail already lives) and keep `carried-debt.md` purely forward-looking. Leave as-is if the file is internal.

---

## Drafts produced

Writer mode is audit-only. No Blocker- or Critical-severity doc gap was caused by this change set — the RESOLVED claims under audit are accurate against the code — so the drafting trigger did not fire. No files were created in `doc-rewrites/`.

## Marketing / honesty audit

The landing page (`docs/index.html`) was checked claim-by-claim against the source:
- "running 100% locally" / "no external server needed" (`:40`, `:115`) — **accurate** (`README.md:49`, `SECURITY.md:51-57`).
- "Bundled Ollama Sidecar … zero separate setup" (`:114-116`) — **accurate**; sidecar is bundled (`llm.rs:342-356`, README `:128`). Soft CPU/GPU expectation gap flagged as DOC-003.
- "DNS Rebinding Guards: Strict host header validation" (`:182`) — **accurate**; host-header validation is real (`tests.rs:45-52` exercises `is_valid_host`).
- "Browser Pairing … secure PIN-pairing tokens" (`:183`) — **accurate** (`tauri_cmds.rs:142-164` generates hashed, 5-min-expiry pairing PINs).
- "SQLite WAL Database" / "Axum loopback server strictly bound to `127.0.0.1:12053`" (`:178-181`) — **accurate** (README `:39,49`).

No overclaim rising to Major. The marketing surface is, by the standards of this repo's history, honest. The two flags are a Minor expectation gap (DOC-003) and a Nit (DOC-005).

## Patterns and systemic observations

**Pattern — "code moved past the docs" (DOC-001, DOC-002, and the DOC-004 hygiene gap share one root cause).** This change set correctly advanced the code and correctly updated `carried-debt.md` and `reproduction_tests.rs`, but did **not** sweep the other docs that reference the same facts: `CHANGELOG.md` (stale test name + stale "requires mock_app / cfg(unix)" claim), `scripts/audit/mutations.json` (stale test name), and the dangling P5-002/005/007 references in `CHANGELOG`/`README`/`FAQ`. The fix is a single reconciliation pass keyed off the renamed/decoupled symbols, plus a uniform disposition rule for `carried-debt.md`. Recommend treating doc reconciliation as a required step of any refactor that renames a test or resolves/removes a debt item — especially in a repo whose own changelog documents prior gamed-verification incidents, where a doc/code mismatch is read as a red flag rather than a typo.

**Counter-pattern worth preserving.** The RESOLVED prose in `carried-debt.md` (P5-001/003/004) is the *right* model: each claim names the symbol, the file, and the mechanism, and each is independently verifiable. This is exactly what the integrity history demands. The fix for DOC-002 is to apply that same model to the removed items rather than deleting them.

## Appendix: docs reviewed

- `carried-debt.md` (full + diff)
- `docs/index.html` (full)
- `docs/script.js` (full + diff)
- `README.md` (full)
- `CHANGELOG.md` (0.2.6 / 0.2.5 / 0.2.4 entries + grep across file)
- `SECURITY.md` (sidecar section)
- `FAQ.md` (updater section)
- `docs/architecture.md`, `docs/user_manual.md`, `docs/install.md` (existence + cross-reference checks)
- `scripts/audit/mutations.json` (lines 40-58)
- Source cross-checked: `src/components/Workbench.tsx`, `src-tauri/src/core/llm.rs`, `src-tauri/src/core/daily_scan.rs`, `src-tauri/src/core/tests.rs`, `src-tauri/src/core/reproduction_tests.rs`, `src-tauri/src/tauri_cmds.rs`, `.agent-workflows/section2-auth.json`, `src-tauri/tests/fixtures/`, `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`
