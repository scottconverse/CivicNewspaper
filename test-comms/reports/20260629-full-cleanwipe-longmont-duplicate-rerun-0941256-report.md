# Full Clean-Wipe Longmont Duplicate-Lead Rerun Report - 0941256

Status: PASS

Directive: `test-comms/directives/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256.md`

Product branch: `stable-readiness-local-gates`

Product commit: `09412560a326379fcf75f327439df8d1d2bb47b4`

Evidence folder: `test-comms/reports/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-evidence/`

here.now URL:

`https://dusky-bloom-9nx3.here.now`

## Summary

The full clean-wipe Longmont rerun passed. The app installed from a wiped CivicNewspaper/Ollama/model state, completed normal first-run setup, selected local model `qwen2.5:7b`, discovered official and public community/social sources, produced 15 leads, generated and approved 5 reader-facing pieces, compiled 22 files, exported a ZIP, and published anonymously to here.now.

The public here.now URL returned HTTP 200, rendered `Longmont Duplicate Lead Retest`, and the final issue contained 5 articles. The duplicate-topic audit found no candidate duplicate story topics and the Building Services permitting-portal duplicate did not recur. Mojibake and public `Draft:` title-prefix checks passed.

## Installer Hashes

Preferred NSIS installer:

`test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/The Civic Desk_0.2.8_x64-setup.exe`

- Expected SHA256: `DC395291F909097A46C273FDC698A0F1822C314F6F019F9092888A6AD7F6B325`
- Observed SHA256: `DC395291F909097A46C273FDC698A0F1822C314F6F019F9092888A6AD7F6B325`

Fallback MSI:

`test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/The Civic Desk_0.2.8_x64_en-US.msi`

- Expected SHA256: `B866845F47C32E643A143CD3E5F70FF9F4BCA33912DB036917572D56252ED407`
- Observed SHA256: `B866845F47C32E643A143CD3E5F70FF9F4BCA33912DB036917572D56252ED407`

The NSIS installer was used. MSI fallback was not needed.

## Clean Wipe And Setup

Clean-wipe evidence: `clean-wipe-log.json`

The prior app process and app-owned Ollama runtime were stopped, the old install was uninstalled, and app data/model state paths were removed. No wiped target path remained before install.

Install evidence: `install-result.json`

Setup evidence screenshots:

- `01-first-launch.png`
- `02-after-setup-wait.png`
- `03-sources-after-setup.png`
- `04-daily-scan-before.png`

Local runtime/model selected by the app: `qwen2.5:7b`

## Sources And Leads

Source count: 6

Sources:

- Longmont Agenda Management Portal - primary_record / official_record - online
- Longmont City Council Meetings - primary_record / official_record - online
- Longmont Public Information - official_comm / official_record - online
- Public Notice Colorado - primary_record / official_record - online
- Longmont subreddit - community_signal / community_signal - online
- Longmont Colorado subreddit - community_signal / community_signal - online

This includes official/primary-record sources plus public community/social subreddit sources. Daily Scan and Story Queue evidence showed 15 leads available.

Lead count: 15

Daily Scan lead count recorded in DB: 7

## Drafting And Editor Workflow

Draft count: 5

Ready/published story or brief count: 5

Killed/held/sent-back count: 0. I avoided drafting the duplicate Building Services permitting-portal leads rather than publishing them and then killing them.

Editor workflow evidence:

- `06-after-first-generate.png`
- `07-advisor-result.png`
- `20-story-queue-after-drafting.png`
- `21-story-queue-after-extra-drafts.png`
- `draft-approval-results.json`
- `extra-draft-results.json`

## Final Public Articles

- Youth Center Programs in Longmont: The City of Longmont is focusing on youth development through programs at the Youth Center to support children, families, and communities.
- A new official primary document from 'Longmont Public Information' was fetched today (https://longmontcolorado.gov/public-information/).
- City Council Meetings with Video Archive: City Council meetings are held regularly, and videos of the meetings are now available online. This provides transparency and allows residents to review past discussions.
- City Council Second Reading Process: The City Council's process for approving ordinances, including the steps of first and second readings, public hearings, and voting requirements, is outlined. This transparency helps ensure the public understands how local laws are enacted.
- Longmont Invites Applications for New Official City Flag: The city of Longmont is seeking public input to design its new official flag, engaging community members in the process of creating a symbol that represents their diverse and welcoming community.

## Compile, ZIP, And Publish

Output folder:

`C:/Users/civic/Desktop/CODE/civicnewspaper-test-comms/test-comms/reports/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-evidence/publication-output/site`

ZIP path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-cleanwipe-longmont-duplicate-rerun-0941256-evidence\publication-output\site\site-package.zip`

ZIP SHA256:

`1A2D6495D39B16C59F9167684644694B6D068C53CC446193CC3C99CA879A7395`

ZIP extract check: True

Compile/publish screenshots:

- `30-publishing-before-compile.png`
- `34-identity-saved.png`
- `35-publishing-output-set.png`
- `36-compile-checklist.png`
- `37-after-compile.png`
- `38-after-zip-export.png`
- `39-connector-test.png`
- `40-after-herenow-publish.png`
- `41-herenow-page.png`

here.now browser verification:

- URL reachable: True
- HTTP status: 200
- Page title: `Longmont Duplicate Lead Retest`
- H1: `LONGMONT DUPLICATE LEAD RETEST`

## Quality Checks

Duplicate story topics found: 0

Building Services permitting-portal duplicate recurred: False

Duplicate-topic audit result: True

Mojibake scan result: PASS, 0 hits

Public `Draft:` title-prefix check: PASS, 0 hits

Starter publication name on public output: False

Expected publication name on public output: True

## Quality Note

This looks like a usable basic local issue for a cleanroom test: it has five distinct Longmont-facing pieces, a visible publication identity, a working public URL, and no duplicate Building Services story. It still reads like a first-pass AI-assisted local issue rather than a polished newsroom product because several titles are long and one article is a generic public-information record, but it is reviewable and not merely a one-story mechanical artifact.