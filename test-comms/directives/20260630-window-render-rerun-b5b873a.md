# Cleanroom rerun: packaged window render gate plus draft safety gates

Tester role: tester
Coder role: coder

Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination path on tester: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
Do not use coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main
Product commit represented by installer: b5b873a0da6ee9712a8ca1633464c6ee261dd5fc

Installer:
test-comms/artifacts/20260630-window-render-rerun-b5b873a/The Civic Desk_0.3.1_x64-setup.exe

Installer SHA256:
A3FDF4BCA93EFBC77A085C5C96063F419DBA640C4B9CA8F913B053BBC5A5439D

Installer size:
5629721

Required visibility report:
test-comms/reports/20260630-window-render-rerun-b5b873a-visibility.md

Required final report:
test-comms/reports/20260630-window-render-rerun-b5b873a-report.md

Required evidence/artifact folder:
test-comms/evidence/20260630-window-render-rerun-b5b873a/

## Why this rerun exists

The previous rerun installed successfully and launched civicnews.exe, and Windows reported a native window handle and title, but no visible app window rendered on the desktop. This build makes the packaged Tauri window configuration explicit and adds a Windows WebView2 startup guard for GPU composition problems.

This rerun must first prove that the installed app opens a visible desktop window from a normal user launch. Do not spend time on AI setup, scanning, drafting, or publishing until that is proven.

## First action

1. Fetch and checkout this branch.
2. Read test-comms/ACTIVE_DIRECTIVE.md.
3. Confirm it points to this directive.
4. Verify this installer path, SHA256, and byte size.
5. Write the visibility report named above.
6. Continue only if the installed app renders a visible window without handle manipulation.

## Cleanroom boundary

Perform a product clean wipe, not a Windows reinstall. Remove CivicNewspaper app data, prior test output, local test publications, bundled runtime remnants created by CivicNewspaper, prior Ollama/model state used for this test, and prior here.now output references. Leave Windows, the user account, browser, Git, and tester coordination checkout intact.

Do not manually install missing product prerequisites. If the app cannot guide setup or download what it needs, report that as product failure.

Anonymous here.now publish for this test is authorized.

## Required window gate

1. Install from the NSIS installer above.
2. Launch The Civic Desk from the installed Start Menu shortcut or installed EXE as a normal user.
3. Confirm a visible native app window appears on the desktop with title The Civic Desk and visible app content.
4. Do not use ShowWindow, MoveWindow, SetForegroundWindow, taskbar tricks, or handle manipulation to make the app visible for this gate.
5. Screenshot the desktop with the visible window and save it in the evidence folder.
6. If no visible window appears within 30 seconds, stop the workflow and write the final report with process details, screenshots, app data state, installed file tree, and any Windows event/log evidence.

## Required full test flow after the window gate passes

Use Longmont, Colorado.

Run the application as an ordinary end user would:

1. Complete first-run setup from a clean profile.
2. On Step 1, click the Longmont starter profile. It should fill fields and remain on Step 1.
3. Click the visible Next button. It must advance to Step 2 without the window disappearing, minimizing, or losing setup state.
4. Continue first-run setup. Let the app inspect hardware and guide AI/runtime setup.
5. Confirm setup wording is clear enough for a non-technical beta user.
6. Discover or add official, local-media, and public social or dark-signal sources readable without login.
7. Run a scan.
8. Confirm the app seeks enough leads for a paper instead of stopping at one item when sources exist.
9. Draft multiple items using the local AI path.
10. Exercise writer and editor workflow: draft, edit, save, hold, send back for more work, ready for review, approve for static publish, cut or kill where appropriate.
11. Attempt to approve at least one lead-based draft that has linked sources but no inline evidence citation. This must be blocked in Workbench before compile.
12. Inspect generated drafts for unsupported high-risk claims such as canceled, cancellation, COVID, pandemic, funding cuts, selected vendor, contractor, project costs, dates, officials, or impacts not present in the linked evidence. Such drafts must be replaced by a source-bound fallback, clearly held for verification, or blocked before public output.
13. Build and export the static publication ZIP/package.
14. Publish to here.now.
15. Save the generated ZIP/output folder, screenshots, logs, and here.now URL into the evidence folder.

## Output quality pass/fail bar

Pass only if:

1. The installer hash and size match.
2. The installed app renders a visible normal desktop window without tester manipulation.
3. First-run Step 1 advances reliably through the visible Next button.
4. The app-guided AI setup succeeds or gives clear product-owned recovery.
5. The run produces a reviewable Longmont publication package and a here.now URL.
6. The public output has no tester notes, no reporter scaffolding, no source check markers, no EDITOR_NOTE, no Body:/Headline:/Nut graf:/Reporting Steps leakage, no [Source needed], no [Verification needed], no [End of Report], and no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
7. Lead-based public stories or briefs have linked evidence and at least one inline evidence citation.
8. Unsupported claims from the prior run class do not reach approved or public output.
9. Duplicate-topic audit passes.
10. Target output is 5-10 reader-facing stories or briefs from 10-25 leads. If the app cannot reach this, the report must state the exact stage and reason: source discovery shortage, scan shortage, draft generation failure, editor workflow failure, compile block, publish failure, or legitimate source-quality shortage.

## Report requirements

Write the final report for a human reader first, then include technical evidence.

Include:

1. Pass/fail summary.
2. Exact installed app version if visible.
3. Whether the local AI path was used and which model.
4. Lead count and story/brief count.
5. Story titles and here.now URLs.
6. ZIP/output path produced by the software.
7. Screenshots of first-run setup, source discovery, scan results, draft workflow, approval blockers, compile/export, and here.now result.
8. Any failure with exact click/path/reproduction steps.
9. Full list of files added to the evidence folder.

If a failure happens, stop only when continuing would hide the root cause. Otherwise continue collecting evidence around the failure.
