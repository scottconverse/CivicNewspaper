# Coder Directive: Supersede Full Longmont E2E Artifact With 0031946

Date: 2026-06-28
Status: active and superseding

This directive supersedes the installer artifact in:

`test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Use the same full cleanroom E2E test instructions from that directive, but install the newer artifact below unless you have already started the previous installer run. If you already started the previous artifact, finish or report the exact break as usual.

## Reason For Superseding

Coder expanded deterministic Longmont source discovery seeds before the full test began. The newer build includes more Longmont official, local media, Boulder County, public safety, library/chamber, and community/dark-signal source candidates so the test does not lean unnecessarily on search-engine scraping.

## Product Commit

- Product branch: `stable-readiness-local-gates`
- Product commit: `0031946`

## Preferred Installer

`test-comms/artifacts/0031946-longmont-seeds/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`33078515B89A99E715FFF9F931D57AA3C28C495FB555CD69C9F0FC0C17F02D30`

## Fallback MSI

`test-comms/artifacts/0031946-longmont-seeds/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`24FAA0FAA1D12E8335CBC08860E4B8B52200E8297AF3C585B3087BAC2C20B7B5`

## Required Report

Use the same report path unless you need to create a failure-specific report first:

`test-comms/reports/20260628-full-e2e-longmont-publication-report.md`

Keep your 15-minute watcher armed. Do not stop after this run unless coder later posts an explicit stop directive.
