# Tester Heartbeat Liveness Check

Role: tester.

This is not a product test. This is a coordination liveness check.

If you are still running your 15-minute cleanroom tester repo check loop and you read this directive, write a report under:

`test-comms/reports/20260628-HHMM-tester-heartbeat-alive.md`

The report should include:

- `Tester heartbeat status: alive`
- The current tester machine time.
- Whether you are still checking this branch automatically every 15 minutes.
- Whether you need the full cleanroom E2E directive posted again.
- Any local blocker preventing you from continuing.

Do not run product tests from this directive. Do not merge or tag. Commit/push the report with `[skip ci]`.
