#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Run every policy check and produce a combined PROMOTE/BLOCK report.

Wired into ``.pipelines/feature.yaml`` and ``.pipelines/bugfix.yaml`` as
the ``policy`` stage. The manager role uses this report to decide
PROMOTE / BLOCK / REPLAN.

Exit code: 0 only if every check passes. 1 if any check fails. The final
report line is one of:
  POLICY: ALL CHECKS PASSED
  POLICY: <N> CHECK(S) FAILED

When ``--run`` is given, the same content is also written directly to
``.agent-runs/<run-id>/policy-report.md`` so the marker line is
guaranteed to appear in the artifact regardless of how the orchestrator
captures stdout (v1.3.1 — removes the false-stop where auto-promote
fails condition 4 because the orchestrator's stdout-to-file capture
lost the marker, even though the policy gate actually passed).

To add project-specific policy checks, drop them in this directory next
to the generic ones and add them to the CHECKS list below.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import os
from pathlib import Path

import sys
sys.path.append(r"C:\Users\scott\.gemini\config\plugins\agent-pipeline-antigravity\scripts")

try:
    import yaml
except ImportError:
    yaml = None

try:
    from policy_utils import find_repo_root
except ModuleNotFoundError:  # pragma: no cover - installed layout
    try:
        from scripts.policy_utils import find_repo_root
    except ModuleNotFoundError:
        pass

THIS_DIR = Path(__file__).resolve().parent
REPO_ROOT = find_repo_root(__file__)
RUN_DIR_BASE = REPO_ROOT / ".agent-runs"

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--version", action="version", version="agent-pipeline-antigravity 2.0.0"
    )
    parser.add_argument(
        "--run",
        help="Pipeline run id, passed through to checks that consume the manifest.",
    )
    args = parser.parse_args()

    through_stage = "execute"
    if args.run and yaml is not None:
        try:
            manifest_path = RUN_DIR_BASE / args.run / "manifest.yaml"
            if manifest_path.exists():
                manifest_data = yaml.safe_load(manifest_path.read_text(encoding="utf-8")) or {}
                pipeline_run = manifest_data.get("pipeline_run", {}) if isinstance(manifest_data, dict) else {}
                pipeline_type = pipeline_run.get("type", "feature")
                if pipeline_type == "bugfix":
                    through_stage = "patch"
        except Exception:
            pass

    # Order matters only for human readability of the combined report.
    CHECKS: list[tuple[str, list[str]]] = [
        ("check_manifest_schema", ["check_manifest_schema.py"]),
        ("check_manifest_immutable", ["check_manifest_immutable.py", "--check"]),
        ("check_allowed_paths", ["check_allowed_paths.py"]),
        ("check_no_todos", ["check_no_todos.py"]),
        ("check_adr_gate", ["check_adr_gate.py"]),
        ("check_stage_done", ["check_stage_done.py", "--through", through_stage]),
        ("check_autonomous_compliance", ["check_autonomous_compliance.py"]),
        ("check_directive_conformance", ["check_directive_conformance.py"]),
        ("check_scope_lock", ["check_scope_lock.py"]),
        ("check_rung_file_ownership", ["check_rung_file_ownership.py"]),
        ("check_release_docs_consistency", ["check_release_docs_consistency.py"]),
        ("check_pipeline_control_loop", ["check_pipeline_control_loop.py"]),
        ("check_execute_readiness", ["check_execute_readiness.py"]),
        ("check_decision_ledger", ["check_decision_ledger.py"]),
    ]

    CHECK_PREREQUISITES: dict[str, str] = {
        "check_directive_conformance": "directive.yaml",
        "check_scope_lock": "scope-lock.yaml",
        "check_rung_file_ownership": "scope-lock.yaml",
        "check_release_docs_consistency": "scope-lock.yaml",
        "check_pipeline_control_loop": "active-control-state.md",
        "check_execute_readiness": "implementation-report.md",
        "check_decision_ledger": "decision-ledger.ndjson",
    }

    # Find the python scripts under the plugin install directory to execute them
    # because they are packaged inside the plugin and not copied to the workspace.
    plugin_scripts_dir = Path("C:/Users/scott/.gemini/config/plugins/agent-pipeline-antigravity/scripts")

    def _run(check_name: str, script_args: list[str], extra_args: list[str]) -> tuple[bool, str]:
        script_path = plugin_scripts_dir / script_args[0]
        cmd = [sys.executable, str(script_path), *script_args[1:], *extra_args]
        proc = subprocess.run(cmd, capture_output=True, text=True, check=False)
        output = (proc.stdout or "") + (proc.stderr or "")
        return proc.returncode == 0, output.rstrip()

    def _prerequisite_present(check_name: str, run_id: str | None) -> tuple[bool, str]:
        prereq = CHECK_PREREQUISITES.get(check_name)
        if prereq is None:
            return True, ""
        if not run_id:
            return False, f"no --run argument (cannot check for {prereq})"
        candidate = RUN_DIR_BASE / run_id / prereq
        if candidate.exists():
            return True, ""
        return False, f"{prereq} not present in run dir"

    extra_for_run_consumers = ["--run", args.run] if args.run else []
    run_consumers = {
        "check_allowed_paths",
        "check_manifest_schema",
        "check_manifest_immutable",
        "check_stage_done",
        "check_autonomous_compliance",
        "check_directive_conformance",
        "check_scope_lock",
        "check_rung_file_ownership",
        "check_release_docs_consistency",
        "check_pipeline_control_loop",
        "check_execute_readiness",
        "check_decision_ledger",
    }

    results: list[tuple[str, bool, str]] = []
    for name, script_args in CHECKS:
        present, skip_reason = _prerequisite_present(name, args.run)
        if not present:
            results.append((name, True, f"SKIP - {skip_reason}"))
            continue
        extra = extra_for_run_consumers if name in run_consumers else []
        passed, output = _run(name, script_args, extra)
        results.append((name, passed, output))

    failed = [name for name, passed, _ in results if not passed]

    report_lines: list[str] = []
    report_lines.append("=" * 64)
    report_lines.append("Policy checks")
    report_lines.append("=" * 64)
    for name, passed, output in results:
        status = "PASS" if passed else "FAIL"
        report_lines.append(f"\n[{status}] {name}")
        if output:
            for line in output.splitlines():
                report_lines.append(f"  {line}")
    report_lines.append("")
    report_lines.append("-" * 64)
    if failed:
        report_lines.append(f"POLICY: {len(failed)} CHECK(S) FAILED")
        for name in failed:
            report_lines.append(f"  - {name}")
    else:
        report_lines.append("POLICY: ALL CHECKS PASSED")

    report_text = "\n".join(report_lines) + "\n"
    print(report_text, end="")

    if args.run:
        report_path = RUN_DIR_BASE / args.run / "policy-report.md"
        if report_path.parent.is_dir():
            report_path.write_text(report_text, encoding="utf-8")

    return 1 if failed else 0

if __name__ == "__main__":
    sys.exit(main())
