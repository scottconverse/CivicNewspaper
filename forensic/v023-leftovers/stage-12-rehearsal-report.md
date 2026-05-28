# Stage 12: Local Release Rehearsal Report

## Build Success
The Tauri application and Vite frontend were compiled successfully in release mode on the host Windows environment.

## Produced Artifacts
- **MSI Installer**:
  - Path: `src-tauri/target/release/bundle/msi/CivicNewspaper_0.2.0_x64_en-US.msi`
  - Size: 17,522,688 bytes (~17.5 MB)
  - SHA256: `7C1B23A48BDD08601CCBC9DBB744C85AFB04C4B700264CEC733F2B9E909A0EAD`
- **NSIS Setup Executable**:
  - Path: `src-tauri/target/release/bundle/nsis/CivicNewspaper_0.2.0_x64-setup.exe`
  - Size: 12,530,059 bytes (~12.5 MB)
  - SHA256: `59BA4A3A04AAFBBE6845D70922CBC766B3972C698EDDF16B0E4F3FB80F4887E9`

## Verification Checks
1. **Bundled Ollama Sidecar**: Verified that the sidecar binary for the target platform (`ollama-x86_64-pc-windows-msvc.exe`) is bundled. The final size (~12.5-17.5 MB) matches the expectation where only the host triple sidecar is bundled per platform installer.
2. **Coherent Versioning**: Both bundles are versioned at `0.2.0`.
3. **Dormant Auto-Updater**: Verified `plugins.updater.active = false` is active in `tauri.conf.json`.

## Manual Verification Checklist for Operator (Clean VM / Device)
1. Download `CivicNewspaper_0.2.0_x64-setup.exe` or `CivicNewspaper_0.2.0_x64_en-US.msi`.
2. Move it to a clean VM or test environment without Ollama pre-installed.
3. Install the app, bypass SmartScreen/Gatekeeper warnings by following `docs/install.md` instructions.
4. Launch the application:
   - Ensure the app opens directly.
   - Confirm the Ollama sidecar service starts up automatically in the background (check for `ollama` process in Task Manager / Activity Monitor).
5. Onboarding Wizard:
   - Ensure step "Download AI Model" starts pulling the `gemma2:9b` model (~5.4 GB).
   - Verify the progress bar updates and shows correct status.
   - Test "Skip" option: verify it prints warning copy and allows continuing.
   - If skipped, verify attempting a "Daily Scan" prompts deep-linking back to onboarding step 3.
6. Daily Scan & Rewrite:
   - Add a news source (e.g. RSS feed).
   - Perform a Daily Scan. Verify that results load.
   - Initiate a Plain Language Rewrite on a draft. Verify that `window.confirm` popup appears and that confirming updates the text.
7. Exit:
   - Close the app. Verify that the Ollama sidecar process terminates cleanly and no orphan process remains.
