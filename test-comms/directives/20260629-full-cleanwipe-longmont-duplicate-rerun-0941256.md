# Tester Directive: Full Clean-Wipe Longmont Rerun After Duplicate-Lead Fix

Status: ACTIVE

Supersedes: `test-comms/directives/20260629-mojibake-evidence-audit-f092852.md`.

Reason: Scott reviewed the f092852 here.now output and found five stories, with two covering the same Building Services online permitting portal outage. That is not an acceptable finished newspaper output. Product commit `09412560a326379fcf75f327439df8d1d2bb47b4` adds paraphrased Daily Scan lead clustering so same-topic leads do not become separate draftable story candidates.

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `09412560a326379fcf75f327439df8d1d2bb47b4`

Artifact folder: `test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/`

Preferred installer:

`test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/The Civic Desk_0.2.8_x64-setup.exe`

Expected preferred NSIS SHA256:

`DC395291F909097A46C273FDC698A0F1822C314F6F019F9092888A6AD7F6B325`

Fallback MSI:

`test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/The Civic Desk_0.2.8_x64_en-US.msi`

Expected fallback MSI SHA256:

`B866845F47C32E643A143CD3E5F70FF9F4BCA33912DB036917572D56252ED407`

Report path:

`test-comms/reports/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-report.md`

Artifact evidence path:

`test-comms/reports/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-evidence/`

## Machine Context

You are the tester on the separate cleanroom machine running as `msi\civic`.

Use this coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct`; that is the coder machine path and is invalid on the tester machine except as a warning example.

All reports and evidence files must be UTF-8 without BOM. Do not write UTF-16 evidence files.

## Required Clean-Wipe Boundary

Wipe only CivicNewspaper, Ollama, local models, test files, app data, PATH changes, and related prerequisites, leaving Windows, the user account, browser, and Git coordination checkout intact.

The tester must not manually install missing product prerequisites for the app. If CivicNewspaper cannot set up what it needs through normal end-user UI, report that as a product failure.

## Required Workflow

Run the complete end-user workflow from a clean product state:

1. Fetch and read `test-comms/ACTIVE_DIRECTIVE.md`. Confirm it points to this directive.
2. Verify installer hashes before installation.
3. Install the NSIS artifact as a normal user. Use MSI only if NSIS fails, and document why.
4. Launch the app as a normal user.
5. Complete first-run setup for Longmont, Colorado.
6. Let the app detect hardware and set up the local AI/runtime path through the normal UI.
7. Use source discovery/import for Longmont with both official and public community/social sources. No private/logged-in scraping.
8. Run Daily Scan from the visible UI.
9. Confirm the app produces 10-25 leads or clearly records why fewer were available.
10. Draft enough leads through the normal writer/editor workflow to produce 5-10 reader-facing stories or briefs.
11. Use editor controls normally: approve/send back/hold/kill/cut as appropriate. Do not publish duplicate story topics just to reach the count.
12. Compile the issue.
13. Export the ZIP/package.
14. Publish anonymously to here.now through the visible app UI. This anonymous public publish is authorized.
15. Verify the here.now URL returns HTTP 200 and visibly contains the generated publication.
16. Save screenshots, downloaded HTML, output folder path, ZIP path, and here.now URL.

## Duplicate Topic Audit

The finished public issue must not contain duplicate story topics.

Run a duplicate-topic audit over the final public article titles and, where practical, the article body/excerpt text. This is not exact title matching. It must catch paraphrases like:

- `Building Services Portal Experiencing Technical Issues`
- `Building Services Online Permitting Portal Experiencing Technical Issues`

Those two are the same story topic and would fail this directive if both appear as separate public articles.

At minimum, save a JSON file named:

`test-comms/reports/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-evidence/duplicate-topic-audit.json`

The JSON must include:

- article count,
- article titles,
- candidate duplicate pairs,
- token overlap or other method used,
- PASS/FAIL.

## Required Quality Checks

The report must include:

- PASS or FAIL.
- here.now URL, if produced.
- installer hashes observed.
- local model/runtime selected by the app.
- source count and source categories, including official and public community/social sources.
- lead count.
- draft count.
- ready/published story or brief count.
- killed/held/sent-back count.
- output folder.
- ZIP path.
- whether any duplicate story topics were found.
- whether the Building Services permitting-portal duplicate recurred.
- mojibake scan result against local output and here.now output.
- public `Draft:` title-prefix check.
- screenshots of setup, sources, Daily Scan, Story Queue, Workbench/editor, Publishing, and here.now page.
- a plain-English quality note: does this look like a usable local issue, or only a mechanical test artifact?

## Pass / Fail Bar

PASS only if:

- Clean product install/setup succeeds without tester-installed prerequisites.
- Daily Scan completes through visible UI.
- The app produces enough leads for a real issue or clearly and honestly explains why it cannot.
- The tester produces 5-10 reader-facing stories or briefs using normal writer/editor workflow.
- No duplicate story topics appear in the public issue.
- The Building Services duplicate specifically does not recur.
- ZIP export exists and can be opened/extracted.
- here.now publish succeeds and returns an HTTP 200 URL.
- Mojibake and public `Draft:` prefix checks pass.

FAIL if:

- Duplicate story topics reach the public issue.
- The publication has only one lead/story, or cannot produce a real issue without manual tester help.
- The app requires the tester to install missing prerequisites manually.
- here.now publish fails.
- The output is mechanically generated but not reviewable as a local issue.

If it fails, do not paper over it. Say exactly where it broke and attach evidence.

Commit the report and evidence with `[skip ci]`.
