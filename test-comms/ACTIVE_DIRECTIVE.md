# CivicNewspaper v0.3.2 Release Candidate - Exact Provenance Handoff

Tester: do not start a cleanroom run against the currently hosted different asset. This directive is the source of truth for the next release verification only after the hosted asset and checksum are replaced with the exact artifact below.

## Source Of Truth

This file is the active directive. Ignore older directive filenames and old report expectations.

## Product/Release Under Test

- Repository: `scottconverse/CivicNewspaper`
- Release URL: https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2
- Public docs URL: https://scottconverse.github.io/CivicNewspaper/
- Product build commit embedded in the locally verified Windows app: `5b454e308b48663252ef47745d4304a4eca28b54`
- Documentation/policy commit: `f7194e4`
- Local installer filename: `The Civic Desk_0.3.2_x64-setup.exe`
- Hosted installer filename: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Required installer SHA256: `0AC1A7C7A5D431C4E99CB03ADE08A58A6ED495E4A8CCEEBD17F86D5EDC8619A5`
- Required installer size: `5279313` bytes
- Checksum asset: `SHA256SUMS.txt`
- App version: `0.3.2`
- Local model used for the passing artifact-bound proof: `phi4-mini:latest`

The current GitHub release serves a different signed asset from commit `796b8700831f964beea97630c51d71a40a9b724f`, SHA256 `BDA7CE85759AD1C475D100D0F04FBC7F3CAF7DFF07DDB74F60B24F1CAAF526DD`, size `5342976` bytes. Do not treat that hosted artifact as this candidate and do not produce another report against it.

## Local Proof Already Recorded

The local Windows installed-package proof passed with zero findings in the recorded runs:

- Broad release-smoke receipt: `civicnewspaper/.agent-runs/release-smoke-20261117-084251/release-smoke-receipt.json`
- Broad release-smoke receipt SHA256: `342AB2DFAAA0F88DAE8C35D892D4AA5F382216EBEFC46A201F570F1786D80BB8`
- Packaged walkthrough receipt: `civicnewspaper/.agent-runs/release-smoke-20261117-084251/packaged-walkthrough/packaged-first-run-walkthrough-receipt.json`
- Packaged walkthrough receipt SHA256: `AF417E8700A0EE691AF38AB6BBB22BA24EEBA44BC2EA13850DD83C387112E17D`
- Core-flow receipt: `civicnewspaper/.agent-runs/release-smoke-20261117-084251/packaged-walkthrough/core-flow/core-flow-webview-result.json`
- Core-flow receipt SHA256: `0B66B08942230C8CE4143E73D543BF4439B994D48E39FB0F1CAFCD31FCCC8D34`
- Installer lifecycle receipt: `civicnewspaper/.agent-runs/windows-installer-smoke-5b454e3/windows-installer-smoke-receipt.json`
- Installer lifecycle receipt SHA256: `50638A43BE22A671C0629C3316F0CED75307656BC34F1B1C9AD9B05356213578`
- Severity rollup for that installed-package run: blocker `0`, critical `0`, major `0`, minor `0`, nit `0`

The local proof covers silent install/uninstall, dependency-absent first run, accessible onboarding, persisted skip state, zero-source guidance, live-model Daily Scan, linked evidence, durable draft generation, Workbench reload, and database persistence. The packaged proof uses a controlled source-backed fixture for deterministic evidence linkage; separate live municipal-source and live local-model checks also passed. The broader local verification passed 241 frontend tests across 27 files and 251 Rust tests.

## Required Hosted-State Gate Before Any Tester Run

Do not run the cleanroom procedure until all of these are true:

1. The release page asset `The.Civic.Desk_0.3.2_x64-setup.exe` hashes to `0AC1A7C7A5D431C4E99CB03ADE08A58A6ED495E4A8CCEEBD17F86D5EDC8619A5` and is `5279313` bytes.
2. `SHA256SUMS.txt` names that asset and contains the same SHA256.
3. The release body and public docs identify product commit `5b454e308b48663252ef47745d4304a4eca28b54`, the exact installer hash, and size.
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

