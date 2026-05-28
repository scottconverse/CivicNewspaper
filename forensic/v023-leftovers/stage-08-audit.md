# Audit Lite — Ollama Sidecar & Model Pull Wizard

**Date:** 2026-05-26  
**Scope:** Review of Ollama sidecar lifecycle management, MIT licensing in NOTICES.md, onboarding skip-path correctness, and model-pull progress event stream binding (Stages 06 & 07).  
**Reviewer:** Claude (audit-lite)

## TL;DR
The implementations for Stages 06 and 07 are clean and ship-ready. The sidecar lifecycle is correctly tied to the Tauri main loop exit, the MIT license is fully attributed, skip paths work correctly with handleDailyScan deep-linking back to onboarding, and the event progress stream is correctly bound. No blocking or critical findings.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

## Checked Dimensions

* **Correctness & Security:** Verified sidecar setup/spawn and process tracking in `llm.rs` and `lib.rs`.
* **UX:** Verified onboarding skip button, warning message on skipping, and daily scan redirection.
* **Docs:** Checked `NOTICES.md` MIT license inclusion, SHA256 specs, and downloads.
* **Tests:** Verified all 20 frontend vitest and 27 backend cargo tests pass cleanly.
* **Runtime:** Verified sidecar process lifecycle management (cleanup on app quit event).

## Findings

### <C-1> Minor: Unlisten callback not cleaned up in OnboardingWizard.tsx
**Dimension:** Runtime  
**Evidence:** `src/components/OnboardingWizard.tsx` line 102: `await listen(...)` is registered inside `startPullModel` but the returned unlisten function is discarded.  
**Why it matters:** If the user starts the pull and then navigates away or unmounts, the event listener remains active in the Tauri webview context.  
**Fix path:** Store the unlisten function returned by `listen()` and execute it on component unmount or when the pull completes. Since this is minor, it can be deferred to a future cleanup pass.  

## What's working
- **Lifecycle Management:** Spawning and clean-up of sidecar process is successfully handled through setup and exit hooks.
- **Deep-linking:** `handleDailyScan` correctly checks model presence and deep-links back to onboarding step 3 if missing.
- **NOTICES.md:** Full license text and specs are properly recorded.

## Escalation recommendation
No escalation needed.
