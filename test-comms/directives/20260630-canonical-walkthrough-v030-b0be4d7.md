# Canonical GauntletGate Walkthrough Directive - CivicNewspaper v0.3.0

Role: tester
Counterpart: coder
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Active directive file: test-comms/ACTIVE_DIRECTIVE.md
Product branch: main
Product commit: b0be4d7432e9f5f791da68770a9631b8c5892697
Lane: Walkthrough only

## Hard Rule

This is the Walkthrough lane of GauntletGate all.

Do not run or report Full. Do not do a five-role audit. Do not treat this as a final gate verdict.

Your job is to produce the Walkthrough report and evidence artifacts. The coder will not start Full until this Walkthrough report exists and has evidence-backed first-run attestation.

## Tester Machine Path

Approved coordination path on tester:

```text
C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
```

Do not use `C:\Users\instynct` on the tester. That path is from the coder machine and is invalid on the tester.

## Artifacts To Test

Use one of these artifacts from the coordination branch:

```text
test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/The Civic Desk_0.3.0_x64-setup.exe
test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/The Civic Desk_0.3.0_x64_en-US.msi
```

Expected hashes:

```text
The Civic Desk_0.3.0_x64-setup.exe
SHA256 6C28D0ACEDAA1A367CA8F2EBFFDCB60B2AFC002F123442D1C7FF84EFD1CC95E4

The Civic Desk_0.3.0_x64_en-US.msi
SHA256 AA510FA91B519883190638CBEDB584648B148731DB842371ECB8671D6D7CA154
```

If hashes do not match, stop and report a Walkthrough Blocker.

## Clean State Requirements

Before installing, reset the tester machine to a product-clean state:

- Remove The Civic Desk / CivicNewspaper installation.
- Remove The Civic Desk app data for the tester user.
- Remove CivicNewspaper test output folders from prior runs.
- Remove Ollama runtime and local models from prior test runs.
- Remove related PATH changes made for prior CivicNewspaper/Ollama tests.
- Leave Windows, the tester user account, browser, Git, and Codex itself intact.

The tester may perform this cleanup as test setup. The tester must not manually install missing product dependencies after the product starts. If the product needs Ollama, models, parsers, or runtime pieces, the app or installer must guide that path.

## Walkthrough Required Steps

Follow the GauntletGate Walkthrough lane exactly.

### 1. Build Product Model

From the installed product and available docs, identify:

- primary workflow
- secondary workflows
- first-run gates
- external dependencies
- empty-data states
- what happens when Ollama/model is absent
- what happens when sources are absent
- what happens when network is unavailable or a source fetch fails
- what a brand-new user must do to reach the core feature

### 2. Verify Environment Attestation

You must produce verified facts, not claims.

Record:

- Windows version and hardware summary.
- Exact installed artifact and hash.
- Exact app-data path used by The Civic Desk.
- Proof the app wrote config/database/first-run files to that path.
- Proof first-run flags were unset before first launch.
- Proof data store was empty before first launch.
- Proof Ollama was absent before first launch.
- Proof local models were absent before first launch.
- Network state.

Every attestation cell must point to an evidence artifact: screenshot, log, file listing, command output, DB/file path capture, trace, or exported diagnostic.

### 3. Zero-State And Dependency-Absent Pass

Start the installed app as a normal user with:

- clean app data
- no sources
- no drafts
- no publish history
- no Ollama runtime
- no local model

Walk onboarding and try to reach the core feature.

Required observations:

- Does the app explain unsigned installer status clearly enough for beta users?
- Does first-run identity setup work?
- Is organization type selectable and saved?
- Does the app detect machine hardware?
- Does the app recommend a model?
- With Ollama absent, does the app guide install/setup or dead-end?
- Does it show progress for runtime/model setup?
- If setup is skipped or unavailable, does degraded mode remain clear?
- With no sources, does Daily Scan route to source setup instead of running empty?

### 4. Provisioning Matrix

Cover these cells and state exactly which were covered:

```text
first-run + dependency absent + empty data + online
first-run + dependency absent + empty data + offline or source-failure simulation
returning user + dependency present + populated data + online
returning user + dependency absent/stopped + populated data + online
```

If a cell cannot be covered, say why and mark it as not verified.

### 5. UI Walk

Click every meaningful visible control across:

- Onboarding
- Daily Scan
- Story Queue
- Dark Signals
- Verification
- Workbench
- Sources
- Publishing
- Browser Pairing
- Settings
- System Status
- AI Model

Check:

- empty states
- loading states
- errors
- success messages
- disabled states
- repeated submits
- stale navigation
- back/forward/refresh where possible
- desktop and narrow/mobile layout
- copy that points to the wrong screen
- controls that look wired but do nothing

### 6. Core Feature Reachability

A brand-new user must be able to reach the core feature by following the in-product path.

For this app, core feature means:

- configure publication identity
- add or discover sources
- run a scan or source fetch
- create at least one lead or draft
- review/edit in Workbench
- approve at least one publishable item
- compile/export a static issue
- export ZIP or output folder

If here.now anonymous publish is available from the app, publish and include the URL. If it fails, report exact error and evidence. This publish is authorized for this test.

### 7. Evidence Output

Write final report to:

```text
test-comms/reports/20260630-canonical-walkthrough-v030-b0be4d7-report.md
```

Write evidence artifacts under:

```text
test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/evidence/
```

Include:

- screenshots
- logs
- hash output
- app-data path proof
- first-run/app-data file listing
- Ollama absence proof before launch
- model absence proof before launch
- install notes
- exported diagnostics if available
- output ZIP/path if produced
- here.now URL if produced
- any public output URL fetch proof

## Report Format

The report must include:

- Verdict for Walkthrough only
- First-run verdict: reaches core feature / dead-ends / not verified
- Environment attestation table
- Evidence artifact list
- Provisioning matrix table
- Readiness-by-area table
- Numbered findings with severity, evidence, cause, fix, and suggested test
- What worked
- What could not be verified

Use severity exactly:

- Blocker
- Critical
- Major
- Minor
- Nit

If clean first-run state cannot be verified, first-run coverage is INVALID.

If the new user cannot reach the core feature, report a Blocker.

## After Report

Commit the report and evidence to the coordination branch with `[skip ci]`.

Do not merge.
Do not tag.
Do not edit product source.
Do not run Full.

