#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const tag = process.argv[2] || "";
const repoRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
const head = execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim();

function fail(message) {
  console.error(`FAIL: ${message}`);
  process.exit(1);
}

function requireCleanString(value, field) {
  if (typeof value !== "string" || !value.trim()) {
    fail(`${field} must be a non-empty string.`);
  }
  if (/placeholder|todo|tbd|fixme|example/i.test(value)) {
    fail(`${field} contains placeholder text.`);
  }
  return value.trim();
}

function requireSha256(value, field) {
  const clean = requireCleanString(value, field);
  if (!/^[a-fA-F0-9]{64}$/.test(clean)) {
    fail(`${field} must be a 64-character SHA256 hex value.`);
  }
  return clean.toLowerCase();
}

function requireOkSection(evidence, field) {
  const section = evidence[field];
  if (!section || section.ok !== true) {
    fail(`${field}.ok must be true.`);
  }
  return section;
}

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8").replace(/^\uFEFF/, ""));
}

if (!/^v\d+\.\d+\.\d+/.test(tag)) {
  fail("usage: verify-hosted-release-evidence.mjs <vX.Y.Z tag>");
}

const evidencePath = join(repoRoot, "docs", "release-evidence", `${tag}.json`);
if (!existsSync(evidencePath)) {
  fail(`missing hosted release evidence file: docs/release-evidence/${tag}.json`);
}

let evidence;
try {
  evidence = readJson(evidencePath);
} catch (error) {
  fail(`could not parse ${evidencePath}: ${error.message}`);
}

if (requireCleanString(evidence.tag, "tag") !== tag) {
  fail(`evidence tag ${evidence.tag} does not match workflow tag ${tag}.`);
}
if (requireCleanString(evidence.commit, "commit") !== head) {
  fail(`evidence commit ${evidence.commit} does not match workflow HEAD ${head}.`);
}

requireCleanString(evidence.generated_at, "generated_at");
requireCleanString(evidence.rc_receipt_path, "rc_receipt_path");
requireSha256(evidence.rc_receipt_sha256, "rc_receipt_sha256");

for (const field of ["release_smoke", "model_bakeoff", "dependency_audit", "windows_installer_smoke", "packaged_first_run_walkthrough"]) {
  const section = requireOkSection(evidence, field);
  requireSha256(section.receipt_sha256, `${field}.receipt_sha256`);
}

const installerSmoke = evidence.windows_installer_smoke;
requireSha256(installerSmoke.installer_sha256, "windows_installer_smoke.installer_sha256");
requireCleanString(installerSmoke.installer_name, "windows_installer_smoke.installer_name");

const cleanroom = requireOkSection(evidence, "cleanroom");
requireCleanString(cleanroom.report_path, "cleanroom.report_path");
requireSha256(cleanroom.report_sha256, "cleanroom.report_sha256");
requireCleanString(cleanroom.tester_machine, "cleanroom.tester_machine");
requireSha256(cleanroom.installer_sha256, "cleanroom.installer_sha256");
if (cleanroom.installer_sha256.toLowerCase() !== installerSmoke.installer_sha256.toLowerCase()) {
  fail("cleanroom.installer_sha256 must match windows_installer_smoke.installer_sha256.");
}

console.log(`OK: hosted release evidence for ${tag} matches commit ${head}.`);
