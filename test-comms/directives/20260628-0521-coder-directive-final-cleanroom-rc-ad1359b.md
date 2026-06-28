# Cleanroom Tester Directive: Final Release-Candidate Gauntlet for ad1359b

Role: tester.
Coder branch under test: `stable-readiness-local-gates`.
Product commit under test: `ad1359b`.
Do not merge or tag. Do not use live external publishing credentials unless the directive explicitly says so; this one does not.

## Why This Directive Exists

The focused source-intake rerun for `ad1359b` passed with severity `0/0/0/0/0` in:

`test-comms/reports/20260628-0517-tester-report-source-intake-rerun-ad1359b.md`

Because `ad1359b` includes product fixes after the prior full cleanroom gauntlet on `513341b`, run one final whole-product release-candidate pass against the updated artifact.

## Artifact Under Test

Use the already-pushed artifact:

`test-comms/artifacts/ad1359b/The-Civic-Desk-0.2.8-ad1359b-windows-x64-cleanroom.zip`

Expected installer hashes:

| File | SHA256 |
|---|---|
| `The Civic Desk_0.2.8_x64-setup.exe` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` |

## Required Final RC Scope

Run this as an end-user cleanroom pass. Prefer a fresh app profile if practical; otherwise clearly state profile reuse and what was reset.

1. Install / first launch
   - Verify installer hash.
   - Install and launch from Start menu or installed app path.
   - Complete onboarding with realistic local-only values.
   - Confirm no missing bundled extension, blank screen, sidebar trap, or first-run dead end.

2. Whole-app nav and controls
   - Visit every sidebar item.
   - Click core buttons/links that do not transmit to external live providers.
   - Confirm no dead buttons, confusing unreachable editor paths, or broken modal exits.

3. Source intake smoke
   - Confirm source discovery UI opens and produces reviewable candidates or clear degraded messaging.
   - Bulk import one small fixture and confirm sources are added.
   - Confirm the previous source-intake fixes stay good at a glance.

4. Daily Scan / local model degraded mode
   - If Ollama/local model is unavailable, confirm the app clearly degrades without claiming the model is merely missing when the runtime is offline.
   - If a local model is available, run one scan and record result. Do not spend more than 20 minutes on model setup.

5. Workbench/editor path
   - Confirm a user can reach drafts/workbench from current state.
   - Confirm optional Press Freedom / legal-risk advisor remains advisory and never blocks editor decisions.
   - Confirm story approval still requires explicit editor attestation.

6. Publishing/output path
   - Compile/export a local static site or use existing safe local export path.
   - Verify homepage/article/RSS/share package basics if articles exist; if no approved story exists, verify the app explains what is needed without trapping the user.
   - Do not live-publish to external platforms.

7. Browser extension path
   - Confirm Browser Pairing can reveal/open the bundled extension folder or provide clear inline status.
   - If Chrome/Edge extension loading is practical, do a quick pairing smoke. If not practical, report why.

8. Narrow/mobile window check
   - Resize to a narrow width.
   - Confirm the app content remains reachable and sidebar does not trap the user.

## Report Back

Write:

`test-comms/reports/20260628-HHMM-tester-report-final-cleanroom-rc-ad1359b.md`

Include:

- Artifact/hash verification.
- Environment/profile reset notes.
- Result by each required final RC scope above.
- Severity counts: Blocker / Critical / Major / Minor / Nit.
- Explicit final line: `RC verdict: CLEAR` or `RC verdict: NOT CLEAR`.
- Screenshots/log paths if captured.

Pass target for public beta/stable-readiness branch: `0 Blocker / 0 Critical / 0 Major`. Minor/nit findings may be acceptable only if they do not block ordinary non-technical local publisher use and are clearly documented.
