# CivicNewspaper cleanroom E2E report - cd038d6

Status: FAIL

UTC run window: 2026-06-29T15:24Z to 2026-06-29T15:43Z

Coordination branch: `test-comms/cleanroom-coder-tester`

Directive: `test-comms/directives/20260629-full-e2e-output-quality-landing-cd038d6.md`

Product commit under test: `cd038d696fe9708aaa54c23dd766eff36112f93b`

Evidence folder: `test-comms/reports/20260629-full-e2e-output-quality-landing-cd038d6-evidence/`

## Summary

The cleanroom install and full product loop mostly ran end to end: the landing page was reachable, the NSIS installer hash matched, The Civic Desk installed and launched, app-driven local AI setup reached `qwen2.5:7b`, Longmont sources were visible, Daily Scan produced leads, drafts were generated and approved, the site compiled, a ZIP was exported, and the issue was published anonymously to here.now.

The build fails the directive because reader-facing public output still contains private/editorial scaffolding. The generated static site and live here.now article pages expose `EDITOR_NOTE` and `Body:` text to readers. That violates the directive's output-quality pass criteria and means a real local publisher should not use this build for a public beta issue without fixing or blocking those drafts before publication.

Live here.now URL:

`https://calm-monsoon-4sv7.here.now`

## Installed Artifact

Preferred NSIS installer used:

`test-comms/artifacts/20260629-full-e2e-output-quality-landing-cd038d6/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`520F226F62FCD94B8BF8D3345EB492A990931938FC49D8AA2222EC22DEA07695`

Observed SHA256:

`520F226F62FCD94B8BF8D3345EB492A990931938FC49D8AA2222EC22DEA07695`

Install result:

- Method: NSIS
- Exit code: 0
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App launched as The Civic Desk.

Evidence:

- `installer-hashes.json`
- `install-result.json`
- `launch-result.json`
- `01-first-launch.png`

## Landing Page Check

Public landing page checked:

`https://scottconverse.github.io/CivicNewspaper/`

Result:

- HTTP 200
- here.now was visible in the public publishing messaging.
- `Publish Statically` was not present.
- Vercel was not presented as the recommended drag-and-drop default.
- Platform download copy was conditional.
- Desktop and mobile screenshots captured.

Evidence:

- `landing-page-checks.json`
- `landing-desktop.png`
- `landing-mobile.png`
- `landing-page.html`

## App-Driven Setup

The app completed the normal-user setup path without manual installer-side intervention for Ollama/model setup during this run. The UI reported:

- City/state display: `LONGMONT / CO`
- AI status: `Local AI ready`
- Model: `qwen2.5:7b`

Evidence:

- `02-after-ai-setup-wait.png`
- `03-sources-visible.png`
- `06-story-queue-after-scan.txt`

## Sources

The app had 6 watched sources visible in the cleanroom run:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit
- Longmont Colorado subreddit

Evidence:

- `03-sources-visible.png`
- `db-final-state.json`

## Daily Scan and Lead Quality

Daily Scan ran from the UI and produced 15 visible story queue leads after the scan. The app showed deterministic/source evidence activity before and during AI summarization. The lead list still contained duplicate/paraphrase pressure and some low-quality/general primary-record leads, including:

- duplicate Youth Center lead variants
- duplicate/overlapping technical-issues lead variants
- generic primary-document leads
- one visible mojibake lead in the story queue: `City Clerkâ€™s Agenda Management Portal Experiencing Technical Issues`

Evidence:

- `daily-scan-states.json`
- `04-daily-scan-before.png`
- `05-daily-scan-after.png`
- `06-story-queue-after-scan.png`
- `06-story-queue-after-scan.txt`

## Editorial Workflow

Five drafts were generated and approved for static publishing. The press-freedom/legal-risk advisor was run on the first draft. A separate draft was opened, held, and returned to queue successfully.

Observed approved drafts:

- Longmont Seeks Designs for Official City Flag
- City Council Meeting Schedule Provides Insight into Ordinance Approval
- Technical Issues Plague Building Services Online Portal
- Youth Center Programs in Longmont: The Potential Impact on Community Development and Social Well-being
- New Official Document from Public Notice Colorado

Held/returned draft:

- Upcoming City Council Meeting: Public Participation Information Available

Evidence:

- `draft-editor-results.json`
- `draft-editor-results.pretty.json`
- `hold-return-result.json`
- `12-advisor-result-1.png`
- `13-after-approve-1.png` through `13-after-approve-5.png`
- `20-hold-candidate-workbench.png`
- `21-after-hold.png`
- `22-story-queue-after-editor-workflow.png`

## Publication Output

The publication workflow compiled the site, exported ZIP, tested here.now preview publishing, and published the issue.

Output folder:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-e2e-output-quality-landing-cd038d6-evidence\publication-output\site`

ZIP:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-e2e-output-quality-landing-cd038d6-evidence\publication-output\site\site-package.zip`

ZIP SHA256:

`9E5E73D8A3ACF177750C8FF9B05B9E2642A6E0EC8068CA770D37524862F121EA`

ZIP verification:

- `site-package.zip` exists.
- 21 entries were readable through .NET ZIP APIs.
- Sample entries included `index.html`, `feed.xml`, `share-package.md`, `substack.md`, `watch/1.html` through `watch/5.html`.

here.now URL:

`https://calm-monsoon-4sv7.here.now`

Evidence:

- `publish-ui-state.json`
- `publish-ui-result.json`
- `publishing-final-ui.txt`
- `zip-hash.json`
- `39-herenow-homepage.png`
- `40-herenow-article.png`
- `41-herenow-mobile.png`
- `herenow-links.json`
- `herenow-homepage.txt`
- `herenow-article.txt`

## Blocking Product Failure

Reader-facing article pages include editor/reporter scaffolding and should not have been publishable as public stories.

Exact local static output hits from `output-quality-audit.json`:

- `publication-output/site/watch/2.html`, line 46: `EDITOR_NOTE: This looks like background material, not a publishable news story yet...`
- `publication-output/site/watch/3.html`, line 42: `Body:`
- `publication-output/site/watch/3.html`, line 43: `EDITOR_NOTE: Not enough verified source material for a publishable story yet.`
- `publication-output/site/watch/4.html`, line 42: `EDITOR_NOTE: Not enough verified source material for a publishable story yet.`
- `publication-output/site/watch/5.html`, line 42: `Body:`
- `publication-output/site/watch/5.html`, line 44: `EDITOR_NOTE: This looks like background material, not a publishable news story yet...`

Live here.now evidence:

`herenow-article.txt` captured the public article `New Official Document from Public Notice Colorado` containing:

- `Body: A new official primary document has been fetched from Public Notice Colorado...`
- `EDITOR_NOTE: This looks like background material, not a publishable news story yet...`

This is not just a hidden draft-state issue. It reached the generated static site, ZIP package, and live public here.now page.

## Mojibake / Encoding Audit

The sidebar UI correctly displayed `LONGMONT / CO`.

However, mojibake was still visible in the workflow evidence:

- Story queue text included `City Clerkâ€™s Agenda Management Portal Experiencing Technical Issues`.
- Draft database content included strings such as `City Clerkâ€™s Office` and `Longmontâ€™s`.
- `output-quality-audit.json` article samples still captured `Longmontâ€™s official website` in static article text.

Because the public-output quality failure already blocks the release, I did not classify mojibake as the sole blocker. It is still present and should be fixed or explicitly normalized before a release pass.

Evidence:

- `06-story-queue-after-scan.txt`
- `db-drafts-precompile.json`
- `output-quality-audit.json`

## Reproduction Steps

1. Use coordination branch `test-comms/cleanroom-coder-tester`.
2. Run directive `test-comms/directives/20260629-full-e2e-output-quality-landing-cd038d6.md`.
3. Install `The Civic Desk_0.2.9_x64-setup.exe`; hash matches the expected NSIS SHA256.
4. Launch The Civic Desk and complete app-driven setup for Longmont, Colorado.
5. Run Daily Scan.
6. Generate drafts from multiple leads.
7. Approve drafts that contain `EDITOR_NOTE` / insufficient-material text.
8. Compile and publish.
9. Open `publication-output/site/watch/3.html` or the live article at `https://calm-monsoon-4sv7.here.now/watch/5.html`.
10. Observe reader-facing `Body:` and/or `EDITOR_NOTE` content.

## Final Determination

FAIL.

The build proves the end-to-end path can execute, including install, AI readiness, Daily Scan, draft generation, ZIP export, and here.now publication. It does not meet the public beta quality bar because the public issue includes explicit editor notes and nonstory/reporter-note scaffolding as reader-facing article copy.

A real local publisher should not use this build to produce a public beta issue until the product either:

- prevents drafts containing editor-only scaffolding from being approved/published,
- strips or transforms editor-only notes from public output,
- clearly labels insufficient-material items as internal holds rather than public articles, and
- resolves the remaining mojibake seen in lead/draft/public text.
