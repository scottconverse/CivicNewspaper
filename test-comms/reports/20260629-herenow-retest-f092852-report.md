# Focused here.now Connector Retest Report - f092852

Status: PASS

Directive: `test-comms/directives/20260629-herenow-retest-f092852.md`

Product branch: `stable-readiness-local-gates`

Product commit: `f092852e9df3808f16cf56b829993f028e31d255`

Evidence folder: `test-comms/reports/20260629-herenow-retest-f092852-evidence/`

here.now URL:

`https://merry-frost-9arx.here.now`

## Summary

The focused f092852 here.now connector retest passed. I installed the f092852 NSIS artifact, reused the already-proven 5a24a5a Longmont output folder, confirmed the visible Publishing screen still showed `Longmont Cleanroom Test`, ran the here.now connector test, and published anonymously through the visible app UI.

The returned here.now URL responded HTTP 200 and the public page rendered with `Longmont Cleanroom Test` as the title and H1. The page did not expose `My Local Publication` in public title/header text, public article titles did not begin with `Draft:`, and the mojibake scanner found no hits.

## Installer Hashes

Preferred NSIS installer:

`test-comms/artifacts/20260629-herenow-retest-f092852/The Civic Desk_0.2.8_x64-setup.exe`

- Expected NSIS SHA256: `140F2893FFD77751E7C69E8542CEF2BA9AB664E8FE12E430AB1E435AFFBD108D`
- Observed NSIS SHA256: `140F2893FFD77751E7C69E8542CEF2BA9AB664E8FE12E430AB1E435AFFBD108D`

Fallback MSI:

`test-comms/artifacts/20260629-herenow-retest-f092852/The Civic Desk_0.2.8_x64_en-US.msi`

- Expected MSI SHA256: `8EA8D5F210A435AB8DBD06478AA3C5816C0CF0953281FAC44B3100287547E333`
- Observed MSI SHA256: `8EA8D5F210A435AB8DBD06478AA3C5816C0CF0953281FAC44B3100287547E333`

The NSIS installer was used. MSI fallback was not needed.

Evidence: `installer-hashes.json`

## App And Output Folder

The app was launched as a normal user. The visible Publishing screen showed:

- Publication: `Longmont Cleanroom Test`
- Tagline: `Temporary cleanroom test publication for Longmont civic coverage.`
- Output folder:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-cleanwipe-longmont-5a24a5a-evidence\publication-output\site`

The selected output folder is the proven 5a24a5a output package required by the directive.

Screenshots:

- `01-publishing-before-connector-test.png`
- `02-connector-test-passed.png`
- `03-publish-result.png`

JSON evidence:

- `launch-result.json`
- `ui-publish-state.json`

## Connector Test And Publish

The here.now connector test passed with the visible app message:

`Test passed: here.now is ready for temporary preview publishing. Save an API key for permanent sites.`

Anonymous here.now publishing then succeeded through the visible app UI. The visible result was:

`Saved live URL: https://merry-frost-9arx.here.now`

No user-visible publish error occurred.

## Public here.now Verification

Public URL checked:

`https://merry-frost-9arx.here.now`

HTTP result:

- Status: `200`
- Content-Type: `text/html; charset=utf-8`
- HTML bytes: `4803`

Identity checks:

- `<title>`: `Longmont Cleanroom Test`
- `<h1>`: `Longmont Cleanroom Test`
- Contains `Longmont Cleanroom Test`: yes
- `My Local Publication` in public title/header text: no

Article title checks:

- Public article titles beginning with `Draft:`: no

Mojibake scanner:

- Result: PASS
- Hits: none

Downloaded HTML:

- `herenow-index.html`

Screenshot:

- `04-herenow-page.png`

JSON evidence:

- `herenow-checks.json`

## Notes

During evidence cleanup I accidentally removed the first set of screenshots while trying to delete only temporary helper scripts from the evidence folder. I immediately reran the connector test and publish from the live visible app state, producing the fresh public URL and complete evidence set listed above. The committed evidence reflects the regenerated successful run.
