#!/bin/bash
# Stronger invariant check: no PARAGRAPH in any production doc instructs the
# user to install Ollama as a separate step, even if the paragraph also
# mentions "bundled" or "sidecar" elsewhere.

set -euo pipefail

TARGET_FILES=""
if [[ $# -ge 2 && "$1" == "--file" ]]; then
  TARGET_FILES="$2"
elif [[ $# -ge 1 ]]; then
  # Support passing file path directly or with --file
  if [[ "$1" == "--file" ]]; then
    echo "Error: --file option requires an argument"
    exit 1
  fi
  TARGET_FILES="$1"
fi

fail=0

if [[ -n "$TARGET_FILES" ]]; then
  if [[ ! -f "$TARGET_FILES" ]]; then
    echo "Error: file $TARGET_FILES not found"
    exit 1
  fi
  FILES_TO_CHECK="$TARGET_FILES"
else
  FILES_TO_CHECK=$(find . -type f \( -name "*.md" -o -name "*.tsx" -o -name "*.ts" -o -name "*.rs" -o -name "*.html" \) \
             ! -path "./.git/*" ! -path "./node_modules/*" ! -path "./target/*" ! -path "./dist/*" ! -path "./.agent-runs/*" \
             ! -path "*/audits/*" ! -path "*/forensic/*" ! -path "*/stop-reports/*")
fi

for f in $FILES_TO_CHECK; do
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
            
            # If the line says "do not need" or "no need" or "no separate", it is not an install instruction:
            if (line ~ /not need/ || line ~ /no need/ || line ~ /no separate/ || line ~ /don\x27t need/) {
              continue
            }
            
            # Check if the paragraph contains developer exemptions:
            if ($0 !~ /[Ff]or developers/ && $0 !~ /[Bb]uild.*from source/) {
              print file ": INVARIANT VIOLATION: " line
              exit 1
            }
          }
        }
      }
    }
  ' "$f" || fail=$((fail+1))
done

if [ $fail -eq 0 ]; then
  echo "OK: Ollama-install invariant holds"
  exit 0
else
  echo "FAIL: $fail file(s) violate the invariant"
  exit 1
fi
