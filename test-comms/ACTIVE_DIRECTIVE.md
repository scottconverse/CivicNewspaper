# CivicNewspaper v0.3.2 Release Candidate - Exact Provenance Handoff

Tester: do not start a cleanroom run against the currently hosted different asset. This directive is the source of truth for the next release verification only after the hosted asset and checksum are replaced with the exact artifact below.

## Source Of Truth

This file is the active directive. Ignore older directive filenames and old report expectations.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Product build commit embedded in the locally verified Windows app: `e18bba5ffa890779954507b4d61d0965c3388552`
- Documentation/policy commit: `f65847a`
- Local installer filename: `The Civic Desk_0.3.2_x64-setup.exe`
- Hosted installer filename: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Required installer SHA256: `33C3AB662D59169E188FAE38B8F659364EAC3C6D0C5A82FC16D16D7C5073E9E0`
- Required installer size: `5281261` bytes
- Checksum asset: `SHA256SUMS.txt`
- App version: `0.3.2`
- Local model used for the passing artifact-bound proof: `phi4-mini:latest`

The current GitHub release serves a different signed asset from commit `796b8700831f964beea97630c51d71a40a9b724f`, SHA256 `BDA7CE85759AD1C475D100D0F04FBC7F3CAF7DFF07DDB74F60B24F1CAAF526DD`, size `5342976` bytes. Do not treat that hosted artifact as this candidate and do not produce another report against it.

## Local Proof Already Recorded

The local Windows installed-package proof passed with zero findings in the recorded run:

- Walkthrough receipt: `civicnewspaper/.agent-runs/packaged-e18bba5-phi4-retry/packaged-first-run-walkthrough-receipt.json`
- Walkthrough receipt SHA256: `6176004CE0B01B828BFE9E0FEA7125105587B2345D14DF62F8B0967E85AB732E`
- Core-flow receipt: `civicnewspaper/.agent-runs/packaged-e18bba5-phi4-retry/core-flow/core-flow-webview-result.json`
- Core-flow receipt SHA256: `6604FBB1730911D556827651FCD10CF1ED4EB8CD928F5DB6848D59F0FE34B243`
- Severity rollup for that installed-package run: blocker `0`, critical `0`, major `0`, minor `0`, nit `0`

The local proof covers silent install/uninstall, dependency-absent first run, accessible onboarding, persisted skip state, zero-source guidance, live-model Daily Scan, linked evidence, durable draft generation, Workbench reload, and database persistence. The broader local verification also passed 241 frontend tests across 27 files and 251 Rust tests.

## Required Hosted-State Gate Before Any Tester Run

Do not run the cleanroom procedure until all of these are true:

1. The release page asset `The.Civic.Desk_0.3.2_x64-setup.exe` hashes to `33C3AB662D59169E188FAE38B8F659364EAC3C6D0C5A82FC16D16D7C5073E9E0` and is `5281261` bytes.
2. `SHA256SUMS.txt` names that asset and contains the same SHA256.
3. The release body and public docs identify product commit `e18bba5ffa890779954507b4d61d0965c3388552`, the exact installer hash, and size.
4. The downloaded installer is from the GitHub release, never from `C:\Users\instynct`.

If any hosted-state gate fails, write a visibility report identifying the mismatch and stop. Do not substitute a local copy, an older release asset, or a different hash.

## Cleanroom Procedure After Hosted-State Gate Passes

Use the existing final release procedure from the latest applicable directive, with these non-negotiable checks:

1. Install only the downloaded GitHub asset in a fresh isolated app-data profile.
2. Complete Longmont, Colorado onboarding and run Daily Scan without manually repairing city/state in Settings.
3. Confirm at least one linked-evidence Story or Brief lead, Article Format default `Brief`, durable draft generation, reader-facing copy quality, editor hold/send-back/approve/cut behavior, static ZIP export, anonymous here.now publication, public visitor inspection, and final SQLite evidence.
4. Record PASS/FAIL and explicit blocker/critical/major/minor/nit findings. A final pass requires `0/0/0/0/0`.

## Required Reports

Write current-run reports on this branch under `test-comms/reports/` and attach screenshots, logs, downloaded release asset notes, SQLite/app-data evidence, exported ZIP path, and the public here.now URL under a matching `-evidence/` directory.

## Do Not Do

- Do not merge, tag, force-push, or publish product code.
- Do not run against the older hosted artifact listed above.
- Do not use an installer copied from the coder machine.
- Do not treat historical macOS/Linux artifacts as supported public-beta installers.
- Do not omit failures because they seem small.
- Do not use any `C:\Users\instynct` path as a tester input except to confirm that it is forbidden.

