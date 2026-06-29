# CivicNewspaper Cleanroom Rerun - Output Scaffolding Cleanup

Status: ACTIVE

Coder timestamp: 2026-06-29T15:48:05Z

## Coordination Source Of Truth

Use this GitHub repo and branch:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`
- This directive: `test-comms/directives/20260629-output-scaffolding-rerun-c01e32f.md`

Your coordination checkout on the tester machine is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct` paths on the tester machine. That path belongs to the coder machine.

Refresh with:

```powershell
cd C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
git fetch origin test-comms/cleanroom-coder-tester --prune
git checkout test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms\ACTIVE_DIRECTIVE.md
```

## Product Under Test

- Product branch: `stable-readiness-local-gates`
- Product commit: `c01e32fdccb50b5a19182b7128f666e8de5cc304`
- Prior failed commit: `cd038d696fe9708aaa54c23dd766eff36112f93b`
- Prior failure report: `test-comms/reports/20260629-full-e2e-output-quality-landing-cd038d6-report.md`

This build is intended to fix the prior public-output failure where generated article pages exposed internal scaffolding strings such as `EDITOR_NOTE` and `Body:`.

## Installer Artifacts

Preferred NSIS installer:

`test-comms/artifacts/20260629-output-scaffolding-rerun-c01e32f/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`9A2828D9B98EBBDEA2F625F5BD3EEFAB824B79E6A80FF8FD57AF7EF534D415DE`

Fallback MSI:

`test-comms/artifacts/20260629-output-scaffolding-rerun-c01e32f/The Civic Desk_0.2.9_x64_en-US.msi`

Expected SHA256:

`669B9B40CECDA12657210EE2247C6920B5A1F91FF23BD50CB05B06FC5A49FBEA`

## Required Test

Run a cleanroom rerun of the Longmont publication flow using the artifact above.

You may preserve the Windows user account, browser, Git, and coordination checkout. Wipe CivicNewspaper/The Civic Desk app data, prior app-owned Ollama/model state if app-owned by the test, previous generated output, and previous installed app version before installing this artifact.

Do not manually install Ollama or models outside the app. If the app cannot set up the needed local AI runtime/model itself, mark that as a product failure.

Use Longmont, Colorado. Include official, public local-media/search, and public/social/community sources readable without login. No private groups, no credentials, and no bypass of private or proprietary material.

Exercise the normal user path:

1. Install the preferred NSIS artifact and verify its SHA256 first.
2. Launch The Civic Desk.
3. Let app-guided AI setup choose/download what it needs.
4. Configure Longmont.
5. Discover/import official and public/social sources.
6. Run Daily Scan.
7. Generate enough leads and drafts to exercise the prior failure path. Aim for 10-25 leads and at least 5 approved reader-facing stories/briefs if the app can produce them.
8. Exercise writer/editor workflow controls: draft, edit, approve, hold, and return/send-back where available.
9. Compile/export the static publication ZIP.
10. Publish the same output to here.now using the already-authorized anonymous here.now test path.
11. Verify the ZIP contents and here.now pages.

## Mandatory Output Quality Checks

The rerun fails if any reader-facing public artifact contains these internal scaffolding markers:

- `EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[End of Report]`
- `[Source needed]`
- `[Verification needed]`

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
- mojibake appears in UI or output,
- the ZIP cannot be opened/extracted,
- here.now publish does not return a usable URL.

If the app produces editor warnings or guardrail cautions, record them, but do not treat warnings as software vetoes. The human editor must remain able to decide.

## Required Evidence To Commit Back

Write the report here:

`test-comms/reports/20260629-output-scaffolding-rerun-c01e32f-report.md`

Write evidence under:

`test-comms/reports/20260629-output-scaffolding-rerun-c01e32f-evidence/`

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

PASS only if the app installs from the artifact, app-guided AI setup works, Longmont sources are scanned, editorial workflow is exercised, ZIP export works, here.now publish works, and no public output leaks internal editor scaffolding.

FAIL with exact reproduction notes if any required item fails.
