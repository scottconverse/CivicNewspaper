# CivicNewspaper cleanroom E2E test - attempt 1

Status: ACTIVE
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: a0b436af3009500714055a2bff01612716ee36c1
Directive path: test-comms/directives/20260630-cleanroom-e2e-a0b436a-attempt1.md
Visibility report path: test-comms/reports/20260630-cleanroom-e2e-a0b436a-visibility-attempt-1.md
Final report path: test-comms/reports/20260630-cleanroom-e2e-a0b436a-report.md
Tester artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/

## Stop old context first

Stop any CivicCast watcher, CivicCast repo context, or old local path assumption before doing anything here. This directive is for CivicNewspaper only.

Do not assume the coder machine path exists on the tester. Do not use `C:\Users\instynct` on the tester except as a warning example of a path that is invalid there. Use your actual tester-machine clone path. The previously known tester fallback path is approved if it exists:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## First action: prove visibility

Before starting the long cleanroom run, fetch this coordination branch and write the visibility report:

`test-comms/reports/20260630-cleanroom-e2e-a0b436a-visibility-attempt-1.md`

The visibility report must include:

- Windows user and hostname.
- Actual local coordination clone path.
- `git rev-parse HEAD` for `test-comms/cleanroom-coder-tester`.
- Confirmation that `test-comms/ACTIVE_DIRECTIVE.md` points to this directive.
- Confirmation that both installer artifacts are visible locally.
- SHA256 hash verification for both installer artifacts.
- Confirmation that any old CivicCast watcher/context has been stopped.
- Confirmation that the 15 minute CivicNewspaper watcher remains armed for follow-up directives.

Commit and push the visibility report with `[skip ci]`, then continue the test.

## Installer artifacts to use

Use these files from the coordination branch:

- `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64-setup.exe`
  - SHA256: `B6777C66A7330A46F6FC443576C06E648E516EC52EC845004044DB4663A23BD8`
  - Size: `5605081`
- `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64_en-US.msi`
  - SHA256: `4C4F40178017853DFA5E65AFD10595306018C0F2B803190A1DB431A28CA8AA2E`
  - Size: `9117696`

Prefer the NSIS installer unless it fails. If the NSIS installer fails, record the exact failure, then try the MSI. Do not silently switch without reporting why.

## Cleanroom wipe boundary

Wipe only CivicNewspaper related state:

- CivicNewspaper app install.
- CivicNewspaper app data.
- CivicNewspaper test files.
- Prior exported publications from this test loop.
- Ollama install, Ollama models, Ollama service/processes, and PATH changes related to Ollama if present.
- Related prerequisites installed only for prior CivicNewspaper testing.

Do not reset Windows, delete the Windows user account, delete unrelated browser profiles, or remove unrelated developer tools.

The tester must not manually install Ollama, models, app prerequisites, or missing runtime pieces. If the product installer or first-run app cannot set up what it needs, stop, report the exact product failure, and include screenshots/logs.

## Required end-to-end test

Run CivicNewspaper as a normal non-technical user would.

1. Install from the artifact and launch the app.
2. Verify first-run setup explains unsigned beta install clearly enough for a normal user.
3. Let the app read machine capability and choose/download/configure the appropriate local AI model. Record the model chosen, progress states, and whether the user can tell it is working.
4. Configure a Longmont publication with neutral identity text. Do not allow made-up publisher identity claims.
5. Use official and public/social/readable sources. No login-only sources, private groups, private data, or credentialed scraping.
6. Run source discovery/import.
7. Run Daily Scan.
8. Produce 10 to 25 leads if the app can. If it cannot, report the exact limit, source evidence, and expansion attempts.
9. Produce 5 to 10 reader-facing stories or briefs if the app can. If it cannot, report the exact break and evidence.
10. Exercise writer/editor workflow on at least three items:
    - create draft
    - edit
    - save
    - send back or rework
    - hold
    - cut
    - restore or reopen where available
    - approve
11. Invoke the press-freedom/legal-risk advisor on at least one story. It must advise only; it must not block the editor.
12. Export the static site ZIP/output package through the app.
13. Publish anonymously to here.now. This anonymous test publish is authorized for this run.
14. Verify the here.now URL loads publicly.
15. Verify the ZIP/static output and here.now output match in story count and visible content.

## Public output quality gates

The final public output must be checked for all of these:

- No duplicate story topics.
- No public leak of: `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, `[End of Report]`.
- No mojibake marker code points: `U+00C2`, `U+00C3`, `U+00E2`, `U+FFFD`.
- Headlines read like actual reader headlines, not lead summaries.
- Public pages are newspaper output, not reporter notes.
- The output does not claim all articles used AI unless the publisher explicitly configured that statement.
- The output does not claim no ads, nonprofit status, public-record-only coverage, or a made-up publication identity unless configured by the publisher.
- here.now is visible in publish docs/UI as the default preview publish path.

## Evidence required

Final report must be written for the human product owner, not just coder-to-coder debugging. Include:

- Pass/fail summary in plain English.
- Installer used and hash verification.
- Clean wipe evidence.
- AI setup evidence and model chosen.
- Source list and source discovery results.
- Lead count and story/brief count.
- Story list with titles, source URLs, and status.
- Screenshots for first-run, source discovery, Daily Scan, Workbench workflow, publish/export, and final here.now site.
- Local output ZIP path on tester disk.
- Committed copy of output ZIP or output folder under `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/` if GitHub size limits allow it.
- here.now URL.
- Exact failure point if any required step fails.
- Whether the tester watcher remains armed for follow-up directives.

If the app breaks, do not patch around it manually. Report the product failure exactly so the coder can fix the product and issue the next artifact.
