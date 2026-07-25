# Hosted Release Evidence

This folder contains hosted release evidence files for release candidates that are ready for tag review.

Before pushing a `vX.Y.Z` tag, add or update `docs/release-evidence/vX.Y.Z.json`
in the commit that will be tagged. The hosted release workflow refuses to build
or upload public beta artifacts without a matching evidence file.

Legacy evidence files bind a prebuilt Windows installer to local smoke and
cleanroom proof:

- local release smoke passed;
- default-model bakeoff passed;
- dependency audit passed;
- Windows installer smoke passed against the packaged installer;
- packaged first-run walkthrough passed;
- an artifact-bound local isolated-package or external cleanroom report passed;
- the installer SHA256 used in cleanroom matches the installer SHA256 from local installer smoke;
- the hosted release asset hash in `SHA256SUMS` matches that same cleanroom-tested installer SHA256.

The `hosted-exact-artifacts-v2` schema avoids claiming that a pre-tag hash
identifies a newly rebuilt installer, DMG, or Debian package. It requires:

- explicit maintainer publication approval and the Windows, Apple Silicon, and
  Linux release scope;
- a merged-code Apple Silicon local preflight;
- an exact hosted Windows receipt proving valid timestamped signatures for the
  installer, installed application, and uninstaller;
- an exact hosted macOS receipt proving ARM64 architecture, packaged resources,
  unsigned/unnotarized state, and isolated first launch;
- when Linux is advertised, an exact hosted Debian receipt proving the x86_64
  package, bundled resources, installed-package isolated launch, and database
  initialization with Ollama absent;
- receipt hashes for every exact hosted installer that match their entries in
  the published `SHA256SUMS`.

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

Linux public beta is deliberately narrow: a `.deb` for x86_64 Debian/Ubuntu
systems built on Ubuntu 22.04. It is not an AppImage, RPM, Flatpak, ARM Linux,
or a claim of support for every Linux distribution. Ollama is installed
separately by the user.
