# CivicNewspaper clean-machine gauntlet report - v0.3.0 b0be4d7

Status: PASS WITH FINDINGS

UTC run window: 2026-06-30T05:49Z to 2026-06-30T06:24Z

Machine/user: tester machine as `civic`

Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Coordination branch: `test-comms/cleanroom-coder-tester`

Coordination HEAD: `e4342ce test-comms: add v0.3.0 clean-machine gauntlet directive [skip ci]`

Directive: `test-comms/directives/20260629-gauntlet-all-cleanmachine-v030-b0be4d7.md`

Product commit under test: `b0be4d7432e9f5f791da68770a9631b8c5892697`

Evidence folder: `test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/evidence/`

## Verdict

The v0.3.0 clean-machine gauntlet passed the core end-to-end path:

- clean dependency-absent state was proven before launch
- NSIS installer hash matched and installed as a normal user
- app-guided Ollama/runtime/model setup worked without tester-installed prerequisites
- local AI generated content
- Longmont setup and source intake worked
- Daily Scan produced usable leads
- editor workflow controls were exercised
- the app compiled a static issue, exported a ZIP package, and published anonymously to here.now
- public output audit found no prohibited reporter scaffolding or mojibake markers in local text output, extracted ZIP output, or live here.now output

The run has findings:

- Only 1 story was published, below the target 5 to 10 stories/briefs. The source set produced 11 leads, but only one clearly publishable current story was approved during this run.
- The guardrail/advisor panel warned that an approved draft still contained an internal `editor_note:` marker before publishing. The final public output did not contain that marker, but the editor-side approved state was concerning.
- The cut workflow exposed `Cut Story` and a confirmation modal, but I did not complete the destructive confirm before publishing. Hold, Resume Editing, and Send Back for More Work were completed.

## Install Artifact

Preferred NSIS installer:

`test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/The Civic Desk_0.3.0_x64-setup.exe`

Expected SHA256:

`F3256C116F04B734C8C311E5B3EFEB69B24DAF3134C521C986BDF2C45CC1DF7E`

Observed SHA256:

`F3256C116F04B734C8C311E5B3EFEB69B24DAF3134C521C986BDF2C45CC1DF7E`

Fallback MSI SHA256 also matched:

`D294096A95FEBF55E0CB30D104ADD8B31BC27981F150BA8B70FEDFD547EC07E1`

Install method: NSIS

Install exit code: 0

Installed executable:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

## Clean Wipe

Clean wipe completed inside the directive boundary.

Removed:

- `C:\Users\civic\AppData\Local\The Civic Desk`
- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- `C:\Users\civic\.ollama`

Dependency-absent proof before launch:

- No The Civic Desk process remained.
- No Ollama process remained.
- `Get-Command ollama` found no Ollama executable on PATH.
- No local model files remained in checked Ollama/app model locations.
- Civic Desk app data was absent.

No OS reset or reimage was performed.

## First-Run AI Setup

The app installed and configured its own local AI stack without tester help.

Observed setup:

- hardware summary: `Local Ram: 15 GB`
- app message: `Starting the local AI service`
- app message: `Downloading the local AI runtime from Ollama. Installing...`
- model selected: `qwen2.5:7b (Recommended)`
- app message: one-time model download of about `~4.7 GB`
- model download showed progress and completed
- final header: `Local AI ready` / `qwen2.5:7b`

Manual tester installation of Ollama, models, runtimes, drivers, or prerequisites was not performed.

Usability note: the UI repeatedly displayed that the setup screen was not receiving input events and therefore continued automatically with a starter Longmont profile / recommended model download. The automatic recovery worked and showed useful progress, but that message is still a rough edge.

## Sources

The app added 6 starter Longmont sources:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit
- Longmont Colorado subreddit

This met the requirement for official/public-record sources plus public readable community/social sources. No private or credentialed services were used.

## Daily Scan and Leads

Daily Scan / Scrape & Detect produced:

- New leads: 11
- Drafts/in drafting: 1 during the run
- High priority: 1
- Sources: 6

Observed lead treatments included:

- Story
- Background
- Watch
- Editor review
- Decision / Vote
- Public Meeting Scheduled
- New Primary Record

The strongest current story lead was:

`Longmont City Council Meeting Agenda`

The app described this as a story because an upcoming city council meeting includes community projects and decisions such as a library roof contract. This was the one story I approved for publication.

## Editor Workflow

Completed:

- Opened generated draft in Story Workbench.
- Edited/inspected draft body.
- Ran press-freedom / legal-risk advisor.
- Guardrails ran and produced advisory warnings.
- Approved one story for static publish.
- Generated a second draft.
- Put the second draft on hold.
- Confirmed held draft exposed `Resume Editing`.
- Confirmed held draft exposed `Send Back for More Work`.
- Clicked `Send Back for More Work`; status changed to `Sent back / needs work`.
- Clicked `Resume Editing`; status returned to drafting.
- Exposed `Cut Story` and its confirmation modal.

Not fully completed:

- I did not complete the destructive `Cut story` confirmation before moving to Publishing. A later attempt occurred after the app was already on the Publishing screen, so no cut confirmation was available then.

Guardrail/advisor finding:

- The advisor/guardrail panel showed 4 advisory issues on the approved story.
- One warning said: `Draft still contains internal reporter-note marker(s): editor_note:. Remove or rewrite them as reader-facing copy before publishing.`
- Despite that editor-side warning, the final local/ZIP/live public output audit found 0 prohibited marker hits.

## Published Story

Published article count: 1

Published story:

- `Upcoming Longmont City Council Meeting Features Key Community Projects`
- Format: `watch`
- Path: `watch/1.html`
- Why it was a real news item: it came from a current Longmont City Council agenda lead involving public meeting/business items and community projects, including a library roof contract.
- Why only one story was published: the remaining leads were background/watch/editor-review items or needed more verification. I did not approve weak evergreen/background material as news merely to hit the numeric target.

## Output and Publish

Local output folder:

`C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`

Evidence copy:

`test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/evidence/publication-output/site/`

ZIP package:

`test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/evidence/publication-output/site/site-package.zip`

ZIP SHA256:

`9BE2D38879ABBC43A9C468858413ED94A5F918D68856B5F96DF0F130EE8ABBAA`

here.now URL:

`https://plucky-pebble-yfam.here.now`

Publish manifest:

- issue id: `issue-20260630-061703-796338400`
- provider: `here_now`
- article count: 1
- files written: 18
- skipped count: 0

## Public Output Audit

Prohibited scaffolding markers checked:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `editor_note:`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`

Mojibake code points checked:

- U+00C2
- U+00C3
- U+00E2
- U+FFFD

Results:

- Local text output hits: 0
- Extracted ZIP text output hits: 0
- Live here.now hits: 0

Note: the binary ZIP file itself was excluded from text-marker scanning after an initial false positive on compressed bytes. Its extracted text content was audited separately and clean.

## Findings

Critical: none in final public output.

Major:

- Only 1 article was published versus the 5 to 10 target. The app found 11 leads, but the run produced/approved one genuinely current story. This may be acceptable for the live source set, but it misses the target output volume.
- Guardrails detected `editor_note:` in an approved draft before publication. Public output was clean, but an approved editor state containing an internal-marker warning should be treated as a serious editorial safety finding.

Minor:

- Setup input events were not received by the app, so it used automatic fallback behavior. The fallback worked, but the message may worry a normal user.
- Mark Ready for Review was not clickable in my advisor pass because the story was already in an approved state by the time I attempted it.
- Cut workflow was exposed, but destructive confirmation was not completed in this run.

Polish:

- Publishing navigation was slightly hard to drive by automation; clicks sometimes landed on Ethics & Backups/Publishing adjacent navigation before settling. Manual users may not see this, but the nav hit areas deserve a look.

## Evidence Index

Selected evidence files:

- `00-context-and-hashes.json`
- `02-clean-wipe-and-absent-proof.json`
- `03-install-result.json`
- `04-launch-result.json`
- `05-first-run-launch.png`
- `06-ai-setup-progress-*.png`
- `08-model-download-progress-*.png`
- `09-model-download-final.png`
- `daily-scan-states.json`
- `14-workbench-field-values.json`
- `editor-control-workflow-result.json`
- `publishing-workflow-result.json`
- `35-output-files-found.json`
- `36-output-quality-audit.json`
- `37-publication-summary.json`
- `publication-output/site/`
- `zip-extract-check/`
- `live-herenow-index.html`

## Repro Notes for Findings

Guardrail/editor-note warning:

1. Clean install and complete Longmont setup.
2. Run source intake and Daily Scan / Scrape & Detect.
3. Open the generated draft for the high-priority council agenda story.
4. Run the press-freedom / legal-risk advisor.
5. Observe advisory warning: `Draft still contains internal reporter-note marker(s): editor_note:.`
6. Approve for static publish.
7. Compile/publish.
8. Public output audit is clean, but editor-side approved state had the warning.

Low story count:

1. Run the same clean-machine gauntlet.
2. Observe 11 leads.
3. Only one generated/approved story reached publishable status during the run.
4. Remaining leads were background/watch/editor-review or needed more verification.
