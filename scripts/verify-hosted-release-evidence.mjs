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
const evidenceCommit = requireCleanString(evidence.commit, "commit");
try {
  execFileSync("git", ["merge-base", "--is-ancestor", evidenceCommit, head], { stdio: "ignore" });
} catch {
  fail(`evidence commit ${evidenceCommit} is not an ancestor of workflow HEAD ${head}.`);
}

requireCleanString(evidence.generated_at, "generated_at");

if (evidence.evidence_schema === "hosted-exact-artifacts-v2") {
  const scope = evidence.release_scope;
  if (scope?.windows_public_beta !== true) {
    fail("release_scope.windows_public_beta must be true.");
  }
  if (scope?.macos_apple_silicon_beta !== true) {
    fail("release_scope.macos_apple_silicon_beta must be true.");
  }
  if (scope?.linux_release !== "deferred-v0.3.4") {
    fail("release_scope.linux_release must be deferred-v0.3.4.");
  }
  if (scope?.scott_approved_public_release !== true) {
    fail("release_scope.scott_approved_public_release must be true.");
  }

  const windowsHostedProof = requireOkSection(evidence, "windows_hosted_proof");
  if (
    requireCleanString(windowsHostedProof.receipt_asset, "windows_hosted_proof.receipt_asset")
    !== "windows-signature-smoke-receipt.json"
  ) {
    fail("windows_hosted_proof.receipt_asset must be windows-signature-smoke-receipt.json.");
  }
  for (const executable of ["installer", "application", "uninstaller"]) {
    if (!windowsHostedProof.required_executables?.includes(executable)) {
      fail(`windows_hosted_proof.required_executables must include ${executable}.`);
    }
  }

  const macPreflight = requireOkSection(evidence, "macos_preflight");
  requireCleanString(macPreflight.receipt_path, "macos_preflight.receipt_path");
  requireSha256(macPreflight.receipt_sha256, "macos_preflight.receipt_sha256");
  requireSha256(macPreflight.artifact_sha256, "macos_preflight.artifact_sha256");
  if (requireCleanString(macPreflight.architecture, "macos_preflight.architecture") !== "aarch64") {
    fail("macos_preflight.architecture must be aarch64.");
  }
  if (requireCleanString(macPreflight.minimum_macos, "macos_preflight.minimum_macos") !== "11.0") {
    fail("macos_preflight.minimum_macos must be 11.0.");
  }
  if (macPreflight.developer_id_signed !== false || macPreflight.notarized !== false) {
    fail("the v0.3.3 macOS beta must explicitly record Developer ID signing and notarization as false.");
  }
  if (requireCleanString(macPreflight.ollama_setup, "macos_preflight.ollama_setup") !== "manual") {
    fail("macos_preflight.ollama_setup must be manual.");
  }

  console.log(`OK: hosted release evidence for ${tag} matches commit ${head}.`);
  process.exit(0);
}

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

if (evidence.release_scope?.macos_apple_silicon_beta === true) {
  const macPreflight = requireOkSection(evidence, "macos_preflight");
  requireSha256(macPreflight.receipt_sha256, "macos_preflight.receipt_sha256");
  if (requireCleanString(macPreflight.architecture, "macos_preflight.architecture") !== "aarch64") {
    fail("macos_preflight.architecture must be aarch64.");
  }
  if (requireCleanString(macPreflight.minimum_macos, "macos_preflight.minimum_macos") !== "11.0") {
    fail("macos_preflight.minimum_macos must be 11.0.");
  }
  if (macPreflight.developer_id_signed !== false || macPreflight.notarized !== false) {
    fail("the v0.3.3 macOS beta must explicitly record Developer ID signing and notarization as false.");
  }
  if (requireCleanString(macPreflight.ollama_setup, "macos_preflight.ollama_setup") !== "manual") {
    fail("macos_preflight.ollama_setup must be manual.");
  }
}

console.log(`OK: hosted release evidence for ${tag} matches commit ${head}.`);
