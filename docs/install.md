# Installation Guide & Binary Verification

This guide explains how to install CivicNewspaper on Windows, macOS, and Linux, and how to verify the integrity of the downloaded files.

---

## 🛡️ The "Trust-Without-Signing" Philosophy

As an open-source, community-focused project, CivicNewspaper does not participate in the costly commercial developer programs run by Microsoft and Apple. Paying hundreds of dollars annually to corporate gatekeepers for signing certificates contradicts our decentralized, grassroots mission.

Instead of code-signing certificates, we establish trust through **cryptographic verification**. Every release on GitHub includes the pre-compiled installer files and a `SHA256SUMS.txt` manifest containing the SHA256 checksum of each binary. By verifying the checksum of your download, you can guarantee that the file has not been altered or corrupted in transit.

---

## 💻 Operating System Installation Paths

### 1. Windows Installation
Because the Windows installer is unsigned, Windows SmartScreen will flag the application on first launch.

1. Download the latest `.msi` or `.exe` installer from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Double-click the downloaded file to run it.
3. A blue warning box will appear: **"Windows protected your PC"** (SmartScreen).
4. Click the small **"More info"** link under the main warning text.

*A blue warning dialog with the title "Windows protected your PC" and a "More info" link.*

> ⚠️ Real installer warning screenshots will be added in v0.2.2 once captured on clean VMs.
> The AI-generated mockups shipped in v0.2.0 were withdrawn because they did not match
> the actual dialogs and could mislead users.

5. A button labeled **"Run anyway"** will appear in the bottom right. Click it to launch the installer.
6. Follow the standard installation prompts to complete the process.

### 2. macOS Installation
macOS Gatekeeper blocks unsigned apps by default, preventing execution if the developer cannot be verified.

1. Download the `.dmg` file from the [latest GitHub Releases](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Double-click the `.dmg` file and drag **CivicNewspaper.app** into your **Applications** folder.
3. **Important**: Do not double-click to launch it immediately. Doing so will trigger a hard block.
4. Instead, navigate to your **Applications** folder in Finder, right-click (or Control-click) the **CivicNewspaper** icon, and select **Open**.
5. A dialog box will appear stating macOS cannot verify the developer. Click the **"Open"** button to confirm.

*A warning dialog stating macOS cannot verify the developer, showing the app name and options to cancel or open.*

> ⚠️ Real installer warning screenshots will be added in v0.2.2 once captured on clean VMs.
> The AI-generated mockups shipped in v0.2.0 were withdrawn because they did not match
> the actual dialogs and could mislead users.

6. *Alternative workaround:* If the app fails to open, go to **System Settings > Privacy & Security**, scroll down to the **Security** section, locate the notification that CivicNewspaper was blocked, and click the **"Open Anyway"** button.

### 3. Linux Installation
We provide Debian/Ubuntu package archives (`.deb`) and portable `.AppImage` packages.

* **Debian/Ubuntu (`.deb`)**:
  1. Download the `.deb` package.
  2. Open a terminal and run:
     ```bash
     sudo dpkg -i civicnewspaper_*.deb
     sudo apt-get install -f # Install any missing dependencies
     ```
* **AppImage**:
  1. Download the `.AppImage` file.
  2. Make the file executable and run it:
     ```bash
     chmod +x CivicNewspaper-*.AppImage
     ./CivicNewspaper-*.AppImage
     ```

---

## 🔍 How to Verify the SHA256 Checksum

To verify that your downloaded binary is safe and matches the exact code compiled by the developers, compare its SHA256 hash.

### Step 1: Get the Release Hash
1. Open the [latest GitHub Releases page](https://github.com/scottconverse/CivicNewspaper/releases/latest).
2. Locate and copy the SHA256 hash listed next to your file, or open the uploaded `SHA256SUMS.txt` file in your browser to view the hashes.

### Step 2: Compute the Hash on Your Computer

Open a terminal or command prompt and run the command matching your operating system:

*(Note: Replace `<version>` with the actual version you downloaded, e.g., `0.2.2`. You can find the canonical checksums in the `SHA256SUMS` file on the GitHub Releases page.)*

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
Compare the hex string output by your terminal to the hash listed on the GitHub Release page. If they match exactly (case-insensitive), the file is safe to run and has not been modified.

> [!WARNING]
> If the computed hash does not match the hash published on the official GitHub Release page, do not run the installer. Delete the file immediately and report the discrepancy on our issue tracker.
