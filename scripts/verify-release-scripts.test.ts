import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { afterEach, describe, expect, test } from "vitest";
import { createHash, randomUUID } from "node:crypto";

const root = process.cwd();
const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).trim();
const writtenEvidenceFiles: string[] = [];
const tempDirs: string[] = [];

function runNode(script: string, args: string[], cwd = root) {
  try {
    const stdout = execFileSync(process.execPath, [script, ...args], {
      cwd,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
    return { ok: true, output: stdout };
  } catch (error: any) {
    return {
      ok: false,
      output: `${error.stdout ?? ""}${error.stderr ?? ""}`,
    };
  }
}

function sha256Text(text: string) {
  return createHash("sha256").update(text).digest("hex");
}

function writeHostedEvidenceFixture(tag: string, overrides: Record<string, any> = {}) {
  const evidence = {
    tag,
    commit: head,
    generated_at: "2026-07-02T12:00:00Z",
    rc_receipt_path: ".agent-runs/release-candidate/release-candidate-receipt.json",
    rc_receipt_sha256: "a".repeat(64),
    release_smoke: { ok: true, receipt_sha256: "b".repeat(64) },
    model_bakeoff: { ok: true, receipt_sha256: "c".repeat(64) },
    dependency_audit: { ok: true, receipt_sha256: "d".repeat(64) },
    windows_installer_smoke: {
      ok: true,
      receipt_sha256: "e".repeat(64),
      installer_name: "The Civic Desk_0.3.2_x64-setup.exe",
      installer_sha256: "f".repeat(64),
    },
    packaged_first_run_walkthrough: { ok: true, receipt_sha256: "1".repeat(64) },
    cleanroom: {
      ok: true,
      report_path: "test-comms/reports/final.md",
      report_sha256: "2".repeat(64),
      tester_machine: "msi-civic",
      installer_sha256: "f".repeat(64),
    },
    ...overrides,
  };
  const path = join(root, "docs", "release-evidence", `${tag}.json`);
  writeFileSync(path, JSON.stringify(evidence, null, 2));
  writtenEvidenceFiles.push(path);
}

describe("release verifier scripts", () => {
  afterEach(() => {
    while (writtenEvidenceFiles.length) {
      rmSync(writtenEvidenceFiles.pop()!, { force: true });
    }
    while (tempDirs.length) {
      rmSync(tempDirs.pop()!, { recursive: true, force: true });
    }
  });

  test("hosted release evidence accepts a complete exact-tag fixture", () => {
    const tag = `v0.0.999-test-${process.pid}-${randomUUID()}`;
    writeHostedEvidenceFixture(tag);

    const result = runNode("scripts/verify-hosted-release-evidence.mjs", [tag]);

    expect(result.ok).toBe(true);
    expect(result.output).toContain(`OK: hosted release evidence for ${tag}`);
  });

  test("hosted release evidence rejects missing cleanroom fields and hash mismatches", () => {
    const tag = `v0.0.998-test-${process.pid}-${randomUUID()}`;
    writeHostedEvidenceFixture(tag, {
      cleanroom: {
        ok: true,
        report_path: "test-comms/reports/final.md",
        report_sha256: "2".repeat(64),
        tester_machine: "msi-civic",
        installer_sha256: "0".repeat(64),
      },
    });

    const result = runNode("scripts/verify-hosted-release-evidence.mjs", [tag]);

    expect(result.ok).toBe(false);
    expect(result.output).toContain("cleanroom.installer_sha256 must match");
  });

  test("asset hash verifier binds SHA256SUMS to cleanroom-tested installer", () => {
    const dir = mkdtempSync(join(tmpdir(), "civic-assets-"));
    tempDirs.push(dir);
    const assetName = "The Civic Desk_0.3.2_x64-setup.exe";
    const assetBody = "installer bytes";
    const assetHash = sha256Text(assetBody);
    writeFileSync(join(dir, assetName), assetBody);
    const manifest = join(dir, "SHA256SUMS");
    writeFileSync(manifest, `${assetHash}  ${assetName}\n`);
    const evidence = join(dir, "evidence.json");
    writeFileSync(
      evidence,
      JSON.stringify({
        windows_installer_smoke: { installer_name: assetName, installer_sha256: assetHash },
        cleanroom: { installer_sha256: assetHash },
      })
    );

    const result = runNode("scripts/verify-release-asset-hashes.mjs", [
      "--assets-dir",
      dir,
      "--manifest",
      manifest,
      "--evidence",
      evidence,
    ]);

    expect(result.ok).toBe(true);
    expect(result.output).toContain("OK: verified 1 published asset hash");
  });

  test("asset hash verifier uses release asset name when local installer name differs", () => {
    const dir = mkdtempSync(join(tmpdir(), "civic-assets-"));
    tempDirs.push(dir);
    const localInstallerName = "The Civic Desk_0.3.2_x64-setup.exe";
    const releaseAssetName = "The.Civic.Desk_0.3.2_x64-setup.exe";
    const assetBody = "installer bytes";
    const assetHash = sha256Text(assetBody);
    writeFileSync(join(dir, releaseAssetName), assetBody);
    const manifest = join(dir, "SHA256SUMS.txt");
    writeFileSync(manifest, `${assetHash}  ${releaseAssetName}\n`);
    const evidence = join(dir, "evidence.json");
    writeFileSync(
      evidence,
      JSON.stringify({
        windows_installer_smoke: {
          installer_name: localInstallerName,
          release_asset_name: releaseAssetName,
          installer_sha256: assetHash,
        },
        cleanroom: { installer_sha256: assetHash },
      })
    );

    const result = runNode("scripts/verify-release-asset-hashes.mjs", [
      "--assets-dir",
      dir,
      "--manifest",
      manifest,
      "--evidence",
      evidence,
    ]);

    expect(result.ok).toBe(true);
    expect(result.output).toContain("OK: verified 1 published asset hash");
  });

  test("asset hash verifier rejects manifest/evidence hash mismatch", () => {
    const dir = mkdtempSync(join(tmpdir(), "civic-assets-"));
    tempDirs.push(dir);
    const assetName = "The Civic Desk_0.3.2_x64-setup.exe";
    const assetBody = "installer bytes";
    const assetHash = sha256Text(assetBody);
    writeFileSync(join(dir, assetName), assetBody);
    const manifest = join(dir, "SHA256SUMS");
    writeFileSync(manifest, `${assetHash}  ${assetName}\n`);
    const evidence = join(dir, "evidence.json");
    writeFileSync(
      evidence,
      JSON.stringify({
        windows_installer_smoke: { installer_name: assetName, installer_sha256: "a".repeat(64) },
        cleanroom: { installer_sha256: "a".repeat(64) },
      })
    );

    const result = runNode("scripts/verify-release-asset-hashes.mjs", [
      "--assets-dir",
      dir,
      "--manifest",
      manifest,
      "--evidence",
      evidence,
    ]);

    expect(result.ok).toBe(false);
    expect(result.output).toContain("published installer hash does not match");
  });
});
