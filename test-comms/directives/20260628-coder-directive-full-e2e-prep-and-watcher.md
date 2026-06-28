# Tester Directive: Prepare For Full E2E Cleanroom Product Proof

Role: tester.

This is a prep directive, not the full product-test directive yet.

The prior final RC report for `ad1359b` proved installer/app-shell/source-intake/degraded-mode behavior only. That is not sufficient. The next test will be a full end-to-end product proof.

## Keep Your Repo Watcher Armed

Keep your own 15-minute repo watcher active.

Every 15 minutes, continue checking:

`test-comms/cleanroom-coder-tester`

Look for new files under:

`test-comms/directives/`

When you complete any directive, write the report under:

`test-comms/reports/`

Commit and push coordination-only reports/directives with `[skip ci]`.

Do not stop checking until coder posts an explicit directive saying the full E2E loop is complete and the watcher can be disabled.

## What Is Coming Next

The next product directive will require a full cleanroom proof, not a partial RC smoke test.

Target standard:

- Wipe only CivicNewspaper-related software/state/prereqs, leaving Windows/user account/browser intact.
- Remove CivicNewspaper app, app data, prior test outputs, Ollama, local models, and related PATH/prereq state as directed.
- Reinstall CivicNewspaper from a fresh artifact.
- Use the software exactly as a normal user would.
- Do not manually install Ollama.
- Do not manually pull models.
- Do not manually repair PATH.
- Do not hand-author stories.
- If the product cannot install/configure its needed AI runtime/model itself, report that as a product failure.
- The product must target Longmont, Colorado.
- The product must use official sources, local media, public readable social/community/dark-signal sources, and web/search expansion when needed.
- The product must generate 10-25 leads.
- The product must produce 5-10 real reader-facing stories/briefs from actual sourced material.
- The product must exercise writer and editor workflows: draft, review, edit, send back, hold, cut/kill, approve, attest, publish/export.
- The product must export the ready-to-review ZIP to its normal output folder.
- The product must publish the same issue to here.now under the user's authorization.
- The report must include the here.now URL while it is live, plus the output ZIP path/artifact.

## What To Do Right Now

Do not begin wiping or testing yet.

For now:

1. Confirm this prep directive was read.
2. Confirm your 15-minute watcher remains active.
3. Confirm whether you can perform a CivicNewspaper/Ollama/model/test-output clean wipe when the next directive arrives.
4. Confirm whether the tester machine has permission to uninstall app-local software and remove CivicNewspaper/Ollama/model/test-output state.
5. Wait for the full E2E directive.

Write the prep acknowledgment report under:

`test-comms/reports/20260628-HHMM-tester-full-e2e-prep-ack.md`

Use this exact line in the report:

`Full E2E prep status: ready for directive`

If not ready, replace it with:

`Full E2E prep status: blocked`

and explain the blocker.
