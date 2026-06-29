# Tester Report - Full E2E Current 5c6f141 Directive Blocked

Date: 2026-06-29T02:05Z
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 5c6f141c87175de187f89a887d4f91f08a73da2d observed at `origin/stable-readiness-local-gates`
Directive: test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: not rechecked for this blocked directive
- Node: not used
- Rust: not used
- npm: not used
- Ollama installed/running: not evaluated for this blocked directive
- Models present: not evaluated for this blocked directive

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` after the `e2ac517` report push race.
2. Rebased and pushed the completed `e2ac517` tester report as commit `05c802f`.
3. Reread `test-comms/ACTIVE_DIRECTIVE.md`.
4. Read `test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md`.
5. Checked `origin/stable-readiness-local-gates`, which points to `5c6f141c87175de187f89a887d4f91f08a73da2d`.
6. Listed `test-comms/artifacts/20260628-full-e2e-current-5c6f141/` and computed observed hashes.

## Results

Blocked before install. The active directive and archived directive contain unexpanded placeholders in required fields, so I cannot verify the product artifact exactly as directed.

Observed problems:

- `test-comms/ACTIVE_DIRECTIVE.md` says to run `$directiveRel`, product commit `$commit`, artifact folder `$artifactRel/`.
- The archived directive also says required product commit is `$commit`.
- Preferred installer path is `$artifactRel/The Civic Desk_0.2.8_x64-setup.exe`.
- Expected NSIS hash is `$nsisHash`.
- Expected MSI hash is `$msiHash`.
- Several paths render as `	est-comms/...` instead of `test-comms/...`.

Concrete files found anyway:

- `test-comms/artifacts/20260628-full-e2e-current-5c6f141/The Civic Desk_0.2.8_x64-setup.exe`
- `test-comms/artifacts/20260628-full-e2e-current-5c6f141/The Civic Desk_0.2.8_x64_en-US.msi`

Observed hashes:

- NSIS: `CF901350E6CA13A109FF1DFBFB3FF733B149CA53AB2D7D73014C2B5F8CCA86B7`
- MSI: `7ADA24DE59243CCF60D39601039AFAB5497D5715B15085EF7C78B04B49311FFA`

I did not install or run this build because the directive explicitly requires hash verification before install and says to stop if hashes mismatch. Placeholder expected hashes cannot be matched.

## Evidence

No new screenshots were needed for this directive-level blocker.

Relevant command evidence:

- `git ls-remote origin refs/heads/stable-readiness-local-gates` returned `5c6f141c87175de187f89a887d4f91f08a73da2d`.
- `Get-FileHash` returned the observed NSIS/MSI hashes listed above.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Active directive is templated and cannot be run exactly

Observed: The new active directive and archived directive contain literal placeholders for directive path, commit, artifact path, and expected hashes.

Expected: The directive should name the exact active directive path, product commit, artifact folder, NSIS hash, MSI hash, and valid `test-comms/...` report/artifact paths.

Impact: Tester cannot safely install or validate the new build under the stated protocol because exact artifact verification is impossible.

Repro:

1. Pull `test-comms/cleanroom-coder-tester` at/after commit `4bd5c7a`.
2. Open `test-comms/ACTIVE_DIRECTIVE.md`.
3. Open `test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md`.
4. Observe `$directiveRel`, `$commit`, `$artifactRel`, `$nsisHash`, and `$msiHash` placeholders.

## Request For Coder

Please republish the directive with expanded concrete values for directive path, product commit, artifact folder, expected NSIS/MSI hashes, and valid `test-comms/...` paths. The artifact folder already appears to contain installers for `5c6f141`; tester needs the expected hashes in the directive before installing.
