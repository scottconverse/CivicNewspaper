#!/bin/bash
# scripts/verify-release-assets.sh
#
# Release-integrity gate. Asserts that the SHA256SUMS manifest is published as a
# release asset AND that every filename it lists is also a published asset. Fails
# the release if the checksum manifest is missing or references an artifact that
# would 404 for a user following docs/install.md.
#
# Usage: verify-release-assets.sh <tag>
set -euo pipefail

TAG="${1:-}"
if [ -z "$TAG" ]; then
  echo "Usage: verify-release-assets.sh <tag>" >&2
  exit 2
fi

REPO="${GITHUB_REPOSITORY:-scottconverse/CivicNewspaper}"

echo "=== Release-integrity gate for $TAG ($REPO) ==="

# Names of all assets currently published on the release.
mapfile -t ASSETS < <(gh release view "$TAG" --repo "$REPO" --json assets --jq '.assets[].name')

if [ "${#ASSETS[@]}" -eq 0 ]; then
  echo "FAIL: release $TAG has no published assets" >&2
  exit 1
fi

if ! printf '%s\n' "${ASSETS[@]}" | grep -qx 'SHA256SUMS'; then
  echo "FAIL: SHA256SUMS manifest is not published as a release asset on $TAG" >&2
  echo "       (docs/install.md, README.md and FAQ.md tell users to verify against it)" >&2
  exit 1
fi

# Pull the manifest and confirm each filename it lists is actually published.
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/assets"
gh release download "$TAG" --repo "$REPO" --dir "$tmp/assets" --clobber

missing=0
while read -r _hash name; do
  [ -z "${name:-}" ] && continue
  if ! printf '%s\n' "${ASSETS[@]}" | grep -qx "$name"; then
    echo "FAIL: SHA256SUMS lists '$name' but it is not a published release asset" >&2
    missing=1
  fi
done < "$tmp/assets/SHA256SUMS"

if [ "$missing" -ne 0 ]; then
  echo "Release-integrity gate FAILED: manifest references artifacts that would 404." >&2
  exit 1
fi

EVIDENCE_FILE="docs/release-evidence/$TAG.json"
EVIDENCE_ARGS=()
if [ -f "$EVIDENCE_FILE" ]; then
  EVIDENCE_ARGS=(--evidence "$EVIDENCE_FILE")
fi
WINDOWS_RECEIPT_ARGS=()
if [ -f "$tmp/assets/windows-signature-smoke-receipt.json" ]; then
  WINDOWS_RECEIPT_ARGS=(--windows-receipt "$tmp/assets/windows-signature-smoke-receipt.json")
fi
MACOS_RECEIPT_ARGS=()
if [ -f "$tmp/assets/macos-packaged-smoke-receipt.json" ]; then
  MACOS_RECEIPT_ARGS=(--macos-receipt "$tmp/assets/macos-packaged-smoke-receipt.json")
fi
LINUX_RECEIPT_ARGS=()
if [ -f "$tmp/assets/linux-packaged-smoke-receipt.json" ]; then
  LINUX_RECEIPT_ARGS=(--linux-receipt "$tmp/assets/linux-packaged-smoke-receipt.json")
fi

node scripts/verify-release-asset-hashes.mjs \
  --assets-dir "$tmp/assets" \
  --manifest "$tmp/assets/SHA256SUMS" \
  "${EVIDENCE_ARGS[@]}" \
  "${WINDOWS_RECEIPT_ARGS[@]}" \
  "${MACOS_RECEIPT_ARGS[@]}" \
  "${LINUX_RECEIPT_ARGS[@]}"

echo "=== Release-integrity gate PASSED: SHA256SUMS present, assets published, and hashes verified ==="
