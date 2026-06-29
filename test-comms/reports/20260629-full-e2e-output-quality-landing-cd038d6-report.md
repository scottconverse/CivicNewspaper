# Full Cleanroom E2E Retest Report - cd038d6

Status: FAIL

Directive: `test-comms/directives/20260629-full-e2e-output-quality-landing-cd038d6.md`

Product branch: `stable-readiness-local-gates`

Product commit tested: `cd038d696fe9708aaa54c23dd766eff36112f93b`

Evidence folder: `test-comms/reports/20260629-full-e2e-output-quality-landing-cd038d6-evidence/`

here.now URL produced:

`https://calm-monsoon-4sv7.here.now`

## Summary

The build passed the install/setup/publish mechanics but failed the required output-quality bar. The cleanroom machine installed the 0.2.9 NSIS artifact, the app drove local AI setup from clean state, selected `qwen2.5:7b`, discovered Longmont sources, ran Daily Scan, generated drafts, exported a ZIP, and published to here.now.

The failure is public output quality: compiled reader-facing pages still expose reporter-note scaffolding including `EDITOR_NOTE` and `Body:`. The directive explicitly requires public story bodies not to expose reporter notes or scaffolding. A real local publisher could use this build to get through the mechanics, but should not use it for a public beta issue without fixing the output cleanup/filtering.

## Landing Page Check

Landing page: `https://scottconverse.github.io/CivicNewspaper/`

- HTTP status: 200
- here.now visible in first-page/product publishing messaging: True
- Contains `Publish Statically`: False
- Mentions Vercel as recommended default: False
- Platform copy conditional on release assets: True

Screenshots: `landing-desktop.png`, `landing-mobile.png`

## Installed Artifact

Preferred NSIS installer:

`test-comms/artifacts/20260629-full-e2e-output-quality-landing-cd038d6/The Civic Desk_0.2.9_x64-setup.exe`

- Expected SHA256: `520F226F62FCD94B8BF8D3345EB492A990931938FC49D8AA2222EC22DEA07695`
- Observed SHA256: `520F226F62FCD94B8BF8D3345EB492A990931938FC49D8AA2222EC22DEA07695`

Fallback MSI was hash-checked but not used:

- Expected SHA256: `C72791A1BC269670EB0D376ED0BA452B2DA375D21588994355535E94294CB2AF`
- Observed SHA256: `C72791A1BC269670EB0D376ED0BA452B2DA375D21588994355535E94294CB2AF`

NSIS install exit code: 0

App launched as The Civic Desk and CDP/WebView capture was available: True

## App-Guided AI Setup

App-driven AI setup worked from clean state. I did not install Ollama or models manually outside the app.

Model selected by the app: `qwen2.5:7b`

The visible app showed `LONGMONT / CO`, not the prior mojibake separator. Current app evidence: `app-sources-current.png`, `app-story-queue-current.png`, `app-publishing-current.png`.

## Sources And Daily Scan

Source count: 6

- Longmont Agenda Management Portal - primary_record / official_record - online
- Longmont City Council Meetings - primary_record / official_record - online
- Longmont Public Information - official_comm / official_record - online
- Public Notice Colorado - primary_record / official_record - online
- Longmont subreddit - community_signal / community_signal - online
- Longmont Colorado subreddit - community_signal / community_signal - online

Daily Scan produced 16 leads. The visible Story Queue evidence shows 15 new leads, 5 high-priority leads, and 6 sources.

## Editorial Workflow

Draft status counts:

- hold: 1
- ready_to_publish: 5

Five drafts were approved for static publish, and one additional draft was held/returned as workflow exercise. The app did not block editor decisions; it allowed approval, hold, and return-to-queue paths.

## Publication Output

Output folder:

`C:/Users/civic/Desktop/CODE/civicnewspaper-test-comms/test-comms/reports/20260629-full-e2e-output-quality-landing-cd038d6-evidence/publication-output/site`

ZIP path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-e2e-output-quality-landing-cd038d6-evidence\publication-output\site\site-package.zip`

ZIP SHA256:

`9E5E73D8A3ACF177750C8FF9B05B9E2642A6E0EC8068CA770D37524862F121EA`

ZIP extract check: True

here.now publish URL:

`https://calm-monsoon-4sv7.here.now`

Public evidence includes desktop, mobile, print, homepage, article page, RSS/feed, and share package captures:

- `public-desktop.png`
- `public-mobile.png`
- `public-print.png`
- `public-watch-1-html.png`
- `public-watch-3-html.png`
- `public-feed-xml.txt`
- `public-share-package-md.txt`

## Public Article Titles

- Longmont Seeks Designs for Official City Flag - Longmont E2E Quality Retest
- City Council Meeting Schedule Provides Insight into Ordinance Approval - Longmont E2E Quality Retest
- Technical Issues Plague Building Services Online Portal - Longmont E2E Quality Retest
- Youth Center Programs in Longmont: The Potential Impact on Community Development and Social Well-being - Longmont E2E Quality Retest
- New Official Document from Public Notice Colorado - Longmont E2E Quality Retest

The title fallback improved: article titles are real headlines rather than long lead-summary sentences.

## Output Quality Failure

The output quality audit failed because public static pages contain reporter-note scaffolding:

- watch/2.html line 46 matched `/EDITOR_NOTE/g`: `<p>EDITOR_NOTE: This looks like background material, not a publishable news story yet. A current development or new information about upcoming meetings or specific ordinance proposals would make it more relevant and timely for publication.<`
- watch/3.html line 42 matched `/Body:/g`: `                <p>Body:`
- watch/3.html line 43 matched `/EDITOR_NOTE/g`: `EDITOR_NOTE: Not enough verified source material for a publishable story yet.</p>`
- watch/4.html line 42 matched `/EDITOR_NOTE/g`: `                <p>EDITOR_NOTE: Not enough verified source material for a publishable story yet.</p>`
- watch/5.html line 42 matched `/Body:/g`: `                <p>Body:`
- watch/5.html line 44 matched `/EDITOR_NOTE/g`: `<p>EDITOR_NOTE: This looks like background material, not a publishable news story yet. Additional information would be needed, such as the subject matter of the document, its significance, or any immediate actions or impacts it might have o`

Audit file: `output-quality-audit.json`

This violates the directive requirement that public story bodies not expose reporter-note scaffolding. The concrete exposed strings in this run were `EDITOR_NOTE` and `Body:`.

## Other Quality Checks

- Duplicate story topics found: 0 candidate pairs in title/topic review
- Public `Draft:` title-prefix hits: 0
- Mojibake scanner hits in generated/public text: 0
- ZIP exists and extracts: True
- here.now publish succeeded: true

## Reproduction Steps For Failure

1. Clean wipe The Civic Desk/CivicNewspaper app data and app-owned Ollama/model state.
2. Install `The Civic Desk_0.2.9_x64-setup.exe` from the cd038d6 artifact folder.
3. Launch the app and let app-driven AI setup complete.
4. Run Daily Scan for Longmont.
5. Generate and approve the five captured drafts.
6. Compile the static site and publish to here.now.
7. Open `watch/3.html`, `watch/4.html`, or `watch/5.html` in the generated output or public site.
8. Observe `EDITOR_NOTE` and/or `Body:` in reader-facing article copy.

## Final Assessment

FAIL. The full workflow is operational and the redesigned landing page checks passed, but this build still publishes reporter-note scaffolding into public pages. A real local publisher could use the app to produce a test issue, but the resulting public beta issue is not acceptable until output cleanup prevents `EDITOR_NOTE`/`Body:` style scaffolding from reaching reader-facing pages.