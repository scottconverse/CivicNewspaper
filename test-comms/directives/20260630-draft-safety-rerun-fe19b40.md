# Cleanroom rerun: draft safety and approval preflight

Tester role: tester
Coder role: coder

Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination path on tester: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
Do not use coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main
Product commit represented by installer: fe19b407102d90256e14bb80fc63577a6fc16890

Installer:
test-comms/artifacts/20260630-draft-safety-rerun-fe19b40/The Civic Desk_0.3.1_x64-setup.exe

Installer SHA256:
967F12990DC98B2B70498872DBD278E3514509E0A74E8E5B3F5C457B3B5E6D20

Installer size:
5633326

Required visibility report:
test-comms/reports/20260630-draft-safety-rerun-fe19b40-visibility.md

Required final report:
test-comms/reports/20260630-draft-safety-rerun-fe19b40-report.md

Required evidence/artifact folder:
test-comms/evidence/20260630-draft-safety-rerun-fe19b40/

## Why this rerun exists

The previous cleanroom run proved that compile-time output-trust blocking worked, but it still failed because the draft generator produced unsupported claims and the Workbench approval flow let bad lead-based copy reach compile. This build adds model-output normalization, source-bound fallback drafts, high-risk unsupported-claim rejection, and Workbench blockers for static publish approval.

## First action

1. Fetch and checkout this branch.
2. Read test-comms/ACTIVE_DIRECTIVE.md.
3. Confirm it points to this directive.
4. Verify this installer path, SHA256, and byte size.
5. Write the visibility report named above.
6. Continue the full cleanroom test unless the installer is missing or hash/size does not match.

## Cleanroom boundary

Perform a product clean wipe, not a Windows reinstall. Remove CivicNewspaper app data, prior test output, local test publications, bundled runtime remnants created by CivicNewspaper, prior Ollama/model state used for this test, and prior here.now output references. Leave Windows, the user account, browser, Git, and tester coordination checkout intact.

Do not manually install missing product prerequisites. If the app cannot guide setup or download what it needs, report that as product failure.

Anonymous here.now publish for this test is authorized.

## Required test flow

Use Longmont, Colorado.

Run the application as an ordinary end user would:

1. Install from the NSIS installer above.
2. Complete first-run setup.
3. Let the app inspect hardware and guide AI/runtime setup.
4. Confirm the setup wording is clear enough for a non-technical beta user.
5. Discover or add official, local-media, and public social/dark-signal sources that are readable without login.
6. Run a scan.
7. Confirm the app seeks enough leads for a paper instead of stopping at one item when sources exist.
8. Draft multiple items using the local AI path.
9. Exercise writer/editor workflow: draft, edit, save, hold, send back for more work, ready for review, approve for static publish, cut/kill where appropriate.
10. Specifically attempt to approve at least one lead-based draft that has linked sources but no inline evidence citation. This must be blocked in Workbench before compile.
11. Specifically inspect generated drafts for unsupported high-risk claims such as canceled, cancellation, COVID, pandemic, funding cuts, selected vendor, contractor, project costs, dates, officials, or impacts not present in the linked evidence. Such drafts must be replaced by a source-bound fallback, clearly held for verification, or blocked before public output.
12. Build/export the static publication ZIP/package.
13. Publish to here.now.
14. Save the generated ZIP/output folder, screenshots, logs, and here.now URL into the evidence folder.

## Output quality pass/fail bar

Pass only if:

1. The installer hash and size match.
2. The app-guided AI setup succeeds or gives clear product-owned recovery.
3. The run produces a reviewable Longmont publication package and a here.now URL.
4. The public output has no tester notes, no reporter scaffolding, no source check markers, no EDITOR_NOTE, no Body:/Headline:/Nut graf:/Reporting Steps leakage, no [Source needed], no [Verification needed], no [End of Report], and no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
5. Lead-based public stories or briefs have linked evidence and at least one inline evidence citation.
6. Unsupported claims from the prior run class do not reach approved/public output.
7. Duplicate-topic audit passes.
8. Target output is 5-10 reader-facing stories/briefs from 10-25 leads. If the app cannot reach this, the report must state the exact stage and reason: source discovery shortage, scan shortage, draft generation failure, editor workflow failure, compile block, publish failure, or legitimate source-quality shortage.

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
