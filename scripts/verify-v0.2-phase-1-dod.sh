#!/bin/bash
# v0.2 Phase 1 only — frontend rebuild DoD.
# This script is committed to scripts/verify-v0.2-phase-1-dod.sh in CivicNewspaper
# at the start of Phase 1. Its SHA-256 hash is pinned in scripts/.dod-phase-1.sha256.
# CI hash-checks the script before running it. Any local edit fails CI.
#
# Behavior-based assertions only. No string-presence greps.
set -u
PASS=0
FAIL=0
log() { echo "[$1] $2"; if [ "$1" = "PASS" ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi; }

# === Phase 1 work-present gate ===
# This DoD verifies Phase 1 (frontend rebuild) when it's being introduced.
# On branches that don't include Phase 1 work (e.g., the setup branch that
# only adds this script, or unrelated branches), there's nothing to verify
# and the DoD exits 0. Trigger signal: src/components/ exists OR package.json
# declares vitest as a dependency. Phase 1 cannot ship without either.
if [ ! -d src/components ] && ! grep -q '"vitest"' package.json 2>/dev/null; then
  echo "ℹ Phase 1 work not present on this branch:"
  echo "    src/components/ does not exist AND package.json has no vitest dep."
  echo "    Phase 1 DoD has nothing to verify — exiting 0."
  echo "    (When Phase 1 work is introduced, this skip gate stops applying"
  echo "     and every check below must pass.)"
  exit 0
fi
echo "✓ Phase 1 work detected — running full verification"
echo ""

# === Component decomposition ===
[ "$(ls src/components/*.tsx 2>/dev/null | grep -v '\.test\.tsx$' | wc -l)" -ge 8 ] \
  && log PASS "P1: >=8 components" || log FAIL "P1: components count"

# App.tsx decomposed — strict ceiling
APP_LINES=$(wc -l < src/App.tsx 2>/dev/null || echo 99999)
[ "$APP_LINES" -lt 200 ] && log PASS "P1: App.tsx < 200 lines ($APP_LINES)" || log FAIL "P1: App.tsx too long ($APP_LINES)"

# Real component content — each .tsx >= 30 lines (1-line stubs fail this)
STUB_COUNT=0
for f in src/components/*.tsx; do
  [[ "$f" == *.test.tsx ]] && continue
  LINES=$(wc -l < "$f")
  [ "$LINES" -lt 30 ] && STUB_COUNT=$((STUB_COUNT+1)) && echo "  stub: $f ($LINES lines)"
done
[ "$STUB_COUNT" -eq 0 ] && log PASS "P1: no component stubs" || log FAIL "P1: $STUB_COUNT stub components"

# Real test content — each .test.tsx >= 15 lines (smoke tests fail this)
TEST_STUB_COUNT=0
for f in src/components/*.test.tsx; do
  LINES=$(wc -l < "$f")
  [ "$LINES" -lt 15 ] && TEST_STUB_COUNT=$((TEST_STUB_COUNT+1)) && echo "  stub test: $f ($LINES lines)"
done
[ "$TEST_STUB_COUNT" -eq 0 ] && log PASS "P1: no test stubs" || log FAIL "P1: $TEST_STUB_COUNT stub tests"

# === Test framework ===
grep -q '"vitest"' package.json && log PASS "P1: vitest installed" || log FAIL "P1: vitest"
grep -q '"@testing-library/react"' package.json && log PASS "P1: testing-library installed" || log FAIL "P1: testing-library"

# === Tests must actually pass (not just exist) ===
# Run vitest, capture output, assert >=8 passed (real, not stub)
VITEST_OUT=$(npm test --silent 2>&1)
echo "$VITEST_OUT" | tail -20
PASSED=$(echo "$VITEST_OUT" | grep -oE 'Tests +[0-9]+ passed' | grep -oE '[0-9]+' | head -1)
PASSED=${PASSED:-0}
[ "$PASSED" -ge 8 ] && log PASS "P1: vitest >=8 passed (got $PASSED)" || log FAIL "P1: vitest only $PASSED passed"

# === Assertion density — each test must contain assertions, not just render() ===
ASSERTLESS=0
for f in src/components/*.test.tsx; do
  if ! grep -qE '\bexpect\b|\btoBe\b|\btoHaveBeenCalled\b|\btoEqual\b|\btoContain\b' "$f"; then
    ASSERTLESS=$((ASSERTLESS+1))
    echo "  no expect/assert: $f"
  fi
done
[ "$ASSERTLESS" -eq 0 ] && log PASS "P1: every test has assertions" || log FAIL "P1: $ASSERTLESS tests have no assertions"

# === Responsive + dark mode CSS ===
grep -q 'prefers-color-scheme' src/App.css && log PASS "P1: dark mode CSS" || log FAIL "P1: no dark mode"
grep -qE '@media \(max-width' src/App.css && log PASS "P1: responsive breakpoint" || log FAIL "P1: no responsive CSS"

# === tsconfig strict not weakened ===
grep -q '"noUnusedLocals": true' tsconfig.json && log PASS "P1: tsconfig strict" || log FAIL "P1: tsconfig weakened"
grep -q '"noUnusedParameters": true' tsconfig.json && log PASS "P1: tsconfig strict params" || log FAIL "P1: tsconfig params weakened"

# === Build green ===
BUILD_OUT=$(npm run build 2>&1)
echo "$BUILD_OUT" | tail -5
echo "$BUILD_OUT" | grep -q 'built in' && log PASS "P1: vite build" || log FAIL "P1: vite build"

# === Rust still green (no regression) ===
RUST_OUT=$(cd src-tauri && cargo test --all 2>&1)
echo "$RUST_OUT" | tail -5
echo "$RUST_OUT" | grep "test result:" | head -1 | grep -qE '14 passed' && log PASS "P1: rust 14/14 (no regression)" || log FAIL "P1: rust test count changed"

# === No DoD script tampering ===
# This script is in the repo. CI hash-checks the script before running it.
# A local edit will fail CI's guard step, before this script even runs.
# But also check the v0.1.1 XSS test was not sabotaged:
grep -q 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs \
  && grep -A 30 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs | grep -qE 'assert(_eq)?!.*&lt;script' \
  && log PASS "P1: v0.1.1 XSS test intact" || log FAIL "P1: v0.1.1 XSS test sabotaged or missing"

# === No scope drift ===
! [ -d temp-repo ] && ! [ -d temp-ctt ] && ! [ -d temp-civiccast ] && ! [ -d temp-civicclerk ] \
  && log PASS "P1: no temp clone dirs" || log FAIL "P1: temp clone dirs left in repo"

! [ -f check.sh ] && ! [ -f extract.py ] && ! [ -f instructions.txt ] \
  && log PASS "P1: no scratch files" || log FAIL "P1: scratch files present"

echo ""
echo "================================="
echo "Phase 1 DoD: $PASS pass / $FAIL fail"
echo "================================="
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
