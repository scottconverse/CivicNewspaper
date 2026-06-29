# Directive - Rerun Draft Controls Fix b1aebf4

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

`b1aebf4`

Installer artifact folder:

`test-comms/artifacts/20260628-draft-controls-b1aebf4/`

Expected installer hashes:

- `The Civic Desk_0.2.8_x64-setup.exe`: `DCBE98E3056AC06D98F4A795F8D80C3362D9F26861201AABD7624911E7862A61`
- `The Civic Desk_0.2.8_x64_en-US.msi`: `84C15F4C56EB24D203199ED497EF745323119ACFEBE277393757999867B8159E`

## Why This Rerun Exists

The previous cleanroom run at `b10d61d` proved the recovered path reaches Story Queue and lead cards at 1280x720. It then blocked because the Workbench draft-generation action bar was clipped at the bottom of the 1280x720 window.

Commit `b1aebf4` makes the draft wizard action bar truly sticky and reachable, and trims the wizard vertical footprint.

## Required Test

Run the same cleanroom product wipe and reinstall as the previous directive, using only the installer artifact above.

Do not manually install Ollama, models, Node, npm, Rust, or any product dependency.

Do not manually edit the app database.

Do not use devtools to inject state.

After launch:

1. Let the app complete its own setup/model/source-intake recovery.
2. Confirm the app lands on Story Queue after Daily Scan.
3. At 1280x720, open at least five produced Longmont leads with the visible UI.
4. Generate at least five drafts through the app UI using the local model.
5. Exercise writer/editor controls:
   - approve at least five stories for publishing,
   - mark at least one draft needs verification or send-back,
   - put at least one draft on hold,
   - kill/cut one draft only if the UI clearly labels it and confirmation is shown.
6. Compile/export the static publication package through Publishing.
7. Publish anonymously to here.now using the app workflow.
8. Verify the returned here.now URL loads.
9. Verify the local ZIP/static output exists and includes the published stories.
10. If the product reaches this point, begin the 12-hour soak from `ACTIVE_DIRECTIVE.md`.

If a UI control is still clipped, unreachable, or clicks/typing still do not register, stop at the exact blocking step, capture screenshot(s), record DB counts read-only, and report precisely where it broke.

## Reporting

Write the next report here:

`test-comms/reports/20260628-draft-controls-rerun-report-b1aebf4.md`

Put artifacts here:

`test-comms/artifacts/20260628-draft-controls-rerun-b1aebf4/`

Commit and push the report/artifacts with `[skip ci]`.
