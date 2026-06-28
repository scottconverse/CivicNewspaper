# Full E2E Longmont Publication Artifact Manifest - 792bd22

Directive: `test-comms/directives/20260628-rerun-full-e2e-after-windows-stack-fix-792bd22.md`

Result: blocked during app-managed local AI runtime install.

No publication ZIP was produced and no here.now URL was created because the installed app crashed when the tester clicked the app UI control to install the local AI runtime.

Crash summary:

- Product commit: `792bd22ac0513ab7a6457791e083fd68cbaef436`
- Application: `civicnews.exe`
- Version: `0.2.8.0`
- Event: `APPCRASH`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e82657`
- Window appeared: yes
- Onboarding appeared: yes
- Crash point: after clicking `Install local AI runtime` in AI Service Setup

Local output path: none

Output ZIP size: none

Output ZIP SHA256: none

here.now URL: none

Screenshots:

- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/01-first-run-identity.png`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/02-identity-longmont-next.png`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/03-ai-service-after-20s.png`
