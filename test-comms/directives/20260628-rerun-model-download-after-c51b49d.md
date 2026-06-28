# Directive: rerun cleanroom after first-run model download fix c51b49d

Tester, continue the 15-minute watcher. This directive replaces the prior run at the exact blocker: runtime setup succeeded, but the recommended model download did not start from first-run setup.

## Product branch and artifact

- Product branch: `stable-readiness-local-gates`
- Product commit to test: `c51b49d`
- Installer artifacts:
  - `test-comms/artifacts/20260628-model-download-rerun-c51b49d/The Civic Desk_0.2.8_x64-setup.exe`
  - `test-comms/artifacts/20260628-model-download-rerun-c51b49d/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256:
  - NSIS setup EXE: `80EA262CA15AC4CAB69D3D1ABC4C1BD3569D76CBDB42F851D189897AE41DB60A`
  - MSI: `663724E48D4248082CF101DD916B88F913E3B0E0AE78EE8ADC0F3CC3AECECF49`

## Clean reset boundary

Wipe only CivicNewspaper, Ollama, local models, test files, app data, PATH changes, and related prerequisites. Leave Windows, the user account, browser, Git, and Codex tester environment intact.

Do not manually install Ollama, models, document tools, OCR tools, or runtime prerequisites. If the product cannot install or drive its own required runtime pieces, report that as a product failure and stop at the exact break.

## Setup rerun

1. Install the new artifact as an end user would.
2. Launch The Civic Desk.
3. Use a 1280x720 viewport/window at least once to retest the constrained layout.
4. Complete first-run setup from a clean machine state.
5. Verify the app-managed runtime setup still starts Ollama automatically.
6. When the app reaches the model prompt:
   - Confirm a visible, reachable `Download qwen2.5:7b` button is present in the main setup body.
   - Confirm the footer `Next` path also starts `pull_ollama_model`.
   - Wait for the model download to complete.
   - Confirm `http://127.0.0.1:11434/api/tags` lists the downloaded model.

If the model cannot be downloaded because of network/runtime/product behavior, stop and report the exact failure with screenshots/logs.

## Full Longmont publication workflow, if setup passes

Target city: Longmont, Colorado.

Use only public, readable sources without account login. Scraping public pages is allowed. Do not access private groups, private content, credentials, or paywalled/private material.

Exercise the full product as a user, not by manipulating the database directly:

1. Configure identity/settings for a neutral local publication. Do not invent claims such as "no ads" or mandatory AI disclosure unless the UI explicitly lets the editor choose them.
2. Add and discover sources covering:
   - official city/county/school/public meeting sources,
   - local media,
   - public social/community sources such as Reddit, YouTube public meeting videos/transcripts, public city social feeds, public local forums, or readable public pages.
3. Run source discovery/import/review and save useful candidates.
4. Run the Daily Scan from the UI. It must fetch/analyze/rank enough material for the publication target.
5. If the first scan produces fewer than 10 leads, use the product's source expansion/discovery paths and run again. Search-engine fallback may be used if the product exposes it; mark unverified one-source items as unverified.
6. The target is 10-25 leads and 5-10 reader-facing stories/briefs/watchlist items for a real Longmont paper. One lead/story is a failure.
7. Exercise writer/editor workflow:
   - open leads,
   - create drafts with the local model,
   - edit and save,
   - run the press-freedom/legal-risk advisor on at least one story,
   - approve at least 5 stories/briefs if the product can support that,
   - send at least one item back or put one on hold,
   - kill/cut at least one unsuitable item if the UI supports it,
   - verify no software veto blocks the editor.
8. Compile a publication issue.
9. Export the ZIP/static output package using the product's export path.
10. Publish anonymously to here.now using the product UI. This live anonymous here.now publish is authorized for this test.
11. Open and review the published site:
    - homepage,
    - at least three article/story pages,
    - RSS,
    - about/ethics/corrections pages if present,
    - source/evidence display,
    - mobile/narrow layout,
    - links/share package.

## Report and artifacts

Write a plain-English human report, not just a coder log:

- `test-comms/reports/20260628-model-download-rerun-report-c51b49d.md`

Include:

- exact pass/fail status,
- what was installed by the product,
- whether model download completed,
- whether AI-generated drafts were created from real Longmont source material,
- number of sources, leads, drafts, approved items, held/cut items, and published stories,
- here.now URL,
- local output folder path,
- ZIP path,
- screenshots,
- any exact breakpoints and product bugs.

Put screenshots, logs, exported ZIP, and any copied publication output under:

- `test-comms/artifacts/20260628-model-download-rerun-c51b49d/`

Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
