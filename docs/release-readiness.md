# Release readiness

This checklist separates public-beta checks that can be run on a normal development machine from stable-release checks that require signing credentials, clean machines, or live provider accounts.

## Local release smoke

Run this before tagging a beta or release candidate:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-smoke.ps1 `
  -Model "phi4-mini:latest"
```

The script writes a receipt under `.agent-runs\release-smoke-*` and runs:

- frontend tests
- Rust tests
- desktop first-run loopback smoke with isolated app data
- seeded static-site output generation
- anonymous here.now publish and live URL fetch
- live Colorado source scan
- real local Ollama Daily Scan with the selected model
- source import extraction across CSV, TXT, XLSX, DOCX, plus PDF-disabled guidance fixtures
- frontend bulk-import review parsing against the extracted fixture text

Use `-SkipLiveModel` only for local diagnostics. A release candidate or stable gate must record every skipped check as a skip and must not treat a partial receipt as complete release evidence.

By default, the smoke script uses the committed review fixtures under `test-fixtures\source-import-extracted`. Those prove the bulk-import review parser on realistic extracted text. For release-candidate or stable evidence, pass the full source-file fixture folder so the Rust extraction gate proves CSV, XLSX, TXT, DOCX extraction and public-beta PDF-disabled guidance.

For stable release evidence, run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-smoke.ps1 `
  -FixtureDir "tests\fixtures\source-import" `
  -Model "phi4-mini:latest" `
  -Stable
```

The stable run fails if the working tree is dirty or if desktop smoke, live model, here.now, or import fixture gates are skipped. Use `-AllowDirty` only for a non-release diagnostic run. The `-FixtureDir` value must point to a folder available on the machine running the gate. Use the committed `tests\fixtures\source-import` fixture folder for normal release checks, or pass any equivalent full source-file fixture folder that contains the expected CSV, TXT, XLSX, DOCX files and PDF-disabled guidance fixtures.

## Release-candidate packaging receipt

After tests and smoke checks pass, generate a local RC evidence receipt before publishing any release asset:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\prepare-rc-evidence.ps1 `
  -ReleaseSmokeReceipt ".agent-runs\release-smoke-YYYYMMDD-HHMMSS\release-smoke-receipt.json" `
  -ModelBakeoffReceipt ".agent-runs\model-bakeoff-YYYYMMDD-HHMMSS.json" `
  -DependencyAuditReceipt ".agent-runs\dependency-audit-YYYYMMDD-HHMMSS.json" `
  -InstallerSmokeReceipt ".agent-runs\windows-installer-smoke-YYYYMMDD-HHMMSS\windows-installer-smoke-receipt.json" `
  -PackagedWalkthroughReceipt ".agent-runs\packaged-first-run-walkthrough-YYYYMMDD-HHMMSS\packaged-first-run-walkthrough-receipt.json"
```

The receipt records the exact branch, commit, app versions, Tauri bundle targets, artifact paths, SHA256 hashes, smoke-check results, model-bakeoff receipt, dependency-audit receipt, Windows installer-smoke receipt, and `SHA256SUMS` output. It is a local evidence artifact; it does not push, merge, tag, or publish. By default it fails if the smoke receipt is missing, from a different repo or commit, dirty/diagnostic, contains failed/skipped checks, no current-version installer artifacts are present, stale historical installer artifacts are present in the bundle folder, the installer-smoke SHA256 does not match the current release artifact SHA256, or the model-bakeoff/dependency-audit/installer-smoke artifacts are missing. Use the diagnostic flags only for local investigation, not release evidence.

The model bakeoff receipt must show that the configured default model in `src\models.json` passed every bakeoff case. The dependency audit receipt must be clean, run npm audit at the documented `moderate` threshold or stricter, and include Rust advisory checking through `cargo-audit`; a receipt that only exists but contains failures is not release evidence. If `cargo-audit` uses ignored RustSec advisories, every ignored ID must have a current machine-readable waiver in `docs/security-advisory-waivers.json`, and the dependency and RC receipts must copy exactly matching waiver entries into release evidence. Missing, extra, duplicated, expired, or incomplete waivers fail RC evidence. The Windows installer smoke receipt must prove NSIS silent install, installed app start from the packaged installer, first-run screenshot capture, uninstaller presence, and silent uninstall, and its recorded installer SHA256 must match the current RC artifact SHA256. MSI lifecycle proof is backlog/proof-needed until MSI is reintroduced as a public beta artifact.

For this release line, RC packaging evidence is Windows public beta only. macOS and Linux installer proof is backlog/proof-needed until a real platform artifact and clean-machine first-run proof exist.

## Hosted release workflow

The GitHub release workflow is intentionally conservative during public beta:

- tag pushes build Windows artifacts into a **draft** prerelease;
- the hosted workflow fails unless `docs/release-evidence/<tag>.json` exists at the tagged commit and verifies local RC evidence, Windows installer smoke, packaged first-run proof, final cleanroom proof, and the matching installer SHA256 for that exact tag;
- the workflow attaches `SHA256SUMS` and runs release-asset integrity checks;
- release-asset integrity recomputes every downloaded asset hash and requires the published Windows installer hash in `SHA256SUMS` to match the cleanroom-tested installer hash from `docs/release-evidence/<tag>.json`;
- the workflow does not publish a non-draft public release by itself;
- Scott must review the local RC receipt, cleanroom report, and release notes before undrafting a release.

This prevents a public unsigned installer from appearing before checksum and local release-gate evidence have been reviewed.

## Current v0.3.2 evidence

The Windows public-beta v0.3.2 release line has cleanroom proof at commit `af4a12b0689dd8de64ce6af707b0c305a9cdaba0`. The current rebuilt release-candidate installer was built from commit `ba49af4d69d2c4d6d88bfd148490494f243cc9d7` after AI setup visibility, installed-app onboarding reachability, legacy malformed-draft quarantine, encoded calendar-rollup story-quality repairs, onboarding identity reconciliation before Daily Scan, unsupported Daily Scan lead downgrading, full state-name discovery normalization, weak scan lead draft gating, reader-facing brief format fallback, official-record brief promotion, source-quality cleanup, source-backed Daily Scan brief promotion, dependency advisory update, durable draft persistence, linked-evidence Brief fallback, publish-preflight editor override, honest offline-AI drafting copy, and release-body provenance checking. It is queued for final cleanroom recheck.

- Hosted evidence file: `docs/release-evidence/v0.3.2.json`
- RC receipt: pending current `ba49af4d69d2c4d6d88bfd148490494f243cc9d7` receipt
- Strict release smoke: pending current `ba49af4d69d2c4d6d88bfd148490494f243cc9d7` receipt
- Dependency audit: `.agent-runs/dependency-audit-20260709-093244.json`
- Windows installer smoke: pending current `ba49af4d69d2c4d6d88bfd148490494f243cc9d7` receipt
- Packaged first-run walkthrough: pending current `ba49af4d69d2c4d6d88bfd148490494f243cc9d7` receipt
- Final cleanroom report: pending final cleanroom rerun for commit `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`
- Cleanroom public URL: pending final cleanroom rerun
- Current rebuilt release-candidate installer SHA256: `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`
- Current rebuilt release-candidate installer size: `5250809` bytes

This does not publish, merge, or tag the release by itself. Scott must still approve the product push, tag, hosted GitHub Release, and GitHub Pages update.

## Evidence levels

| Level | Required evidence | Allowed skips |
|---|---|---|
| Public beta | Frontend tests, Rust tests, static-site output gate, release notes, known limitations, install guide, user manual, and troubleshooting guide. | Live provider credentials, signing, true clean-machine proof. Skips must be explicit in the receipt. |
| Release candidate | Beta evidence plus desktop smoke, current-version Windows installer artifacts, source-import fixtures, live Colorado scan, model bakeoff, dependency audit, anonymous here.now publish. | External providers without credentials. |
| Stable | RC evidence plus no skipped release-smoke gates, clean first-run artifact, signed Windows installer, cross-platform installer proof for every advertised OS, and credentialed live connector verification for supported providers. | None for the release-critical gates. |

## Source import fixtures

The full local fixture suite expects realistic files in a machine-available fixture
folder. For repo-local release evidence, restore or copy the fixture artifact to:

```text
tests\fixtures\source-import
```

If a tester machine uses a different workspace path, pass that machine's equivalent
full source-file fixture folder with `-FixtureDir`. Do not hard-code another user's
Windows profile path in release directives.

The committed lightweight review fixtures live under:

```text
test-fixtures\source-import-extracted
```

The full source-file set includes clean CSV, messy XLSX, human notes TXT, DOCX briefing, PDF examples, and XLSX edge cases. PDF examples should fail with public-beta PDF-disabled guidance until hardened PDF parsing is added.

## Model bakeoff

There are two model checks:

- The release-candidate gate is the configured-default check: the model named in `src\models.json` must pass every bakeoff case in the receipt used by `prepare-rc-evidence.ps1`.
- The comparison bakeoff is model-selection evidence. Run it before changing the default model, after meaningful prompt/model changes, or when deciding whether the public-beta default should change.

Run this to record comparison reliability and timing:

```powershell
$env:MODEL_BAKEOFF_MODELS="qwen2.5:7b,gpt-oss:20b,gemma4:e4b,phi4-mini:latest,llama3.2:3b"
$env:MODEL_BAKEOFF_TIMEOUT_MS="240000"
node scripts\model-bakeoff.mjs
```

The result is written under `.agent-runs\model-bakeoff-*.json`. A release candidate may use a narrower default-model receipt when the purpose is to prove the current configured default; do not describe that receipt as a full comparison bakeoff.

For this public-beta line, `phi4-mini:latest` is the default scan model because `.agent-runs\model-bakeoff-2026-06-30T20-26-21-425Z.json` recorded valid JSON output for both `civic-signals` and `empty-noise` cases. That comparison receipt included `qwen2.5:7b`, `gpt-oss:20b`, `gemma4:e4b`, `phi4-mini:latest`, and `llama3.2:3b` with `MODEL_BAKEOFF_TIMEOUT_MS=240000`; `gemma4:e4b` also passed both cases but was slower on the civic-signal case in that run. Keep using the comparison set for model-selection decisions until repeated evidence supports a different default.

## Security checks

Run:

```powershell
npm audit --audit-level=moderate
cd src-tauri
cargo audit
```

Warnings from transitive desktop framework dependencies should be recorded in the release notes if they cannot be upgraded safely before release.

Current Rust advisory exceptions live in `src-tauri\.cargo\audit.toml`; the matching rationale, owner, release-note text, and review date live in `docs\security-advisory-waivers.json`. The release gate should not accept a dependency audit receipt that suppresses advisories without matching current waiver metadata.

## Stable-release blockers

These cannot be fully completed from one unsigned Windows development machine:

- Windows code-signing certificate and signed installer verification
- Mac installer build, signing/notarization decision, and clean-machine proof
- Linux installer/package build and clean-machine or VM proof
- clean-machine installer proof on every OS advertised in public docs
- permanent here.now API-key publish verification
- Netlify, WordPress, GitHub Pages, and permanent here.now live connector verification with real target accounts
- Cloudflare Pages direct API connector work and proof, if it is reintroduced after the public beta assisted/manual workflow

## Rollback

If a release is bad:

1. Mark the GitHub release as pre-release or remove it if it is actively harmful.
2. Publish a correction note in the release body.
3. Tag a patch release from the last known-good commit.
4. Keep the failed smoke receipt and model bakeoff result for diagnosis.
