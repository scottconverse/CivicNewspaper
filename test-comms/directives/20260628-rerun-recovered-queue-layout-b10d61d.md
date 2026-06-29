# Directive - Rerun Recovered Queue Layout Fix b10d61d

Status: ACTIVE

Tester machine: `msi\civic`

Approved coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Repo:

`https://github.com/scottconverse/CivicNewspaper`

Coordination branch:

`test-comms/cleanroom-coder-tester`

Product branch:

`stable-readiness-local-gates`

Product commit to test:

`b10d61d`

Installer artifact folder:

`test-comms/artifacts/20260628-recovered-queue-layout-b10d61d/`

Expected installer hashes:

- `The Civic Desk_0.2.8_x64-setup.exe`: `9EE583BAEBE1C93F76A8AC683C37165A6F9F9A0BE53DF44F14DEFDCC9C9AF3A9`
- `The Civic Desk_0.2.8_x64_en-US.msi`: `AC846CD62DC51392948AEA6F93C3C7417B54A8B011370DA112EE6AD1FC18265B`

## Why This Rerun Exists

The previous cleanroom run at `4658500` proved:

- clean install,
- app-managed Ollama startup,
- app-managed `qwen2.5:7b` availability,
- first-run recovery to the main app,
- Longmont starter source import,
- evidence ingest,
- Daily Scan,
- Story Queue lead creation.

It then blocked because the 1280x720 packaged app layout left the active content trapped off to the right/top, so the tester could not reliably open leads and continue to drafting/publishing.

Commit `b10d61d` changes the recovered path to route to Story Queue after the automatic scan and compacts/stacks the sidebar at constrained widths.

## Required Test

Run the same cleanroom product wipe and reinstall as the previous directive, using only the installer artifact above.

Do not manually install Ollama, models, Node, npm, Rust, or any product dependency.

Do not manually edit the app database.

Do not use devtools to inject state.

After launch:

1. Let the app complete its own setup/model/source-intake recovery.
2. Confirm the app lands on Story Queue after Daily Scan, not stranded on the top of Daily Scan.
3. At 1280x720, verify the Story Queue content and lead cards/buttons are visible and reachable.
4. Open at least five produced Longmont leads with the visible UI.
5. Generate drafts through the app UI using the local model.
6. Exercise writer/editor controls on at least two drafts:
   - approve for publishing,
   - send back / needs verification,
   - hold,
   - kill or delete only if the UI clearly labels it and confirmation is shown.
7. Leave at least five drafts/stories usable for a finished issue.
8. Compile/export the static publication package through Publishing.
9. Publish anonymously to here.now using the app workflow.
10. Save/report:
   - here.now URL,
   - local output folder,
   - ZIP path,
   - count of leads,
   - count of drafts,
   - count of approved/published stories,
   - screenshots of Story Queue, Workbench, Publishing, exported site homepage, and at least one story page.

If a UI control is still unreachable or clicks/typing still do not register, stop at the exact blocking step, capture screenshot(s), record DB counts read-only, and report precisely where it broke.

## Reporting

Write the next report here:

`test-comms/reports/20260628-recovered-queue-layout-rerun-report-b10d61d.md`

Put artifacts here:

`test-comms/artifacts/20260628-recovered-queue-layout-rerun-b10d61d/`

Commit and push the report/artifacts with `[skip ci]`.
