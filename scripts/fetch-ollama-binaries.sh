#!/bin/bash
set -eu

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
LLM_FILE="$ROOT_DIR/src-tauri/src/core/llm.rs"

echo "v0.3.x releases use app-managed Ollama runtime setup."
echo "Checking pinned runtime metadata in src-tauri/src/core/llm.rs."

if ! grep -q 'OLLAMA_RUNTIME_VERSION' "$LLM_FILE"; then
  echo "FAIL: OLLAMA_RUNTIME_VERSION is missing from $LLM_FILE" >&2
  exit 1
fi

if ! grep -q 'OLLAMA_WINDOWS_AMD64_SHA256' "$LLM_FILE"; then
  echo "FAIL: OLLAMA_WINDOWS_AMD64_SHA256 is missing from $LLM_FILE" >&2
  exit 1
fi

if ! grep -q 'https://github.com/ollama/ollama/releases/download' "$LLM_FILE"; then
  echo "FAIL: pinned Ollama download URL is missing from $LLM_FILE" >&2
  exit 1
fi

echo "OK: app-managed Ollama runtime metadata is pinned."
