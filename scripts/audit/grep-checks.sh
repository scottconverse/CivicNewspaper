#!/bin/bash
# Strengthened model-construction and quote-evasion checks for gemma2:9b

set -euo pipefail

# Find repo root to be cwd-insensitive (Wnit-4)
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR/../.." && pwd)

# Parse arguments
TARGET_FILE=""
TARGET_DIR=""
FITNESS_TEST=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --file)
      if [[ -z "${2:-}" ]]; then
        echo "Error: --file requires an argument"
        exit 1
      fi
      TARGET_FILE="$2"
      shift 2
      ;;
    --dir)
      if [[ -z "${2:-}" ]]; then
        echo "Error: --dir requires an argument"
        exit 1
      fi
      TARGET_DIR="$2"
      shift 2
      ;;
    --fitness-test)
      FITNESS_TEST=true
      shift
      ;;
    *)
      TARGET_FILE="$1"
      shift
      ;;
  esac
done

check_content() {
  local f="$1"
  local violations=""
  
  if [[ ! -f "$f" ]]; then
    return 0
  fi
  
  # Check if the filename itself is a test file (exclude from scan)
  if [[ "$f" =~ \.test\.tsx$ || "$f" =~ tests\.rs$ || "$f" =~ server_tests\.rs$ ]]; then
    return 0
  fi

  while IFS= read -r line || [[ -n "$line" ]]; do
    if echo "$line" | grep -iqE "gemma2.*9b|9b.*gemma2"; then
      # Otherwise it is a violation!
      violations="$violations\n$f: $line"
    fi
  done < "$f"
  
  if [[ -n "$violations" ]]; then
    echo -e "$violations" | sed '/^$/d'
    return 1
  fi
  return 0
}

if [ "$FITNESS_TEST" = true ]; then
  echo "Running fitness test..."
  FIXTURE_DIR="$REPO_ROOT/scripts/audit/fixtures"
  fail=0
  
  # Test fixtures 1-4 must FAIL
  for n in 1 2 3 4; do
    fixture="$FIXTURE_DIR/known-bad-model-construction-fixture$n.ts"
    if check_content "$fixture" >/dev/null; then
      echo "FITNESS-TEST FAIL: fixture$n passed but should have failed"
      fail=1
    else
      echo "  - fixture$n caught (OK)"
    fi
  done
  
  # Test fixture 5 must PASS
  fixture5="$FIXTURE_DIR/known-good-model-construction-fixture5.ts"
  if ! check_content "$fixture5" >/dev/null; then
    echo "FITNESS-TEST FAIL: fixture5 failed but should have passed"
    fail=1
  else
    echo "  - fixture5 passed (OK)"
  fi
  
  if [ $fail -eq 0 ]; then
    echo "FITNESS-TEST PASS"
    exit 0
  else
    exit 1
  fi
fi

# Normal mode
fail=0

if [[ -n "$TARGET_FILE" ]]; then
  check_content "$TARGET_FILE" || fail=$((fail+1))
elif [[ -n "$TARGET_DIR" ]]; then
  FILES=$(find "$TARGET_DIR" -type f \( -name "*.ts" -o -name "*.tsx" -o -name "*.rs" -o -name "*.js" \) \
          ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" ! -path "*/dist/*")
  for f in $FILES; do
    check_content "$f" || fail=$((fail+1))
  done
else
  # Scan entire production code under src/ and src-tauri/src/
  FILES=$(find "$REPO_ROOT/src" "$REPO_ROOT/src-tauri/src" -type f \( -name "*.ts" -o -name "*.tsx" -o -name "*.rs" -o -name "*.js" \) \
          ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" ! -path "*/dist/*" \
          ! -name "*.test.tsx" ! -name "tests.rs" ! -name "server_tests.rs" 2>/dev/null || true)
  for f in $FILES; do
    check_content "$f" || fail=$((fail+1))
  done
fi

if [ $fail -eq 0 ]; then
  echo "PASS: No model construction / quote-evasion issues found."
  exit 0
else
  echo "FAIL: Found $fail file(s) with model construction / quote-evasion issues."
  exit 1
fi
