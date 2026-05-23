# CivicNews User Manual
## *A Simple Guide for Local Community Editors*

Welcome to CivicNews! This application is designed to help you watch your local government, extract important public record signals, write objective news reports, and publish them to a webpage. 

This guide is written in plain English. You do not need to know any programming or computer code to use this application.

---

## 🗺️ How CivicNews Works

CivicNews follows a simple, repeating loop:

1. **Scrape Sources**: The app reads webpage links or RSS feeds from your local city hall, school board, or police department.
2. **Find Leads**: Automatic detectors look at the records and flag important items (e.g., expenditures over $250k, upcoming public meetings, or names on your watchlist).
3. **Draft Reports**: You use a local artificial intelligence assistant to write objective draft stories based strictly on the raw records.
4. **Verify & Link Evidence**: You review a checklist, ensure every fact links to an evidence file, and approve the story.
5. **Publish**: You compile the approved stories into a standard, flat webpage that can be hosted online for free.

---

## 🚀 Step 1: First-Time Setup

### 1. Install Ollama (Your AI Assistant)
CivicNews writes drafts using a local AI program. This means your data stays on your machine and you don't pay any subscriptions to big tech companies.
* Go to [https://ollama.com](https://ollama.com) on your computer.
* Click the download button for your operating system (Mac or Windows).
* Double-click the downloaded installer file and follow the onscreen setup prompts.
* Once installed, make sure Ollama is running (you will see a small llama icon in your taskbar near the clock on Windows, or at the top of your screen on Mac).

### 2. Launch CivicNews
* Open the CivicNews desktop app.
* On the first run, the **Onboarding Wizard** will check if Ollama is running. 
* If Ollama is running, it will automatically pull the **Gemma 2:9B** or **Llama 3** writing model. If your computer doesn't have much RAM (memory), the wizard will guide you to select a smaller, faster model so your computer doesn't slow down.
* Fill in your basic **Community Profile** details:
  * **Site Title**: e.g., *Brighton Town Council Observer*
  * **Site Subtitle**: e.g., *Raw Records & Factual Public Reports*
  * **About Text**: A short paragraph explaining who you are and why you write this page.
  * **Ethics Statement**: e.g., *No rumors, no opinion. Every claim is linked directly to a public record.*
  * **Money Threshold**: Enter the dollar amount you care about (e.g., $100,000). Any contract or purchase above this amount will automatically trigger a lead.

---

## 📰 Step 2: Managing Your Sources

To watch your government, you need to tell CivicNews which pages to monitor.
1. Click on the **Sources** tab in the sidebar.
2. Click **Add Source**. (You can also use the **Bulk Import** tool or the **Auto-Discovery Wizard** to add many sources at once).
3. Fill in the details:
   * **Source Name**: e.g., *Brighton Council Meeting Agendas*
   * **Source URL**: Paste the link to their agenda feed or main webpage.
   * **Source Type**: 
     * *Primary Record*: Official meeting minutes, agendas, budgets, and resolutions.
     * *Official Communication*: City press releases, email notices, or newsletters.
     * *Media Lead*: Local newspapers or blogs. (CivicNews will only store the headlines of these, never the full text, to respect copyright).
4. Click **Save**.

CivicNews will now monitor this page. Every time it updates, it will grab the text and scan it for news signals.

### Browser Extension (Optional but Recommended)
For easier capturing, you can install the CivicNews browser extension.
1. Go to the **Browser Pairing** tab.
2. Follow the 3-step wizard to open your browser's extensions page.
3. Click "Open Extension Folder" and simply drag-and-drop the folder into your browser to install. This lets you capture articles and primary records seamlessly as you browse the web.

---

## 🕵️ Step 3: Reviewing the News Queue

Click the **Queue** tab in the sidebar. Here is your daily inbox:

### 1. Generated Leads (Inbox)
Whenever CivicNews finds something interesting, it generates a **Lead** with a confirmation checklist.
* **Example Lead**: *Money Threshold: Found a contract for $350,000 in Brighton Council Agenda.*
* Click on the lead. You will see a list of linked **Evidence Items** (the exact sentences pulled from the city document) and a **Confirmation Checklist** (e.g., *"Verify the contractor name," "Confirm the source page URL"*).
* If you want to write a story about this, click **Draft Story**.

### 2. The Story Workbench
Here is where you write and verify your articles:
* **Format Selector**: Select if you are writing a *Brief* (short report), *Watch* (longer explanation), or *Investigation*.
* **AI Draft Generation**: Click **Generate Draft**. The local AI will read the raw council minutes and write a strictly factual report.
* **Citations**: You will notice the AI adds links like `[Record](evidence:12)`. When readers click this on your published site, it will scroll them directly to the exact quote inside the official document.
* **Verification Check**: Before you can publish, CivicNews runs an automated guardrail test. It checks:
  * Do all paragraphs have a citation?
  * Did you use accusatory words (like *corrupt*, *stole*, *lied*) without a direct citation?
  * Did you include presumption-of-innocence words (like *alleged* or *charged*) when talking about arrests?
* **Social Media Promo Pack**: Underneath the editor, you can click "Generate Posts" to automatically create optimized Twitter/X, Facebook, and Reddit posts based on your verified story.
* If the guardrail check passes, click **Ready to Review**, then **Approve for Static Publish**.

---

## 🌐 Step 4: Publishing to the Web

Publishing creates a flat HTML website (a folder of files like `index.html`, `styles.css`, and a `feed.xml` RSS file) directly on your computer.

1. Click on the **Settings** tab and locate the **Static Compilation & Publishing Wizard**.
2. **Step 1:** Enter the output folder path on your computer where you want to output the website (e.g., `C:\Users\Name\Desktop\CivicNewsSite`).
3. **Step 2:** Click **Compile Static Site**.
4. **Step 3:** The wizard provides quick links to drag-and-drop hosting services (like Netlify Drop). Click **Open Folder in Explorer** to pop open your compiled site folder, then simply drag and drop it into the hosting webpage.

Your site is now live! Because the files are "flat" (plain HTML pages), they load instantly, never crash, and don't cost anything to maintain.

---

## 💾 Step 5: Backups and Recovery

Since all your data is stored locally on your hard drive, it is important to keep backups.
* **To Backup**: Click the **Settings** tab. Click **Save Backup** and choose a location (like a USB thumb drive or your Dropbox folder) to save a backup of your CivicNews database.
* **To Restore**: If you get a new computer or need to rollback to an older state, go to the **Settings** tab, click **Restore Backup**, and select your saved backup file. CivicNews will check the file integrity first to ensure it's not corrupt, and swap it safely without losing any active links.
