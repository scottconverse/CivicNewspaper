# Cleanroom rerun: packaged window reveal plus Step 1 button fix

Tester role: tester
Coder role: coder

Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination path on tester: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
Do not use coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main
Product commit represented by installer: 9519547e35be59ad2002af6759cf11097f4d25f1

Installer:
test-comms/artifacts/20260630-window-button-rerun-9519547/The Civic Desk_0.3.1_x64-setup.exe

Installer SHA256:
10635FFB94C222D1A03BF569DEF104A21FD30ABBA260E1BEE41873A24538C65B

Installer size:
5635559

Required visibility report:
test-comms/reports/20260630-window-button-rerun-9519547-visibility.md

Required final report:
test-comms/reports/20260630-window-button-rerun-9519547-report.md

Required evidence/artifact folder:
test-comms/evidence/20260630-window-button-rerun-9519547/

## Why this rerun exists

The previous f8da868 build included the Step 1 real-button fix, but the cleanroom tester hit the visible-window gate first: the installed process started, Windows reported a main window handle and title, but no app content appeared on the desktop without manipulation.

This build keeps the Step 1 button fix and adds a Tauri startup visibility guard that repeatedly unminimizes, centers, shows, and focuses the main window during startup and the Ready event.

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

## Required first gates

1. Install from the NSIS installer above.
2. Launch The Civic Desk from the installed Start Menu shortcut or installed EXE as a normal user.
3. Confirm a visible native app window appears on the desktop with title The Civic Desk and visible app content.
4. Do not use ShowWindow, MoveWindow, SetForegroundWindow, taskbar tricks, or handle manipulation to make the app visible for this gate.
5. Wait up to 15 seconds on Step 1 if fields are empty.
6. Confirm the Publication Name, Editor Name, City, and State fields are filled by either the Longmont click or the no-input recovery notice.
7. Click the visible Next control.
8. Confirm Step 2 appears and identity settings are persisted in the app DB.
9. If these first gates fail, stop the workflow and write the final report with screenshots, DB snapshot, runtime diagnostics, and exact click/repro steps.

## Required full test flow after first gates pass

Use Longmont, Colorado.

Run the application as an ordinary end user would:

1. Continue first-run setup. Let the app inspect hardware and guide AI/runtime setup.
2. Confirm setup wording is clear enough for a non-technical beta user.
3. Discover or add official, local-media, and public social or dark-signal sources readable without login.
4. Run a scan.
5. Confirm the app seeks enough leads for a paper instead of stopping at one item when sources exist.
6. Draft multiple items using the local AI path.
7. Exercise writer and editor workflow: draft, edit, save, hold, send back for more work, ready for review, approve for static publish, cut or kill where appropriate.
8. Attempt to approve at least one lead-based draft that has linked sources but no inline evidence citation. This must be blocked in Workbench before compile.
9. Inspect generated drafts for unsupported high-risk claims such as canceled, cancellation, COVID, pandemic, funding cuts, selected vendor, contractor, project costs, dates, officials, or impacts not present in the linked evidence. Such drafts must be replaced by a source-bound fallback, clearly held for verification, or blocked before public output.
10. Build and export the static publication ZIP/package.
11. Publish to here.now.
12. Save the generated ZIP/output folder, screenshots, logs, and here.now URL into the evidence folder.

## Output quality pass/fail bar

Pass only if:

1. The installer hash and size match.
2. The installed app renders a visible normal desktop window without tester manipulation.
3. Step 1 identity fields fill through Longmont or the no-input recovery, and Step 1 Next advances to Step 2.
4. Identity settings are persisted before leaving Step 1.
5. The app-guided AI setup succeeds or gives clear product-owned recovery.
6. The run produces a reviewable Longmont publication package and a here.now URL.
7. The public output has no tester notes, no reporter scaffolding, no source check markers, no EDITOR_NOTE, no Body:/Headline:/Nut graf:/Reporting Steps leakage, no [Source needed], no [Verification needed], no [End of Report], and no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
8. Lead-based public stories or briefs have linked evidence and at least one inline evidence citation.
9. Unsupported claims from the prior run class do not reach approved or public output.
10. Duplicate-topic audit passes.
11. Target output is 5-10 reader-facing stories or briefs from 10-25 leads. If the app cannot reach this, the report must state the exact stage and reason: source discovery shortage, scan shortage, draft generation failure, editor workflow failure, compile block, publish failure, or legitimate source-quality shortage.

## Report requirements

Write the final report for a human reader first, then include technical evidence.

Include pass/fail summary, installed app version if visible, local AI/model result, lead and story counts, story titles and here.now URLs, ZIP/output path, screenshots for each major workflow area, any exact failure repro, and full evidence file list.
