# CivicNews Safari Browser Extension

This directory contains the Safari Web Extension configuration and scripts for CivicNews. 

Safari Web Extensions utilize standard Web Extension APIs (similar to Chromium/MV3) but are packaged as a native macOS App extension inside an Xcode project for distribution and runtime registration in macOS.

## Setup & Compilation in Xcode

To convert and compile the browser extension for Safari:

1. **Convert the Extension**
   Open Terminal on macOS and run the standard Apple command-line utility to convert the Web Extension directory into an Xcode project:
   ```bash
   xcrun safari-web-extension-converter ./browser-extension/chromium --output-directory ./browser-extension/safari
   ```
   This will generate an Xcode project inside `browser-extension/safari/` wrapping the extension background and content scripts.

2. **Open the Xcode Project**
   Open the generated project in Xcode:
   ```bash
   open ./browser-extension/safari/CivicNews\ Browser\ Bridge/CivicNews\ Browser\ Bridge.xcodeproj
   ```

3. **Configure & Build**
   - In Xcode, select the **CivicNews Browser Bridge** target.
   - Choose a development signing certificate or configure Xcode for local unsigned builds.
   - Click the **Run** button (or press `Cmd + R`) to compile and launch the companion App wrapper.

---

## Enabling the Extension in Safari

Because Apple enforces signing requirements, you must enable developer options to test the extension locally without a developer license:

1. **Allow Unsigned Extensions**
   - Open **Safari** on macOS.
   - Go to **Safari > Settings... > Advanced**.
   - Check the box for **"Show features for web developers"** (or **"Show Develop menu in menu bar"** depending on macOS version).
   - In Safari's menu bar, click the new **Develop** menu and check **"Allow Unsigned Extensions"**.

2. **Enable the Extension**
   - Go to **Safari > Settings... > Extensions**.
   - Check the checkbox next to **CivicNews Browser Bridge**.
   - Select **"Always Allow on Every Website"** or grant access to the supported AI chat domains:
     - `claude.ai`
     - `chatgpt.com`
     - `chat.openai.com`
     - `gemini.google.com`

---

## Architecture details

- **Communication Namespace**: Standard `browser` or `chrome` API bindings are used.
- **Port Listener**: Queries `http://127.0.0.1:12053` using loopback CORS.
- **Token Storage**: Uses `browser.storage.local` to store the pairing token.
- **Origins CORS**: Handled on the local Axum API server by checking if the origin starts with `safari-extension://`.
