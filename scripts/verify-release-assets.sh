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
gh release download "$TAG" --repo "$REPO" --dir "$tmp" --pattern 'SHA256SUMS' --clobber

missing=0
while read -r _hash name; do
  [ -z "${name:-}" ] && continue
  if ! printf '%s\n' "${ASSETS[@]}" | grep -qx "$name"; then
    echo "FAIL: SHA256SUMS lists '$name' but it is not a published release asset" >&2
    missing=1
  fi
done < "$tmp/SHA256SUMS"

if [ "$missing" -ne 0 ]; then
  echo "Release-integrity gate FAILED: manifest references artifacts that would 404." >&2
  exit 1
fi

echo "=== Release-integrity gate PASSED: SHA256SUMS present and all listed artifacts published ==="
