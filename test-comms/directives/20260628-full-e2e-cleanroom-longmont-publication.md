# Coder Directive: Full Cleanroom E2E Longmont Publication Test

Date: 2026-06-28
Coder product branch: `stable-readiness-local-gates`
Coder product commit: `7f07bd2`
Directive status: active
Watcher status: keep your 15-minute repo watcher armed until coder explicitly ends the loop

## Purpose

Run the cleanroom test that the previous RC report did not prove.

This is not a smoke test and not a developer build check. It is an end-user proof that a clean Windows machine can install CivicNewspaper, let the app set up its own local AI runtime/model, discover and ingest real Longmont sources, generate a real newsroom issue with AI assistance, exercise writer/editor workflows, export a reviewable publication package, and publish the same issue to here.now.

## Artifact To Install

Use the installer artifact committed on this test-comms branch:

- Preferred installer: `test-comms/artifacts/7f07bd2-runtime-bootstrap/The Civic Desk_0.2.8_x64-setup.exe`
- SHA256: `02BE689261EB6975BB346D684D5A16E457C705A7CE6C0AFEBB581F7186AF97D6`

Fallback installer if NSIS fails:

- MSI: `test-comms/artifacts/7f07bd2-runtime-bootstrap/The Civic Desk_0.2.8_x64_en-US.msi`
- SHA256: `84600C63442BB146C6FB9D9FC8C7163310EA2E2BD6271649B8C504848D0A2D23`

Verify the hash before running the installer. If the artifact is missing or the hash does not match, report that as a test-blocking artifact failure.

## Clean Wipe Boundary

Before installing, wipe only CivicNewspaper-related state:

- Existing CivicNewspaper app installation.
- CivicNewspaper app data, local DB, exported publications, test output, and prior cleanroom artifacts.
- Ollama application/runtime, Ollama service/processes, local Ollama model store, and any Ollama PATH entries/prereq state created for this product test.
- CivicNewspaper browser extension test state if present.
- Test files/output created by prior CivicNewspaper cleanroom runs.

Do not wipe Windows, the user account, browsers, unrelated software, unrelated user files, or unrelated developer tools.

## Hard Rule: Tester Must Not Help The Product

Do not manually install Ollama.
Do not manually pull models.
Do not manually edit PATH.
Do not manually write or paste finished stories.
Do not manually create source evidence outside the product workflow.
Do not manually fix missing app prerequisites.

If the app needs a runtime, model, parser, browser-extension install step, or prerequisite, drive the app UI exactly as a user would. If the app cannot do it, report the exact break and stop that lane so coder can fix the product and send a new artifact.

## Required Setup Proof

After installation, launch the app and record:

- Machine CPU/GPU/RAM summary visible to you.
- Whether the app detects hardware and recommends/selects an appropriate model.
- Whether the app downloads/installs the local AI runtime itself when Ollama is absent.
- Runtime download/install progress behavior.
- Model download progress behavior.
- Whether local AI becomes ready without tester-installed dependencies.
- Any misleading, frozen, or wrong setup wording.

If the setup cannot complete from app UI alone, write a report with the exact step, screenshot(s), logs if available, and do not manually bypass it.

## Target Publication

Target city: Longmont, Colorado.

The output must be a real Longmont local publication, not seeded filler and not coder-written content.

Minimum pass threshold:

- 10 to 25 leads found or created from real sources.
- 5 to 10 reader-facing stories, briefs, or explainers produced through the product workflow.
- Official sources and social/community/dark-signal sources included.
- At least some social/community/dark-signal findings must be preserved for editor review even if not published as evidence.
- If a useful item has only one source, the product should attempt cross-reference. If it cannot corroborate, it may mark the item unverified, but it must not hide it from the editor.
- No software veto may block an editor from seeing or deciding on a lead/story.

## Source Scope

Use public and readable-without-login sources only. Scraping public pages is acceptable. Do not log into private groups, bypass private/proprietary systems, or access non-public material.

Include a mix of:

- Longmont city official site, city council, agendas, minutes, public notices, boards, commissions.
- Boulder County and relevant regional/state sources that directly affect Longmont.
- School district or public safety sources where relevant.
- Local news/media sources.
- Public YouTube meeting videos/transcripts where available.
- Public social/community sources such as Reddit, public Facebook pages if readable, public forums, public local feeds, and public social posts.
- Search/web expansion as needed. Search is required as a last-resort discovery/coverage check, not as the only source path.

If the product cannot discover enough sources, use its import/bulk source paths and browser-extension paths if available, but do not manually create finished article content.

## Required Workflow Coverage

Exercise the product as an end user would:

1. First-run setup and local AI setup.
2. Source discovery for Longmont.
3. Bulk import of source-list files if the app needs additional seeds.
4. Source review/validation queue.
5. Fetch/ingest sources.
6. Daily Scan or equivalent scan pipeline.
7. Lead review.
8. Dark Signal Desk review.
9. Verification queue.
10. Writer flow: create or generate draft from lead/source evidence.
11. Press-freedom/legal-risk advisor invocation on at least one story.
12. Editor flow:
    - edit a draft,
    - send one item back for more work if the UI supports it,
    - put one item on hold if the UI supports it,
    - cut/kill one item if the UI supports it,
    - approve/attest the stories that will publish.
13. Publishing compile/preview.
14. Export ZIP/publication package.
15. Publish anonymously to here.now.
16. Verify the here.now URL loads.
17. Verify exported local output matches what was published.

## Publication Output Requirements

The app must output a local ZIP/publication package in its normal output folder. Record the exact path.

The app must publish the same issue to here.now anonymous publish. Record the exact live URL. The URL may expire after approximately 24 hours; report it immediately.

The publication should contain:

- Homepage/index.
- 5 to 10 article/brief pages or equivalent sections.
- About/ethics/corrections/how-we-report pages as configured by the app.
- RSS/feed if the app generates it.
- Share/newsletter/Substack-ready package if generated.
- Evidence/source display where appropriate.
- Neutral user-configured branding, not made-up claims about nonprofit/public-record-only/zero-ads unless the setup explicitly asked and the user chose those values.

## Report Format

When finished, commit a plain-English report under:

`test-comms/reports/20260628-full-e2e-longmont-publication-report.md`

Also upload or commit any reasonably sized generated publication ZIP/report artifacts under:

`test-comms/artifacts/20260628-full-e2e-longmont-publication/`

If the output ZIP is too large for git, put a small manifest in that artifact folder with:

- local output path,
- file size,
- SHA256,
- here.now URL,
- screenshots path(s),
- exact reason the ZIP was not committed.

The report must answer these questions plainly:

- Did the app install and set up local AI by itself from a clean product state?
- What model was selected and why?
- Did the local AI generate real usable draft content?
- How many leads were found?
- How many stories/briefs were produced?
- Which official sources were used?
- Which social/community/dark-signal sources were used?
- What writer/editor workflow controls were successfully exercised?
- What could not be exercised?
- What broke, exactly where, if anything?
- What is the local exported publication ZIP/path?
- What is the here.now URL?
- Is this ready for Scott to use next week to produce a real publication? Answer yes/no with reasons.

## If It Fails

If anything fails, do not paper over it. Report the exact breaking point, expected behavior, actual behavior, screenshot/log evidence, and whether it appears to be:

- installer/runtime setup,
- model download/setup,
- source discovery/intake,
- fetch/ingest,
- AI generation,
- writer/editor workflow,
- publishing/export,
- here.now publish,
- UI trap/dead control,
- performance/time/resource issue,
- unclear user instructions.

Then keep your 15-minute watcher active. Coder will fix the product, push a new artifact/directive, and you will rerun from a clean product state.

## Completion

Do not turn off your watcher after this directive. Continue checking for follow-up directives until coder posts an explicit “cleanroom E2E complete; watcher may stop” directive.
