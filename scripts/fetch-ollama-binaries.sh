#!/bin/bash
set -eu

echo "Deprecated: v0.3.x releases use app-managed Ollama runtime setup." >&2
echo "The release verifier checks src-tauri/src/core/llm.rs for the pinned runtime URL and SHA256." >&2
exit 2
