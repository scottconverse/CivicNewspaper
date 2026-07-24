#!/usr/bin/env node
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const args = new Map();
for (let i = 2; i < process.argv.length; i += 2) {
  args.set(process.argv[i], process.argv[i + 1]);
}

const assetsDir = args.get("--assets-dir");
const manifestPath = args.get("--manifest");
const evidencePath = args.get("--evidence");
const windowsReceiptPath = args.get("--windows-receipt");
const macosReceiptPath = args.get("--macos-receipt");

function fail(message) {
  console.error(`FAIL: ${message}`);
  process.exit(1);
}

function sha256File(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8").replace(/^\uFEFF/, ""));
}

function cleanHash(value, field) {
  if (typeof value !== "string" || !/^[a-fA-F0-9]{64}$/.test(value.trim())) {
    fail(`${field} must be a 64-character SHA256 hex value.`);
  }
  return value.trim().toLowerCase();
}

if (!assetsDir || !manifestPath) {
  fail("usage: verify-release-asset-hashes.mjs --assets-dir <dir> --manifest <SHA256SUMS> [--evidence <json>] [--windows-receipt <json>] [--macos-receipt <json>]");
}
if (!existsSync(assetsDir)) {
  fail(`assets directory does not exist: ${assetsDir}`);
}
if (!existsSync(manifestPath)) {
  fail(`SHA256SUMS manifest does not exist: ${manifestPath}`);
}

const manifest = new Map();
for (const [index, line] of readFileSync(manifestPath, "utf8").split(/\r?\n/).entries()) {
  if (!line.trim() || line.trimStart().startsWith("#")) continue;
  const match = line.match(/^([a-fA-F0-9]{64})\s+(.+)$/);
  if (!match) {
    fail(`invalid SHA256SUMS line ${index + 1}: ${line}`);
  }
  const hash = cleanHash(match[1], `SHA256SUMS line ${index + 1}`);
  const fileName = match[2].trim();
  if (!fileName || fileName.includes("/") || fileName.includes("\\")) {
    fail(`invalid SHA256SUMS filename on line ${index + 1}: ${fileName}`);
  }
  if (manifest.has(fileName)) {
    fail(`SHA256SUMS lists duplicate asset: ${fileName}`);
  }
  manifest.set(fileName, hash);
}

if (manifest.size === 0) {
  fail("SHA256SUMS has no asset hashes.");
}

for (const [fileName, expectedHash] of manifest) {
  const assetPath = join(assetsDir, fileName);
  if (!existsSync(assetPath)) {
    fail(`SHA256SUMS lists '${fileName}' but the downloaded asset is missing.`);
  }
  const actualHash = sha256File(assetPath);
  if (actualHash !== expectedHash) {
    fail(`asset hash mismatch for '${fileName}': manifest=${expectedHash} actual=${actualHash}`);
  }
}

if (evidencePath && existsSync(evidencePath)) {
  let evidence;
  try {
    evidence = readJson(evidencePath);
  } catch (error) {
    fail(`could not parse release evidence ${evidencePath}: ${error.message}`);
  }
  if (evidence.evidence_schema === "hosted-exact-artifacts-v2") {
    if (evidence?.release_scope?.windows_public_beta === true) {
      if (!windowsReceiptPath || !existsSync(windowsReceiptPath)) {
        fail("Windows public-beta scope requires the exact hosted Windows signature-smoke receipt.");
      }
      let windowsReceipt;
      try {
        windowsReceipt = readJson(windowsReceiptPath);
      } catch (error) {
        fail(`could not parse Windows signature-smoke receipt ${windowsReceiptPath}: ${error.message}`);
      }
      if (windowsReceipt.ok !== true) {
        fail("Windows signature-smoke receipt must report ok=true.");
      }
      const requiredExecutables = new Set(["installer", "application", "uninstaller"]);
      for (const executable of windowsReceipt.executables || []) {
        if (executable?.status === "Valid" && executable?.timestamp_subject) {
          requiredExecutables.delete(executable.name);
        }
      }
      if (requiredExecutables.size !== 0) {
        fail(`Windows signature-smoke receipt is missing valid timestamped proof for: ${[...requiredExecutables].join(", ")}.`);
      }
      const releaseAssetName = windowsReceipt.release_asset_name;
      if (typeof releaseAssetName !== "string" || !releaseAssetName.trim()) {
        fail("Windows signature-smoke receipt is missing release_asset_name.");
      }
      const expectedInstallerHash = cleanHash(
        windowsReceipt.installer_sha256,
        "Windows signature-smoke installer_sha256",
      );
      const manifestHash = manifest.get(releaseAssetName);
      if (!manifestHash) {
        fail(`SHA256SUMS does not list the Windows release asset '${releaseAssetName}'.`);
      }
      if (manifestHash !== expectedInstallerHash) {
        fail(
          `published Windows hash does not match hosted signature-smoke hash for '${releaseAssetName}': ` +
            `manifest=${manifestHash} receipt=${expectedInstallerHash}`,
        );
      }
    }
  } else {
    const installerName = evidence?.windows_installer_smoke?.installer_name;
    const releaseAssetName = evidence?.windows_installer_smoke?.release_asset_name || installerName;
    if (typeof installerName !== "string" || !installerName.trim()) {
      fail("release evidence is missing windows_installer_smoke.installer_name.");
    }
    if (typeof releaseAssetName !== "string" || !releaseAssetName.trim()) {
      fail("release evidence is missing windows_installer_smoke.release_asset_name.");
    }
    const expectedInstallerHash = cleanHash(
      evidence?.windows_installer_smoke?.installer_sha256,
      "windows_installer_smoke.installer_sha256"
    );
    const cleanroomHash = cleanHash(evidence?.cleanroom?.installer_sha256, "cleanroom.installer_sha256");
    if (cleanroomHash !== expectedInstallerHash) {
      fail("cleanroom installer SHA256 does not match Windows installer-smoke SHA256.");
    }
    const manifestHash = manifest.get(releaseAssetName);
    if (!manifestHash) {
      fail(`SHA256SUMS does not list the release installer asset '${releaseAssetName}'.`);
    }
    if (manifestHash !== expectedInstallerHash) {
      fail(
        `published installer hash does not match cleanroom-tested hash for '${releaseAssetName}': ` +
          `manifest=${manifestHash} evidence=${expectedInstallerHash}`
      );
    }
  }

  if (evidence?.release_scope?.macos_apple_silicon_beta === true) {
    if (!macosReceiptPath || !existsSync(macosReceiptPath)) {
      fail("Apple Silicon release scope requires the exact hosted macOS packaged-smoke receipt.");
    }
    let macReceipt;
    try {
      macReceipt = readJson(macosReceiptPath);
    } catch (error) {
      fail(`could not parse macOS packaged-smoke receipt ${macosReceiptPath}: ${error.message}`);
    }
    if (macReceipt.ok !== true) {
      fail("macOS packaged-smoke receipt must report ok=true.");
    }
    if (macReceipt.architecture !== "aarch64") {
      fail("macOS packaged-smoke receipt must report architecture=aarch64.");
    }
    if (macReceipt.developer_id_signed !== false || macReceipt.notarized !== false) {
      fail("macOS packaged-smoke receipt must record Developer ID signing and notarization as false.");
    }
    const macAssetName = macReceipt.artifact_name;
    if (typeof macAssetName !== "string" || !macAssetName.trim()) {
      fail("macOS packaged-smoke receipt is missing artifact_name.");
    }
    const expectedMacHash = cleanHash(
      macReceipt.artifact_sha256,
      "macOS packaged-smoke artifact_sha256",
    );
    const publishedMacHash = manifest.get(macAssetName);
    if (!publishedMacHash) {
      fail(`SHA256SUMS does not list the macOS release asset '${macAssetName}'.`);
    }
    if (publishedMacHash !== expectedMacHash) {
      fail(
        `published macOS hash does not match cleanroom-tested hash for '${macAssetName}': ` +
          `manifest=${publishedMacHash} evidence=${expectedMacHash}`,
      );
    }
  }
}

console.log(`OK: verified ${manifest.size} published asset hash${manifest.size === 1 ? "" : "es"}.`);
