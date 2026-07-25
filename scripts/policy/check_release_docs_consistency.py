#!/usr/bin/env python3
"""Check public release claims against the published v0.3.3 contract."""

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[2]
RELEASE_TAG = "v0.3.3"
RELEASE_COMMIT = "e94a2f94885c1e6013129c3d854662cc3c8e5b27"
WINDOWS_NAME = "The.Civic.Desk_0.3.3_x64-setup.exe"
WINDOWS_SHA256 = "3d08ec394d87329043acd57f8f714cdcfdf10b3670631861ba16bc397c6befd2"
WINDOWS_SIZE = "5343200"
MAC_NAME = "The.Civic.Desk_0.3.3_aarch64.dmg"
MAC_SHA256 = "95c82afa6549a5648e919306b3fbc6b7f7336ee331ca7f3c7091d87d3d11f01b"
MAC_SIZE = "7891351"

CURRENT_DOCS = (
    "README.md",
    "FAQ.md",
    "docs/index.html",
    "docs/install.md",
    "docs/user_manual.md",
    "docs/release-readiness.md",
    "docs/implementation-plan-v0.3.0-to-v1.0.0.md",
    "docs/prd-local-llm-newsroom-v1.md",
    "docs/discussion_seeds.md",
)


def read(path: str) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def require(path: str, needle: str, failures: list[str]) -> None:
    if needle.lower() not in read(path).lower():
        failures.append(f"{path}: missing `{needle}`")


def forbid(path: str, needle: str, failures: list[str]) -> None:
    if needle.lower() in read(path).lower():
        failures.append(f"{path}: forbidden stale claim `{needle}`")


def check_release_body(failures: list[str]) -> None:
    if os.environ.get("CIVICNEWS_SKIP_GITHUB_RELEASE_CHECK") == "1":
        return
    gh = shutil.which("gh")
    if not gh:
        failures.append("GitHub release body check requires `gh`.")
        return
    result = subprocess.run(
        [gh, "release", "view", RELEASE_TAG, "--json", "body", "--jq", ".body"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        failures.append(
            f"GitHub release body check failed: {result.stderr.strip() or result.stdout.strip()}"
        )
        return
    for expected in (
        RELEASE_COMMIT,
        WINDOWS_NAME,
        WINDOWS_SHA256,
        WINDOWS_SIZE,
        MAC_NAME,
        MAC_SHA256,
        MAC_SIZE,
        "macOS 11",
        "unsigned",
        "unnotarized",
        "v0.3.4",
    ):
        if expected.lower() not in result.stdout.lower():
            failures.append(f"GitHub release body missing `{expected}`")


def main() -> int:
    failures: list[str] = []

    for path in ("README.md", "docs/index.html", "docs/install.md"):
        for value in (
            RELEASE_COMMIT,
            WINDOWS_NAME,
            WINDOWS_SHA256,
            MAC_NAME,
            MAC_SHA256,
            "macOS 11",
            "v0.3.4",
        ):
            require(path, value, failures)

    for path in CURRENT_DOCS:
        forbid(path, "v0.3.3 candidate", failures)
        forbid(path, "planned v0.3.3", failures)
        forbid(path, "currently published v0.3.2", failures)

    require("README.md", WINDOWS_SIZE, failures)
    require("README.md", MAC_SIZE, failures)
    require("README.md", "not Developer ID signed or notarized", failures)
    require("docs/index.html", "Get v0.3.3 for Mac or Windows", failures)
    require("docs/install.md", "Control-click", failures)
    require("docs/install.md", "Do not disable Gatekeeper globally", failures)
    require("docs/architecture.md", "WebView2", failures)
    require("docs/release-readiness.md", "tests\\fixtures\\source-import", failures)
    forbid(
        "docs/release-readiness.md",
        "C:\\Users\\instynct\\Desktop\\CivicNewspaperTestFiles",
        failures,
    )

    # Historical v0.3.2 proof remains intentionally preserved and labeled.
    require(
        "docs/release-readiness.md",
        "Historical v0.3.2 evidence",
        failures,
    )
    require(
        "docs/release-evidence/v0.3.2.json",
        "bfa37f87dda8aa61c98da4bd7bc2be907581a416",
        failures,
    )
    require("docs/release-evidence/v0.3.3.json", '"tag": "v0.3.3"', failures)
    check_release_body(failures)

    if failures:
        print("Release docs consistency check failed:")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print("OK: public docs match the published v0.3.3 Windows and Apple Silicon contract.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
