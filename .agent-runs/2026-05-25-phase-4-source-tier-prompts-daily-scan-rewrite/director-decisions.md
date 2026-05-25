# Director Decisions

- DR-1 (get_prompt validation): Adopted researcher recommendation. Implement strict check against an enumerated list in `get_prompt` to prevent path-traversal.
- DR-2 (since_hours bounds): Adopted researcher recommendation. Explicitly enforce `0 < since_hours <= 168` in `run_daily_scan`.
