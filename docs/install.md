# Installation Guide And Checksum Verification

This guide explains how to install The Civic Desk from the CivicNewspaper releases page and how to verify a downloaded installer.

The v0.3.3 release page serves the signed Windows x64 installer and the unsigned, unnotarized Apple Silicon macOS DMG. Both are public-beta artifacts built from commit `e94a2f94885c1e6013129c3d854662cc3c8e5b27`.

## Download

Open the releases page:

<https://github.com/scottconverse/CivicNewspaper/releases>

Use the v0.3.3 public-beta release:

<https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.3>

## Windows

Initial installation requires an internet connection when Microsoft Edge WebView2 Runtime is not already installed. Keep the computer online for the installer and for first-run local AI setup, which downloads the app-managed Ollama runtime and selected model. This public-beta installer is not an offline installer.

1. Confirm the release page identifies product build commit `e94a2f94885c1e6013129c3d854662cc3c8e5b27`.
2. Download `The.Civic.Desk_0.3.3_x64-setup.exe` and `SHA256SUMS` from that release.
3. Verify the SHA256 checksum.
4. Verify Authenticode reports `Status` as `Valid` and the signer contains **Scott Converse**.
5. Only after both checks pass, double-click the installer.
6. Follow the installer prompts.
7. Launch **The Civic Desk** from the Start menu.

## Apple Silicon macOS — v0.3.3

The v0.3.3 macOS build supports Apple Silicon only. Intel Macs and Rosetta are not supported. The DMG is intentionally not Developer ID signed and not notarized.

1. Download `The.Civic.Desk_0.3.3_aarch64.dmg` and `SHA256SUMS` from the official v0.3.3 release page.
2. In Terminal, run:

   ```bash
   shasum -a 256 ~/Downloads/The.Civic.Desk_0.3.3_aarch64.dmg
   ```

3. Compare the result exactly with the DMG line in `SHA256SUMS`. Delete the DMG if it does not match.
4. Open the DMG and drag **The Civic Desk** to Applications.
5. In Applications, Control-click or right-click **The Civic Desk**, choose **Open**, and confirm **Open**. macOS may still block or warn about this unsigned build.
6. Install the official Ollama app from <https://ollama.com/download/mac>, open it, and then choose **Check again** in The Civic Desk.

The published receipt proves direct isolated launch of the exact hosted DMG; it does not prove the quarantined Finder or Control-click prompt. If macOS still reports that the checksum-verified app is quarantined, the narrow fallback is:

```bash
xattr -d com.apple.quarantine "/Applications/The Civic Desk.app"
```

Run that command only after the DMG checksum matches the official release manifest. Do not disable Gatekeeper globally.

## Linux

Linux is deferred to v0.3.4. No `.deb`, AppImage, or other Linux package is supported or published as part of v0.3.3.

## Clean-Machine Proof

Every release needs artifact-bound installed-package evidence. v0.3.3 is bound to commit `e94a2f94885c1e6013129c3d854662cc3c8e5b27`. Its Windows installer SHA256 is `3d08ec394d87329043acd57f8f714cdcfdf10b3670631861ba16bc397c6befd2` (size `5343200` bytes), and its Apple Silicon DMG SHA256 is `95c82afa6549a5648e919306b3fbc6b7f7336ee331ca7f3c7091d87d3d11f01b` (size `7891351` bytes). The repository's [v0.3.2 local isolated-package report](release-evidence/v0.3.2-local-isolated-package-report.md) remains historical evidence only.

Required proof for a cleanroom-proven release:

1. Build the artifact from a named commit.
2. Install it on a clean machine or VM.
3. Verify first-run setup, local AI setup or the AI-skipped source-review path, source intake, draft generation, ZIP export, and here.now publishing.
4. Record the report path, report hash, installer hash, tester machine, and any signing, notarization, permission, or package-manager warnings honestly in the release notes.
5. Verify that hosted release evidence and published asset hashes match the cleanroom-tested installer.

The current hosted-release evidence file is [release-evidence/v0.3.3.json](release-evidence/v0.3.3.json). The [v0.3.2 evidence](release-evidence/v0.3.2.json) is historical.

Every advertised platform requires a real artifact and platform-specific clean-machine proof. For v0.3.3, the fresh hosted Mac runner smoke-tests the exact DMG and publishes a receipt binding its hash, ARM64 architecture, resources, and isolated launch. Local preflight separately records the unsigned, unnotarized Gatekeeper and manual-Ollama experience.

## Verify The SHA256 Checksum

A SHA256 checksum proves that the file you downloaded matches the file listed in the release manifest. It is not the same as code signing and does not prove who built the file.

1. Open the release page for the installer you downloaded.
2. Find the `SHA256SUMS` file or the checksum listed for your installer.
3. Compute the local hash.
4. Compare the two strings exactly.

### Windows PowerShell

```powershell
Get-FileHash -Algorithm SHA256 "C:\Users\YourName\Downloads\The.Civic.Desk_0.3.3_x64-setup.exe"
```

Verify the Authenticode signature separately:

```powershell
Get-AuthenticodeSignature "C:\Users\YourName\Downloads\The.Civic.Desk_0.3.3_x64-setup.exe" |
  Select-Object Status, @{Name='Signer'; Expression={$_.SignerCertificate.Subject}}
```

Do not install unless `Status` is `Valid` and `Signer` contains `Scott Converse`.

If the hash does not match, delete the installer and report it on the project issue tracker: <https://github.com/scottconverse/CivicNewspaper/issues>. Do not run a mismatched installer.

## First Launch

On first launch, the setup flow asks for:

- Publication name.
- Editor name.
- Organization type.
- City and state.
- Local AI/model setup.
- Backup and publishing folders.

On Windows, the app may recommend and download a local model through its product-owned Ollama runtime. On macOS, install and start the official Ollama app separately; The Civic Desk detects that loopback service and downloads the selected model through it. Model downloads can be large and slow. Both platforms allow an explicit AI-skipped source-review path.

If setup or publishing gets stuck, see [troubleshooting.md](troubleshooting.md) for installer provenance, model download, local AI runtime, here.now preview, ZIP/static output, weak-story, and source-import guidance.

## Current Public-Beta Limits

- Authenticode signature verification is required before a release installer is published.
- Windows x64 v0.3.3 is signed and published.
- Apple Silicon macOS 11+ v0.3.3 is published; it is unsigned and unnotarized.
- Intel macOS is unsupported.
- Linux is deferred to v0.3.4.
- Some external publishing providers require user-owned credentials.
- PDF source-list import is disabled in the public beta until hardened parsing is available; convert PDFs to TXT/CSV/DOCX/XLSX or paste URLs directly.
- Clean-machine installer coverage is improving but not yet stable-release grade.
