#!/bin/bash
# Strengthened quote-evasion checks for gemma2:9b

echo "Running quote-evasion checks..."
# Exclude tests.rs, frontend test files, and the two authorized fallback lines
matches=$(grep -rnE "['\"]gemma2:9b['\"]" src/ src-tauri/src/ | grep -vE "tests.rs|\.test\.tsx|fallback =|model = ram >= 12")

if [ -n "$matches" ]; then
    echo "FAIL: Found raw 'gemma2:9b' or \"gemma2:9b\" literals in production code (outside of authorized fallback sites):"
    echo "$matches"
    exit 1
else
    echo "PASS: No quote-evasion issues found."
    exit 0
fi
