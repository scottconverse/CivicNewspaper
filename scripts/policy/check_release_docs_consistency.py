#!/usr/bin/env python3
"""Check release-scope documentation claims against the current beta contract.

This is intentionally narrow. It protects the claims that have repeatedly drifted:
Windows-only public-beta installer proof, Authenticode verification, here.now as
the live-tested default, credentialed providers as proof-needed, Cloudflare as
assisted/manual, app-managed local AI setup, hosted release evidence, and
portable fixture paths.
"""

from __future__ import annotations

from pathlib import Path
import os
import shutil
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[2]
RELEASE_TAG = "v0.3.2"
LOCAL_CANDIDATE_COMMIT = "bfa37f87dda8aa61c98da4bd7bc2be907581a416"
LOCAL_CANDIDATE_SHA256 = "636D87041396603456634E6B47AE1071E8726D8D05C0FC08768D5B9E92A71C83"
LOCAL_CANDIDATE_SIZE = "5274104"
PUBLISHED_COMMIT = "796b8700831f964beea97630c51d71a40a9b724f"
PUBLISHED_SHA256 = "BDA7CE85759AD1C475D100D0F04FBC7F3CAF7DFF07DDB74F60B24F1CAAF526DD"
PUBLISHED_SIZE = "5342976"
STALE_PUBLISHED_VALUES = (
    "35e6cf0f4a8f01d74ef79247feaaadbd34dbb3da",
    "8204BB4210DD284518D114C57A3089BAC11D7B0EC8E0F83D8D61928D44FEB6E0",
    "5240548",
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
    if not shutil.which("gh"):
        failures.append("GitHub release body check requires `gh`, or set CIVICNEWS_SKIP_GITHUB_RELEASE_CHECK=1 for offline diagnostics.")
        return
    result = subprocess.run(
        ["gh", "release", "view", RELEASE_TAG, "--json", "body", "--jq", ".body"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        failures.append(f"GitHub release body check failed: {result.stderr.strip() or result.stdout.strip()}")
        return
    body = result.stdout
    for expected in (PUBLISHED_COMMIT, PUBLISHED_SHA256, PUBLISHED_SIZE):
        if expected.lower() not in body.lower():
            failures.append(f"GitHub release body missing `{expected}`")
    for stale in STALE_PUBLISHED_VALUES:
        if stale.lower() in body.lower():
            failures.append(f"GitHub release body contains stale `{stale}`")


def main() -> int:
    failures: list[str] = []

    require("README.md", "AuthentiCode-signs and timestamps", failures)
    require("README.md", PUBLISHED_COMMIT, failures)
    require("README.md", PUBLISHED_SHA256, failures)
    require("README.md", PUBLISHED_SIZE, failures)
    require("README.md", "downloadable GitHub asset is the signed, cleanroom-tested candidate", failures)
    require("README.md", "preserved as historical evidence", failures)
    require("README.md", "docs/release-evidence/v0.3.2.json", failures)
    require("README.md", "v0.3.2 is a Windows public beta", failures)
    require("README.md", "initial installation requires an internet connection", failures)
    forbid("README.md", "queued for final cleanroom recheck", failures)
    forbid("README.md", "true clean-machine or remote tester run is still required", failures)

    require("CONTRIBUTING.md", "app-managed local AI setup", failures)
    require("CONTRIBUTING.md", "not a required source-build step", failures)
    forbid("CONTRIBUTING.md", "Ollama is bundled as a sidecar binary", failures)
    forbid("CONTRIBUTING.md", "REQUIRED: downloads", failures)

    for path in ("docs/install.md", "docs/user_manual.md", "docs/release-readiness.md"):
        require(path, "backlog/proof-needed", failures)
        require(path, "macOS and Linux", failures)

    require("docs/install.md", "records an earlier candidate", failures)
    require("docs/install.md", PUBLISHED_COMMIT, failures)
    require("docs/install.md", PUBLISHED_SHA256, failures)
    require("docs/install.md", PUBLISHED_SIZE, failures)
    require("docs/install.md", "https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2", failures)
    require("docs/install.md", "initial installation requires an internet connection", failures)
    require("docs/user_manual.md", "initial installation requires an internet connection", failures)
    require("docs/index.html", "initial installation requires an internet connection", failures)
    require("docs/architecture.md", "WebView2", failures)
    require("docs/release-readiness.md", "Historical v0.3.2 evidence and the final-candidate rule", failures)
    require("docs/release-readiness.md", "docs/release-evidence/v0.3.2.json", failures)
    require("docs/release-evidence/v0.3.2.json", LOCAL_CANDIDATE_COMMIT, failures)
    require("docs/release-evidence/v0.3.2.json", LOCAL_CANDIDATE_SHA256, failures)
    require("docs/release-evidence/v0.3.2.json", LOCAL_CANDIDATE_SIZE, failures)
    require("docs/release-evidence/v0.3.2-local-isolated-package-report.md", LOCAL_CANDIDATE_COMMIT, failures)
    require("docs/index.html", PUBLISHED_SHA256, failures)
    require("docs/index.html", "Download signed Windows beta", failures)
    forbid("docs/index.html", "Candidate pending", failures)
    check_release_body(failures)

    require("docs/publishing-connectors.md", "anonymous here.now preview publishing is the tested default fast path", failures)
    require("docs/publishing-connectors.md", "Cloudflare Pages API publishing is disabled", failures)
    require("docs/publishing-connectors.md", "require user-owned credentials and real target accounts", failures)
    require("docs/publishing-connectors.md", "release-specific live proof", failures)

    require("docs/release-readiness.md", "tests\\fixtures\\source-import", failures)
    forbid("docs/release-readiness.md", "C:\\Users\\instynct\\Desktop\\CivicNewspaperTestFiles", failures)

    if failures:
        print("Release docs consistency check failed:")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print("OK: release docs match the Windows public-beta proof contract.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
