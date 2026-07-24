# Hosted Release Evidence

This folder contains hosted release evidence files for release candidates that are ready for tag review.

Before pushing a `vX.Y.Z` tag, add or update `docs/release-evidence/vX.Y.Z.json` in the same commit that will be tagged. The hosted release workflow refuses to build or upload public beta artifacts unless that JSON file proves, for the exact tag and commit:

- local release smoke passed;
- default-model bakeoff passed;
- dependency audit passed;
- Windows installer smoke passed against the packaged installer;
- packaged first-run walkthrough passed;
- an artifact-bound local isolated-package or external cleanroom report passed;
- the installer SHA256 used in cleanroom matches the installer SHA256 from local installer smoke.
- the hosted release asset hash in `SHA256SUMS` matches that same cleanroom-tested installer SHA256.

When `release_scope.macos_apple_silicon_beta` is true, the checked-in evidence
file must also record a completed Apple Silicon preflight with:

- an ARM64 packaged-app preflight receipt;
- an explicit record that the beta is not Developer ID signed and is not notarized;
- a macOS 11 minimum and manual Ollama setup.

DMGs are not byte-reproducible across rebuilds, so a pre-tag local DMG hash must
not be presented as the hash of a newly built hosted artifact. The tag workflow
therefore smoke-tests the exact DMG on a fresh hosted Mac runner, publishes
`macos-packaged-smoke-receipt.json`, and requires that receipt's hash to match
the DMG entry in the published `SHA256SUMS`.

This keeps GitHub Releases from going green on tag-only automation when the real local RC and cleanroom evidence has not been reviewed.
