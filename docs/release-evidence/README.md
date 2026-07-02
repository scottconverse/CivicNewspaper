# Hosted Release Evidence

This folder is intentionally empty until a release candidate is ready to tag.

Before pushing a `vX.Y.Z` tag, add `docs/release-evidence/vX.Y.Z.json` in the same commit that will be tagged. The hosted release workflow refuses to build or upload public beta artifacts unless that JSON file proves, for the exact tag and commit:

- local release smoke passed;
- default-model bakeoff passed;
- dependency audit passed;
- Windows installer smoke passed against the packaged installer;
- packaged first-run walkthrough passed;
- final cleanroom tester report passed;
- the installer SHA256 used in cleanroom matches the installer SHA256 from local installer smoke.
- the hosted release asset hash in `SHA256SUMS` matches that same cleanroom-tested installer SHA256.

This keeps GitHub Releases from going green on tag-only automation when the real local RC and cleanroom evidence has not been reviewed.
