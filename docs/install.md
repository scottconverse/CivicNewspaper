# Installation Guide And Checksum Verification

This guide explains how to install The Civic Desk from the CivicNewspaper releases page and how to verify a downloaded installer.

The public beta installers are unsigned. Windows SmartScreen and macOS Gatekeeper may warn you before the app opens. That warning is expected for an unsigned beta and does not by itself mean the file is malicious. It also does not prove the file is safe. Use the checksum steps below to confirm the download matches the release artifact.

## Download

Open the releases page:

<https://github.com/scottconverse/CivicNewspaper/releases>

Use the newest release that actually includes an installer for your platform. The latest source tag and the latest published installer release may not always be the same during public beta.

## Windows

1. Download the `.msi` or `.exe` installer from the release assets if present.
2. Optional but recommended: verify the SHA256 checksum before opening the installer.
3. Double-click the installer.
4. If Windows SmartScreen says "Windows protected your PC," click **More info**.
5. Confirm the publisher and filename match the release you downloaded.
6. Click **Run anyway**.
7. Follow the installer prompts.
8. Launch **The Civic Desk** from the Start menu.

## macOS

1. Download the `.dmg` file from the release assets if present.
2. Optional but recommended: verify the SHA256 checksum before opening the image.
3. Open the `.dmg` and drag the app to Applications.
4. Because the app is unsigned during public beta, open it with right-click or Control-click, then choose **Open**.
5. If macOS blocks it, go to **System Settings > Privacy & Security** and choose **Open Anyway** for The Civic Desk.

## Linux

1. Download the `.deb` package if one is available for the release.
2. Optional but recommended: verify the SHA256 checksum.
3. Install with:

```bash
sudo dpkg -i the-civic-desk_*.deb
sudo apt-get install -f
```

The Linux package format is currently Debian/Ubuntu oriented. AppImage and Flatpak are not stable-release promises yet.

## Verify The SHA256 Checksum

A SHA256 checksum proves that the file you downloaded matches the file listed in the release manifest. It is not the same as code signing and does not prove who built the file.

1. Open the release page for the installer you downloaded.
2. Find the `SHA256SUMS` file or the checksum listed for your installer.
3. Compute the local hash.
4. Compare the two strings exactly.

### Windows PowerShell

```powershell
Get-FileHash -Algorithm SHA256 "C:\Users\YourName\Downloads\The Civic Desk_0.2.9_x64_en-US.msi"
```

### macOS

```bash
shasum -a 256 "$HOME/Downloads/The Civic Desk_0.2.9_x64.dmg"
```

### Linux

```bash
sha256sum "$HOME/Downloads/the-civic-desk_0.2.9_amd64.deb"
```

If the hash does not match, delete the installer and report it on the project issue tracker. Do not run a mismatched installer.

## First Launch

On first launch, the setup flow asks for:

- Publication name.
- Editor name.
- Organization type.
- City and state.
- Local AI/model setup.
- Backup and publishing folders.

The app may recommend and download a local model through Ollama. Model downloads can be large and slow. The app should show progress and allow an explicit skip/degraded path.

## Current Public-Beta Limits

- Installers are unsigned.
- Some external publishing providers require user-owned credentials.
- Scanned image-only PDFs require OCR support before URLs can be extracted.
- Clean-machine installer coverage is improving but not yet stable-release grade.
