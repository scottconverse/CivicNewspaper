# Final Cleanroom Release Recheck - Civic Desk v0.3.2 8261de9

## Hard Routing

Tester must stop using any old CivicCast or stale CivicNewspaper watcher context.

Use this repo and branch only:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`

Tester local coordination path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use this coder-machine path on tester:

`C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms`

Refresh commands:

```powershell
cd C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
git fetch origin test-comms/cleanroom-coder-tester --prune
git checkout test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms/ACTIVE_DIRECTIVE.md
```

## Supersedes

This directive supersedes:

- `test-comms/directives/20260702-final-cleanroom-v032-b0f4ce2.md`
- `test-comms/directives/20260702-final-cleanroom-v032-18eb480.md`
- `test-comms/directives/20260702-final-cleanroom-v032-916653b.md`
- `test-comms/directives/20260702-final-cleanroom-v032-20cfedc.md`
- `test-comms/directives/20260702-final-cleanroom-v032-c93d10f.md`
- `test-comms/directives/20260702-final-cleanroom-v032-bdd0a40.md`

The bdd0a40 run passed the release mechanics but found one Major issue: `Improve for Publication` changed a Colorado General Assembly source reference into a California Legislature reference. This build includes the linked evidence excerpt in the improve prompt and rejects improved text that introduces unsupported U.S. state names.

## Product Under Test

- Product branch label: `main`
- Product commit represented by installer: `8261de957b37beeda07944c8b12ab758494d1796`
- Product version: `0.3.2`
- Installer type: Windows NSIS

Installer artifact:

`test-comms/artifacts/20260702-final-cleanroom-v032-8261de9/The Civic Desk_0.3.2_x64-setup.exe`

Expected installer SHA256:

`7A08193C2BBA216C4E16291EB8EC45F89B6161B07BBF59D0A169D7DD590960D8`

Expected installer size:

`5225374`

## Required Reports

Write the visibility report first:

`test-comms/reports/20260702-final-cleanroom-v032-8261de9-visibility.md`

Write the final human-readable report here:

`test-comms/reports/20260702-final-cleanroom-v032-8261de9-report.md`

Write evidence under:

`test-comms/evidence/20260702-final-cleanroom-v032-8261de9/`

## Required Flow

Run a clean install and the final release path. Reuse the bdd0a40 flow shape, but explicitly verify the prior Major finding:

1. Verify active directive path and installer SHA256/size.
2. Clean wipe product/runtime state.
3. Install only the NSIS artifact named above.
4. Confirm native app identity says The Civic Desk.
5. Complete first-run identity setup for Longmont, Colorado.
6. Use app-guided AI setup until AI Status is Ready.
7. Add or discover Longmont starter sources through the app.
8. Run Daily Scan and wait for completion.
9. Generate at least two linked-source drafts from different leads.
10. Generate one no-source verification assignment and confirm it cannot be approved for static publish.
11. In Workbench, run `Improve for Publication` on at least one linked-source draft whose linked evidence contains a state or jurisdiction term such as Colorado, Colorado General Assembly, Longmont, or Boulder County.
12. Verify improved text does not change that jurisdiction to an unsupported state or jurisdiction.
13. If the model tries to introduce an unsupported U.S. state name, verify the app rejects the improved text and leaves the editor content unchanged with a visible error.
14. Verify citation shorthand normalization still works for linked citations.
15. Verify unlinked evidence citation approval blocking still works.
16. Approve only a valid, source-linked, attributed, reader-facing draft.
17. Go to Publishing, open the output folder, compile the site, and verify static output.
18. Verify these files exist:
    - `index.html`
    - at least one article page
    - CSS assets
    - RSS or feed/share artifacts where applicable
    - `site-package.zip`
    - a publish run recorded in the app database
19. Publish to here.now using the app flow.
20. Open the here.now URL and inspect public pages.
21. Record the ZIP path and here.now URL.

## Public Output Requirements

Public pages and ZIP contents must not contain:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`
- mojibake marker code points such as U+00C3, U+00C2, U+00E2, or U+FFFD
- unlinked `evidence:` citations
- disabled `unlinked-evidence-` citations
- source claims that do not match linked evidence
- unsupported jurisdiction changes such as Colorado becoming California

## Report Content

The final report must state PASS or BLOCKED and include:

- Product SHA tested.
- Installer path, SHA256, and size observed.
- Clean wipe steps performed.
- AI setup result.
- Daily Scan database summary.
- Draft rows considered.
- Improve for Publication jurisdiction-drift result.
- Evidence-citation normalization/blocking result.
- Publish folder path.
- ZIP path and file listing.
- Publish run database row.
- here.now URL.
- Public page inspection notes.
- Any blocker with exact repro steps and evidence paths.

If blocked, stop after collecting enough evidence to make the blocker actionable.
