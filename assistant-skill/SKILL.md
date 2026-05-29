---
name: civicnews
description: Interact with the local CivicNews instance. Allows checking the story queue, fetching raw public records evidence packets, pushing draft articles, and running pre-publication guardrails.
---

# CivicNews Skill

Use this skill to interface directly with the local-first CivicNews application. It allows you (the coding assistant) to help the user manage their civic newsroom.

## How it works

The CivicNews application hosts a secure loopback API server on `127.0.0.1:12053`.
Access is authorized via a paired token saved in the user's home configuration directory:
* Windows: `%APPDATA%\civicnews-token.json`
* macOS: `~/Library/Application Support/civicnews-token.json` or `~/.config/civicnews-token.json`

A CLI utility is provided at [client.js](./client.js) to automate these calls.

## Token file security

The paired token is the *only* gate on the loopback API, so the token file must be readable only by the owning user.

* **macOS / Linux:** `client.js` writes the file with mode `0600` and runs `chmod 0600` after writing, so only your user account can read or write it. No action needed.
* **Windows:** Unix file modes are ignored by the filesystem, so `client.js` cannot tighten the ACL programmatically. `%APPDATA%` (`C:\Users\<you>\AppData\Roaming`) already inherits a per-user ACL that excludes other standard users, so the default location is private. If you relocate the token file, confirm its ACL grants access only to your user — for example:
  ```powershell
  icacls "$env:APPDATA\civicnews-token.json" /inheritance:r /grant:r "$($env:USERNAME):(R,W)"
  ```

Treat the token like a password: any local process running as your user can read this file and drive the API. Revoke a leaked token from the desktop app's "Browser Pairing" tab.

## Available Actions via CLI Bridge

You can run the CLI bridge using the `node` environment:

### 1. Pairing client
Pair with the local desktop app using the pairing token displayed in the "Browser Pairing" tab of the desktop GUI:
```bash
node assistant-skill/client.js pair <22-char-token>
```

### 2. View Queue
View today's generated leads and editorial drafts:
```bash
node assistant-skill/client.js queue
```

### 3. Fetch Evidence Packet
Fetch raw text records associated with a specific lead:
```bash
node assistant-skill/client.js evidence <lead_id>
```

### 4. Submit a Draft
Write an article based on the evidence, incorporating markdown citations like `[Brighton Minutes](evidence:ID)`, and upload it:
```bash
node assistant-skill/client.js draft <lead_id> <format> <title> <content>
```
* formats: `brief` | `watch` | `explainer` | `investigation` | `opinion`

### 5. Check Guardrails
Validate a draft against pre-publish guardrails checks (overlap, accusatory language, citation coverage):
```bash
node assistant-skill/client.js check <draft_id>
```

### 6. Run LLM task
Use the local Ollama backend for general purpose intelligence:
```bash
node assistant-skill/client.js llm "<prompt>" "[system-instructions]"
```

## Guidelines for the Assistant

1. **Factuality first**: Do not invent facts. Write drafts based STRICTLY on the evidence excerpts returned by `evidence <lead_id>`.
2. **Citation requirement**: Every factual claim you write must end with an explicit citation link, e.g. `[Excerpt text](evidence:ID)`.
3. **Presumption of innocence**: If reporting on criminal accusations, use terms like "alleged", "accused", or "charged". Avoid definitive guilt statements.
4. **Guardrails**: Always run `check <draft_id>` after creating or modifying a draft to ensure it complies with local community standards.
