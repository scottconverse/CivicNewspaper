# Directive: GauntletGate All Clean-Machine v0.3.0 Test

Status: ACTIVE

This directive is for the CivicNewspaper cleanroom tester machine only.

Tester machine context:

- You are running as `msi\civic`.
- Use coordination checkout `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Do not use any path under `C:\Users\instynct`; that is the coder machine and is invalid on the tester machine.
- Stop any old CivicCast or other-project watcher context. Watch only the CivicNewspaper repo and this branch.
- Single source of truth: `https://github.com/scottconverse/CivicNewspaper`, branch `test-comms/cleanroom-coder-tester`, file `test-comms/ACTIVE_DIRECTIVE.md`.

Product under test:

- Repository: `https://github.com/scottconverse/CivicNewspaper`
- Product branch: `main`
- Product commit: `b0be4d7432e9f5f791da68770a9631b8c5892697`
- Version: `0.3.0`

Installer artifacts on this coordination branch:

- Preferred NSIS installer: `test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/The Civic Desk_0.3.0_x64-setup.exe`
- Expected NSIS SHA256: `F3256C116F04B734C8C311E5B3EFEB69B24DAF3134C521C986BDF2C45CC1DF7E`
- Fallback MSI installer: `test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/The Civic Desk_0.3.0_x64_en-US.msi`
- Expected MSI SHA256: `D294096A95FEBF55E0CB30D104ADD8B31BC27981F150BA8B70FEDFD547EC07E1`

Required report:

- `test-comms/reports/20260629-gauntlet-all-cleanmachine-v030-b0be4d7-report.md`

Required evidence/artifact folder:

- `test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/evidence/`

Clean wipe boundary:

Wipe only CivicNewspaper/The Civic Desk, Ollama, local models, test files, app data, PATH changes, and related prerequisites. Leave Windows, the tester user account, browser, Git, and the coordination checkout intact. Do not perform an OS reset or reimage.

Before installing:

1. Verify you are in `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
2. Fetch and pull `test-comms/cleanroom-coder-tester`.
3. Read `test-comms/ACTIVE_DIRECTIVE.md`.
4. Confirm the active directive path matches this file.
5. Verify installer SHA256 hashes exactly.
6. Record `pwd`, `git branch --show-current`, `git log -1 --oneline`, and the active directive filename in the report.

Clean-machine test requirements:

1. Remove prior The Civic Desk/CivicNewspaper install, app data, test output, Ollama install, Ollama models, and PATH entries related to this app or Ollama.
2. Prove the dependency-absent state before launch:
   - No The Civic Desk process.
   - No Ollama process.
   - No Ollama executable available on PATH.
   - No local models installed.
   - The Civic Desk app data is empty or absent.
3. Install The Civic Desk from the provided v0.3.0 installer as a normal user.
4. Launch the app as a normal end user.
5. Do not manually install Ollama, models, runtimes, drivers, or app prerequisites. If the app cannot install or guide required prerequisites itself, report that as a product failure.
6. Let the app read the machine capability and select the recommended local model.
7. Verify the app clearly tells the user what it is downloading/installing and shows progress so it does not appear hung.
8. Complete setup for Longmont, Colorado.
9. Include official and public/social sources. Public readable sources are allowed. Do not use private groups, credentialed scraping, or non-public/proprietary sources.
10. Run the full source discovery / Daily Scan / lead generation path.
11. Target output:
    - 10 to 25 leads.
    - 5 to 10 reader-facing stories or briefs if the evidence genuinely supports them.
    - If the app cannot reach that output, record exactly where and why it fails, then continue with the best in-product recovery path.
12. Exercise writer/editor workflow:
    - Generate drafts.
    - Edit at least one draft.
    - Mark ready for review.
    - Approve at least one story.
    - Put one draft on hold.
    - Send one draft back for more work if the UI exposes it.
    - Cut one draft.
    - Restore or resume at least one held/cut draft if the UI exposes it.
    - Run guardrails.
    - Run the optional press-freedom/legal-risk advisor on at least one story.
13. Verify story quality:
    - No reporter scaffolding leaks publicly: `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.
    - No mojibake marker code points in public output: U+00C2, U+00C3, U+00E2, U+FFFD.
    - Duplicate-topic audit: do not publish two stories that are the same topic in different words.
    - Weak evergreen/background material should be labeled or held, not silently published as news unless the editor deliberately approves it after review.
14. Compile the static publication.
15. Export the ZIP/static site package using the app's built-in export path.
16. Publish anonymously to here.now. This here.now anonymous test publish is authorized.
17. Record the here.now URL.
18. Save the exported ZIP/output path and upload a copy or clear path reference in the evidence folder.
19. Capture screenshots or logs proving:
    - Clean state before launch.
    - Installer hash verification.
    - First-run setup.
    - App-guided AI/Ollama/model setup or exact failure.
    - Source discovery/import.
    - Daily Scan results.
    - Draft generation.
    - Editor workflow controls.
    - Guardrail/advisor output.
    - Publication preview.
    - ZIP/export result.
    - here.now publish result.

Report requirements:

Write a plain-English human report, not just a coder note. Include:

- Pass/fail verdict.
- Exact install artifact used.
- Exact machine/user/path context.
- Whether the clean wipe was completed.
- Whether first-run dependency-absent state was proven.
- Whether the app installed everything it needed without tester help.
- Whether local AI actually generated content.
- Lead count, draft count, approved story count, published story count.
- A short description of every published story and why it was or was not a real news item.
- Export ZIP/output path.
- here.now URL.
- All blockers, criticals, majors, minors, and polish findings.
- Exact repro steps for every failure.
- Evidence artifact paths.

If the test is blocked:

Do not go silent. Commit a blocked report to the required report path with the exact blocker, what was attempted, what evidence was captured, and what product change would unblock the next run.

Watcher rule:

After reporting, keep the 15-minute watcher armed on this same coordination branch unless the active directive explicitly says STOP.

Commit report and artifacts with `[skip ci]`.
