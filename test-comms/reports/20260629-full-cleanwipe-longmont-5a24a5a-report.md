# Full Clean-Wipe Longmont Publication E2E Report - 5a24a5a

Status: FAIL - here.now publish still sends empty display name

Directive: `test-comms/directives/20260629-full-cleanwipe-longmont-5a24a5a.md`

Product branch: `stable-readiness-local-gates`

Product commit: `5a24a5a597b78907ca5d64019432c1468b3ff30a`

Evidence folder: `test-comms/reports/20260629-full-cleanwipe-longmont-5a24a5a-evidence/`

## Summary

The 5a24a5a NSIS installer hash matched, the expanded clean wipe removed the prior stale output-path state, the app installed, app-owned AI setup completed, and the product generated a five-story Longmont local output package from a clean run.

Several targeted fixes passed:

- Default output path after clean wipe was clean: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`, not an old `test-comms/reports/...` folder.
- The UI blocked starter identity and warned that public publishing was paused until a real publication name was chosen.
- I entered and saved `Longmont Cleanroom Test` through the visible app UI.
- Local output and ZIP output use `Longmont Cleanroom Test`.
- Local output and ZIP output do not use `My Local Publication`.
- Kill/cut persisted: final database has 5 `ready_to_publish` drafts and 1 `killed` draft.
- Exact mojibake scanner passed on local output and ZIP extraction.
- Public article titles do not begin with `Draft:`.

The run still fails because anonymous here.now publish failed from the product with:

`here.now publish create failed with status 400 Bad Request: {"error":"Invalid request","code":"invalid_request","message":"Fix the request body or parameters and retry.","details":"Display name must not be empty after normalization.","docs_url":"https://here.now/docs"}`

That happened even while the Publishing screen visibly showed `Publication: Longmont Cleanroom Test`. So the publication identity is fixed for local compile/export but is not being passed correctly to the here.now connector request.

## Installer And Clean Wipe

Preferred NSIS installer:

`test-comms/artifacts/20260629-full-cleanwipe-longmont-5a24a5a/The Civic Desk_0.2.8_x64-setup.exe`

Hash checks:

- NSIS expected: `A19456F776E319E0850463A3494A47B2CBA5668C556724BB1A96C4963E412082`
- NSIS observed: `A19456F776E319E0850463A3494A47B2CBA5668C556724BB1A96C4963E412082`
- MSI expected: `A519ADE9DD15EE20887BB189F6CECD78E6B7BE1CB584B54FB4ACD8159DABF61A`
- MSI observed: `A519ADE9DD15EE20887BB189F6CECD78E6B7BE1CB584B54FB4ACD8159DABF61A`

Clean wipe details:

- Stopped `civicnews.exe`.
- Stopped app-owned `ollama.exe`.
- Found and stopped lingering app-owned `llama-server.exe`.
- Ran The Civic Desk uninstaller silently; exit code 0.
- Removed Roaming product state after stopping `llama-server.exe`.
- Removed Local product state.
- Removed `.ollama`.
- Removed expanded product/WebView-style state names under Local/Roaming when found.

Evidence:

- `clean-wipe-log.json`
- `installer-hashes.json`
- `install-result.json`
- `launch-result.json`

## App-Owned Setup

The app launched as a normal user and completed app-owned local AI setup:

- Local AI ready.
- Model: `qwen2.5:7b`.
- No manual Ollama/model/dependency installation was performed.

The setup recovery banner still appeared:

`The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile. You can edit identity later in Settings.`

However, I then used the product's visible identity UI before compile/publish.

## Publication Identity

Default Publishing screen before identity edit:

- Publication: `My Local Publication`
- Warning: `Publication name still uses starter text. Public publishing is paused until you choose one.`
- Default output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`

Identity entered through visible app UI:

- Publication name: `Longmont Cleanroom Test`
- Tagline: `Temporary cleanroom test publication for Longmont civic coverage.`
- Footer/legal note: `Temporary cleanroom test output.`

After save, Publishing showed:

- Publication: `Longmont Cleanroom Test`
- Footer note: configured
- The starter identity warning was gone.

Evidence:

- `03-publishing-default-path.png`
- `04-edit-identity-screen.png`
- `05-identity-saved.png`
- `06-publishing-after-identity.png`

## Sources / Leads / Drafts

Final read-only database counts:

- Sources: 6
- Evidence items: 27
- Leads: 30
- Daily scan leads: 22
- Drafts: 6
- Draft statuses: 5 `ready_to_publish`, 1 `killed`
- Publish runs: 1 local export
- Published posts: 5

Sources included:

- Longmont Agenda Management Portal - official record
- Longmont City Council Meetings - official record
- Longmont Public Information - official record
- Public Notice Colorado - official record
- Longmont subreddit - community signal
- Longmont Colorado subreddit - community signal

Evidence:

- `07-daily-scan-before.png`
- `08-after-scan-wait.png`
- `09-story-queue-after-scan.png`
- `50-story-queue-after-drafts.png`
- `db-state-after-editor-flow.json`
- `final-db-state.json`

## Writer / Editor / Advisor / Kill Flow

The run used the visible installed app UI to:

- Draft six stories from Story Queue leads.
- Edit and save the first draft body.
- Run the advisor path on the first story.
- Put the first story on hold.
- Approve five stories for static publishing.
- Kill one extra non-publish draft and confirm the kill modal.

Final database proves kill/cut persisted:

- 5 `ready_to_publish`
- 1 `killed`

Evidence screenshots include:

- `101-workbench-approve.png` through Workbench screenshots
- `20-save-draft.png`
- `21-advisor.png`
- `22-hold.png`
- approved screenshots
- `30-kill-modal.png`
- `31-after-kill.png`
- `50-story-queue-after-drafts.png`

## Compile / Export

Output folder set through the app UI:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-cleanwipe-longmont-5a24a5a-evidence\publication-output\site`

Compile/export result:

- Article count: 5
- Files written: 22
- Skipped: 0
- ZIP exists: yes
- ZIP SHA256: `5916D55D4D49F5730D2C3B89D0D709515DA455ACD26AA67A7215BAD7DAB21E9E`

Evidence:

- `60-publishing-output-path-set.png`
- `61-compile-checklist.png`
- `62-after-compile.png`
- `63-after-zip.png`
- `output-file-summary.json`
- `publication-output/site/`
- `publication-output/site/site-package.zip`

## Output Checks

Exact mojibake scanner:

- Local output: PASS
- ZIP extraction: PASS

Draft prefix check:

- Local output: PASS
- ZIP extraction: PASS

Publication identity check:

- Local output: PASS. `Longmont Cleanroom Test` appears in site and article titles.
- ZIP extraction: PASS. `Longmont Cleanroom Test` appears in site and article titles.
- `My Local Publication`: not found in local or ZIP public titles.

Evidence:

- `output-checks.json`
- `zip-extracted/`

## here.now Publish

here.now publish was attempted through the visible Publishing connector UI after `Test connection` passed.

Result: FAIL.

Visible product error:

`Something went wrong: here.now publish create failed with status 400 Bad Request: {"error":"Invalid request","code":"invalid_request","message":"Fix the request body or parameters and retry.","details":"Display name must not be empty after normalization.","docs_url":"https://here.now/docs"}`

No here.now URL was produced for this run.

Evidence:

- `64-test-connection.png`
- `65-after-publish.png`

## Findings

### Major - here.now request still has empty display name

The product correctly showed `Longmont Cleanroom Test` in Publishing and compiled that identity into local output, but the here.now connector request still failed because the display name was empty after normalization.

Impact: this fails the directive because here.now publish must work and public output must use the tester-entered publication name.

### Minor - setup recovery banner still appears

The app still reports that the setup screen did not receive input events and continues with a starter Longmont profile. The later identity gate allowed correction before compile, but this remains a confusing first-run setup path.

## Final Assessment

Not ready for release certification.

The clean-wipe state, identity gate, output path cleanup, kill/cut persistence, local compile/export, mojibake checks, and `Draft:` title checks all improved and passed locally. The remaining blocker is the here.now connector: it does not pass the saved/visible publication display name into the publish request.
