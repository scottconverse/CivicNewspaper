# CivicNewspaper Cleanroom E2E Attempt 8 Report

Verdict: FAIL

UTC report time: 2026-06-30T15:10:00Z

Directive: test-comms/directives/20260630-cleanroom-e2e-c3247ba-attempt8.md

Coordination branch: test-comms/cleanroom-coder-tester

Coordination HEAD at test time: dac6e90 test-comms: visibility for cleanroom e2e c3247ba [skip ci]

Tester identity: msi\civic

Coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Product commit under test: c3247bab7c20129e99d8beb8515b124a2e49248f

Product version: 0.3.0

## Installer And Cleanroom Setup

Installer used: NSIS

Installer path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\artifacts\20260630-cleanroom-e2e-c3247ba\The Civic Desk_0.3.0_x64-setup.exe`

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

Installer exit code: 0

Installer hash verification:

- NSIS expected SHA256: 6801E4C41B081B55045646102DBFA6EE3CD2360AB0827BBFBCC5753D6FF861A8
- NSIS actual SHA256: 6801E4C41B081B55045646102DBFA6EE3CD2360AB0827BBFBCC5753D6FF861A8
- NSIS expected size: 5621164
- NSIS actual size: 5621164
- MSI expected SHA256: F2F3B35C92143DDF1C30B39FB6DDE1546E00A795BF4A9CE5B3925AD620EDF9F6
- MSI actual SHA256: F2F3B35C92143DDF1C30B39FB6DDE1546E00A795BF4A9CE5B3925AD620EDF9F6
- MSI expected size: 9129984
- MSI actual size: 9129984

Cleanroom reset evidence:

- Removed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- Removed `C:\Users\civic\AppData\Local\The Civic Desk`
- Removed `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- Removed `C:\Users\civic\.ollama`
- `C:\Users\civic\AppData\Local\civicnews` was absent
- `C:\Users\civic\.civicnewspaper` was absent

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-c3247ba/00-clean-wipe-summary.json`
- `test-comms/evidence/20260630-cleanroom-e2e-c3247ba/01-install-launch-summary.json`

## Result

The installed app launched, onboarding began, and identity entry proceeded far enough to reach app-guided AI setup. The run then failed during the product's own local AI runtime setup.

The app displayed:

`Initialization Error: Local AI runtime install failed: The Civic Desk didn't have permission to complete this. Check the file or folder permissions and try again.`

The same screen also showed:

`Couldn't reach the AI service`

and:

`The private AI service did not start. First try restarting Civic Desk. If Windows or antivirus asked about this app, allow it, then retry. If it still fails, save a diagnostics file for support.`

The directive says:

`If the app installer or first-run flow cannot set up a required dependency, report that as product failure. Do not repair it manually unless this directive explicitly says to.`

Because of that instruction, I did not manually install Ollama, models, product dependencies, or otherwise repair the local AI runtime outside the app.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-c3247ba/12-runtime-install-failed-current.json`
- `test-comms/evidence/20260630-cleanroom-e2e-c3247ba/12-runtime-install-failed-current.png`

## Final Report Requirements

- App-guided AI/runtime/model setup worked without tester manual installation: No. It failed at AI Setup Step 2 with a local AI runtime permission error.
- Identity displayed cleanly as Longmont / CO: Not reached to final app shell because setup failed during AI runtime installation.
- First-run starter sources seeded without manual import, including local media count and total count: Not reached.
- Daily Scan outcome: Not reached.
- Lead count and story/brief count: Not reached.
- Editorial workflow outcomes: Not reached.
- Warned ready_to_review approval path: Not reached.
- Final database statuses for approved, held, sent-back, and cut drafts: Not reached.
- Weak or generic items held, cut, sent back, or approved after warning: Not reached.
- Local static output path: Not produced.
- ZIP output path: Not produced.
- here.now URL: Not produced.
- Manifest URL, UI URL, and whether they agree: Not produced.
- Output quality scan results: Not run because no issue was generated or published.
- Public taxonomy/path results for formerly internal watch-format drafts: Not run because no issue was generated or published.

## Blocker

Exact blocker: the product failed to complete app-guided local AI runtime setup on a cleanroom install. The tester could not proceed to Daily Scan, source verification, drafting, approval, static export, ZIP export, here.now publishing, or public-output quality checks without violating the directive's explicit instruction not to manually repair runtime/model setup.

What the user could not do next: continue normal first-run setup into a working Longmont newsroom without manually fixing the AI runtime outside the product.
