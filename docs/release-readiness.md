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
- installed-package first run with the local AI endpoint forced absent, accessible-name actions, persisted completion markers, and zero-source guidance
- installed-package live-model flow from fetched municipal sources through linked lead, generated/saved draft, and Workbench reload
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

For this Windows beta release-candidate line, cleanroom evidence may be produced locally in an isolated app-data profile, Windows Sandbox, or VM. An external tester is optional, not mandatory, when the local report binds the exact installer SHA256/size and commit, proves the dependency-absent first run, and proves the live-model source-to-Workbench path against the installed package.

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

The receipt records the exact branch, commit, app versions, Tauri bundle targets, artifact paths, SHA256 hashes, smoke-check results, model-bakeoff receipt, dependency-audit receipt, Windows installer-smoke receipt, packaged first-run/core-flow receipt, and checksum-manifest output. It is a local evidence artifact; it does not push, merge, tag, or publish. By default it fails if the smoke receipt is missing, from a different repo or commit, dirty/diagnostic, contains failed/skipped checks, no current-version installer artifacts are present, stale historical installer artifacts are present in the bundle folder, the installer-smoke SHA256 does not match the current release artifact SHA256, or the required model-bakeoff/dependency-audit/installer/package artifacts are missing. Use the diagnostic flags only for local investigation, not release evidence.

The model bakeoff receipt must show that the configured default model in `src\models.json` passed every bakeoff case. The dependency audit receipt must be clean, run npm audit at the documented `moderate` threshold or stricter, and include Rust advisory checking through `cargo-audit`; a receipt that only exists but contains failures is not release evidence. If `cargo-audit` uses ignored RustSec advisories, every ignored ID must have a current machine-readable waiver in `docs/security-advisory-waivers.json`, and the dependency and RC receipts must copy exactly matching waiver entries into release evidence. Missing, extra, duplicated, expired, or incomplete waivers fail RC evidence. The Windows installer smoke receipt must prove NSIS silent install, installed app start from the packaged installer, first-run screenshot capture, uninstaller presence, and silent uninstall, and its recorded installer SHA256 must match the current RC artifact SHA256. MSI lifecycle proof is backlog/proof-needed until MSI is reintroduced as a public beta artifact.

For this release line, RC packaging evidence is Windows public beta only. macOS and Linux installer proof is backlog/proof-needed until a real platform artifact and clean-machine first-run proof exist.

## Hosted release workflow

The GitHub release workflow is intentionally conservative during public beta:

- tag pushes build Windows artifacts into a **draft** prerelease;
- the hosted workflow fails unless `docs/release-evidence/<tag>.json` exists at the tagged commit and verifies local RC evidence, Windows installer smoke, isolated packaged first-run/core-flow cleanroom proof, and the matching installer SHA256 for that exact tag;
- the workflow attaches a checksum manifest and runs release-asset integrity checks;
- release-asset integrity recomputes every downloaded asset hash and requires the published Windows installer hash in the checksum manifest to match the cleanroom-tested installer hash from `docs/release-evidence/<tag>.json`;
- the workflow does not publish a non-draft public release by itself;
- Scott must review the local RC receipt, cleanroom report, and release notes before undrafting a release.

This prevents an unverified installer from appearing before Authenticode, checksum, and local release-gate evidence have been reviewed.

## Current v0.3.2 evidence

Windows release candidates now pass an Authenticode signing gate that validates the installer, installed application, and uninstaller, including timestamps. Candidate-specific cleanroom evidence is recorded separately and must match the exact commit and installer hash being evaluated. This is Windows beta-candidate proof, not cross-platform or credentialed-provider stable-release proof. The GitHub release asset has not been replaced in this work unit.

- Hosted evidence file: `docs/release-evidence/v0.3.2.json`
- RC receipt: `.agent-runs/release-candidate-20260709-182734/release-candidate-receipt.json`, SHA256 `12C80AB694F484BB1176CF1F37590E3BEDD853985D020997BF47BCAEDBBCBE75`
- Strict release smoke: `.agent-runs/release-smoke-20260709-181659/release-smoke-receipt.json`, SHA256 `213887F59B4907A46B770987C5468A75318F1286A67B0FA60FE8BF238AAF1F8B`
- Dependency audit: `.agent-runs/beta-rc-bfa37f8-dependency-audit/dependency-audit-20260709-182436.json`, SHA256 `F2DC84A23551F1638285758E20EBC5927CE4EC2BFEBE21721997657C7A18D30C`
- Windows installer smoke: `.agent-runs/beta-rc-bfa37f8-windows-installer-smoke/windows-installer-smoke-receipt.json`, SHA256 `B765AC71F86B8A3128B1F083CAE01762CECAB07C7EF0BF2BD99D31EBD59C7DFA`
- Packaged first-run/core-flow walkthrough: `.agent-runs/release-smoke-20260709-181659/packaged-walkthrough/packaged-first-run-walkthrough-receipt.json`, SHA256 `EE82560ED42AD4530C23A7A1D57E55397D345F95060D4819BF77B35B8F271C99`
- Local isolated-package report: `docs/release-evidence/v0.3.2-local-isolated-package-report.md`, SHA256 `DAAAB19F700E5C8D4BB7F68AFF2E79D3B2EEA4A0000B94A51497E7F526B4B9F4`
- Evidence ZIP: `.agent-runs/release-candidate-20260709-182734.zip`, SHA256 `0405CA6CFA7843C6E6B643AB53973F7BE846EE13D488F393AF506F6DE16540CD`
- Tested release-candidate installer SHA256: `636D87041396603456634E6B47AE1071E8726D8D05C0FC08768D5B9E92A71C83`
- Tested release-candidate installer size: `5274104` bytes

This does not publish, merge, or tag the release by itself. Scott must still approve the product push, tag, hosted GitHub Release, and GitHub Pages update.

## Evidence levels

| Level | Required evidence | Allowed skips |
|---|---|---|
| Public beta | Frontend tests, Rust tests, static-site output gate, Authenticode verification, release notes, known limitations, install guide, user manual, and troubleshooting guide. | Live provider credentials and true clean-machine proof. Skips must be explicit in the receipt. |
| Release candidate | Beta evidence plus enforced coverage floors, desktop smoke, current-version Windows installer artifacts, isolated packaged first-run/core-flow proof, source-import fixtures, live Colorado scan, model bakeoff, dependency audit, anonymous here.now publish. | External providers without credentials; external tester optional when local packaged proof is complete. |
| Stable | RC evidence plus no skipped release-smoke gates, clean first-run artifact, signed Windows installer, cross-platform installer proof for every advertised OS, and credentialed live connector verification for supported providers. | None for the release-critical gates. |

## Source import fixtures

The full local fixture suite expects realistic files in a machine-available fixture
folder. For repo-local release evidence, restore or copy the fixture artifact to:

```text
tests\fixtures\source-import
```

If the gate machine uses a different workspace path, pass that machine's equivalent
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

These cannot be fully completed from one Windows development machine:

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
