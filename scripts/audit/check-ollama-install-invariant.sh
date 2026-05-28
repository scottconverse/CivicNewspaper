#!/bin/bash
# Stronger invariant check: no PARAGRAPH in any production doc instructs the
# user to install Ollama as a separate step, even if the paragraph also
# mentions "bundled" or "sidecar" elsewhere.

set -euo pipefail

# Find repo root to resolve fixture paths
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)

# Parse arguments
TARGET_FILES=""
SELF_TEST=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --file)
      if [[ -z "${2:-}" ]]; then
        echo "Error: --file option requires an argument"
        exit 1
      fi
      TARGET_FILES="$2"
      shift 2
      ;;
    --self-test)
      SELF_TEST=true
      shift
      ;;
    *)
      TARGET_FILES="$1"
      shift
      ;;
  esac
done

# Function to check a single file for the invariant
check_file() {
  local f="$1"
  local fail=0
  
  # For each paragraph, check if it contains an install instruction:
  awk -v file="$f" '
    BEGIN { RS = ""; FS = "\n" }
    {
      # RS="" splits by blank lines (paragraphs).
      # We iterate over each line (since FS="\n", each line is a field $1, $2, ..., $NF)
      for (i = 1; i <= NF; i++) {
        line = $i
        if (line ~ /(install|download).*[Oo]llama/ && line !~ /uninstall/) {
          # Check if this line is an instruction to the user:
          if (line ~ /you.*(install|download).*[Oo]llama/ ||
              line ~ /[Pp]lease (install|download).*[Oo]llama/ ||
              line ~ /[Yy]ou(.| )?ll need to (install|download).*[Oo]llama/ ||
              line ~ /(visit|go to).*ollama\.(com|ai)/) {
            
            # If the line says "do not need" or "no need" or "no separate" or "don\x27t need" or "donot need", it is not an install instruction:
            if (line ~ /not need/ || line ~ /no need/ || line ~ /no separate/ || line ~ /don\x27t need/) {
              continue
            }
            
            # Check if the paragraph contains developer exemptions:
            if (tolower($0) !~ /for developers building from source/) {
              print file ": INVARIANT VIOLATION: " line
              exit 1
            }
          }
        }
      }
    }
  ' "$f" || fail=$((fail+1))
  return $fail
}

if [ "$SELF_TEST" = true ]; then
  echo "Running self-test..."
  FIXTURE_DIR="$REPO_ROOT/scripts/audit/fixtures"
  
  bad_elsewhere="$FIXTURE_DIR/known-bad-ollama-invariant-bundled-elsewhere.md"
  good_exempt="$FIXTURE_DIR/known-good-ollama-invariant-dev-exempt.md"
  good_empty="$FIXTURE_DIR/known-good-ollama-invariant-empty.md"
  good_unrelated="$FIXTURE_DIR/known-good-ollama-invariant-unrelated.md"
  
  # Check bad_elsewhere - MUST FAIL
  if check_file "$bad_elsewhere" >/dev/null 2>&1; then
    echo "SELF-TEST FAIL: bad_elsewhere passed but should have failed"
    exit 1
  fi
  echo "  - bad_elsewhere caught (OK)"
  
  # Check good_exempt - MUST PASS
  if ! check_file "$good_exempt" >/dev/null 2>&1; then
    echo "SELF-TEST FAIL: good_exempt failed but should have passed"
    exit 1
  fi
  echo "  - good_exempt passed (OK)"
  
  # Check good_empty - MUST PASS
  if ! check_file "$good_empty" >/dev/null 2>&1; then
    echo "SELF-TEST FAIL: good_empty failed but should have passed"
    exit 1
  fi
  echo "  - good_empty passed (OK)"
  
  # Check good_unrelated - MUST PASS
  if ! check_file "$good_unrelated" >/dev/null 2>&1; then
    echo "SELF-TEST FAIL: good_unrelated failed but should have passed"
    exit 1
  fi
  echo "  - good_unrelated passed (OK)"
  
  echo "SELF-TEST PASS"
  exit 0
fi

# Normal mode
fail=0

if [[ -n "$TARGET_FILES" ]]; then
  if [[ ! -f "$TARGET_FILES" ]]; then
    echo "Error: file $TARGET_FILES not found"
    exit 1
  fi
  FILES_TO_CHECK="$TARGET_FILES"
else
  # Enumerate whole repository
  FILES_TO_CHECK=$(find "$REPO_ROOT" -type f \( -name "*.md" -o -name "*.tsx" -o -name "*.ts" -o -name "*.rs" -o -name "*.html" \) \
             ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" ! -path "*/dist/*" ! -path "*/.agent-runs/*" \
             ! -path "*/audits/*" ! -path "*/forensic/*" ! -path "*/stop-reports/*" ! -path "*/scripts/audit/fixtures/*")
fi

for f in $FILES_TO_CHECK; do
  check_file "$f" || fail=$((fail+1))
done

if [ $fail -eq 0 ]; then
  echo "OK: Ollama-install invariant holds"
  exit 0
else
  echo "FAIL: $fail file(s) violate the invariant"
  exit 1
fi
