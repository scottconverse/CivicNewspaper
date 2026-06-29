# Directive - Rerun Draft Action Above Fold e2ac517

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

`e2ac517`

Installer artifact folder:

`test-comms/artifacts/20260628-draft-action-above-fold-e2ac517/`

Expected installer hashes:

- `The Civic Desk_0.2.8_x64-setup.exe`: `F1A958082A220BC3E25562CD03BABAF57274B4CE19D9434F5845055D128013A6`
- `The Civic Desk_0.2.8_x64_en-US.msi`: `9FB4ADBFFE432B2616D55BE081E25FC8FA777A06CF88B220888C3AD134C49A44`

## Why This Rerun Exists

The previous cleanroom run at `b1aebf4` still failed because the bottom draft-generation action area was clipped at 1280x720. Commit `e2ac517` removes the lower duplicate action and puts the single primary `Generate Draft` button directly above the fold under the lead summary.

## Required Test

Run the same cleanroom product wipe and reinstall as before, using only the installer artifact above.

Do not manually install Ollama, models, Node, npm, Rust, or any product dependency.

Do not manually edit the app database.

Do not use devtools to inject state.

After launch:

1. Let the app complete its own setup/model/source-intake recovery.
2. Confirm the app lands on Story Queue after Daily Scan.
3. At 1280x720, open produced Longmont leads with the visible UI.
4. Confirm the draft wizard has one visible `Generate Draft` button above the article format controls.
5. Generate at least five drafts through the app UI using the local model.
6. Exercise writer/editor controls:
   - approve at least five stories for publishing,
   - mark at least one draft needs verification or send-back,
   - put at least one draft on hold,
   - kill/cut one draft only if the UI clearly labels it and confirmation is shown.
7. Compile/export the static publication package through Publishing.
8. Publish anonymously to here.now using the app workflow.
9. Verify the returned here.now URL loads.
10. Verify the local ZIP/static output exists and includes the published stories.
11. If the product reaches this point, begin the 12-hour soak from `ACTIVE_DIRECTIVE.md`.

If a UI control is still unreachable, or clicks/typing still do not register, stop at the exact blocking step, capture screenshot(s), record DB counts read-only, and report precisely where it broke.

## Reporting

Write the next report here:

`test-comms/reports/20260628-draft-action-above-fold-rerun-report-e2ac517.md`

Put artifacts here:

`test-comms/artifacts/20260628-draft-action-above-fold-rerun-e2ac517/`

Commit and push the report/artifacts with `[skip ci]`.
