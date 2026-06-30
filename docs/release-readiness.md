# Release readiness

This checklist separates public-beta checks that can be run on a normal development machine from stable-release checks that require signing credentials, clean machines, or live provider accounts.

## Local release smoke

Run this before tagging a beta or release candidate:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-smoke.ps1 `
  -Model "qwen2.5:7b"
```

The script writes a receipt under `.agent-runs\release-smoke-*` and runs:

- frontend tests
- Rust tests
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
  -FixtureDir "C:\Users\instynct\Desktop\CivicNewspaperTestFiles" `
  -Model "qwen2.5:7b" `
  -Stable
```

The stable run fails if the working tree is dirty or if live model, here.now, or import fixture gates are skipped. Use `-AllowDirty` only for a non-release diagnostic run.

## Evidence levels

| Level | Required evidence | Allowed skips |
|---|---|---|
| Public beta | Frontend tests, Rust tests, static-site output gate, release notes, known limitations. | Live provider credentials, signing, true clean-machine proof. Skips must be explicit in the receipt. |
| Release candidate | Beta evidence plus source-import fixtures, live Colorado scan, model bakeoff, dependency audit, anonymous here.now publish. | External providers without credentials. |
| Stable | RC evidence plus no skipped release-smoke gates, clean first-run artifact, signed Windows installer, macOS notarization, clean-machine installer proof, and credentialed live connector verification for supported providers. | None for the release-critical gates. |

## Source import fixtures

The full local fixture suite expects realistic files in:

```text
C:\Users\instynct\Desktop\CivicNewspaperTestFiles
```

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

For the current Windows 32 GB machine, qwen2.5:7b remains the safer default scan model until a newer model repeatedly proves strict JSON reliability in the bakeoff and live scan gates.

## Security checks

Run:

```powershell
npm audit --audit-level=moderate
cd src-tauri
cargo audit
```

Warnings from transitive desktop framework dependencies should be recorded in the release notes if they cannot be upgraded safely before release.

## Stable-release blockers

These cannot be fully completed from one unsigned Windows development machine:

- Windows code-signing certificate and signed installer verification
- macOS signing and notarization
- clean-machine installer proof on Windows, macOS, and Linux
- permanent here.now API-key publish verification
- Cloudflare Pages, Netlify, WordPress, and GitHub Pages live connector verification with real target accounts

## Rollback

If a release is bad:

1. Mark the GitHub release as pre-release or remove it if it is actively harmful.
2. Publish a correction note in the release body.
3. Tag a patch release from the last known-good commit.
4. Keep the failed smoke receipt and model bakeoff result for diagnosis.
