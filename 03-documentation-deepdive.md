# Documentation Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-23
**Role:** Technical Writer
**Scope audited:** README.md, docs/architecture.md, docs/user_manual.md, FAQ.md, docs/manual-smoke.md, docs/discussion_seeds.md, CONTRIBUTING.md, SECURITY.md, and assistant-skill/SKILL.md
**Writer mode:** audit+draft
**Auditor posture:** Balanced

---

## TL;DR

CivicNewspaper's primary documentation assets have been successfully updated to align with the actual system capabilities. The previous code-blocking bugs (draft status mismatch in compile flow, Host validation blocking localhost, and Origin check rejecting CLI requests) are fully resolved in the codebase. Corresponding core docs (`README.md`, `docs/architecture.md`, `docs/user_manual.md`, `FAQ.md`, and `docs/manual-smoke.md`) have been successfully updated. However, secondary files (`CONTRIBUTING.md`, `SECURITY.md`, and `docs/discussion_seeds.md`) still contain outdated advisories regarding code modularization, stored-XSS, and compile-blocking guardrails.

## Severity roll-up (documentation)

| Severity | Count (Before Fixes) | Count (Remaining Doc Gaps) |
|---|---|---|
| Blocker | 2 | 0 |
| Critical | 2 | 0 |
| Major | 3 | 2 |
| Minor | 2 | 1 |
| Nit | 0 | 0 |

## What's working

- **Ollama / Local LLM Hardware Guide** — [FAQ.md, "What hardware do I need?", L27-33] is accurate and matches the automated memory diagnostics implemented in the onboarding wizard.
- **OSINT Regex Specs** — [FAQ.md, "What does the OSINT Detector Engine actually do?", L85-98] perfectly lists the eight regular expressions and accurately documents the case-insensitivity and exact-string deduplication logic.
- **CLI/IDE Tooling Concepts** — [assistant-skill/SKILL.md, L11-17] correctly explains the loopback API server concept on port `12053`, and the implementation now supports it.

## What couldn't be assessed

- All documentation and files listed in the scope were fully accessible locally in the workspace directory. No external paywalled documents or broken links prevented audit completion.

---

## Doc asset inventory

| Asset | Exists? | Status | Finding(s) |
|---|---|---|---|
| README.md | Yes | Strong | None (Resolved) |
| ARCHITECTURE.md | Yes (as docs/architecture.md) | Strong | None (Resolved) |
| User manual / guide | Yes (as docs/user_manual.md) | Strong | None (Resolved) |
| API reference | No (only CLI help in skill) | Missing | (Dealt with in CLI tool pairings) |
| FAQ | Yes | Strong | None (Resolved) |
| CHANGELOG | Yes | Adequate | None (Resolved) |
| CONTRIBUTING | Yes | Adequate | DOC-007 |
| SECURITY | Yes | Adequate | DOC-006 |
| LICENSE | Yes | Strong | None |
| Landing / marketing page | Yes (as docs/index.html) | Adequate | None |

---

## Persona walk-through

### First-time user
A first-time user follows the user manual, installs Ollama, launches CivicNews, configures their community profile, scrapes a local council agenda, and generates a draft article. They review the guardrails report, click "Approve for Static Publish", and run the Static Compilation Wizard. Because `compiler.rs` now includes `"ready_to_publish"` drafts, the static newsroom site is generated correctly. The user succeeds.

### Returning user
A returning user wishes to integrate their editor with the command-line assistant skill (`client.js`) or other developer scripts. They run `node assistant-skill/client.js pair <token>`. The loopback server accepts the connection because the Origin check allows absent Origin headers. Additionally, if they connect via `localhost:12053`, the Host check successfully validates the host, matching the architecture design.

### New team member
A new developer clones the repository and reads `README.md` and `CONTRIBUTING.md`. While `README.md` has been successfully updated to show the modular `src/components/` structure, `CONTRIBUTING.md` still advises them that `App.tsx` is a single monolithic 1,918-line component that needs refactoring. This creates a minor point of confusion during orientation.

---

## Findings

> **Finding ID prefix:** `DOC-`
> **Categories:** Accuracy / Completeness / Onboarding / Architecture / API / FAQ / Marketing / Tone / Hygiene

### [DOC-001] — Blocker — Onboarding / Accuracy — Status Mismatch Between UI and Compiler Prevents Story Publication

**Evidence**
- `src/components/Workbench.tsx` (Line 257):
  ```typescript
  <button className="btn btn-primary btn-sm" onClick={() => onDecision("ready_to_publish")} id="btn-status-publish">
    Approve for Static Publish
  </button>
  ```
- `src-tauri/src/core/compiler.rs` (Lines 88-90):
  ```rust
  if d.status == "published" || d.status == "corrected" {
      published_drafts.push(d);
  }
  ```

**Why this matters**
Approved stories are saved in SQLite with the status `"ready_to_publish"`. When the compiler ran, it previously ignored drafts with this status, resulting in an empty static website.

**Status: Resolved**
Verified that `src-tauri/src/core/compiler.rs` now checks for `"ready_to_publish"`:
```rust
if d.status == "published" || d.status == "corrected" || d.status == "ready_to_publish" {
    published_drafts.push(d);
}
```
Core documentation (`docs/user_manual.md` and `docs/manual-smoke.md`) has been updated to reflect this behavior.

---

### [DOC-002] — Blocker — API / Accuracy — Missing Origin Header Rejection Blocks Assistant Skill CLI Tool

**Evidence**
- `src-tauri/src/core/auth.rs` (Lines 61-71):
  ```rust
  } else {
      // Reject missing Origin
      return Err(StatusCode::FORBIDDEN);
  }
  ```

**Why this matters**
CLI tools, scripts, and local developer plugins do not run inside a browser context and therefore do not send an `Origin` header. Rehearsing these requests previously blocked the assistant skill (`client.js`) with `403 Forbidden`.

**Status: Resolved**
Verified that `auth_middleware` in `src-tauri/src/core/auth.rs` now allows absent `Origin` headers:
```rust
} else {
    // Absent origin is allowed for local CLI and IDE clients (e.g. assistant skill)
}
```
`docs/architecture.md` has been updated to align with this security model.

---

### [DOC-003] — Critical — API / Accuracy — Host Header Validation Blocks `localhost` Port Requests

**Evidence**
- `src-tauri/src/core/auth.rs` (Lines 11-13):
  ```rust
  pub fn is_valid_host(host: &str) -> bool {
      host.trim() == "127.0.0.1:12053"
  }
  ```

**Why this matters**
Requests using `localhost:12053` instead of the literal loopback IP `127.0.0.1:12053` were previously blocked with Host validation errors, violating the documented API specifications.

**Status: Resolved**
Verified that `is_valid_host` in `src-tauri/src/core/auth.rs` now accepts `"localhost:12053"`:
```rust
pub fn is_valid_host(host: &str) -> bool {
    let h = host.trim();
    h == "127.0.0.1:12053" || h == "localhost:12053"
}
```
`docs/architecture.md` matches this updated behavior.

---

### [DOC-004] — Critical — Accuracy — Guardrail Checks Do Not Block Compilation or UI Status Changes

**Evidence**
- `docs/architecture.md` (Line 158) claimed:
  "Otherwise, it triggers a blocking compilation error."
- `docs/manual-smoke.md` (Step 5) claimed:
  "Verify the Factual Guardrail Inspector blocks the publish due to the accusatory language rule."

**Why this matters**
The documentation repeatedly stated that guardrail checks are compiler-enforcing. In reality, they are frontend visual warnings only. Users can publish articles violating safety policies (e.g. accusatory language without citations) without any blocking error, presenting journalistic and legal risks if misunderstood.

**Status: Resolved**
Verified that `docs/architecture.md` and `docs/manual-smoke.md` have been updated to clarify that guardrails function as lints and helper indicators within the UI rather than compiler-blocking restrictions.

---

### [DOC-005] — Major — FAQ / Accuracy — FAQ Falsely Claims App Never Runs Update Checks

**Evidence**
- `FAQ.md` (Line 125):
  "There is no telemetry, no analytics, no update check, no error reporting service."

**Why this matters**
Privacy-conscious users trust the explicit claim that the app is completely offline and does not check for updates. Finding background network requests to GitHub releases will damage project credibility.

**Status: Resolved**
Verified that `FAQ.md` was updated to accurately state:
```markdown
Additionally, the frontend utilizes the Tauri updater plugin which checks for updates from the official GitHub releases endpoint (`latest.json`) on app launch. This check does not send user tracking or telemetry metrics.
```

---

### [DOC-006] — Major — Accuracy / Security — SECURITY.md Misstates Stored-XSS Vulnerability In Markdown Compiler

**Evidence**
- `SECURITY.md` (Lines 47-49) claims:
  "Markdown compiled to HTML is not sanitized in this audit's read of compiler.rs. pulldown-cmark by default allows raw HTML."

**Why this matters**
The security documentation falsely advertises a major stored-XSS vulnerability in the core compiler. This creates unnecessary security concern for users who might think they cannot copy records containing HTML tags without risking malware injection on their static site.

**Status: Partially Resolved (Outdated Warning Remains)**
The compiler (`compiler.rs` lines 42-53) actively sanitizes output by filtering out `pulldown_cmark::Event::Html` and `Event::InlineHtml` events. However, `SECURITY.md` still contains the outdated warning in the "Known weak spots" section.

**Fix path**
Update `SECURITY.md` (L47-49) to indicate that raw HTML blocks and inline HTML tags are filtered out by the compiler:
```markdown
- **HTML input is sanitized.** The markdown compiler filters out raw HTML blocks and inline HTML elements, rendering the output safe from XSS.
```

---

### [DOC-007] — Major — Onboarding / Accuracy — Outdated Single-File App.tsx Structure in Onboarding Docs

**Evidence**
- `CONTRIBUTING.md` (Line 44):
  Describes `App.tsx` as a "1,918-line single-page React component" and recommends modularization.

**Why this matters**
This confuses new contributors. They expect a massive, monolithic file to modularize, but find a refactored directory (`src/components/`, `useApp.ts`) in the workspace, indicating that onboarding documentation has not been kept up to date.

**Status: Partially Resolved (Outdated Warning Remains)**
`README.md` has been successfully updated to show the modular `src/components/` layout. However, `CONTRIBUTING.md` (L44) still contains the outdated refactoring description.

**Fix path**
Remove the outdated description and recommendation in `CONTRIBUTING.md`.

---

### [DOC-008] — Minor — Accuracy — README Lists Outdated Naming TODOs

**Evidence**
- `README.md` (Lines 49-50, L140):
  Notes that package name is still `"tauri-app"` in `Cargo.toml` and `package.json`.

**Why this matters**
Lists outdated TODO tasks as outstanding issues, making the repository feel unmaintained.

**Status: Resolved**
Verified that these naming TODO notes have been removed from `README.md` (the package has already been successfully renamed to `"civicnews"`).

---

### [DOC-009] — Minor — Marketing / Accuracy — Discussion Seeds Claim Guardrails Block Compiler

**Evidence**
- `docs/discussion_seeds.md` (Line 97):
  `* Note: The CivicNews compiler will block publication if arrest keywords are used...`

**Why this matters**
Reinforces the misconception about compiler-enforced guardrails to new community members.

**Status: Partially Resolved (Outdated Warning Remains)**
`docs/discussion_seeds.md` still contains this sentence.

**Fix path**
Modify `docs/discussion_seeds.md` line 97 to state:
```markdown
* Note: The CivicNews editor will flag a warning if arrest keywords are used without a presumption-of-innocence modifier like "alleged".
```

---

## Drafts produced

Writer mode is audit-only for this phase; verification report produced. Core rewrites previously drafted by the writer have been successfully integrated into the main repository branch.

## Marketing / honesty audit

CivicNewspaper's landing pages and readme introduction text are generally very honest:
- **Pre-alpha Warnings**: [README.md] explicitly warns users that the software is pre-alpha, database schemas are fragile, and it lacks security review.
- **Local AI Limitations**: [FAQ.md] transparently states that local models are weaker, hallucinate, and the responsibility for truth lies with the author.
- **Outrage vs. Objective wording**: [docs/discussion_seeds.md] provides excellent guidelines advocating for objective formatting instead of sensationalism.
- **Telemetry Check discrepancy**: The background update check is now accurately documented in the FAQ.

## Patterns and systemic observations

There is a clear systemic drift where documentation lags behind codebase updates:
1. **Frontend Refactoring Drift**: React code was refactored into components, but onboarding text in `CONTRIBUTING.md` remained unchanged.
2. **Guardrails vs. Compiler**: The design originally specified that guardrails should block compilation, but the compiler was developed independently as a simple flat generator. While core docs were updated to reflect this, the launch templates (`discussion_seeds.md`) still carry the older expectation.
3. **Security Policy Alignment**: Code fixes to prevent XSS were implemented and verified with tests, but the `SECURITY.md` advisory was not updated to claim this improvement.

## Appendix: docs reviewed

- `README.md`
- `docs/architecture.md`
- `docs/user_manual.md`
- `FAQ.md`
- `docs/manual-smoke.md`
- `docs/discussion_seeds.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `assistant-skill/SKILL.md`
- `src-tauri/src/core/auth.rs`
- `src-tauri/src/core/compiler.rs`
- `src-tauri/src/core/server.rs`
