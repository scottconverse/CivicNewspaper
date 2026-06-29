# Tester Report - Full E2E Continuation 637e941

Date: 2026-06-29T06:52:00Z
Tester machine: msi\civic
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Active directive: test-comms/directives/20260629-continue-full-e2e-after-637e941-partial.md
Product branch: stable-readiness-local-gates
Required product commit: 637e941ac77361033fc22b48fac33ae1aa50a6b3
Result: PASS for functional cleanroom E2E workflow; editorial readiness still requires human review

## Summary

The 637e941 installed build was resumed from the prior partial state. The app had 18 Longmont leads and 2 drafts. I used the installed product UI via WebView2/CDP control, not a rebuild, to continue the cleanroom release loop.

Final outcome:

- 18 reviewable leads available.
- 7 total drafts generated through the product.
- 6 drafts approved as `ready_to_publish`.
- 1 spare/nonessential draft killed through the product confirmation flow.
- Press-freedom/legal-risk advisor was run on multiple drafts.
- Title/body edit and save path were exercised.
- Static site compiled with 6 articles and 23 files.
- ZIP package exported.
- Anonymous here.now publish completed.
- Public URL verified HTTP 200: https://oaken-bloom-z7nj.here.now
- Public page title verified as `Longmont Civic Desk`.

## Resume / Install State

The run resumed from the installed state left by the previous 637e941 partial test. I did not wipe or reinstall.

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

The installer artifacts remained present and hashes matched the directive:

- NSIS: `test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64-setup.exe`
- NSIS SHA256: `50F64FFCE76106BC1745766CA3AF0A50A46C5464F22BDB65220C8EDED348F67F`
- MSI: `test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64_en-US.msi`
- MSI SHA256: `04DCB36733FD969C4E17C763220BD9E135256524101883432FCD09E50EC1C7F1`

## Environment

- Windows tester account: msi\civic
- App-owned local AI status: UI showed `Local AI ready`
- Selected model: `qwen2.5:7b`
- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10

The product handled the runtime/model state from the installed app; I did not manually install Ollama, models, PATH fixes, browser helpers, or prerequisites during this continuation.

## Workflow Results

Already-drafted lead behavior:

- Opened an existing draft from Workbench.
- Returned to Story Queue using the direct `Back to Queue` button at 1280x720.
- Existing draft cards showed `Open draft` instead of creating duplicate drafts.

Draft generation:

- Continued from 2 drafts.
- Generated drafts 3, 4, 5, 6, and a spare draft 7 through the installed product.
- Database verified 7 drafts total.

Editor controls:

- Ran press-freedom/legal-risk advisor on multiple drafts.
- Edited title/body content and saved.
- Approved drafts for static publish with editor attestation.
- Killed one spare nonessential draft through the confirmation flow.
- Hold control was clicked during the spare-draft control pass, but final durable proof is the killed spare draft and six publishable drafts.

Publishing:

- Changed publication identity from starter text to `Longmont Civic Desk`.
- Recompiled static output.
- Exported ZIP package.
- Tested here.now connector.
- Published anonymously to here.now.
- Verified HTTP 200 and visible Longmont publication.

## Final Counts

Draft status counts:

```json
[
  ["killed", 1],
  ["ready_to_publish", 6]
]
```

Latest publish run:

```text
issue-20260629-064922-482434100
provider: here_now
url: https://oaken-bloom-z7nj.here.now
articles: 6
files: 23
skipped: 0
generated: 2026-06-29T06:49:22.482434100+00:00
```

Published post rows were created for six article files:

- `watch/1.html`
- `watch/2.html`
- `watch/3.html`
- `watch/4.html`
- `watch/5.html`
- `watch/6.html`

Killed spare draft:

- Draft 7, lead 14, `Public Participation Rules`, status `killed`

## Output Paths

Local output folder:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/publication-output/site/`

ZIP package:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/publication-output/site/site-package.zip`

Public URL:

https://oaken-bloom-z7nj.here.now

HTTP verification:

```json
{
  "url": "https://oaken-bloom-z7nj.here.now",
  "status": 200,
  "title": "Longmont Civic Desk",
  "containsLongmont": true,
  "containsCivicDesk": true
}
```

## Key Artifacts

Main artifact folder:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/`

Selected proof files:

- `cont637-02-existing-draft-opened.png`
- `cont637-03-back-to-queue-direct.png`
- `cont637-04-story-queue-cdp.png`
- `cont637-draft3-after-generation.png`
- `cont637-draft4-after-generation.png`
- `cont637-draft5-after-generation.png`
- `cont637-draft6-after-generation.png`
- `cont637-approve-draft1-after-click.png`
- `cont637-approve-draft2-after-click.png`
- `cont637-approve-draft4-after-click.png`
- `cont637-approve-draft5-after-click.png`
- `cont637-kill-confirm-after.png`
- `cont637-17-after-recompile.png`
- `cont637-19-after-publish-click.png`
- `cont637-20-herenow-public-verified.png`
- `cont637-final-db-summary.txt`
- `herenow-oaken-bloom-index.html`
- `publication-output/site/index.html`
- `publication-output/site/publish-manifest.json`
- `publication-output/site/site-package.zip`

## Findings

Blockers: 0

Major findings:

1. The publication identity initially compiled as starter text (`My Local Publication`). I corrected this through the app identity UI and republished as `Longmont Civic Desk`.

2. The draft cards still publish titles with `Draft:` prefixes. This is usable as proof that the workflow functions, but the resulting copy is not ready for a real public Longmont issue without editor cleanup.

Minor findings:

1. The kill flow requires a second confirmation button. The first status click alone does not complete the kill; the confirmation click changed draft 7 to `killed`.

2. The anonymous here.now publish is temporary. A permanent site would need a saved here.now API key or another durable hosting target.

Mojibake / encoding:

- No obvious mojibake markers were visible in the publishing UI or public page proof.

## Human Quality Assessment

The release workflow is functionally usable: the installed product can resume a cleanroom state, generate Longmont briefs, run advisor/editor controls, approve stories, compile static output, export a ZIP, and publish anonymously to here.now.

Scott should not use this exact generated issue as a real Longmont publication next week without human editorial review. The app workflow works, but the generated stories still need normal editor cleanup, fact checking, title cleanup, and publication judgment before real-world use.

Release-gate interpretation: functional E2E pass; editorial/publication-quality pass requires human owner review.
