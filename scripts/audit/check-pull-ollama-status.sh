#!/usr/bin/env bash
set -euo pipefail

# Get repository root
REPO_ROOT=$(git rev-parse --show-toplevel)
FILE_PATH="$REPO_ROOT/src-tauri/src/core/llm.rs"

if [ ! -f "$FILE_PATH" ]; then
  echo "Error: llm.rs not found at $FILE_PATH"
  exit 1
fi

# Extract pull_ollama_model function body
BODY=$(awk '/pub async fn pull_ollama_model/,/^}/' "$FILE_PATH")

if [ -z "$BODY" ]; then
  echo "Error: Could not extract pull_ollama_model body"
  exit 1
fi

if echo "$BODY" | grep -q "is_success"; then
  echo "PASS: pull_ollama_model checks response status using is_success"
  exit 0
else
  echo "FAIL: pull_ollama_model does not check response status with is_success"
  exit 1
fi
