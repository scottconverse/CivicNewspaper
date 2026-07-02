# Loopback HTTP API Reference

CivicNewspaper's Rust backend runs an Axum HTTP server bound strictly to `127.0.0.1:12053`. It exists so local clients, including the Chromium browser extension and local assistant tools, can pair with the desktop app and exchange data over a controlled local channel.

This file is the route contract. If `src-tauri/src/core/server.rs` or `src-tauri/src/core/auth.rs` changes, update this document.

## Trust Boundary

The trust boundary is the local user account, not the process. Any process running as the same OS user can reach `127.0.0.1:12053`. After pairing, the bearer token is the authorization gate. See [SECURITY.md](../SECURITY.md) for the full threat model.

## Base URL

```text
http://127.0.0.1:12053
```

The server binds `127.0.0.1` only. It is not intended to be reachable from another machine.

## Request Prerequisites

All API routes are Host/Origin-gated.

1. **Host header:** must be exactly `127.0.0.1:12053`. Other values return `403 Forbidden`.
2. **Origin header:** if present, must be the bundled Civic Desk Browser Bridge origin `chrome-extension://fobahchhglbihbjfldjlnbbaoagbmjif` or an explicitly configured development origin. `Origin: null`, arbitrary Chrome extensions, and untrusted web origins return `403 Forbidden`.
3. **No-Origin callers:** protected routes may omit `Origin` only because they still need `Authorization: Bearer <token>`. The pairing route may omit `Origin` only when the request includes `x-civicnews-pair: 1`.
4. **Bearer token:** every route except `POST /api/pair` requires `Authorization: Bearer <token>`. Missing, malformed, revoked, or unknown tokens return `401 Unauthorized`.

`POST /api/pair` has no bearer-token requirement because it issues the bearer token. It still enforces Host/Origin rules, the explicit no-Origin pairing header, the one-time PIN check, and the rate limit.

## Routes

| Method | Path | Auth | Purpose |
|---|---|---|---|
| POST | `/api/pair` | Pairing PIN, Host/Origin gate, rate limit | Exchange the one-time pairing token for a bearer token |
| GET | `/api/queue` | Bearer | List all leads and drafts |
| GET | `/api/evidence/:lead_id` | Bearer | List evidence items behind a lead |
| POST | `/api/drafts` | Bearer | Create a new draft, forced to `draft_generated` |
| POST | `/api/llm/task` | Bearer | Run a one-shot local-LLM prompt |
| POST | `/api/guardrails/check` | Bearer | Run advisory guardrails over a draft |

## `POST /api/pair`

Exchanges the one-time pairing token shown in the desktop app's Browser Pairing tab for a long-lived bearer token. The field is named `pin` for historical reasons; the value is the 22-character pairing token, not a numeric PIN.

No-Origin local clients must include:

```text
x-civicnews-pair: 1
Host: 127.0.0.1:12053
```

Browser-extension clients may instead use the bundled Civic Desk Browser Bridge Origin `chrome-extension://fobahchhglbihbjfldjlnbbaoagbmjif`. Developer builds may allow extra origins only through `CIVICNEWS_ALLOWED_EXTENSION_ORIGINS`.

Rate limit: 5 failed attempts per client IP per 1800 seconds. A successful pair resets the counter.

Request:

```json
{ "pin": "<22-char pairing token>" }
```

Response:

```json
{ "token": "<long-lived bearer token>" }
```

Errors: `401 Unauthorized`, `403 Forbidden`, `429 Too Many Requests`, `500 Internal Server Error`.

## `GET /api/queue`

Returns every lead and draft in the local database.

Response:

```json
{
  "leads": [
    {
      "id": 1,
      "detector_name": "Money Threshold",
      "why": "Why the lead was flagged",
      "confidence": "medium",
      "risk_level": "low",
      "confirmation_checklist": "[]",
      "from_scan_lead_id": null,
      "story_type": "brief",
      "disposition": "ready_to_draft",
      "novelty_score": 4,
      "novelty_reason": "New meeting agenda item",
      "recurrence_count": 0,
      "recurrence_note": null,
      "created_at": "2026-05-28T12:00:00Z"
    }
  ],
  "drafts": [
    {
      "id": 1,
      "lead_id": 1,
      "format": "brief",
      "title": "Council sets hearing date",
      "content": "Draft Markdown",
      "status": "draft_generated",
      "verification_checklist": "[]",
      "missing_evidence_notes": null,
      "correction_note": null,
      "created_at": "2026-05-28T12:00:00Z",
      "updated_at": "2026-05-28T12:00:00Z"
    }
  ]
}
```

Checklist fields are JSON strings, not nested JSON arrays.

## `GET /api/evidence/:lead_id`

Returns evidence items associated with a lead id.

```json
[
  {
    "id": 1,
    "source_id": 3,
    "url": "https://example.gov/minutes/2026-05",
    "fetched_at": "2026-05-28T12:00:00Z",
    "excerpt": "Relevant excerpt",
    "content_hash": "b94d27b9...",
    "entities": "[\"City Council\"]"
  }
]
```

`entities` is a JSON string containing an array of strings.

## `POST /api/drafts`

Creates a new draft. The server forces `status` to `draft_generated`; clients cannot create an approved or published draft through this route.

Request:

```json
{
  "lead_id": 1,
  "format": "brief",
  "title": "Draft title",
  "content": "Draft Markdown",
  "verification_checklist": "[]"
}
```

Response:

```json
{ "id": 42 }
```

## `POST /api/llm/task`

Runs a single prompt against the selected local Ollama model, falling back to the default model if no model is selected.

Request:

```json
{
  "prompt": "Prompt text",
  "system": "System instruction"
}
```

Response:

```json
{ "result": "Model output" }
```

Errors: `503 Service Unavailable` or `504 Gateway Timeout`.

## `POST /api/guardrails/check`

Runs advisory guardrails over an existing draft. Guardrails warn and record concerns; they do not veto the publisher's final decision.

Request:

```json
{ "draft_id": 42 }
```

Response:

```json
{
  "is_clean": false,
  "issues": [
    {
      "category": "Citation Coverage",
      "message": "Missing citation",
      "severity": "warning",
      "paragraph_index": 3
    }
  ]
}
```

## Status Codes

| Code | Meaning |
|---|---|
| `200 OK` | Success |
| `401 Unauthorized` | Missing, malformed, revoked, unknown bearer token, or wrong pairing token |
| `403 Forbidden` | Bad Host, untrusted Origin, or no-Origin pair request missing `x-civicnews-pair` |
| `429 Too Many Requests` | Pairing rate limit hit |
| `503 Service Unavailable` | Local Ollama instance could not be reached or the model call failed |
| `504 Gateway Timeout` | Local model call exceeded the configured timeout |
| `500 Internal Server Error` | Database or internal failure |
