# Directive: Verify Publication Output Cleanup Build 7fe1145

Status: ACTIVE

Tester machine: `msi\civic`

Coordination checkout path on tester:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct` on the tester machine. That path belongs only to the coder machine.

## Purpose

Verify the product fixes in commit `7fe11452ea7ccbb9425df291a030da58ff8e48bf` against the blockers found in the 637e941 cleanroom run:

1. Published output contained mojibake markers that appear when UTF-8 text is decoded incorrectly.
2. Stale generated article pages could remain in the export folder after a story was killed or no longer publishable.
3. Killed stories needed clearer protection against accidental approval.
4. Anonymous here.now publishing should have a nonempty display name without manual repair.

This is a focused blocker-verification rerun. Do not wipe the full tester machine for this directive unless the installed app cannot be upgraded cleanly. Use the existing Longmont cleanroom state so the old mojibake-producing draft content is exercised by the fixed compiler.

## Product Branch And Commit

Product repo:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Required product commit:

`7fe11452ea7ccbb9425df291a030da58ff8e48bf`

## Installer Artifacts

Artifact folder in this coordination repo:

`test-comms/artifacts/20260629-rerun-full-e2e-7fe1145/`

Preferred NSIS installer:

`test-comms/artifacts/20260629-rerun-full-e2e-7fe1145/The Civic Desk_0.2.8_x64-setup.exe`

Expected NSIS SHA256:

`9F495209FFA6254B095EA946F5C2553067D5362834FC7BF62D662522B9F36C4A`

Fallback MSI installer:

`test-comms/artifacts/20260629-rerun-full-e2e-7fe1145/The Civic Desk_0.2.8_x64_en-US.msi`

Expected MSI SHA256:

`18B9C45C7896A42C554177A063D08B4462A44C2563FF11437E19F5DA8ACFB154`

## Required Steps

1. Fetch and fast-forward the coordination checkout on `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md` and this directive from the tester checkout.
3. Verify both installer hashes exactly.
4. Install or upgrade using the NSIS installer. Use the MSI only if NSIS fails.
5. Confirm the installed app launches and shows Longmont state from the existing cleanroom run.
6. Confirm Local AI still shows ready or degrades cleanly. Do not manually install Ollama, models, PATH entries, packages, or browser helpers.
7. In the app, recompile/export the existing Longmont publication using the fixed build.
8. Publish anonymously to here.now from the product UI or product command path. This live anonymous here.now publish is already authorized.
9. Record the here.now URL and verify HTTP 200.
10. Copy the generated output folder and `site-package.zip` into:

`test-comms/artifacts/20260629-output-cleanup-7fe1145/publication-output/`

11. Scan every generated `.html`, `.md`, `.txt`, `.xml`, and `.json` file in that copied output for mojibake marker characters. Use codepoint-based markers so this directive is not itself dependent on console encoding:

```powershell
$markers = @(
  [string][char]0x00C3, # capital A with tilde, common in double-encoded UTF-8
  [string][char]0x00C2, # capital A with circumflex, common in nonbreaking-space mojibake
  [string][char]0x00E2  # lowercase a with circumflex, common in smart quote/dash mojibake
)
```

12. Confirm the scan result is empty. If not empty, report exact files and snippets.
13. Verify killed-story behavior:
    - If the existing killed draft is visible in Workbench, try to open it.
    - Confirm the app clearly indicates it is killed and cannot be approved directly.
    - Confirm the publish output does not include an article page for the killed draft.
14. Verify stale-output cleanup:
    - Confirm the generated output folder does not contain old article pages for drafts that are not `ready_to_publish`, `published`, or `corrected`.
    - Compare the manifest article list to actual article page files.
15. Save screenshots for the app state, export/publish result, here.now public page, killed-story UI if available, and output scan result.

## Report Requirements

Write the report to:

`test-comms/reports/20260629-output-cleanup-7fe1145-report.md`

Write artifacts to:

`test-comms/artifacts/20260629-output-cleanup-7fe1145/`

The report must include:

- pass/fail status,
- installed app path,
- installer used,
- both installer hash checks,
- app visible state,
- Local AI state,
- article count,
- killed draft status and whether it was excluded,
- manifest article paths,
- actual article files found,
- mojibake scan result,
- here.now URL,
- HTTP verification result,
- ZIP path and SHA256,
- screenshots list,
- any user-facing UI confusion found.

Commit the report and artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.

If this focused blocker rerun passes, leave a note recommending the next directive be a full clean wipe and end-to-end Longmont publication run using commit `7fe11452ea7ccbb9425df291a030da58ff8e48bf`.
