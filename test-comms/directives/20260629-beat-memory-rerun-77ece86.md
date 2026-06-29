# Directive: Beat Memory Story-Quality Rerun

Status: ACTIVE

Tester identity and path:

- You are the tester on the separate cleanroom machine running as `msi\civic`.
- Use this coordination checkout path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use any path under `C:\Users\instynct`. That path belongs to the coder machine and is invalid on the tester machine.
- If a previous `006c800` story-quality run is still active and no report has been committed yet, stop that run after saving any brief partial notes locally, then run this directive instead. This directive supersedes it.

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Coordination branch:

`test-comms/cleanroom-coder-tester`

Product branch:

`stable-readiness-local-gates`

Product commit:

`77ece863db668df9889828587416696f3a39b6cc`

Artifact folder:

`test-comms/artifacts/20260629-beat-memory-rerun-77ece86/`

Preferred installer:

`test-comms/artifacts/20260629-beat-memory-rerun-77ece86/The Civic Desk_0.2.9_x64-setup.exe`

Preferred installer SHA256:

`FBAA8AB176A0AB256A0D710B781472DEC15216F99250C30D787D99D430DC85F0`

Fallback installer:

`test-comms/artifacts/20260629-beat-memory-rerun-77ece86/The Civic Desk_0.2.9_x64_en-US.msi`

Fallback installer SHA256:

`EA30BB05B5FFFFDEB7576D42B6C61DB780B2BBE5EF3C6D727AEC94C70125622F`

Expected report:

`test-comms/reports/20260629-beat-memory-rerun-77ece86-report.md`

Expected evidence folder:

`test-comms/reports/20260629-beat-memory-rerun-77ece86-evidence/`

## Purpose

Run a focused cleanroom rerun of Longmont story quality and editor workflow, now including advisory beat memory for recurring topics.

This is not a full release gauntlet. It verifies the next output-quality layer:

1. Clean install still works from the artifact above.
2. App-guided AI setup still works without manual tester installation of Ollama or models.
3. Longmont source discovery/import still includes official and public social/community sources.
4. Daily Scan still produces usable editor leads.
5. Recurring/background topics are not inflated into fresh news stories.
6. Recurring/background topics are not hidden or blocked; they must remain visible with advisory beat-memory context.
7. Held drafts expose Resume Editing and Send Back for More Work.
8. Draft generation respects story-quality context instead of producing reporter-note scaffolding as public copy.
9. If a genuine current story is found, ZIP export and here.now anonymous publish still work.

## Required setup

1. Fetch the coordination branch.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Confirm it points to this directive.
4. Confirm the product commit is exactly `77ece863db668df9889828587416696f3a39b6cc`.
5. Confirm installer hash before running it.
6. Prefer the NSIS installer. Use MSI only if NSIS fails.
7. Use a product-clean app profile/data state if practical. Do not reset Windows.
8. Do not manually install Ollama, models, app prerequisites, or developer tools. If the product cannot do its own setup, report that as a product failure.

## Test flow

### A. Install and first run

Install the app from the preferred artifact. Launch it. Record any unsigned-installer warning and whether the beta explanation is understandable.

Complete first-run setup for Longmont, Colorado.

Let the app choose and install/download any AI runtime or model it needs. Record:

- detected hardware summary shown by the app, if shown
- selected model
- progress feedback quality
- whether the app looked hung at any point
- whether manual tester dependency installation was needed

### B. Source setup

Use Longmont. Include:

- official city/county/school/public-record sources
- at least one general/evergreen official page, such as a council archive, service page, or meeting-video page
- at least one public readable social/community source

Do not log in to private services or private groups.

### C. First scan

Run the normal scan/fetch/detect/Daily Scan flow.

Record:

- number of leads
- number of story-worthy items
- number of briefs/watch/background/verification items
- whether any general evergreen page is labeled as current news
- screenshots or exported evidence showing the labels

If no current publishable story appears, do not fabricate one. Continue with the beat-memory check.

### D. Beat-memory rerun

Run a second scan over the same or overlapping Longmont sources so recurring topics can appear again.

Pass conditions:

- recurring or evergreen material remains visible to the editor
- recurring or evergreen material includes clear beat-memory/editor context
- the app says, in substance, that it has seen a similar topic before
- the app warns that a story needs a new vote, date, filing, outage, dollar amount, meeting item, public impact, or other new reportable fact
- the app does not veto, hide, auto-delete, or block the editor

Fail conditions:

- the same recurring topic is presented as a fresh full story with no warning
- the app hides or blocks the lead from the editor
- the warning is too vague to help an editor decide what changed

### E. Editor workflow

Generate a draft from one lead that is either:

- clearly current and story-worthy, or
- clearly recurring/background/watch

For a current item, the draft should read like a newspaper draft, not reporter notes.

For a recurring/background/watch item, the draft should remain cautious and explain what is missing or what must be verified before publication.

Put the draft on Hold. Confirm the held state exposes:

- Resume Editing
- Send Back for More Work

Click Send Back for More Work. Confirm the draft clearly moves into a needs-more-work / needs-verification state and remains recoverable.

Resume editing. Confirm the editor can still decide what to do.

### F. Output package

If at least one genuinely current story is found and approved by the tester as editor:

- export ZIP
- publish anonymously to here.now
- verify public pages have no reporter scaffolding, editor notes, `[Source needed]`, `[Verification needed]`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, or mojibake
- include the here.now URL and local ZIP/output path in the report

If no genuinely current story is found:

- do not approve fake or evergreen story copy as news
- export or screenshot the watch/background/verification package if possible
- report clearly that the mechanics worked but the source set did not yield a publishable current story during this run

## Required report contents

Write:

`test-comms/reports/20260629-beat-memory-rerun-77ece86-report.md`

Include:

- machine identity and local coordination path
- installer used and hash verified
- app version/build/commit shown or inferred
- AI setup result
- selected model
- source list summary
- first scan lead counts
- second scan/beat-memory result
- screenshots or text evidence for beat-memory labels
- editor workflow result for Hold, Resume Editing, and Send Back for More Work
- ZIP/output path if produced
- here.now URL if published
- public output quality notes if published
- exact pass/fail conclusion
- blockers with exact reproduction steps

Commit the report and evidence folder to the coordination branch with `[skip ci]`.
