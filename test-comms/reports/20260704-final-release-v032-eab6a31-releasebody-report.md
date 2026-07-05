# Tester Report - v0.3.2 eab6a31 Release Body Rerun

Date: 2026-07-05T01:23:41Z
Tester machine: `msi\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product release: `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2`
Release/docs target: `f0cb4a96183da91f262ec15c8035a03d1da78250`
Product build commit per directive: `eab6a31e0bfb1463bcb8f0f26d8909adc4d77d8c`
Directive: `test-comms/ACTIVE_DIRECTIVE.md`
Result: FAIL

## Environment

- Windows/user: `msi\civic`
- Repo branch: `test-comms/cleanroom-coder-tester`
- Coordination commit pulled: `af3d166`
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Local AI visible in app: `Local AI ready`, `phi4-mini:latest`

## Steps Run

1. Pulled `origin/test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and recent files in `test-comms/directives/`.
3. Reached the GitHub release URL and public docs URL.
4. Verified release page HTML contains SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766` and size `5227476`.
5. Verified release page HTML did not contain `$hash` or `$sha` placeholders.
6. Verified GitHub release API listed exactly one Windows installer and one checksum asset.
7. Downloaded `SHA256SUMS.txt`; it names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.
8. Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` from the GitHub release; verified size `5227476` and SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.
9. Ran existing uninstall from `C:\Users\civic\AppData\Local\The Civic Desk\uninstall.exe /S`.
10. Removed prior app-data/profile paths that existed under this tester profile, then installed the downloaded GitHub release installer with `/S`.
11. Launched the installed Windows app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
12. Inspected Story Queue and Workbench in the installed app using Windows app automation.

## Results

- Visibility gate: PASS.
- Installer download and SHA256 verification: PASS.
- Installed Windows app launch: PASS.
- First visible app state after reinstall: FAIL for clean first-run proof. The app opened directly into the Longmont newsroom with existing scan/draft state rather than city onboarding.
- Daily Scan / Story Queue quality gate: FAIL.
- Workbench draft quality gate: FAIL.
- Static export, here.now publish, and public visitor inspection: NOT RUN because major release-quality failures were found before approval/publish.

## Evidence

- Visibility report: `test-comms/reports/20260704-final-release-v032-eab6a31-releasebody-visibility.md`
- Checksum evidence: `test-comms/reports/20260704-final-release-v032-eab6a31-releasebody-evidence/SHA256SUMS.txt`
- Installer receipt: downloaded from GitHub release during this pass, verified size `5227476`, SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`, then removed before commit to avoid storing the public 5 MB installer binary in the comms branch.
- UI evidence excerpts are included below from the installed Windows app accessibility text.

Note: The directive names `test-comms/evidence/20260704-final-release-v032-eab6a31-releasebody/`, but this heartbeat explicitly constrains tester writes and evidence to `test-comms/reports/`, so evidence receipts for this pass are kept under the report folder.

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 3
- Minor: 1
- Nit: 0

### Major 1 - Encoded HTML entity and broad calendar rollup are still ready/open-draft material

Observed: Story Queue contained a lead marked `Ready to draft` with `Open draft` whose text still included encoded `&#8211;` and a broad multi-item calendar concatenation:

`Longmont official city website: Independence Weekend Free Concert Friday, July 3 • 6 pm &#8211; 10 pm ... LIBRARY CLOSED ... Free Fitness in the Park ... July 4th Longmont Symphony Concert ... Independence Weekend Festival ...`

Expected: The directive specifically requires malformed leads like encoded `&#8211;`, mojibake bullets, multi-event calendar concatenations, generic pages, and markup debris to be suppressed, downgraded, or blocked from draftable strong-news treatment.

Impact: This is the exact category the eab6a31 rerun was meant to repair, and it remains available as a draft/open-draft item.

### Major 2 - Workbench draft/source panel still leaks encoded entity and copied event-listing content

Observed: Opening the existing draft in Workbench showed reader-facing/editor-visible content retaining encoded entity text and copied calendar listings:

`Independence Weekend Free Concert`

`Friday, July 3 • 6 pm &#8211; 10 pm`

`LIBRARY CLOSED`

`Free Fitness in the Park`

`July 4th Longmont Symphony Concert`

`Independence Weekend Festival`

Several event entries repeated in the same panel.

Expected: Draft output and workbench/public-copy surfaces should have no encoded HTML entity leakage and should not copy broad multi-item event listings into story body/source-copy context.

Impact: The app still exposes unpublishable copy/scaffold in the workflow. I did not approve, export, or publish this story.

### Major 3 - Daily Scan still exposes weak or markup-debris signals instead of suppressing them

Observed: The queue still included weak community-signal rows such as:

`Review community signal from Longmont city events: -->`

and:

`Review community signal from Longmont city events: Discover all the departments that make up the City of Longmont&#8217;s government...`

They were labeled `Needs verification`, which is better than ready-to-publish, but the directive asks that markup-debris, navigation/index, generic, and unsupported items be clearly labeled, suppressed, or downgraded. These remain visible as scan output and one entity-leaking multi-item event lead remained `Ready to draft`.

Expected: Markup debris and generic/navigation pages should not remain in the actionable scan queue in a way that can feed drafting.

Impact: Editors still have to manually separate real leads from debris, and malformed source text can still flow into draft workflows.

### Minor 1 - Clean reinstall did not produce true first-run onboarding state

Observed: After uninstalling and removing known app-state paths, the installed app launched directly into `LONGMONT / CO` newsroom with 24 leads and one existing draft. The directive asks for first-run setup for Longmont, Colorado. I could verify installed app launch and AI-ready state, but not natural first-run onboarding from a blank profile.

Expected: Clean reinstall/removal should produce app-guided first-run setup or the report should identify the exact persistence location that survived cleanup.

Impact: First-run setup proof remains incomplete for this pass.

## Not Run

- Approving a draft.
- Static export ZIP creation.
- here.now publish.
- Public here.now visitor inspection.

These were not run because the release already had major failures in the exact Daily Scan/draft quality areas required by the active directive. I also did not automate the Windows Security/Defender prompt that appeared over the app asking to review/send files to Microsoft; the app became usable without touching that prompt.

## Request For Coder

Please rework the scan-to-draft gates so encoded HTML entities and broad multi-event calendar/listing content cannot appear as `Ready to draft`, `Open draft`, or Workbench body/source content. Also clarify or harden the clean-profile reset location so the tester can prove natural first-run onboarding rather than inheriting Longmont newsroom state.
