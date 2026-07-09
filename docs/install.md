# Installation Guide And Checksum Verification

This guide explains how to install The Civic Desk from the CivicNewspaper releases page and how to verify a downloaded installer.

The public beta Windows installer is unsigned. Windows SmartScreen may warn you before the app opens. That warning is expected for an unsigned beta and does not by itself mean the file is malicious. It also does not prove the file is safe. Use the checksum steps below to confirm the download matches the release artifact.

## Download

Open the releases page:

<https://github.com/scottconverse/CivicNewspaper/releases>

Use the v0.3.2 Windows-only public-beta release when installing this version:

<https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2>

## Windows

1. Download `The.Civic.Desk_0.3.2_x64-setup.exe` from the release assets.
2. Optional but recommended: verify the SHA256 checksum before opening the installer.
3. Double-click the installer.
4. If Windows SmartScreen says "Windows protected your PC," click **More info**.
5. Confirm the filename matches the release asset you downloaded. Because the beta is unsigned, Windows may show an unknown publisher; use the checksum below to verify the artifact.
6. Click **Run anyway**.
7. Follow the installer prompts.
8. Launch **The Civic Desk** from the Start menu.

## macOS And Linux

macOS and Linux installers are backlog/proof-needed for this release line. Do not assume a historical `.dmg`, `.deb`, package config, or build target is supported until the release notes include a clean-machine proof for that platform.

## Clean-Machine Proof

Every release candidate needs a clean-machine or remote-tester report before it can be called cleanroom-proven. For v0.3.2, the Windows public-beta installer represented by commit `af4a12b0689dd8de64ce6af707b0c305a9cdaba0` passed final remote cleanroom testing with zero blocker, critical, or major findings. The current rebuilt release-candidate installer was built from commit `17766b7ccb0cc744522090e28997b764676ce1c5` after AI setup visibility, installed-app onboarding reachability, legacy malformed-draft quarantine, encoded calendar-rollup story-quality repairs, onboarding identity reconciliation before Daily Scan, unsupported Daily Scan lead downgrading, full state-name discovery normalization, weak scan lead draft gating, reader-facing brief format fallback, official-record brief promotion, source-quality cleanup, source-backed Daily Scan brief promotion, and dependency advisory update, durable draft persistence, and linked-evidence Brief fallback; its SHA256 is `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`, it passed stable release smoke, local installer smoke, packaged first-run walkthrough locally, full Rust/frontend verification, and it is queued for final cleanroom recheck.

Required proof for a cleanroom-proven release:

1. Build the artifact from a named commit.
2. Install it on a clean machine or VM.
3. Verify first-run setup, local AI setup or the skip/degraded path, source intake, draft generation, ZIP export, and here.now publishing.
4. Record the report path, report hash, installer hash, tester machine, and any signing, notarization, permission, or package-manager warnings honestly in the release notes.
5. Verify that hosted release evidence and published asset hashes match the cleanroom-tested installer.

The v0.3.2 hosted-release evidence file is [release-evidence/v0.3.2.json](release-evidence/v0.3.2.json).

macOS and Linux additionally require real platform artifacts and platform-specific clean-machine proof before public docs advertise them as supported installer paths.

## Verify The SHA256 Checksum

A SHA256 checksum proves that the file you downloaded matches the file listed in the release manifest. It is not the same as code signing and does not prove who built the file.

1. Open the release page for the installer you downloaded.
2. Find the `SHA256SUMS` file or the checksum listed for your installer.
3. Compute the local hash.
4. Compare the two strings exactly.

### Windows PowerShell

```powershell
Get-FileHash -Algorithm SHA256 "C:\Users\YourName\Downloads\The.Civic.Desk_0.3.2_x64-setup.exe"
```

If the hash does not match, delete the installer and report it on the project issue tracker: <https://github.com/scottconverse/CivicNewspaper/issues>. Do not run a mismatched installer.

## First Launch

On first launch, the setup flow asks for:

- Publication name.
- Editor name.
- Organization type.
- City and state.
- Local AI/model setup.
- Backup and publishing folders.

On the Windows public-beta path, the app may recommend and download a local model through its product-owned Ollama runtime. Model downloads can be large and slow. The app should show progress and allow an explicit skip/degraded path. macOS and Linux runtime/installer automation remains backlog until platform clean-machine proof is recorded.

If setup or publishing gets stuck, see [troubleshooting.md](troubleshooting.md) for SmartScreen, model download, local AI runtime, here.now preview, ZIP/static output, weak-story, and source-import guidance.

## Current Public-Beta Limits

- Installers are unsigned.
- Windows only is the tested public-beta installer path for this release line.
- macOS and Linux installer proof is backlog/proof-needed.
- Some external publishing providers require user-owned credentials.
- PDF source-list import is disabled in the public beta until hardened parsing is available; convert PDFs to TXT/CSV/DOCX/XLSX or paste URLs directly.
- Clean-machine installer coverage is improving but not yet stable-release grade.
