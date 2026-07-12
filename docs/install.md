# Installation Guide And Checksum Verification

This guide explains how to install The Civic Desk from the CivicNewspaper releases page and how to verify a downloaded installer.

Use the official Windows installer from the release page. Verify the checksum published beside that exact release asset to confirm the download matches it.

## Download

Open the releases page:

<https://github.com/scottconverse/CivicNewspaper/releases>

Use the v0.3.2 Windows-only public-beta release when installing this version:

<https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2>

## Windows

1. Download `The.Civic.Desk_0.3.2_x64-setup.exe` from the release assets.
2. Optional but recommended: verify the SHA256 checksum before opening the installer.
3. Double-click the installer.
4. Confirm the filename and publisher match the official release asset you downloaded.
5. Verify the checksum published for that exact release asset before installing.
6. Follow the installer prompts.
7. Launch **The Civic Desk** from the Start menu.

## macOS And Linux

macOS and Linux installers are backlog/proof-needed for this release line. Do not assume a historical `.dmg`, `.deb`, package config, or build target is supported until the release notes include a clean-machine proof for that platform.

## Clean-Machine Proof

Every release candidate needs an artifact-bound installed-package report from a clean machine, VM, Windows Sandbox, external tester, or isolated app-data profile before it can be called proved at that evidence level. The repository's [local isolated-package report](release-evidence/v0.3.2-local-isolated-package-report.md) records an earlier candidate and is retained as historical evidence. It is not the checksum for the candidate currently under evaluation or for the existing GitHub download. The final beta candidate must publish a new exact-candidate report and checksum only after the merged commit is signed and the same installer bytes complete the cleanroom gate.

Required proof for a cleanroom-proven release:

1. Build the artifact from a named commit.
2. Install it on a clean machine or VM.
3. Verify first-run setup, local AI setup or the AI-skipped source-review path, source intake, draft generation, ZIP export, and here.now publishing.
4. Record the report path, report hash, installer hash, tester machine, and any signing, notarization, permission, or package-manager warnings honestly in the release notes.
5. Verify that hosted release evidence and published asset hashes match the cleanroom-tested installer.

The v0.3.2 hosted-release evidence file is [release-evidence/v0.3.2.json](release-evidence/v0.3.2.json).

macOS and Linux additionally require real platform artifacts and platform-specific clean-machine proof before public docs advertise them as supported installer paths.

## Verify The SHA256 Checksum

A SHA256 checksum proves that the file you downloaded matches the file listed in the release manifest. It is not the same as code signing and does not prove who built the file.

1. Open the release page for the installer you downloaded.
2. Find the `SHA256SUMS.txt` file or the checksum listed for your installer.
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

On the Windows public-beta path, the app may recommend and download a local model through its product-owned Ollama runtime. Model downloads can be large and slow. The app should show progress and allow an explicit AI-skipped source-review path. macOS and Linux runtime/installer automation remains backlog until platform clean-machine proof is recorded.

If setup or publishing gets stuck, see [troubleshooting.md](troubleshooting.md) for installer provenance, model download, local AI runtime, here.now preview, ZIP/static output, weak-story, and source-import guidance.

## Current Public-Beta Limits

- Authenticode signature verification is required before a release installer is published.
- Windows only is the tested public-beta installer path for this release line.
- macOS and Linux installer proof is backlog/proof-needed.
- Some external publishing providers require user-owned credentials.
- PDF source-list import is disabled in the public beta until hardened parsing is available; convert PDFs to TXT/CSV/DOCX/XLSX or paste URLs directly.
- Clean-machine installer coverage is improving but not yet stable-release grade.
