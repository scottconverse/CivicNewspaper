# Final Cleanroom Release Recheck - Civic Desk v0.3.2 bdd0a40

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

The c93d10f run was blocked because `Compile site` did not produce a package, ZIP, publish run, or here.now publish path after an approved draft. The follow-up fix in this build blocks unlinked or malformed evidence citations before publish approval and normalizes model shorthand citations before editor approval.

## Product Under Test

- Product branch label: `main`
- Product commit represented by installer: `bdd0a40e0af46701c8a8eb1b815178bf830caae9`
- Product version: `0.3.2`
- Installer type: Windows NSIS

Installer artifact:

`test-comms/artifacts/20260702-final-cleanroom-v032-bdd0a40/The Civic Desk_0.3.2_x64-setup.exe`

Expected installer SHA256:

`D7FA08CDB668E49D229F6C403B1666F351B5627F1B8EB94333944F3FB9F8A4F8`

Expected installer size:

`5229228`

## Required Reports

Write the visibility report first:

`test-comms/reports/20260702-final-cleanroom-v032-bdd0a40-visibility.md`

Write the final human-readable report here:

`test-comms/reports/20260702-final-cleanroom-v032-bdd0a40-report.md`

Write evidence under:

`test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/`

Include screenshots, installer verification, install/launch logs, database summaries, draft rows, publish folder listings, ZIP listings, here.now URL evidence, and public-output scans.

## Cleanroom Setup

Use a true clean product state:

1. Stop all running `civicnews` and The Civic Desk processes.
2. Run the existing uninstaller if present.
3. Remove the installed app folder if it remains.
4. Remove `%APPDATA%\com.scottconverse.civicdesk`.
5. Remove `%LOCALAPPDATA%\com.scottconverse.civicdesk`.
6. Remove `%LOCALAPPDATA%\The Civic Desk`.
7. Remove any previous test-only `%USERPROFILE%\.ollama` state if it belongs to this cleanroom test.
8. Install only the NSIS artifact named above.
9. Launch the installed app normally from the installed location.

Do not manually install product dependencies outside the app-guided flow unless reporting a blocker. The app should guide local AI setup.

## Required Flow

Run the complete release path:

1. Verify active directive path and report it in the visibility report.
2. Verify installer SHA256 and size before install.
3. Clean wipe and install.
4. Confirm native app identity says The Civic Desk, not a dev-server identity.
5. Complete first-run identity setup for Longmont, Colorado.
6. Use the app-guided AI setup until AI Status is Ready.
7. Add or discover Longmont starter sources through the app.
8. Run Daily Scan and wait for completion.
9. Confirm the latest daily scan row is not stuck `in_progress` after leads exist.
10. Generate at least two drafts from different leads.
11. Confirm a no-source lead remains a verification assignment and cannot be approved for static publish.
12. Confirm a linked-source draft is attributed, reader-facing, and contains only linked evidence citations.
13. Use Workbench draft picker and top action strip.
14. Exercise `Improve for Publication` on a linked-source draft.
15. If `Improve for Publication` emits shorthand such as `(evidence:13)` or `[evidence:13]`, verify the editor normalizes it to `[Source](evidence:13)` when that evidence is linked.
16. If a draft cites an evidence ID that is not linked to that lead, verify approval is blocked with a visible, human-readable message.
17. Approve only a valid, source-linked, attributed, reader-facing draft.
18. Go to Publishing.
19. Open the output folder before compiling.
20. Click `Compile site`.
21. Verify the static site is written:
    - `index.html`
    - at least one article page
    - CSS assets
    - RSS or feed/share artifacts where applicable
    - `site-package.zip`
    - a publish run recorded in the app database
22. Verify `Publish to here.now` is available after successful compile.
23. Publish to here.now using the app flow.
24. Open the here.now URL and inspect the public pages.
25. Export or record the ZIP/path.

## Public Output Requirements

The final publication must be a real Longmont publication, not just mechanics proof.

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
- mojibake marker code points or visible mojibake such as U+00C3, U+00C2, U+00E2, or U+FFFD replacement characters
- unlinked `evidence:` citations
- disabled `unlinked-evidence-` citations
- source claims that do not match the linked evidence
- generic city navigation pages treated as news stories

Public output should include:

- A real headline, not a lead-summary blob.
- Reader-facing article copy.
- Clear attribution to linked public source material.
- No unsupported reporter names, staff names, or contact people.
- No unsupported future details, costs, dates, or votes.

## Report Content

The final report must state PASS or BLOCKED and include:

- Product SHA tested.
- Installer path, SHA256, and size observed.
- Clean wipe steps performed.
- App identity/title observed.
- AI setup result.
- Source discovery result.
- Daily Scan database summary.
- Draft rows considered, including title, lead ID, status, citations, and approval decision.
- Evidence-citation normalization/blocking results.
- Publish folder path.
- ZIP path and file listing if produced.
- Publish run database row if produced.
- here.now URL if published.
- Public page inspection notes.
- Any blocker with exact repro steps and evidence paths.

If blocked, stop after collecting enough evidence to make the blocker actionable.
