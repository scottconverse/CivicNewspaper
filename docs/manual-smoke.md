# CivicNewspaper v0.2.8 Manual Smoke Test

This clean-profile smoke test verifies the desktop-only paths that browser component tests cannot prove: installer startup, bundled AI sidecar behavior, first-run onboarding, source setup, browser pairing, guardrails, publishing, and recovery from missing dependencies.

Save screenshots, logs, and notes beside the release receipt. A stable release should not claim first-run coverage without these artifacts.

## Prerequisites

- A clean Windows user profile, Windows Sandbox, or VM.
- No existing `%APPDATA%\com.scottconverse.civicdesk\` folder.
- No user-installed Ollama service running before launch. The bundled sidecar path should be tested first.
- Network available for model download, live source scan, and anonymous here.now preview publish.
- Test source files available at `C:\Users\instynct\Desktop\CivicNewspaperTestFiles`.

## 1. Installation

1. Build or download the installer artifact.
2. Install CivicNewspaper.
3. Launch the app and capture the initial screen.
4. Confirm the app writes its database/config under the clean profile, not a reused developer profile.

## 2. First-Run Onboarding

1. Complete identity and city setup.
2. Confirm the local AI service step reports whether the bundled sidecar is reachable.
3. If the service is unavailable, verify the UI offers Retry and diagnostics.
4. Confirm the model step recommends `qwen2.5:7b` for 8 GB RAM or more, or `llama3.2:3b` below 8 GB.
5. Try pressing Next before download. The app must require explicit skip confirmation.
6. Download the recommended model or explicitly skip and record the degraded-mode copy.
7. Finish onboarding and confirm the main workspace loads.

## 3. Source Setup

1. Open Sources.
2. Run Discover for a Colorado city.
3. Verify discovered sources show credibility/review labels and preserve official-source tier when imported.
4. Import the fixture files from `C:\Users\instynct\Desktop\CivicNewspaperTestFiles`:
   - CSV
   - TXT
   - XLSX
   - DOCX
   - text-backed PDF
   - scanned-style PDF
5. Confirm imported rows are reviewable, duplicates are visible, and image-only scanned PDFs produce OCR/readable-text guidance.

## 4. Daily Scan

1. With zero sources, confirm Daily Scan routes to Sources instead of running an empty scan.
2. With sources configured, run Daily Scan.
3. Confirm staged progress shows source fetching, deterministic checks, optional AI review, saving, and completion/failure.
4. Stop or hide the AI service and confirm deterministic/fallback copy is understandable.

## 5. Browser Pairing

1. Open `chrome://extensions/`.
2. Enable Developer Mode.
3. Load the unpacked extension from `browser-extension/chromium/`.
4. In CivicNewspaper, generate a pairing code.
5. Pair the extension.
6. Confirm the extension popup shows the paired state only, and the app's paired-device list updates.

## 6. Guardrails, Attestation, and Publishing

1. Create or generate a draft with cited evidence.
2. Add a guardrail-triggering phrase such as "corrupt" without adequate context.
3. Confirm the guardrail inspector shows the warning and any configured high-concern terms open the editor-note flow.
4. Confirm approval remains an editor decision and records human review.
5. Compile the issue.
6. Verify drafts with review warnings publish with visible editor-review notes instead of being silently skipped.
7. Open and inspect:
   - homepage
   - article page
   - RSS feed
   - corrections/about/ethics pages
   - ZIP
   - newsletter markdown
   - Substack markdown
   - Facebook/subreddit/Nextdoor/short-link share files
8. Run anonymous here.now publish and verify the live URL loads.
9. Test a connector configuration before using Publish with connector.

## 7. Evidence to Save

- App-data path proof.
- Screenshots of onboarding, source import review, Daily Scan progress, guardrail/attestation, publish receipt, and here.now live output.
- Release smoke receipt.
- Model bakeoff result.
- Any failure logs and recovery notes.
