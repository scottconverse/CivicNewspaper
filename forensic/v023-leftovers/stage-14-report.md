# Stage 14: Tag and Release Report

The v0.2.0 release has been successfully tagged and compiled across all three platforms (Windows, macOS, and Linux) via GitHub Actions.

## Tag and GitHub Actions Run Details
- **Release Tag**: `v0.2.0` (Commit: `3c89862`)
- **GitHub Actions Run**: `26475888796`
- **Build Outcomes**:
  - **macOS (`macos-latest`)**: Succeeded in 2m 54s. Uploaded `.dmg` and `.app.tar.gz`.
  - **Windows (`windows-latest`)**: Succeeded in 4m 1s. Uploaded `.exe` and `.msi`.
  - **Linux (`self-hosted, Linux, scott-desktop`)**: Succeeded in 7m 14s. Uploaded `.deb`, `.AppImage`, and `.rpm`.
- **Status**: Live and verified on the GitHub release page.

## Verified Release Assets & Checksums
All platform installers were downloaded and verified locally. The SHA256 checksums match expectations:

| Platform | Asset Name | Size (Bytes) | SHA256 Checksum |
| :--- | :--- | :--- | :--- |
| **Linux (RPM)** | `CivicNewspaper-0.2.0-1.x86_64.rpm` | 10,170,143 | `0D94C0A8E1F6BA34A0B3797E45E55EA88885857476DC058773872A15DF55CA38` |
| **Linux (AppImage)** | `CivicNewspaper_0.2.0_amd64.AppImage` | 85,953,016 | `685778E5CBC685143E5FC28F75FE304BA6CFE399797EEDB5F6A7080280A7D5AF` |
| **Linux (DEB)** | `CivicNewspaper_0.2.0_amd64.deb` | 10,170,736 | `E3DBD8F6BF39778E76107DDB3AE2EF153278D15635C1BD6916BFAA55AB6161D8` |
| **Windows (EXE)** | `CivicNewspaper_0.2.0_x64-setup.exe` | 12,539,938 | `1BAB21CA4879EA7C73C0598CD64B11D4DC265B22AE0DFBA53FB675D9B8A7E6F0` |
| **macOS (DMG)** | `CivicNewspaper_0.2.0_x64.dmg` | 32,053,338 | `571F8F61388984DCBC1089A4020FD74447710277B42405377621246121A80093` |
| **Windows (MSI)** | `CivicNewspaper_0.2.0_x64_en-US.msi` | 17,522,688 | `3FA177239FE80F6AA09821F602DC4B8201856F5CE35FB7A27CA97E19604AA8CA` |
| **macOS (Tarball)** | `CivicNewspaper_x64.app.tar.gz` | 31,988,429 | `AA9DAA55E8A744AE5B5ABD34B0081FE9BFE7C1B2F47FD0F9806421779D258E4E` |

## Release Notes Update
The release notes placeholder ("See the assets to download this version and install.") was replaced with detailed changelog details focusing on:
- **Ollama Sidecar Integration**: Native packaging of the Ollama server for one-click setup.
- **Onboarding Progress Wizard**: Interactive local model download of `gemma2:9b` (5.4 GB) with streaming progress events.
- **Phase 4 Hardening**: Fixing database schema migrations, Plain Language Rewrite confirmation dialogs, mobile design alignment, and modular LLM abstraction.
- **Audience-tailored documentation**: Operator, technical, and developer manuals including inline Mermaid diagrams.
