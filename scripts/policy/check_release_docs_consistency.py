#!/usr/bin/env python3
"""Check release-scope documentation claims against the current beta contract.

This is intentionally narrow. It protects the claims that have repeatedly drifted:
Windows-only public-beta installer proof, unsigned installer honesty, here.now as
the live-tested default, credentialed providers as proof-needed, Cloudflare as
assisted/manual, app-managed local AI setup, hosted release evidence, and
portable fixture paths.
"""

from __future__ import annotations

from pathlib import Path
import sys


REPO_ROOT = Path(__file__).resolve().parents[2]


def read(path: str) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def require(path: str, needle: str, failures: list[str]) -> None:
    if needle.lower() not in read(path).lower():
        failures.append(f"{path}: missing `{needle}`")


def forbid(path: str, needle: str, failures: list[str]) -> None:
    if needle.lower() in read(path).lower():
        failures.append(f"{path}: forbidden stale claim `{needle}`")


def main() -> int:
    failures: list[str] = []

    require("README.md", "Local isolated-profile smoke", failures)
    require("README.md", "queued for final cleanroom recheck", failures)
    require("README.md", "docs/release-evidence/v0.3.2.json", failures)
    require("README.md", "v0.3.2 is a Windows public beta", failures)
    forbid("README.md", "true clean-machine or remote tester run is still required", failures)

    require("CONTRIBUTING.md", "app-managed local AI setup", failures)
    require("CONTRIBUTING.md", "not a required source-build step", failures)
    forbid("CONTRIBUTING.md", "Ollama is bundled as a sidecar binary", failures)
    forbid("CONTRIBUTING.md", "REQUIRED: downloads", failures)

    for path in ("docs/install.md", "docs/user_manual.md", "docs/release-readiness.md"):
        require(path, "backlog/proof-needed", failures)
        require(path, "macOS and Linux", failures)

    require("docs/install.md", "passed final remote cleanroom testing", failures)
    require("docs/install.md", "https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2", failures)
    require("docs/release-readiness.md", "Current v0.3.2 evidence", failures)
    require("docs/release-readiness.md", "docs/release-evidence/v0.3.2.json", failures)
    require("docs/release-evidence/v0.3.2.json", "38e328bdaa0a74d4f59b30b63738d5ece8cf7f5c", failures)
    require("docs/release-evidence/v0.3.2.json", "B71F47CCD68DC41280C209C5BCB91B143BE6F20A6B65513045DE001A2FE7B37D", failures)

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
