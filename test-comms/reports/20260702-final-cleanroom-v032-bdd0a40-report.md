# Final Cleanroom Report - Civic Desk v0.3.2 bdd0a40

Status: PASS

## Scope

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Branch: `test-comms/cleanroom-coder-tester`
- Active directive: `test-comms/directives/20260702-final-cleanroom-v032-bdd0a40.md`
- Product commit represented by installer: `bdd0a40e0af46701c8a8eb1b815178bf830caae9`
- Evidence folder: `test-comms/evidence/20260702-final-cleanroom-v032-bdd0a40/`

## Installer

- Installer path: `test-comms/artifacts/20260702-final-cleanroom-v032-bdd0a40/The Civic Desk_0.3.2_x64-setup.exe`
- Observed SHA256: `D7FA08CDB668E49D229F6C403B1666F351B5627F1B8EB94333944F3FB9F8A4F8`
- Observed size: `5229228`
- Evidence: `installer-verify.txt`

## Clean Wipe And Launch

Clean wipe removed prior app state under:

- `%APPDATA%\com.scottconverse.civicdesk`
- `%LOCALAPPDATA%\com.scottconverse.civicdesk`
- `%LOCALAPPDATA%\The Civic Desk`

The installed app launched from:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

The app identity showed `The Civic Desk`, `LONGMONT / CO`, not a Vite/dev identity.

Evidence:

- `install-clean-launch.log`
- `screenshot-01-launch.png`
- `screenshot-02-after-identity-next.png`
- `screenshot-03-ai-ready.png`

## AI Setup And Sources

App-guided local AI setup completed. The app reported `Local AI ready` with model `phi4-mini:latest`.

Database summary after setup:

- `sources`: 9
- `daily_scan_runs`: 1
- `daily_scan_leads`: 24
- `leads`: 27
- `evidence_items`: 26
- `lead_evidence`: 11

The latest Daily Scan was completed and was not stuck `in_progress`.

Evidence:

- `db-after-ai-ready.txt`
- `db-after-scan-wait120.txt`
- `final-db-summary.json`

## Drafts And Verification Gates

Three drafts were generated from different leads:

- Draft 1, lead 26: St. Vrain Valley Schools, `draft_generated`, linked citation `[Source](evidence:16)`.
- Draft 2, lead 25: Longmont Area Chamber of Commerce, improved to `Longmont Chamber outlines 2026 state-session issues`, `ready_to_publish`, linked citation `[Source](evidence:19)`.
- Draft 3, lead 24: City of Longmont art winners, `needs_verification`, no source documents linked.

The no-source lead remained a verification assignment with this message:

`No source documents are linked to this lead yet. Treat it as a verification assignment until public source material is attached or cited.`

Evidence:

- `drafts-after-improve.jsonl`
- `final-drafts.jsonl`
- `screenshot-07-story-queue-scrolled.png`
- `screenshot-08-after-verify-first.png`
- `screenshot-09-after-generate-verification-notes.png`

## Workbench And Approval

Workbench draft picker and the top action strip were present. The selected linked-source Chamber draft exposed:

- `Back to Queue`
- `Save Draft`
- `Delete`
- `Improve for Publication`

`Improve for Publication` rewrote the Chamber draft into a reader-facing brief with the normalized linked citation `[Source](evidence:19)`.

The editor approval flow displayed one advisory citation-coverage warning, asked for an editor note, and logged the approval decision. Draft 2 was approved as `ready_to_publish`, with:

- `attested_by`: `Cleanroom Tester`
- `guardrail_override_reason`: `Editor reviewed pre-publication warnings and chose to publish.`
- one `publish_decision_audits` row

Evidence:

- `screenshot-10-workbench-open.png`
- `screenshot-11-workbench-draft-select.png`
- `screenshot-12-after-improve-click.png`
- `screenshot-14-workbench-pagedown2.png`
- `screenshot-10-approved-linked-draft.png`
- `db-after-approval.jsonl`
- `uia-after-approval.txt`

Note: I did not intentionally corrupt a valid draft to synthesize an unlinked evidence ID. I verified the no-source assignment block, verified linked-source citation normalization, and verified the final static and public output did not contain unlinked `evidence:` citations.

## Compile, ZIP, And Publish

Publishing showed the compile controls and a ready approved story count. `Compile site` produced a static site package and a database publish run.

Publish run:

- `issue_id`: `issue-20260703-011137-759360100`
- `output_path`: `C:/Users/civic/AppData/Roaming/com.scottconverse.civicdesk/sites/default`
- `article_count`: 1
- `skipped_count`: 0
- `files_written`: 18
- `provider`: `here_now`
- `published_url`: `https://hearty-clover-fzqa.here.now`
- `deployment_id`: `slug=hearty-clover-fzqa;version=01KWJRJ2V5MBBEKXAGBQND9VHN;created_slug=hearty-clover-fzqa`

Generated files included:

- `index.html`
- `briefs/2.html`
- `styles.css`
- `print.css`
- `feed.xml`
- `newsletter.md`
- `substack.md`
- `share-package.md`
- `site-package.zip`

Evidence:

- `uia-publishing-before-compile.txt`
- `uia-publishing-after-compile.txt`
- `screenshot-11-after-compile.png`
- `db-after-compile.jsonl`
- `publish-folder-listing.txt`
- `zip-listing.txt`
- `publish-manifest.json`
- `final-publish-runs.jsonl`

## Public Output Audit

The local static output and here.now output were inspected for the directive's prohibited public-output markers:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`
- mojibake marker code points or visible mojibake
- unlinked `evidence:` citations
- disabled `unlinked-evidence-` citations
- generic city navigation pages treated as news stories

Both audits returned empty arrays:

- `local-public-output-audit.json`: `[]`
- `here-now-public-audit.json`: `[]`

The here.now public output included:

- `here-now-index.html`
- `here-now-briefs-2.html`
- `here-now-feed.xml`

Evidence:

- `article-briefs-2.html.txt`
- `here-now-index.html`
- `here-now-briefs-2.html`
- `here-now-feed.xml`
- `here-now-index.html.text-snippet.txt`
- `here-now-briefs-2.html.text-snippet.txt`
- `here-now-feed.xml.text-snippet.txt`

## Conclusion

The bdd0a40 cleanroom release path passed the previously blocked finish-line area: static compile produced files and `site-package.zip`, the database recorded a publish run, `Publish to here.now` completed, and the public output audit found no prohibited markers or unlinked evidence citations.
