# CivicNews GitHub Discussions
## *Seed Topics & Templates for Community Launch*

This document provides ready-made discussion templates to kickstart the GitHub Discussions tab in the CivicNews repository. Copy and paste these to introduce the project, answer frequently asked questions, and align contributors on editorial and architectural principles.

---

## 📌 Category: General
### 🏷️ Topic 1: Welcome to CivicNews! 🏛️
* **Title**: Welcome to CivicNews: Reclaiming Factual Local News
* **Body**:
  Welcome to the CivicNews community! 

  CivicNews was born out of a simple realization: **local community news is disappearing, and the tools built for modern newsrooms are too complex, expensive, and cloud-dependent for a single community observer.**

  Our goal is to give a single editor (often a citizen observer with limited technical skills but a passion for local transparency) a complete, local-first desktop workspace that does all of the following:
  * Automatically scans city council minutes, agendas, and boards for critical OSINT signals (meetings, large expenditures, watchlists).
  * Uses private, local AI models (running fully on your own computer) to draft neutral summaries, translate dense municipal jargon into plain-language community news, and (optionally) generate copy-ready social blurbs you can paste into your own channels.
  * Runs advisory pre-publication guardrails that flag any claim missing a primary-record citation (warnings help you self-edit; they do not block publishing).
  * Compiles articles into a static, fast, flat HTML folder using a guided drag-and-drop wizard that can be hosted online for free.

  #### 🤝 How to Get Involved:
  1. **Test the builds**: Grab the latest installer for Mac or Windows and report any bugs or edge cases.
  2. **Improve detectors**: If your local city hall uses unique wording for meeting notices or resolutions, help us refine the regex filters in `detectors.rs`.
  3. **Share your workflow**: Tell us how you are hosting your compiled flat HTML pages (e.g., GitHub Pages, Netlify) and what local feeds you are monitoring.

  *Let's rebuild trust in community news through raw evidence.*

---

## 💡 Category: Q&A / FAQ
### 🏷️ Topic 2: Frequently Asked Questions about Local LLMs (Ollama)
* **Title**: FAQ: Why Local AI (Ollama) and What Hardware is Required?
* **Body**:
  CivicNews relies on **Ollama** to run language models locally on your computer rather than sending public records data to external servers. Here are answers to the most common questions:

  #### 1. Why local AI instead of ChatGPT or Claude APIs?
  * **Privacy**: Your database, leads, and drafts never leave your device.
  * **Cost**: Running local models is **free of software cost** — no API keys, paywalls, or monthly subscriptions. (You do supply your own hardware and electricity; inference runs on your machine.)
  * **Offline Support**: You can draft and review reports even without an active internet connection.

  #### 2. What are the system requirements?
  CivicNews automatically checks your system memory (RAM) during onboarding:
  * **16 GB RAM or more (Recommended)**: The app pulls `qwen3:14b` (≈9.3 GB download — our standard model). Qwen3 is a best-in-class local model in 2026 with notably reliable JSON/structured output, which the Daily Scan feature relies on.
  * **8 GB RAM or more**: The onboarding wizard will recommend pulling the smaller `qwen3:8b` (≈5.2 GB download).
  * **Below 8 GB RAM**: The app will guide you to the lightweight `qwen3:4b` (≈2.5 GB download) or suggest running in manual drafting mode.

  #### 3. How do I change my AI model after onboarding?
  Open the **Settings** tab in the CivicNews dashboard. You can enter any model name that you have pulled inside Ollama (e.g., `mistral`, `llama3.2`) and click Save.

---

## 🛠️ Category: Ideas & Development
### 🏷️ Topic 3: Deep Dive: How the OSINT Detector Logic Works
* **Title**: Technical Guide: Refine and Customize the OSINT Detectors
* **Body**:
  CivicNews parses scrapings through automated detectors inside `core/detectors.rs`. Here is how they operate:

  #### 🛡️ The 8 Core Detectors
  1. **Source Went Quiet**: Warns you if a monitored feed hasn't successfully updated in 7+ days (helps detect URL changes or posting recess).
  2. **New Primary Record**: Automatically flags any new agenda, resolution, or minutes file uploaded by a primary feed.
  3. **Money Threshold**: Scans for transaction values (e.g., `$350,000` or `$1,200,000.50`). If they exceed your configured threshold, a lead is generated.
  4. **Decision / Vote**: Detects parliamentary terms like *unanimously*, *resolved*, *passed*, *adopted*, *motion*, *rejected*.
  5. **Personnel Change**: Scans for staff keywords: *appoint*, *resign*, *hire*, *terminate*, *successor*, *vacancy*.
  6. **Public Meeting**: Finds date and room announcements: *public hearing*, *council chamber*, *meeting scheduled*, *town hall*.
  7. **Deadline**: Scans for bid proposals and comment timelines: *rfp*, *bid due*, *submit by*, *due date*.
  8. **Watchlist Hit**: Matches your custom watchlist terms (names, vendors, departments) case-insensitively using word boundaries.

  #### ✏️ How to Contribute:
  If you notice that your city council's meeting minutes don't trigger the **Decision / Vote** detector, let us know here! We can add their specific terminology to the regular expression:
  ```rust
  let re_vote = Regex::new(r"(?i)\b(unanimously|voted|approved|resolved|passed|adopted|motion)\b").unwrap();
  ```

---

## ✍️ Category: Editorial Guidelines
### 🏷️ Topic 4: Editorial Standards: Evidence vs. Outrage
* **Title**: Factual Guidelines: Writing for the Flat HTML Compiler
* **Body**:
  CivicNews runs advisory pre-publication guardrails to help you maintain strict neutrality (they flag issues for you to fix; they don't block publishing). As an editor, here is the style template we recommend:

  #### 🚫 Avoid "Outrage" Wording
  Do not use adjectives that assign motive or pass judgment.
  * **Bad**: *"The council corruptly voted to double the budget for their crony contractor."*
  * **Good**: *"The council approved a budget amendment increasing the road maintenance contract value by $150,000 (evidence:12)."*

  #### 🔗 Mandatory Citation Anchors
  Every paragraph containing factual claims must have a citation link using Markdown:
  `The town planning commission approved the development permit [Brighton Minutes, p. 4](evidence:104).`
  * When compiled, this links directly to the raw evidence block displayed at the bottom of the page, ensuring readers can verify the claims.

  #### ⚖️ Presumption of Innocence
  When writing about police records or audits, always include qualifier terms.
  * **Incorrect**: *"A town administrator embezzled funds from the sewer department."*
  * **Correct**: *"A town administrator was arrested for the alleged embezzlement of sewer department funds (evidence:54)."*
  * *Note: When arrest keywords are used without a presumption-of-innocence modifier like "alleged", the Workbench raises an advisory **Legal Naming** guardrail warning so you can fix the wording. Guardrails are advisory only — they flag issues in the Workbench but do **not** block saving, approval, or publication, and the static-site compiler never runs them.*
