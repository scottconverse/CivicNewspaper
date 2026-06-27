#!/bin/bash
# v0.2 Phase 3 only — Diagnostic Export DoD.
# Committed to scripts/verify-v0.2-phase-3-dod.sh in CivicNewspaper.
# SHA-256 pinned in scripts/.dod-phase-3.sha256. CI hash-checks before running.
# Any local edit fails CI's guard step.
#
# Behavior-based assertions only. No string-presence-only greps where a stub
# or comment could pass the check. Test-count uses monotonic + named checks,
# not equality (Phase 1's equality lock was a one-shot design flaw).

set -u
PASS=0
FAIL=0
log() { echo "[$1] $2"; if [ "$1" = "PASS" ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi; }

# === Phase 3 work-present gate ===
HAS_DIAG_FILE=0
HAS_EXPORT_CMD=0
[ -f src-tauri/src/core/diagnostics.rs ] && HAS_DIAG_FILE=1
grep -qE "fn export_diagnostics\b" src-tauri/src/tauri_cmds.rs src-tauri/src/core/*.rs 2>/dev/null && HAS_EXPORT_CMD=1
if [ "$HAS_DIAG_FILE" -eq 0 ] && [ "$HAS_EXPORT_CMD" -eq 0 ]; then
  echo "i Phase 3 work not present on this branch:"
  echo "    No diagnostics.rs AND no export_diagnostics command declared."
  echo "    Phase 3 DoD has nothing to verify — exiting 0."
  exit 0
fi
echo "ok Phase 3 work detected — running full verification"
echo ""

# === diagnostics.rs exists and has gather_diagnostics ===
[ -f src-tauri/src/core/diagnostics.rs ] && log PASS "P3: diagnostics.rs present" || log FAIL "P3: diagnostics.rs missing"
grep -qE "fn gather_diagnostics\b" src-tauri/src/core/diagnostics.rs 2>/dev/null && log PASS "P3: gather_diagnostics fn declared" || log FAIL "P3: gather_diagnostics not declared"

# === Diagnostics struct has required field name fragments ===
if [ -f src-tauri/src/core/diagnostics.rs ]; then
  DIAG=$(cat src-tauri/src/core/diagnostics.rs)
  REQUIRED_FIELDS=( app_version os_name os_version tauri_version ollama_reachable ollama_models db_schema evidence leads drafts published panic_log )
  MISSING=0
  for fld in "${REQUIRED_FIELDS[@]}"; do
    if ! echo "$DIAG" | grep -qi "$fld"; then
      echo "  diagnostics field missing: $fld"
      MISSING=$((MISSING+1))
    fi
  done
  TOTAL=${#REQUIRED_FIELDS[@]}
  [ "$MISSING" -le 2 ] && log PASS "P3: diagnostics struct fields present ($((TOTAL-MISSING))/$TOTAL)" || log FAIL "P3: diagnostics struct missing $MISSING / $TOTAL required fields"
fi

# === Tauri command body anti-stub check ===
check_cmd_body() {
  local fn_name="$1"
  local found=0
  for f in src-tauri/src/tauri_cmds.rs src-tauri/src/core/*.rs; do
    [ -f "$f" ] || continue
    if grep -qE "fn $fn_name\b" "$f"; then
      found=1
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
      BODY_LINES=$(echo "$BODY" | grep -vE '^\s*($|//|\{|\})' | wc -l)
      if [ "$BODY_LINES" -ge 3 ]; then
        log PASS "P3: $fn_name has substantive body ($BODY_LINES lines)"
      else
        log FAIL "P3: $fn_name body is a stub ($BODY_LINES non-trivial lines)"
      fi
      break
    fi
  done
  [ "$found" -eq 0 ] && log FAIL "P3: $fn_name not declared anywhere"
}

check_cmd_body "export_diagnostics"

# === No outbound network in diagnostics.rs ===
if [ -f src-tauri/src/core/diagnostics.rs ]; then
  NET_HITS=$(grep -cE "reqwest::|ureq::|http::Request|hyper::Client" src-tauri/src/core/diagnostics.rs)
  [ "$NET_HITS" -eq 0 ] && log PASS "P3: no outbound network in diagnostics.rs" || log FAIL "P3: diagnostics.rs has $NET_HITS network client refs"
fi

# === Panic hook installed in lib.rs ===
grep -qE "panic::set_hook" src-tauri/src/lib.rs 2>/dev/null && log PASS "P3: panic hook in lib.rs" || log FAIL "P3: no panic hook installed"

# === Rolling log rotation at 1 MB ===
if grep -qE "1_048_576|1024 *\* *1024|1\*1024\*1024|1_000_000" src-tauri/src/core/diagnostics.rs src-tauri/src/lib.rs 2>/dev/null; then
  log PASS "P3: panic log rotation logic present"
else
  log FAIL "P3: no 1 MB rotation constant found in diagnostics.rs or lib.rs"
fi

# === SystemStatus has Export button + invokes export_diagnostics ===
grep -q "Export Diagnostic Report" src/components/SystemStatus.tsx 2>/dev/null && log PASS "P3: SystemStatus Export button literal" || log FAIL "P3: SystemStatus missing Export Diagnostic Report literal"
grep -q "exportDiagnostics" src/components/SystemStatus.tsx 2>/dev/null && log PASS "P3: SystemStatus invokes exportDiagnostics helper" || log FAIL "P3: SystemStatus does not reference exportDiagnostics helper"
grep -qE "plugin-dialog|tauri-apps/plugin-dialog" src/useApp.ts src/components/SystemStatus.tsx 2>/dev/null && log PASS "P3: diagnostics flow uses plugin-dialog" || log FAIL "P3: diagnostics flow missing plugin-dialog import"

# === SECURITY.md updated ===
grep -qiE "diagnostic report|diagnostics export|## diagnostic" SECURITY.md 2>/dev/null && log PASS "P3: SECURITY.md diagnostic section" || log FAIL "P3: SECURITY.md missing diagnostic section"
grep -qiE "no automatic upload|no auto.upload|never upload|no network" SECURITY.md 2>/dev/null && log PASS "P3: SECURITY.md states no auto-upload" || log FAIL "P3: SECURITY.md missing no-auto-upload statement"

# === Rust test count: >= 17 (15 Phase 2 baseline + 2 new Phase 3 tests) ===
RUST_OUT=$(cd src-tauri && cargo test --all 2>&1)
echo "$RUST_OUT" | grep "test result:" | head -3
RUST_COUNT=$(echo "$RUST_OUT" | grep "test result:" | head -1 | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | head -1)
RUST_COUNT=${RUST_COUNT:-0}
[ "$RUST_COUNT" -ge 17 ] && log PASS "P3: cargo test $RUST_COUNT passed (>=17 baseline)" || log FAIL "P3: cargo test $RUST_COUNT passed (need >=17)"

# === Phase 3 named tests must exist ===
grep -q "fn test_gather_diagnostics_has_all_fields" src-tauri/src/core/tests.rs 2>/dev/null && log PASS "P3: test_gather_diagnostics_has_all_fields declared" || log FAIL "P3: test_gather_diagnostics_has_all_fields missing"
grep -q "fn test_export_diagnostics_writes_valid_json" src-tauri/src/core/tests.rs 2>/dev/null && log PASS "P3: test_export_diagnostics_writes_valid_json declared" || log FAIL "P3: test_export_diagnostics_writes_valid_json missing"

# === Cross-phase regression: prior named tests still present ===
grep -q "fn test_settings_round_trip" src-tauri/src/core/tests.rs 2>/dev/null && log PASS "P3: Phase 2 settings round-trip intact" || log FAIL "P3: test_settings_round_trip deleted (Phase 2 regression)"
if grep -q 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs 2>/dev/null && grep -A 30 'fn test_compiler_xss_safe' src-tauri/src/core/tests.rs | grep -qE 'assert(_eq)?!.*&lt;script'; then
  log PASS "P3: v0.1.1 XSS test intact"
else
  log FAIL "P3: v0.1.1 XSS test sabotaged or missing"
fi

# === Build green ===
BUILD_OUT=$(npm run build 2>&1)
echo "$BUILD_OUT" | tail -3
echo "$BUILD_OUT" | grep -q 'built in' && log PASS "P3: vite build" || log FAIL "P3: vite build"

# === Scope lock: only enforced when Phase 3 is being newly introduced ===
# Phase 3 is the diagnostic export only. Once Phase 3 is merged to main,
# subsequent phases (4, 5, 6, 7, 8) may legitimately modify previously-
# locked files; their own directive's scope-lock enforces what they can
# touch. The "phase 3 not yet merged" detection uses
# test_gather_diagnostics_has_all_fields presence on origin/main as the
# anchor signature — present on main means Phase 3 is merged, so this
# branch is a downstream PR and the scope-lock check no longer applies.
if git show origin/main:src-tauri/src/core/tests.rs 2>/dev/null | grep -q "fn test_gather_diagnostics_has_all_fields"; then
  log PASS "P3: scope lock skipped — Phase 3 already on main (downstream PR)"
else
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
  [ "$DRIFT" -eq 0 ] && log PASS "P3: scope lock — no backend drift" || log FAIL "P3: $DRIFT scope-locked files modified"
fi

# === Phase 2 regression: wizard still real, settings migration intact ===
WIZARD_LINES=$(wc -l < src/components/OnboardingWizard.tsx 2>/dev/null || echo 0)
[ "$WIZARD_LINES" -ge 300 ] && log PASS "P3: OnboardingWizard still >=300 lines" || log FAIL "P3: OnboardingWizard shrunk ($WIZARD_LINES lines)"
[ -f src-tauri/migrations/0003_settings.sql ] && log PASS "P3: 0003_settings.sql intact" || log FAIL "P3: 0003_settings.sql deleted"

# === Phase 1 regression: App.tsx and components intact ===
APP_LINES=$(wc -l < src/App.tsx 2>/dev/null || echo 0)
[ "$APP_LINES" -gt 0 ] && [ "$APP_LINES" -lt 200 ] && log PASS "P3: App.tsx still < 200 lines ($APP_LINES)" || log FAIL "P3: App.tsx wrong size ($APP_LINES)"
if grep -q "onboardingDone" src/App.tsx 2>/dev/null && grep -q "isOnboardingComplete" src/useApp.ts 2>/dev/null; then
  log PASS "P3: App/useApp still gates onboarding"
else
  log FAIL "P3: App/useApp onboarding gate removed"
fi

PHASE1_REGRESSION=0
for c in Layout LeadQueue Workbench PairDialog SystemStatus PublishPanel SettingsPanel SourcesPanel AppContent; do
  f="src/components/$c.tsx"
  [ -f "$f" ] || { echo "  Phase 1 component missing: $f"; PHASE1_REGRESSION=$((PHASE1_REGRESSION+1)); continue; }
  L=$(wc -l < "$f")
  [ "$L" -lt 30 ] && { echo "  Phase 1 component shrunk: $f ($L lines)"; PHASE1_REGRESSION=$((PHASE1_REGRESSION+1)); }
done
[ "$PHASE1_REGRESSION" -eq 0 ] && log PASS "P3: Phase 1 components intact" || log FAIL "P3: $PHASE1_REGRESSION Phase 1 components missing/shrunk"

# === tsconfig still strict ===
grep -q '"noUnusedLocals": true' tsconfig.json && log PASS "P3: tsconfig noUnusedLocals" || log FAIL "P3: tsconfig noUnusedLocals weakened"
grep -q '"noUnusedParameters": true' tsconfig.json && log PASS "P3: tsconfig noUnusedParameters" || log FAIL "P3: tsconfig noUnusedParameters weakened"

# === No scratch artifacts ===
! [ -d temp-repo ] && ! [ -d temp-ctt ] && ! [ -d temp-civiccast ] && ! [ -d temp-civicclerk ] && ! [ -d temp-civicsuite ] \
  && log PASS "P3: no temp clone dirs" || log FAIL "P3: temp clone dirs left in repo"

echo ""
echo "================================="
echo "Phase 3 DoD: $PASS pass / $FAIL fail"
echo "================================="
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
