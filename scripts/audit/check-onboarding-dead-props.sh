#!/usr/bin/env bash
set -euo pipefail

# Default file path
REPO_ROOT=$(git rev-parse --show-toplevel)
FILE_PATH="$REPO_ROOT/src/components/OnboardingWizard.tsx"
FITNESS_TEST=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --file)
      FILE_PATH="$2"
      shift 2
      ;;
    --fitness-test)
      FITNESS_TEST=true
      shift
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

run_check() {
  local target_file="$1"
  if [ ! -f "$target_file" ]; then
    echo "Error: File $target_file not found"
    return 1
  fi

  # Extract interface block
  local interface_block
  interface_block=$(awk '/interface OnboardingWizardProps {/,/}/' "$target_file")
  if [ -z "$interface_block" ]; then
    echo "Error: Could not find interface OnboardingWizardProps in $target_file"
    return 1
  fi

  # Get the prop names
  local props
  props=$(echo "$interface_block" | grep -E '^\s*[a-zA-Z0-9_]+\??\s*:' | sed -E 's/^\s*([a-zA-Z0-9_]+).*/\1/')

  local unused_found=false
  for prop in $props; do
    # Count occurrences of the exact word
    local count
    count=$(grep -o -w "$prop" "$target_file" | wc -l)
    if [ "$count" -le 1 ]; then
      echo "FAIL: Prop '$prop' is defined in OnboardingWizardProps but not used"
      unused_found=true
    fi
  done

  if [ "$unused_found" = true ]; then
    return 1
  else
    echo "PASS: All props in OnboardingWizardProps are used"
    return 0
  fi
}

if [ "$FITNESS_TEST" = true ]; then
  echo "Running fitness test..."
  TMPDIR=$(mktemp -d)
  
  # 1. Create a known-bad component
  BAD_FILE="$TMPDIR/BadComponent.tsx"
  cat <<'EOF' > "$BAD_FILE"
interface OnboardingWizardProps {
  usedProp: boolean;
  unusedProp: string;
}
export const OnboardingWizard = ({ usedProp }) => {
  console.log(usedProp);
}
EOF

  # 2. Create a known-good component
  GOOD_FILE="$TMPDIR/GoodComponent.tsx"
  cat <<'EOF' > "$GOOD_FILE"
interface OnboardingWizardProps {
  usedProp: boolean;
  anotherUsed: string;
}
export const OnboardingWizard = ({ usedProp, anotherUsed }) => {
  console.log(usedProp, anotherUsed);
}
EOF

  # Test bad file (must fail)
  if run_check "$BAD_FILE" > /dev/null; then
    echo "FAIL: Fitness test failed: bad component was not flagged"
    rm -rf "$TMPDIR"
    exit 1
  else
    echo "OK: Bad component correctly flagged"
  fi

  # Test good file (must pass)
  if ! run_check "$GOOD_FILE" > /dev/null; then
    echo "FAIL: Fitness test failed: good component was flagged"
    rm -rf "$TMPDIR"
    exit 1
  else
    echo "OK: Good component correctly passed"
  fi

  rm -rf "$TMPDIR"
  echo "Fitness test passed successfully."
  exit 0
fi

# Run normal check
run_check "$FILE_PATH"
