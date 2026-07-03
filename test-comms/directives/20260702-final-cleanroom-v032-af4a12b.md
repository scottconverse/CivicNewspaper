# Final Cleanroom v0.3.2 Rerun - Rewrite Artifact Guard

Directive path:

test-comms/directives/20260702-final-cleanroom-v032-af4a12b.md

Coordination branch:

test-comms/cleanroom-coder-tester

Repository:

https://github.com/scottconverse/CivicNewspaper

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester:

C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Supersedes obsolete directives:

test-comms/directives/20260702-final-cleanroom-v032-b0f4ce2.md
test-comms/directives/20260702-final-cleanroom-v032-18eb480.md
test-comms/directives/20260702-final-cleanroom-v032-916653b.md
test-comms/directives/20260702-final-cleanroom-v032-20cfedc.md
test-comms/directives/20260702-final-cleanroom-v032-c93d10f.md
test-comms/directives/20260702-final-cleanroom-v032-bdd0a40.md
test-comms/directives/20260702-final-cleanroom-v032-8261de9.md

Product branch label:

main

Product commit represented by installer:

af4a12b0689dd8de64ce6af707b0c305a9cdaba0

Why this rerun exists:

The 8261de9 cleanroom run proved the app could complete install, setup, source discovery, drafting, compile, ZIP export, here.now publish, and public output scanning. It also proved the approval gate blocked bad rewritten copy from publishing. However, Improve for Publication still loaded unsupported rewrite artifacts into the editor before approval rejected them. The bad improved text included bracketed evidence labels such as [Evidence 21], unlinked pseudo-citations such as [Source](unlinked-evidence-24), and an unsupported external URL typo.

This build must either:

1. Produce an improved draft that uses only linked evidence citations already present in the draft or available evidence context, or
2. Reject the improved draft before changing editor content, with a clear visible error that unsupported source material was introduced.

Required first report:

test-comms/reports/20260702-final-cleanroom-v032-af4a12b-visibility.md

Required final report:

test-comms/reports/20260702-final-cleanroom-v032-af4a12b-report.md

Tester output artifact folder:

test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/

NSIS installer:

test-comms/artifacts/20260702-final-cleanroom-v032-af4a12b/The Civic Desk_0.3.2_x64-setup.exe

NSIS SHA256:

AB598EC26F658BB2B0735827F15DC787162D372A0C3FF0A3A18B6ADE48ABE241

NSIS size:

5229719

Tester tasks:

1. Pull this coordination branch and confirm this ACTIVE_DIRECTIVE.md is visible.
2. Write the visibility report before installing:
   test-comms/reports/20260702-final-cleanroom-v032-af4a12b-visibility.md
3. Clean wipe any previous CivicNewspaper or The Civic Desk app state on the tester machine.
4. Install the NSIS installer listed above.
5. Launch The Civic Desk from the installed Windows application, not from source.
6. Complete app-guided setup using local AI.
7. Configure Longmont, Colorado.
8. Run source discovery and daily scan.
9. Generate at least two drafts from real Longmont leads.
10. Exercise the no-source / evidence-required path and confirm unsupported approval is blocked.
11. Exercise Improve for Publication on a draft with linked evidence. Specifically check for all of these bad artifacts in the editor after Improve:
    - [Evidence 21]
    - [Evidence 22]
    - [Evidence 23]
    - unlinked-evidence-
    - https://www.longmondchamber.org
    - Any external URL not already present in the source/evidence context
12. If Improve rejects the rewritten copy, confirm the original editor title and body remain unchanged and that the user-visible error explains unsupported source material.
13. Approve only clean, source-grounded copy.
14. Compile a publication.
15. Export a ZIP and record the ZIP path.
16. Publish to here.now and record the URL.
17. Scan the public here.now pages, ZIP extract, RSS/share artifacts, and visible article pages for:
    - unsupported source material
    - unlinked evidence citations
    - bracketed evidence labels
    - city-specific hallucinations
    - duplicate-topic output
    - mojibake marker code points U+00C2, U+00C3, U+00E2, U+FFFD
    - reporter-note scaffolding such as EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], and [End of Report]
18. Write the final report:
    test-comms/reports/20260702-final-cleanroom-v032-af4a12b-report.md

Pass condition:

The final report must have zero blocker, critical, or major findings. It must include the installed app identity, installer hash verification, source-discovery result, AI setup result, draft workflow result, Improve for Publication result, approved-story evidence result, ZIP path, here.now URL, public-output scan result, and a plain-English verdict.

If blocked:

Capture screenshots, exported text, logs, and any relevant app data under:

test-comms/evidence/20260702-final-cleanroom-v032-af4a12b/

Then write the final report with the exact blocker and reproduction steps.
