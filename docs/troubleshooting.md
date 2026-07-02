# Troubleshooting The Civic Desk

This page is for public-beta users who are trying to install, set up, scan, draft, or publish and something feels stuck or confusing.

The Civic Desk is local-first desktop software. That means several things happen on your machine: a local database is created, a local AI runtime may be installed, a model may be downloaded, and the app may compile files into a static website before publishing. Some steps are slow the first time.

## Windows Says The App Is Unknown

The public-beta Windows installer is unsigned. Windows SmartScreen may show "Windows protected your PC" or "Unknown publisher."

What to do:

1. Confirm you downloaded the installer from the official GitHub Releases page.
2. Confirm the filename matches the release asset.
3. Compare the SHA256 checksum using the install guide.
4. If the checksum matches and you are comfortable continuing, click **More info** and then **Run anyway**.

What not to assume:

- The warning does not automatically mean the file is malicious.
- A matching checksum does not replace code signing.
- If the checksum does not match, do not run the installer.

## Model Download Looks Stuck

Local AI models are large. The first download can take a long time, especially on slower connections.

What to check:

1. Stay on the AI Model screen and look for progress text.
2. Confirm the computer is online.
3. Leave the app open while the model downloads.
4. If progress has not changed for a long time, cancel and retry from the AI Model screen.
5. If the recommended model is too slow for the machine, choose a smaller model if the app offers one.

You can still use source management, manual drafting, editing, backup, export, and publishing without the local model. AI-assisted drafting, ranking, summarizing, and advisor tools will be limited until a model is ready.

## Local AI Runtime Is Offline

The app may show **Local AI offline**, **Choose model**, or **No model selected**.

What those mean:

- **Local AI offline:** the runtime is not reachable.
- **Choose model:** the runtime may be available, but no model is selected for newsroom work.
- **No model selected:** setup is incomplete.

What to do:

1. Open **AI Model**.
2. Let the app check the machine.
3. Select or download the recommended model.
4. Return to **System & Status** and confirm the state changes to ready.

If the app still says offline, export diagnostics from **System & Status** and include that file when reporting the bug.

## Daily Scan Finds Nothing

This can be normal. It can also mean the source list is too thin.

Check:

1. Open **Sources** and confirm at least a few useful sources are enabled.
2. Include official records, public notices, local news, and public community/social sources where appropriate.
3. Open a few source URLs manually to confirm they still load.
4. Run Daily Scan again.

If the app says there are no useful leads, treat that as an assignment-desk signal, not a failure. Some days there is not enough new material. If this happens repeatedly, add more sources or use discovery.

## The App Suggests Weak Or Generic Stories

The app can surface weak leads. It should warn you, but it should not make your editorial decision. If a static package-integrity blocker appears, fix the cited evidence, copy, or unsupported citation before export.

Before drafting or approving, ask:

- Why now?
- Is there a new decision, deadline, vote, cost, risk, conflict, opportunity, or public impact?
- Is this just an evergreen city information page?
- Is there enough evidence?
- Should this be a brief, watch item, verification task, or cut item instead?

Use Workbench actions:

- **Improve for Publication** when the draft has useful facts but reads like notes.
- **Make this a brief** when the item is real but small.
- **Send Back** when it needs fresh reporting, a second source, or clearer evidence.
- **Hold** when it may matter later.
- **Cut** when it should not be in the issue.

## here.now Preview Did Not Publish

Anonymous here.now previews are the recommended fast test path, but they still require internet access and a successful static-site compile.

Check:

1. Open **Publishing**.
2. Compile the issue first.
3. Confirm there is at least one approved story or brief.
4. Export the ZIP so you have a local copy.
5. Publish to here.now.
6. Copy the resulting URL immediately because anonymous previews are temporary.

If publishing fails:

- Try exporting the ZIP; if the ZIP works, the issue compiled and the problem is likely publishing/network related.
- Try again later if here.now or your network appears unavailable.
- Export diagnostics and include the publish error message in the bug report.

## ZIP Or Static Output Looks Wrong

The ZIP should represent the same issue the app tried to publish.

Check:

1. Open the exported folder or ZIP.
2. Open `index.html`.
3. Check article pages, RSS, newsletter markdown, and share copy.
4. Look for reporter notes, placeholder text, broken links, or missing headlines.

If public output contains editor-only notes such as "Reporting Steps", "EDITOR_NOTE", "Source needed", or "Verification needed", report it as a release-blocking bug.

The app may refuse to compile if an approved item is not actually reader-facing copy. Common causes:

- The article body is only an editor note, tester note, or approval note.
- A draft from a scanned lead has no linked source evidence.
- Linked source material exists, but the article body has no inline evidence citation.
- A paragraph cites evidence that does not appear to support the paragraph.

This does not mean the app is deciding what you are allowed to publish. It means the static-site package would be structurally invalid or misleading. Send the item back, attach the correct source, add citations, rewrite the body as public copy, or cut/hold it.

## Reset Or Cleanroom Test The App On Windows

Uninstalling the app is not always enough to create a clean first-run state. The app stores local data under both the install folder and app-data folders.

For a clean beta test, remove only The Civic Desk/CivicNewspaper data. Do not wipe unrelated Windows user data.

Typical Windows paths:

```text
%LOCALAPPDATA%\The Civic Desk
%LOCALAPPDATA%\com.scottconverse.civicdesk
%APPDATA%\com.scottconverse.civicdesk
%USERPROFILE%\.ollama
```

Remove `%USERPROFILE%\.ollama` only when you intentionally want to test local AI setup from scratch and that Ollama/model state was created for this app test.

## Source Import Misses URLs

CSV, TXT, XLSX, and DOCX files should produce reviewable source candidates. PDF import is disabled in the public beta until hardened parsing is available. Convert PDF source lists to TXT, CSV, DOCX, or XLSX, or paste the URLs directly.

What to do:

1. Try the import again with a small file first.
2. If the source list is a PDF, convert it to TXT, CSV, DOCX, or XLSX with your own trusted tool, or enter key URLs manually.
3. Review every imported candidate before enabling it.
4. If a normal text file imports poorly, report the file type and a small example.

## How To Report A Bug

Use GitHub Issues for bugs:

<https://github.com/scottconverse/CivicNewspaper/issues>

Use GitHub Discussions for questions, setup help, and feedback that is not clearly a bug:

<https://github.com/scottconverse/CivicNewspaper/discussions>

Include:

- App version.
- Windows version.
- Installer filename.
- Whether the installer checksum matched.
- What you clicked.
- What you expected.
- What happened.
- Any here.now URL, output ZIP path, or screenshots.
- Diagnostics exported from **System & Status**.

Do not paste private API keys, credentials, unpublished sensitive reporting notes, or private source lists into a public bug report.

If a useful report requires private material, say in the public issue that you have private diagnostics available, but do not attach them publicly unless you are comfortable sharing them.
