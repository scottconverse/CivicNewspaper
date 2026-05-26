# Director Decisions for 2026-05-25-execute-v0-2-phase-4

The director has approved the following recommendations from the research phase. These are binding constraints for the planning phase:

1. **Prompt String Match:** Load `aggregator` prompt as-is (Option A) without trimming.
2. **Detectors Widen Scope:** Strict field propagation only (Option A) to avoid scope lock violations.
3. **Validation Pattern in `get_prompt`:** Use a hardcoded enum/array of IDs (Option B) to satisfy the DoD.
4. **Audit-Skills Verifier:** Implement the `audit-lite` runtime executor check (Option A) as requested in director notes.
5. **LLM Invocation:** Call `crate::core::llm::call_local_ollama` natively without modifying `llm.rs` (Option A).
6. **First Push:** The executor must use `git push -u origin v0.2-phase-4` (Option B).
7. **Migrations Registration:** Register `0004_source_tier` and `0005_daily_scans` explicitly in numeric order (Option A).
