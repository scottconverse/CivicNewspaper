#!/usr/bin/env node

import { createHash } from "node:crypto";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { basename, join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { execFileSync, spawn } from "node:child_process";

function fail(message) {
  throw new Error(message);
}

function command(commandName, args, options = {}) {
  return execFileSync(commandName, args, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
    ...options,
  }).trim();
}

function commandDiagnostic(commandName, args) {
  try {
    return command(commandName, args);
  } catch (error) {
    return `${error.stdout ?? ""}${error.stderr ?? ""}`.trim();
  }
}

function listFiles(root) {
  const files = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const path = join(root, entry.name);
    if (entry.isDirectory()) files.push(...listFiles(path));
    else if (entry.isFile()) files.push(path);
  }
  return files;
}

function sha256(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function delay(ms) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, ms));
}

async function main() {
  if (process.platform !== "darwin") {
    fail("macos-packaged-smoke.mjs must run on macOS.");
  }

  const artifact = resolve(process.argv[2] || "");
  const receiptArgumentIndex = process.argv.indexOf("--receipt");
  const requestedReceipt = receiptArgumentIndex >= 0
    ? resolve(process.argv[receiptArgumentIndex + 1] || "")
    : "";
  if (!artifact || !existsSync(artifact) || !artifact.endsWith(".dmg")) {
    fail("usage: node scripts/macos-packaged-smoke.mjs <Apple Silicon .dmg> [--receipt <path>]");
  }
  if (receiptArgumentIndex >= 0 && !requestedReceipt) {
    fail("--receipt requires an output path.");
  }

  const work = mkdtempSync(join(tmpdir(), "civicdesk-macos-smoke-"));
  const mount = join(work, "mount");
  const profile = join(work, "app-data");
  mkdirSync(mount);
  mkdirSync(profile);
  let mounted = false;
  let child;

  try {
    command("hdiutil", ["attach", artifact, "-readonly", "-nobrowse", "-mountpoint", mount]);
    mounted = true;

    const app = join(mount, "The Civic Desk.app");
    const executable = join(app, "Contents", "MacOS", "civicnews");
    const resources = join(app, "Contents", "Resources");
    if (!existsSync(executable)) fail(`packaged executable is missing: ${executable}`);
    if (!existsSync(resources)) fail(`packaged resources are missing: ${resources}`);

    const architecture = command("lipo", ["-archs", executable]);
    if (architecture !== "arm64") {
      fail(`expected an arm64-only executable; found '${architecture}'`);
    }

    const resourceFiles = listFiles(resources);
    const extensionManifest = resourceFiles.find((path) => path.endsWith("/browser-extension/chromium/manifest.json"));
    const promptFiles = resourceFiles.filter((path) => path.includes("/prompts/"));
    if (!extensionManifest) fail("browser-extension manifest is missing from the DMG app.");
    if (promptFiles.length === 0) fail("prompt resources are missing from the DMG app.");

    const codesignInspection = commandDiagnostic("codesign", ["-dvv", app]);
    const developerIdSigned = /Authority=Developer ID Application/.test(codesignInspection);
    if (developerIdSigned) {
      fail("the unsigned beta unexpectedly carries a Developer ID Application identity.");
    }
    const gatekeeperAssessment = commandDiagnostic("spctl", ["--assess", "--type", "execute", "-vv", app]);

    const logPath = join(work, "packaged-launch.log");
    child = spawn(executable, [], {
      cwd: join(app, "Contents", "MacOS"),
      env: {
        ...process.env,
        CIVICNEWS_APP_DATA_DIR: profile,
        CIVICNEWS_OLLAMA_BASE_URL: "http://127.0.0.1:1",
      },
      stdio: ["ignore", "pipe", "pipe"],
    });
    const logChunks = [];
    child.stdout.on("data", (chunk) => logChunks.push(chunk));
    child.stderr.on("data", (chunk) => logChunks.push(chunk));
    await delay(5000);
    if (child.exitCode !== null) {
      fail(`packaged app exited during launch smoke with code ${child.exitCode}`);
    }
    child.kill("SIGTERM");
    await Promise.race([
      new Promise((resolveExit) => child.once("exit", resolveExit)),
      delay(5000),
    ]);
    if (child.exitCode === null) child.kill("SIGKILL");
    writeFileSync(logPath, Buffer.concat(logChunks));

    const profileFiles = listFiles(profile);
    if (!profileFiles.some((path) => path.endsWith(".db"))) {
      fail("packaged app launched but did not initialize an isolated database.");
    }

    const artifactHash = sha256(artifact);
    const receipt = {
      ok: true,
      generated_at: new Date().toISOString(),
      commit: command("git", ["rev-parse", "HEAD"], { cwd: resolve(".") }),
      artifact_path: artifact,
      artifact_name: basename(artifact),
      artifact_sha256: artifactHash,
      artifact_size: statSync(artifact).size,
      architecture: "aarch64",
      minimum_macos: "11.0",
      developer_id_signed: false,
      notarized: false,
      codesign_inspection: codesignInspection,
      gatekeeper_assessment: gatekeeperAssessment,
      browser_extension_manifest: true,
      prompt_file_count: promptFiles.length,
      isolated_launch: {
        ok: true,
        ollama_forced_absent: true,
        database_initialized: true,
      },
    };

    const stamp = new Date().toISOString().replace(/[:.]/g, "-");
    const receiptPath = requestedReceipt || join(
      resolve("."),
      ".agent-runs",
      `macos-packaged-smoke-${stamp}`,
      "macos-packaged-smoke-receipt.json",
    );
    mkdirSync(resolve(receiptPath, ".."), { recursive: true });
    writeFileSync(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
    console.log(`OK: Apple Silicon packaged smoke passed for ${basename(artifact)}`);
    console.log(`SHA256: ${artifactHash}`);
    console.log(`Receipt: ${receiptPath}`);
  } finally {
    if (child?.exitCode === null) child.kill("SIGKILL");
    if (mounted) commandDiagnostic("hdiutil", ["detach", mount]);
    rmSync(work, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(`FAIL: ${error.message}`);
  process.exit(1);
});
