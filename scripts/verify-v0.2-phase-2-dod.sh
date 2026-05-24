#!/bin/bash
# v0.2 Phase 2 only — real onboarding wizard DoD.
# Committed to scripts/verify-v0.2-phase-2-dod.sh in CivicNewspaper.
# SHA-256 pinned in scripts/.dod-phase-2.sha256. CI hash-checks before running.
# Any local edit fails CI's guard step.
#
# Behavior-based assertions only. No string-presence-only greps where a
# stub or comment could pass the check.

set -u
PASS=0
FAIL=0
log() { echo "[$1] $2"; if [ "$1" = "PASS" ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi; }

# === Phase 2 work-present gate ===
# Trigger signals: new 0003_settings.sql migration OR a real OnboardingWizard
# with substantive content. If neither, this branch doesn't introduce Phase 2.
WIZARD_LINES=$(wc -l < src/components/OnboardingWizard.tsx 2>/dev/null || echo 0)
if [ ! -f src-tauri/migrations/0003_settings.sql ] && [ "$WIZARD_LINES" -lt 200 ]; then
  echo "ℹ Phase 2 work not present on this branch:"
  echo "    No 0003_settings.sql migration AND OnboardingWizard.tsx is < 200 lines."
  echo "    Phase 2 DoD has nothing to verify — exiting 0."
  exit 0
fi
echo "✓ Phase 2 work detected — running full verification"
echo ""

# === Settings migration: present and UTF-8 (NOT UTF-16) ===
[ -f src-tauri/migrations/0003_settings.sql ] && log PASS "P2: settings migration present" || log FAIL "P2: 0003_settings.sql missing"
if [ -f src-tauri/migrations/0003_settings.sql ]; then
  ENC=$(file src-tauri/migrations/0003_settings.sql)
  echo "$ENC" | grep -qE "ASCII|UTF-8" && log PASS "P2: migration is UTF-8/ASCII" || log FAIL "P2: migration encoding wrong ($ENC)"
  grep -qE "CREATE TABLE +settings" src-tauri/migrations/0003_settings.sql && log PASS "P2: settings table DDL" || log FAIL "P2: settings table DDL missing"
fi

# === Tauri commands: exist AND have non-empty function bodies ===
# Anti-stub: detect `pub fn X() {}` or `pub fn X() -> Result<...> {}` patterns.
# A real implementation has at least one substantive line between the braces.
check_cmd_body() {
  local fn_name="$1"
  local found=0
  local empty=0
  for f in src-tauri/src/tauri_cmds.rs src-tauri/src/core/*.rs; do
    [ -f "$f" ] || continue
    if grep -qE "fn $fn_name\b" "$f"; then
      found=1
      # Extract ~30 lines starting from the fn declaration; check the body
      # has at least 3 non-trivial lines (not just `{}` or whitespace).
      BODY=$(awk -v fn="fn $fn_name" '
        $0 ~ fn { in_fn=1; brace=0 }
        in_fn {
          for (i=1;i<=length($0);i++) {
            c = substr($0,i,1)
            if (c == "{") brace++
            if (c == "}") { brace--; if (brace==0) { print; in_fn=0; exit } }
          }
          if (in_fn) print
        }
      ' "$f")
      # Count non-blank, non-brace-only, non-comment lines in the body
      BODY_LINES=$(echo "$BODY" | grep -vE '^\s*($|//|\{|\})' | wc -l)
      if [ "$BODY_LINES" -ge 3 ]; then
        log PASS "P2: $fn_name has substantive body ($BODY_LINES lines)"
      else
        log FAIL "P2: $fn_name body is a stub ($BODY_LINES non-trivial lines)"
        empty=1
      fi
      break
    fi
  done
  [ "$found" -eq 0 ] && log FAIL "P2: $fn_name not declared anywhere"
}

check_cmd_body "ollama_health"
check_cmd_body "ollama_pull_model"
check_cmd_body "is_onboarding_complete"
check_cmd_body "set_onboarding_complete"

# === OnboardingWizard is substantive (real implementation, not Phase 1 placeholder) ===
# Phase 1 shipped a 138-line placeholder. Phase 2 must replace it with a real
# 6-step wizard. Lower bound 300 lines.
[ "$WIZARD_LINES" -ge 300 ] && log PASS "P2: OnboardingWizard.tsx real ($WIZARD_LINES lines)" || log FAIL "P2: OnboardingWizard.tsx too small ($WIZARD_LINES lines, need >= 300)"

# Wizard must reference the 4 new Tauri commands by name (real wiring, not just
# a step counter). At least 3 of the 4 must appear.
WIZARD_CMD_REFS=0
for cmd in ollama_health ollama_pull_model is_onboarding_complete set_onboarding_complete; do
  grep -q "$cmd" src/components/OnboardingWizard.tsx 2>/dev/null && WIZARD_CMD_REFS=$((WIZARD_CMD_REFS+1))
done
[ "$WIZARD_CMD_REFS" -ge 3 ] && log PASS "P2: wizard wires >=3 of 4 Tauri commands ($WIZARD_CMD_REFS/4)" || log FAIL "P2: wizard only wires $WIZARD_CMD_REFS/4 Tauri commands"

# Wizard step content: at least 6 distinct step markers (not just a counter).
STEP_MARKERS=$(grep -cE 'step\s*===?\s*[1-6]|currentStep\s*===?\s*[1-6]|Step\s+[1-6]' src/components/OnboardingWizard.tsx)
[ "$STEP_MARKERS" -ge 6 ] && log PASS "P2: wizard has >=6 step references ($STEP_MARKERS)" || log FAIL "P2: wizard has only $STEP_MARKERS step references"

# === App.tsx invokes is_onboarding_complete on mount ===
grep -q "is_onboarding_complete" src/App.tsx && log PASS "P2: App.tsx invokes is_onboarding_complete" || log FAIL "P2: App.tsx does not invoke is_onboarding_complete"
APP_LINES=$(wc -l < src/App.tsx)
[ "$APP_LINES" -lt 200 ] && log PASS "P2: App.tsx still < 200 lines ($APP_LINES)" || log FAIL "P2: App.tsx too long ($APP_LINES)"

# === Wizard test extended (must mock ollama_health and assert behavior) ===
WIZARD_TEST_LINES=$(wc -l < src/components/OnboardingWizard.test.tsx 2>/dev/null || echo 0)
[ "$WIZARD_TEST_LINES" -ge 60 ] && log PASS "P2: wizard test extended ($WIZARD_TEST_LINES lines)" || log FAIL "P2: wizard test too small ($WIZARD_TEST_LINES, need >=60)"
grep -q "ollama_health" src/components/OnboardingWizard.test.tsx 2>/dev/null && log PASS "P2: wizard test mocks ollama_health" || log FAIL "P2: wizard test missing ollama_health mock"
# Test must contain ollama-unreachable and complete-flow cases
grep -qE "unreachable|not detected|offline|reachable:\s*false" src/components/OnboardingWizard.test.tsx 2>/dev/null && log PASS "P2: wizard test covers unreachable Ollama" || log FAIL "P2: wizard test missing unreachable case"

# === Tests must actually pass (not just exist) ===
VITEST_OUT=$(npm test --silent 2>&1)
echo "$VITEST_OUT" | tail -5
PASSED=$(echo "$VITEST_OUT" | grep -oE 'Tests +[0-9]+ passed' | grep -oE '[0-9]+' | head -1)
PASSED=${PASSED:-0}
# Phase 1 had 10 component tests. Phase 2 adds at least 3 wizard cases → expect >= 13.
[ "$PASSED" -ge 13 ] && log PASS "P2: vitest >=13 passed (got $PASSED)" || log FAIL "P2: vitest only $PASSED passed (need >=13)"

# === Rust test count: >= 15 (Phase 2 baseline = 14 + 1 settings round-trip) ===
# Monotonic, not equality. Anti-padding is enforced by the named-test-presence
# check below — equality was a one-shot design flaw (see Phase 1 DoD post-mortem).
# Subsequent phases may add Rust tests; they must not delete tests below 15.
RUST_OUT=$(cd src-tauri && cargo test --all 2>&1)
echo "$RUST_OUT" | grep "test result:" | head -3
RUST_COUNT=$(echo "$RUST_OUT" | grep "test result:" | head -1 | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | head -1)
RUST_COUNT=${RUST_COUNT:-0}
[ "$RUST_COUNT" -ge 15 ] && log PASS "P2: cargo test $RUST_COUNT passed (>=15 baseline)" || log FAIL "P2: cargo test $RUST_COUNT passed (need >=15)"

# Settings round-trip test must exist by name (and pass — captured above)
grep -q "fn test_settings_round_trip" src-tauri/src/core/tests.rs 2>/dev/null && log PASS "P2: settings round-trip test declared" || log FAIL "P2: test_settings_round_trip not declared"

# === Build green ===
BUILD_OUT=$(npm run build 2>&1)
echo "$BUILD_OUT" | tail -3
echo "$BUILD_OUT" | grep -q 'built in' && log PASS "P2: vite build" || log FAIL "P2: vite build"

# === Scope lock: these files MUST be unchanged from main ===
# Phase 2 is onboarding only. No backend security/auth/db/scraper/etc changes.
# Compare against origin/main to catch scope drift.
SCOPE_LOCKED=(
  "src-tauri/src/core/auth.rs"
  "src-tauri/src/core/db.rs"
  "src-tauri/src/core/scraper.rs"
  "src-tauri/src/core/detectors.rs"
  "src-tauri/src/core/compiler.rs"
  "src-tauri/src/core/guardrails.rs"
  "src-tauri/src/core/llm.rs"
  "src-tauri/src/core/backups.rs"
  "src-tauri/src/core/server.rs"
  "src-tauri/src/core/discovery.rs"
)
DRIFT=0
for f in "${SCOPE_LOCKED[@]}"; do
  if ! git diff --quiet origin/main..HEAD -- "$f" 2>/dev/null; then
    echo "  scope drift: $f modified vs main"
    DRIFT=$((DRIFT+1))
  fi
done
[ "$DRIFT" -eq 0 ] && log PASS "P2: scope lock — no backend drift" || log FAIL "P2: $DRIFT scope-locked files modified"

# === Phase 1 components must be intact ===
# Each component file from Phase 1 must still be >= 30 lines (no stubs).
PHASE1_REGRESSION=0
for c in Layout LeadQueue Workbench PairDialog SystemStatus PublishPanel SettingsPanel SourcesPanel AppContent; do
  f="src/components/$c.tsx"
  [ -f "$f" ] || { echo "  Phase 1 component missing: $f"; PHASE1_REGRESSION=$((PHASE1_REGRESSION+1)); continue; }
  L=$(wc -l < "$f")
  [ "$L" -lt 30 ] && { echo "  Phase 1 component shrunk: $f ($L lines)"; PHASE1_REGRESSION=$((PHASE1_REGRESSION+1)); }
done
[ "$PHASE1_REGRESSION" -eq 0 ] && log PASS "P2: Phase 1 components intact" || log FAIL "P2: $PHASE1_REGRESSION Phase 1 components missing/shrunk"

# === v0.1.1 XSS test still has its assertion ===
grep -q 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs \
  && grep -A 30 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs | grep -qE 'assert(_eq)?!.*&lt;script' \
  && log PASS "P2: v0.1.1 XSS test intact" || log FAIL "P2: v0.1.1 XSS test sabotaged or missing"

# === tsconfig strict not weakened ===
grep -q '"noUnusedLocals": true' tsconfig.json && log PASS "P2: tsconfig strict" || log FAIL "P2: tsconfig weakened"
grep -q '"noUnusedParameters": true' tsconfig.json && log PASS "P2: tsconfig strict params" || log FAIL "P2: tsconfig params weakened"

# === No scope-drift artifacts ===
! [ -d temp-repo ] && ! [ -d temp-ctt ] && ! [ -d temp-civiccast ] && ! [ -d temp-civicclerk ] && ! [ -d temp-civicsuite ] \
  && log PASS "P2: no temp clone dirs" || log FAIL "P2: temp clone dirs left in repo"
! [ -f check.sh ] && ! [ -f extract.py ] && ! [ -f instructions.txt ] && ! [ -f extract.cjs ] && ! [ -f inject.cjs ] \
  && log PASS "P2: no scratch files" || log FAIL "P2: scratch files present"

echo ""
echo "================================="
echo "Phase 2 DoD: $PASS pass / $FAIL fail"
echo "================================="
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
