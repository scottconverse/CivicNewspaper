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
- source import extraction across CSV, TXT, XLSX, DOCX, and PDF fixtures
- frontend bulk-import review parsing against the extracted fixture text

Use `-SkipLiveModel` only for local diagnostics. A release candidate or stable gate must record every skipped check as a skip and must not treat a partial receipt as complete release evidence.

By default, the smoke script uses the committed review fixtures under `test-fixtures\source-import-extracted`. Those prove the bulk-import review parser on realistic extracted text. For release-candidate or stable evidence, pass the full source-file fixture folder so the Rust extraction gate also proves CSV, XLSX, TXT, DOCX, and PDF extraction.

For stable release evidence, run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-smoke.ps1 `
  -FixtureDir ".agent-runs\source-fixture-artifact" `
  -Model "phi4-mini:latest" `
  -Stable
```

The stable run fails if the working tree is dirty or if desktop smoke, live model, here.now, or import fixture gates are skipped. Use `-AllowDirty` only for a non-release diagnostic run. The `-FixtureDir` value must point to a folder available on the machine running the gate. Use `.agent-runs\source-fixture-artifact` when the fixture artifact has been restored into the repo workspace, or pass any equivalent full source-file fixture folder that contains the expected CSV, TXT, XLSX, DOCX, and PDF files.

## Release-candidate packaging receipt

After tests and smoke checks pass, generate a local RC evidence receipt before publishing any release asset:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\prepare-rc-evidence.ps1 `
  -ReleaseSmokeReceipt ".agent-runs\release-smoke-YYYYMMDD-HHMMSS\release-smoke-receipt.json" `
  -ModelBakeoffReceipt ".agent-runs\model-bakeoff-YYYYMMDD-HHMMSS.json" `
  -DependencyAuditReceipt ".agent-runs\dependency-audit-YYYYMMDD-HHMMSS.json" `
  -InstallerSmokeReceipt ".agent-runs\windows-installer-smoke-YYYYMMDD-HHMMSS\windows-installer-smoke-receipt.json"
```

The receipt records the exact branch, commit, app versions, Tauri bundle targets, artifact paths, SHA256 hashes, smoke-check results, model-bakeoff receipt, dependency-audit receipt, Windows installer-smoke receipt, and `SHA256SUMS` output. It is a local evidence artifact; it does not push, merge, tag, or publish. By default it fails if the smoke receipt is missing, from a different repo or commit, dirty/diagnostic, contains failed/skipped checks, no current-version installer artifacts are present, stale historical installer artifacts are present in the bundle folder, or the model-bakeoff/dependency-audit/installer-smoke artifacts are missing. Use the diagnostic flags only for local investigation, not release evidence.

The model bakeoff receipt must show that the configured default model in `src\models.json` passed every bakeoff case. The dependency audit receipt must be clean, run npm audit at the documented `moderate` threshold or stricter, and include Rust advisory checking through `cargo-audit`; a receipt that only exists but contains failures is not release evidence. If `cargo-audit` uses ignored RustSec advisories, every ignored ID must have a current machine-readable waiver in `docs/security-advisory-waivers.json`, and the dependency and RC receipts must copy those waiver entries into release evidence. Missing, expired, or incomplete waivers fail RC evidence. The Windows installer smoke receipt must prove NSIS silent install, installed app start from the packaged installer, first-run screenshot capture, uninstaller presence, and silent uninstall. MSI lifecycle proof is backlog/proof-needed until MSI is reintroduced as a public beta artifact.

For this release line, RC packaging evidence is Windows public beta only. macOS and Linux installer proof is backlog/proof-needed until a real platform artifact and clean-machine first-run proof exist.

## Hosted release workflow

The GitHub release workflow is intentionally conservative during public beta:

- tag pushes build Windows artifacts into a **draft** prerelease;
- the workflow attaches `SHA256SUMS` and runs release-asset integrity checks;
- the workflow does not publish a non-draft public release by itself;
- Scott must review the local RC receipt, cleanroom report, and release notes before undrafting a release.

This prevents a public unsigned installer from appearing before checksum and local release-gate evidence have been reviewed.

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
.agent-runs\source-fixture-artifact
```

If a tester machine uses a different workspace path, pass that machine's equivalent
full source-file fixture folder with `-FixtureDir`. Do not hard-code another user's
Windows profile path in release directives.

The committed lightweight review fixtures live under:

```text
test-fixtures\source-import-extracted
```

The full source-file set includes clean CSV, messy XLSX, human notes TXT, DOCX briefing, text-backed PDF, scanned-style PDF, and XLSX edge cases. The scanned-style PDF should fail with OCR/readable-text guidance until OCR support is added.

## Model bakeoff

Run this to record local JSON reliability and timing:

```powershell
$env:MODEL_BAKEOFF_MODELS="qwen2.5:7b,gpt-oss:20b,gemma4:e4b,phi4-mini:latest,llama3.2:3b"
$env:MODEL_BAKEOFF_TIMEOUT_MS="240000"
node scripts\model-bakeoff.mjs
```

The result is written under `.agent-runs\model-bakeoff-*.json`.

For this public-beta line, `phi4-mini:latest` is the default scan model because `.agent-runs\model-bakeoff-2026-06-30T20-26-21-425Z.json` recorded valid JSON output for both `civic-signals` and `empty-noise` cases. That receipt compared `qwen2.5:7b`, `gpt-oss:20b`, `gemma4:e4b`, `phi4-mini:latest`, and `llama3.2:3b` with `MODEL_BAKEOFF_TIMEOUT_MS=240000`; `gemma4:e4b` also passed both cases but was slower on the civic-signal case in that run. Keep all comparison models in the bakeoff until repeated release evidence supports a different default.

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
