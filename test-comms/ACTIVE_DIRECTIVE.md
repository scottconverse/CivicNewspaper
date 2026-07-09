# Final Cleanroom Release Verification - The Civic Desk v0.3.2 ba49af4 Publish Flow Rerun

Tester: run this from `msi\civic` on the cleanroom Windows tester machine.

## Source Of Truth

This file is the active directive. Ignore older directive filenames.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Product build commit embedded in the Windows app build: `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`
- Release/docs commit on `main`: `2cb62b8262a04111d00b1b4e1d0ebd9b4a78eeb1`
- Windows installer asset: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Windows installer size: `5250809` bytes
- Windows installer SHA256: `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`
- Checksum asset: `SHA256SUMS.txt`

Do not use any installer copied from `C:\Users\instynct`. Download the installer and checksum file from the GitHub release URL above.

## Required Reports

Write these reports on this branch:

- Visibility: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-visibility.md`
- Full report: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-report.md`

Attach screenshots, logs, exported ZIP path notes, downloaded release asset notes, SQLite/app-data evidence, and publication URLs under:

- `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/`

## Why This Rerun Exists

The `17766b7ccb0cc744522090e28997b764676ce1c5` rerun fixed durable draft persistence: a ready Brief generated and saved a draft. It still failed final cleanroom because the editor could not finish the core release path:

- Static publish approval hard-blocked a generated linked-evidence Brief on heuristic topic/attribution warnings with no editor override.
- `Improve for Publication` could reject the app's own `CO` workspace context when a model wrote `Colorado`.
- The release body advertised stale commit/hash/size provenance.
- The UI promised an unreachable Manual Mode for offline AI drafting.
- The Story Queue button label `Verify first` opened the draft wizard without any verification step.

Coder fixes in `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`:

- Static package-integrity problems still block approval, but topic/quality heuristics route through logged editor confirmation instead of vetoing the editor.
- `CO` and `Colorado` are treated as the same supported jurisdiction during Improve for Publication checks.
- Offline AI copy no longer promises Manual Mode; draft generation, improvement, and social copy plainly require a reachable local model.
- Cautious Story Queue actions are labeled `Review`, not `Verify first`.
- The dead `generate_draft` command/export was removed; only durable `generate_and_save_draft` remains.
- Source-backed Brief classifiers now use generalized current-action/public-impact checks instead of fixture-shaped result-count/service keywords.
- Release body, checksum asset, README, install guide, manual, landing page, release-readiness page, and release-evidence JSON were updated to the `ba49af4` installer SHA/size; the landing page now marks the rebuilt Windows candidate as awaiting final cleanroom recheck.
- The release docs consistency gate now checks the live GitHub release body for matching commit/hash/size.

## Visibility Check

Before installing, write the visibility report confirming:

1. You are on machine/user `msi\civic`.
2. Your repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
3. Branch is `test-comms/cleanroom-coder-tester`.
4. You read `test-comms/ACTIVE_DIRECTIVE.md`.
5. You can reach the GitHub release URL and public docs URL.
6. The release page shows installer SHA256 `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`, size `5250809`, and product commit `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`.
7. The release page has exactly one Windows installer asset and one checksum asset.
8. `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains SHA256 `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`.
9. The public docs URL shows installer SHA256 `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`, size `5250809`, `More info`, `Run anyway`, Windows-only beta language, and `Rebuilt Windows candidate awaiting final cleanroom recheck`, with no stale `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`, `8204BB4210DD284518D114C57A3089BAC11D7B0EC8E0F83D8D61928D44FEB6E0`, or `E7B620C4D51837DDD43028B511E396643EE9A67D1CD23DC0B59BC5442277DCD7` hash.

## Cleanroom Test Procedure

1. Pull this branch and verify this ACTIVE_DIRECTIVE is current.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release page.
3. Verify the installer SHA256 exactly matches `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`.
4. Verify `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
5. Confirm the release page and public docs explain that the Windows installer is unsigned, why SmartScreen may warn, and that the expected install path is `More info` then `Run anyway` when the hash matches.
6. Uninstall any prior The Civic Desk app instance so the installer lifecycle is tested. Do not delete unrelated user data.
7. Record whether these state paths exist before this test: `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%APPDATA%\The Civic Desk`, `%LOCALAPPDATA%\The Civic Desk`, `%APPDATA%\org.civicnews.app`, and `%LOCALAPPDATA%\org.civicnews.app`.
8. Install from the downloaded GitHub release installer.
9. Create and use a fresh isolated app-data folder for this cleanroom run:

```powershell
$cleanProfile = Join-Path $env:TEMP "civicdesk-final-v032-ba49af4-publish-flow"
Remove-Item -LiteralPath $cleanProfile -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $cleanProfile -Force | Out-Null
$env:CIVICNEWS_APP_DATA_DIR = $cleanProfile
& "$env:LOCALAPPDATA\The Civic Desk\civicnews.exe"
```

10. Launch only the installed EXE from the GitHub release installer. Do not launch from source.
11. Verify first-run onboarding appears, not inherited Longmont state.
12. Complete first-run setup for Longmont, Colorado using the app-guided local AI setup. If direct field automation is unreliable, use the visible `Longmont` starter profile.
13. After onboarding, go directly to Daily Scan and run it without manually repairing city/state in Settings.
14. Capture saved profile/settings/SQLite evidence showing city `Longmont` and state `CO`.
15. Run source discovery/import twice if practical: once using `Longmont` / `Colorado`, and once using `Longmont` / `CO`.
16. Run Daily Scan and inspect leads. Confirm unsupported/no-source/model-suggested/navigation/index items are downgraded or verification work, not strong ready-to-draft story work.
17. Confirm leads with no linked `lead_evidence` do not show a strong Draft path and do not show `Draft anyway`.
18. Confirm cautious/background/watch/verification leads use honest `Review` labeling, not `Verify first`.
19. Find at least one credible Longmont lead with linked evidence and Story or Brief treatment. Confirm Article Format defaults to Brief for Story/Brief leads.
20. Draft at least one credible Longmont story or brief from linked source evidence.
21. Confirm generated public copy has a real headline, coherent reader-facing body, no watch-fragment phrasing for Brief output, no reporter scaffolding, no `EDITOR_NOTE`, no `Body:`, no `Headline:`, no `Nut graf`, no `[Source needed]`, no `[Verification needed]`, no `[End of Report]`, no mojibake marker code points, no encoded HTML entity leakage, and no unsupported facts.
22. Exercise editor workflow: send back, hold, restore/resume where available, improve if local AI is ready, approve, cut if available.
23. Specifically retest the prior blocker: after generating a linked-evidence Brief, check `I reviewed this story`, click `Approve`, and confirm heuristic topic/quality warnings can be resolved through the logged editor confirmation instead of blocking static publish approval.
24. Compile/export the static site and record the exported ZIP path.
25. Publish through the default here.now anonymous preview flow. This anonymous here.now preview is authorized for this release cleanroom test. Do not use credentialed external providers.
26. Record the public here.now URL.
27. Inspect the public here.now site as a visitor. Confirm there are no duplicate topic stories, no raw Markdown, no internal tester/developer paths, no unsupported Mac/Linux installer claims, and no broken story links.
28. Inspect the public docs/landing page from https://scottconverse.github.io/CivicNewspaper/ and the release page. Confirm v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, discussion links, release download links, installer SHA256, size, and commit are all present and honest.
29. Write a full human-readable report with PASS/FAIL and explicit blocker/critical/major/minor/nit findings. The release only passes if there are zero blocker, critical, major, minor, and nit findings.

## Do Not Do

- Do not merge, tag, or publish product code.
- Do not use an installer copied from the coder machine.
- Do not treat historical Mac/Linux artifacts as release-supported.
- Do not omit failures because they seem small.
- Do not use any `C:\Users\instynct` path except to confirm that it is forbidden as a tester input.
