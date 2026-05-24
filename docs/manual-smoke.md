# CivicNews v0.1.0-alpha Manual Smoke Test

Since CivicNews is a local-first desktop application with sensitive GUI workflows, it requires manual verification on a clean system to ensure packaging and onboarding flows work as expected.

## Prerequisites
- A clean Windows User Account or a fresh Windows Sandbox / VM.
- No existing `%APPDATA%\com.tauri.dev\` folder.
- No local Ollama running yet (or installed freshly).

## Test Steps

### 1. Installation
1. Build the release binary: `npm run tauri build` (or download the artifact if available).
2. Install the application on the clean system.
3. Launch CivicNewspaper.

### 2. Onboarding Flow
1. Verify the initial screen welcomes you and offers to start the setup wizard.
2. Complete the identity and city setup.
3. Arrive at the "Local AI / Ollama" step.
4. Verify the RAM inspection correctly identifies system memory and recommends an appropriate model.
5. If Ollama is not installed, follow the link to install it. Start Ollama.
6. Pull the recommended model in the background (e.g. `gemma2:9b` or `llama3:8b`).
7. Complete the wizard and arrive at the main Newsroom dashboard.

### 3. Pairing Flow (Browser Extension)
1. In Chrome/Edge/Brave, navigate to `chrome://extensions/`.
2. Enable Developer Mode.
3. Select "Load Unpacked" and point to the `browser-extension/chromium/` directory.
4. Open the CivicNews Desktop App and go to the "Browser Pairing" tab.
5. Note the 22-char base64 pairing token displayed.
6. Click the CivicNews extension icon in the browser toolbar.
7. Paste the 22-char token into the pairing input and click "Pair Client".
8. Verify the pairing succeeds and the extension is ready to scrape.

### 4. End-to-End Scraping & Drafting
1. Navigate to a known city council meeting minutes page or an RSS feed.
2. Use the browser extension to extract a document.
3. Verify the document appears in the CivicNews "Leads" queue.
4. Request a draft generation from the Lead.
5. Wait for the local Ollama instance to stream the response.
6. Verify the drafted article appears, with proper `evidence:` markdown citations.

### 5. Guardrails & Compilation
1. Edit the generated draft to include the word "corrupt" without a citation.
2. Verify the Factual Guardrail Inspector raises a visual warning warning about the accusatory language rule. (Note: the guardrails act as editor helpers in the UI and do not block compilation or status changes).
3. Approve for publish (status transitions to "Ready to Publish").
4. Run the "Static Compilation & Publishing Wizard".
5. Pick an output folder.
6. Verify `index.html`, the article page, `styles.css`, and `feed.xml` are created correctly.
7. Verify the article page does not execute any raw HTML (XSS test: insert `<script>alert(1)</script>` into the draft body before compile, verify it is escaped or stripped).

---
*Note: This smoke test replaces the automated clean-VM verification for v0.1.0-alpha, as AI assistants cannot natively drive the Tauri desktop GUI.*
