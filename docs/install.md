# Installation Guide & Binary Verification

This guide explains how to install CivicNewspaper on Windows, macOS, and Linux, and how to verify the integrity of the downloaded files.

---

## 🛡️ Unsigned public beta, and what checksums do (and don't) prove

CivicNewspaper is a public beta, and its installers are **not code-signed**. As an open-source, community-led project, it does not currently participate in the paid Microsoft and Apple developer-signing programs. Because the installers are unsigned, Windows SmartScreen and macOS Gatekeeper will warn you on first launch — this is expected, and the steps to proceed are in the OS sections below.

Every release on GitHub includes the installer files and a `SHA256SUMS` manifest listing the SHA256 checksum of each binary. Verifying that checksum confirms your download **matches the file published on the release page** — i.e. that it was not corrupted or altered in transit. Be clear about the limit of this guarantee: a checksum file fetched from the *same* release page as the binary is **not** a substitute for code signing. It does not prove who built the binary or that the publisher's account or build pipeline was not compromised. If you need a build you can fully trust end-to-end, **building from source is the only tamper-proof path** (see the README's "Building from source" section). This matches the "No code signing" note in [SECURITY.md](../SECURITY.md).

---

## 💻 Operating System Installation Paths

### 1. Windows Installation
Because the Windows installer is unsigned, Windows SmartScreen will flag the application on first launch.

1. Download the latest `.msi` or `.exe` installer from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Double-click the downloaded file to run it.
3. A blue warning box will appear: **"Windows protected your PC"** (SmartScreen).
4. Click the small **"More info"** link under the main warning text.

This screenshot placeholder describes the Windows SmartScreen warning window, where users click "More info" to reveal the "Run anyway" button.

> ⚠️ Real installer screenshots are pending and will be added in v0.3 once captured on clean VMs.
> The AI-generated mockups shipped in v0.2.0 were withdrawn because they did not match
> the actual dialogs and could mislead users.

5. A button labeled **"Run anyway"** will appear in the bottom right. Click it to launch the installer.
6. Follow the standard installation prompts to complete the process.

### 2. macOS Installation
macOS Gatekeeper blocks unsigned apps by default, preventing execution if the developer cannot be verified.

1. Download the `.dmg` file from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Double-click the `.dmg` file and drag **CivicNewspaper.app** into your **Applications** folder.
3. **Important**: Do not double-click to launch it immediately. Doing so will trigger a Gatekeeper warning prompt (which can be bypassed by opening via right-click or System Settings).
4. Instead, navigate to your **Applications** folder in Finder, right-click (or Control-click) the **CivicNewspaper** icon, and select **Open**.
5. A dialog box will appear stating macOS cannot verify the developer. Click the **"Open"** button to confirm.

This screenshot placeholder describes the macOS Gatekeeper warning popup, which prompts the user to open or cancel the unsigned application.

> ⚠️ Real installer screenshots are pending and will be added in v0.3 once captured on clean VMs.
> The AI-generated mockups shipped in v0.2.0 were withdrawn because they did not match
> the actual dialogs and could mislead users.

6. *Alternative workaround:* On macOS 14+ (Sonoma) and later, if the app fails to open or is blocked, go to **System Settings > Privacy & Security**, scroll down to the **Security** section, locate the notification that CivicNewspaper was blocked, and click the **"Open Anyway"** button.

### 3. Linux Installation
We provide Debian/Ubuntu package archives (`.deb`). Linux builds are deb-only — there is no AppImage build.

* **Debian/Ubuntu (`.deb`)**:
  1. Download the `.deb` package.
  2. Open a terminal and run:
     ```bash
     sudo dpkg -i civicnewspaper_*.deb
     sudo apt-get install -f # Install any missing dependencies
     ```

---

## 🔍 How to Verify the SHA256 Checksum

To confirm your download matches the file published on the release page (i.e. it was not corrupted or altered in transit), compare its SHA256 hash against the published manifest.

### Step 1: Get the Release Hash
1. Open the [latest GitHub Releases page](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Locate and copy the SHA256 hash listed next to your file, or open the uploaded `SHA256SUMS` file in your browser to view the hashes.

### Step 2: Compute the Hash on Your Computer

Open a terminal or command prompt and run the command matching your operating system:

*(Note: Replace `<version>` with the version number in the filename you downloaded — it matches the release tag. You can find the canonical checksums in the `SHA256SUMS` file on the GitHub Releases page.)*

* **Windows (PowerShell)**:
  ```powershell
  Get-FileHash -Algorithm SHA256 C:\Users\YourUsername\Downloads\CivicNewspaper_<version>_x64_en-US.msi
  ```
* **macOS (Terminal)**:
  ```bash
  shasum -a 256 ~/Downloads/CivicNewspaper_<version>_x64.dmg
  ```
* **Linux (Terminal)**:
  ```bash
  sha256sum ~/Downloads/civicnewspaper_<version>_amd64.deb
  ```

### Step 3: Compare the Hashes
Compare the hex string output by your terminal to the hash listed on the GitHub Release page. If they match exactly (case-insensitive), your download is identical to the file published on the release page and was not altered in transit. (As noted above, this confirms transit integrity, not publisher authenticity — it is not the same assurance code signing provides.)

> [!WARNING]
> If the computed hash does not match the hash published on the official GitHub Release page, do not run the installer. Delete the file immediately and report the discrepancy on our issue tracker.
