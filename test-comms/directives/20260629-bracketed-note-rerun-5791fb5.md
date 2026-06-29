# CivicNewspaper Cleanroom Rerun - Bracketed Editor Note And Mojibake Cleanup

Status: ACTIVE

Coder timestamp: 2026-06-29T16:48:06Z

## Coordination Source Of Truth

Use this GitHub repo and branch:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`
- This directive: `test-comms/directives/20260629-bracketed-note-rerun-5791fb5.md`

Your coordination checkout on the tester machine is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct` paths on the tester machine. That path belongs to the coder machine.

## Product Under Test

- Product branch: `stable-readiness-local-gates`
- Product commit: `5791fb5146d76fc5e97012488c995d0de1bb99c6`
- Prior failed commit: `c01e32fdccb50b5a19182b7128f666e8de5cc304`
- Prior failure report: `test-comms/reports/20260629-output-scaffolding-rerun-c01e32f-report.md`

This build is intended to fix the prior public-output failure where generated article pages exposed bracketed internal scaffolding strings such as `[EDITOR_NOTE: ...]`. It also includes an adjacent public-output cleanup for evidence-excerpt mojibake.

## Installer Artifacts

Preferred NSIS installer:

`test-comms/artifacts/20260629-bracketed-note-rerun-5791fb5/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`9CF4714A253E32D04E1FB1394B6D583B37CCC77C21FDACEBE212D6F1BBDD117C`

Fallback MSI:

`test-comms/artifacts/20260629-bracketed-note-rerun-5791fb5/The Civic Desk_0.2.9_x64_en-US.msi`

Expected SHA256:

`D53AF37831195AD2F36B59436ADA30D14D59313AADB819FBE7E5703AAB85ACCF`

## Required Test

Run a cleanroom Longmont publication rerun using the artifact above.

You may preserve the Windows user account, browser, Git, and coordination checkout. Wipe CivicNewspaper/The Civic Desk app data, prior app-owned Ollama/model state if app-owned by the test, previous generated output, and previous installed app version before installing this artifact.

Do not manually install Ollama or models outside the app. If the app cannot set up the needed local AI runtime/model itself, mark that as a product failure.

Exercise the normal user path:

1. Install the preferred NSIS artifact and verify its SHA256 first.
2. Launch The Civic Desk.
3. Let app-guided AI setup choose/download what it needs.
4. Configure Longmont, Colorado.
5. Discover/import official, public local-media/search, and public/social/community sources readable without login.
6. Run Daily Scan.
7. Draft and approve at least five reader-facing stories/briefs if the app can produce them.
8. Exercise editor controls including approve and hold. Record if return/send-back remains unavailable in the tested state.
9. Compile/export the static publication ZIP.
10. Publish the same output to here.now using the already-authorized anonymous here.now test path.
11. Verify generated output, extracted ZIP output, and live here.now pages.

## Mandatory Output Quality Checks

The rerun fails if any reader-facing public artifact contains these internal scaffolding markers:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[End of Report]`
- `[Source needed]`
- `[Verification needed]`

The rerun also fails if any reader-facing public artifact contains mojibake marker code points:

- U+00C2, commonly seen as an unwanted leading capital A-with-circumflex
- U+00C3, commonly seen as an unwanted leading capital A-with-tilde
- U+00E2, commonly seen as an unwanted leading lowercase a-with-circumflex
- U+FFFD, the Unicode replacement character

Check at minimum:

- generated `index.html`
- every generated `watch/*.html`
- generated `feed.xml`
- `newsletter.md`
- `substack.md`
- `share-package.md`
- extracted ZIP copy of the same files
- live here.now homepage
- live here.now article pages

Also fail if:

- any article page is blank or only a note saying it needs more reporting,
- any public headline is just a long lead-summary sentence instead of a reader headline,
- duplicate story topics are published as separate stories without clear differentiation,
- the ZIP cannot be opened/extracted,
- here.now publish does not return a usable URL.

Warnings are allowed. Software vetoes are not. The human editor must remain able to decide.

## Required Evidence To Commit Back

Write the report here:

`test-comms/reports/20260629-bracketed-note-rerun-5791fb5-report.md`

Write evidence under:

`test-comms/reports/20260629-bracketed-note-rerun-5791fb5-evidence/`

Include:

- installer hash check result,
- install result,
- model/runtime setup evidence,
- source list,
- lead/story counts,
- screenshots or text captures of source discovery, Story Queue, Workbench, Publishing,
- output-quality audit JSON with marker scan results,
- generated output folder path on tester disk,
- ZIP path and SHA256,
- extracted ZIP check,
- here.now URL,
- homepage/article/RSS/share captures from generated output and here.now,
- final pass/fail summary in plain English.

Commit and push reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.

## Pass/Fail Bar

PASS only if the app installs from the artifact, app-guided AI setup works, Longmont sources are scanned, editorial workflow is exercised, ZIP export works, here.now publish works, and no public output leaks internal scaffolding or mojibake markers.

FAIL with exact reproduction notes if any required item fails.
