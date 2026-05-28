# Stage 11 Landing Page Refresh Report

The CivicNewspaper landing page has been refreshed to reflect the new features, architecture, and download links for the v0.2.0 release.

## Changes Made

### 1. HTML updates ([docs/index.html](docs/index.html))
- **Hero Section**:
  - Updated the hero title to exactly: `"The newsroom for the community observer"`.
  - Added an showcase image `<img>` tag pointing to `assets/hero.png` showcasing the dashboard.
  - Replaced the generic download button with a premium grid container containing download options for **Windows (MSI)**, **macOS (DMG)**, and **Linux (DEB)**.
  - Added a `"First time installing?"` helper link under each platform's download button pointing to `install.md`.
- **v0.2.0 Features**:
  - Replaced the old list of features with the v0.2.0 capabilities:
    - **Daily Scan** (Aggregates municipality signals)
    - **Plain Language Summary & Rewrite** (Offline LLM support)
    - **Source Tier Enforcement** (Classify feeds by reliability)
    - **Bundled Ollama Sidecar** (Zero separate setup required)
- **Technical Architecture**:
  - Replaced the static, custom CSS layout with a standard, CDN-loaded **Mermaid.js** diagram showing the pairing and sidecar details:
    ```mermaid
    graph TD
        A[Browser Extension / UI] <-->|Pairing PIN / API| B[Tauri HTTP Loopback Server Axum]
        B <--> C[Tauri Rust Core]
        C <--> D[SQLite DB WAL Mode]
        C <--> E[Ollama Sidecar Gemma 2]
        C --> F[Flat HTML Compiler]
    ```
- **Navigation & Links**:
  - Updated both header and footer links to point to the local `./user_manual.md` instead of the old GitHub main branch URL.

### 2. JavaScript logic ([docs/script.js](docs/script.js))
- **Mermaid Initialization**:
  - Added initialization code to dynamically setup Mermaid with a dark theme on load: `mermaid.initialize({ startOnLoad: true, theme: 'dark' });`
- **Platform Detection**:
  - Added client-side platform detection checking both `navigator.userAgent` and `navigator.platform` on window load to identify if the visitor is running **Windows**, **macOS**, or **Linux**.
  - Dynamically appends the `.highlighted` class to the appropriate card/button container matching the detected platform.

### 3. Styling adjustments ([docs/style.css](docs/style.css))
- **Hero Image**:
  - Placed the screenshot in a relative container with a soft, blurred background glow (using a custom pseudo-element) and a modern box-shadow.
  - Added a responsive, smooth scale/translate transition on hover to feel premium and reactive.
- **Download Cards & Buttons**:
  - Designed responsive cards featuring a modern, dark-glassmorphism background, subtle borders, and flex layout.
  - Structured a clean state for the `.highlighted` class that expands the card scale, swaps the default dark button to a high-contrast gradient primary theme button, and adds a floating `"Recommended Platform"` label on top.
  - Standardized the `"First time installing?"` callouts with clean margins, a help-circle icon, and subtle hover transition effects.
- **Mermaid Container**:
  - Packaged the diagram in a responsive glass container with overflow horizontal scroll support for smaller displays.
  - Custom-styled the Mermaid node classes with specific stroke, fill, and drop-shadow overrides to blend in seamlessly with the dark Outrun aesthetic of the site.
