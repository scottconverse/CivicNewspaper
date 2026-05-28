# Sprint Punch List — CivicNewspaper

**Audit date:** 2026-05-26
**For sprint ending:** 2026-05-26 (v0.2.0 Release Gate)

This list is the dev team's actionable fixes for the **current sprint**. Every item has an ID, severity, owner hint, and a one-line description of the fix. Estimated size (S/M/L) is a rough guide for planning, not a commitment.

---

## Must-fix (Blockers + Criticals)

| # | ID | Severity | Role | What to do | Size |
|---|---|---|---|---|---|
| 1 | [ENG-012] | Blocker | Engineering | Manage `Arc<dyn LlmClient>` in `lib.rs` to avoid Tauri setup state panic. | S |
| 2 | [UX-015] | Blocker | UI/UX | Fetch recommended model dynamically in wizard instead of hardcoding `gemma2:9b`. | S |
| 3 | [TEST-011] | Blocker | Test | Update plain language rewrite test to call command handler rather than mocking self. | S |
| 4 | [QA-002] | Blocker | QA | Allow requests with missing Origin headers if they originate from localhost. | S |
| 5 | [ENG-013] | Critical | Engineering | Disable foreign keys dynamically during table drop/rename inside migration 0007. | S |
| 6 | [ENG-014] | Critical | Engineering | Verify HTTP status codes in model pull tasks and emit error event to frontend on failure. | M |
| 7 | [UX-016] | Critical | UI/UX | Provide a UI command or export link to package/extract browser extensions for production users. | M |
| 8 | [UX-017] | Critical | UI/UX | Fetch rewritten text first and show a side-by-side preview diff before overwriting draft. | L |
| 9 | [UX-013] | Critical | UI/UX | Update `style.css` media query to prevent hiding primary navigation and CTAs on mobile. | S |
| 10 | [DOC-010] | Critical | Docs | Update database paths in user manual to use correct `org.civicnews.app` identifier. | S |
| 11 | [TEST-012] | Critical | Test | Restructure `test_source_tier_backfill_media_lead` to insert legacy rows before migration and assert values. | M |
| 12 | [TEST-014] | Critical | Test | Implement Prompt Library dropdown in `Workbench.tsx` and write Vitest coverage. | M |

---

## Should-fix (high-leverage Majors)

Majors that are cheap, urgent, or high-leverage. Tackle these after Blockers/Criticals if sprint capacity allows.

| # | ID | Severity | Role | What to do | Size |
|---|---|---|---|---|---|
| 1 | [ENG-015] | Major | Engineering | Save onboarding model to settings; query dynamically instead of hardcoding `gemma2:9b`. | M |
| 2 | [UX-004] | Major | UI/UX | Display the list of guardrails warning issues in the sidebar even if `is_clean` is true. | S |
| 3 | [UX-005] | Major | UI/UX | Bind onboarding wizard profile and model download handlers to state save callbacks. | M |
| 4 | [UX-006] | Major | UI/UX | Remove the dead verify input from `PairDialog.tsx` or wire it to a verification endpoint. | S |
| 5 | [DOC-011] | Major | Docs | Correct user manual's Mermaid diagram to show React UI connecting via Tauri IPC, not HTTP. | S |
| 6 | [QA-013] | Major | QA | Load plain language rewrite template `07-plain-language.md` in Tauri command rather than hardcoding. | S |
| 7 | [QA-014] | Major | QA | Expand `VALID_PROMPT_IDS` in `prompts.rs` to support all 9 prompts and scan directory. | M |

---

## Suggested sequencing

1. **Fix ENG-012 and ENG-013 first**: These block application startup and core daily scan workflows.
2. **Fix UX-015, UX-013, and DOC-010**: These fix onboarding, manual navigation, and documentation accuracy.
3. **Fix TEST-011, TEST-012, and TEST-014**: Clean up the test suite and implement missing Vitest coverage.
4. **Fix QA-002**: Restore local developer CLI and IDE pairing.

---

## Sign-off gate

The dev team should not consider the sprint done until:

- [x] All Blockers fixed and verified in the running product (locally built)
- [x] All Criticals fixed and tests added where gaps existed
- [x] Regression pass done on any code touched by these fixes (blast radius from the deep-dives)
- [x] Docs updated for any user-facing or API-contract changes
