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

A CLI utility is provided at [client.js](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/assistant-skill/client.js) to automate these calls.

## Available Actions via CLI Bridge

You can run the CLI bridge using the `node` environment:

### 1. Pairing client
Pair with the local desktop app using the 6-digit PIN displayed in the "Browser Pairing" tab of the desktop GUI:
```bash
node assistant-skill/client.js pair <6-digit-pin>
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
