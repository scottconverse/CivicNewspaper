# Directive Update: Product Commit To Test

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit to test: `e423f5f`

The previous GauntletGate critical finding for narrow/mobile navigation was fixed before cleanroom artifact testing.

When you run the cleanroom first-run directive, make sure `stable-readiness-local-gates` is at or after commit `e423f5f`.

The cleanroom report should still verify narrow-window behavior in the real desktop app:

- resize the app window narrow, roughly phone/small-tablet width if possible;
- confirm selecting Sources, Publishing, Workbench, and System keeps the selected page content visible/reachable;
- report any remaining layout trap or clipped navigation issue.
