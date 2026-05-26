#!/bin/bash
# scripts/verify-release.sh
# Verifies that both frontend and backend crates build successfully in release mode.
set -eu

echo "=== Verifying Release Packaging ==="

# 1. Build frontend
echo "Building frontend..."
npm run build

# 2. Build backend in release mode
echo "Building backend in release mode..."
cd src-tauri
cargo build --release

echo "=== Packaging Verification Passed ==="
exit 0
