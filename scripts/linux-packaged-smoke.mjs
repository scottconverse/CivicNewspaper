#!/usr/bin/env node

import { createHash } from "node:crypto";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
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

function sha256(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function delay(ms) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, ms));
}

async function main() {
  if (process.platform !== "linux") {
    fail("linux-packaged-smoke.mjs must run on Linux.");
  }

  const artifact = resolve(process.argv[2] || "");
  const receiptArgumentIndex = process.argv.indexOf("--receipt");
  const requestedReceipt = receiptArgumentIndex >= 0
    ? resolve(process.argv[receiptArgumentIndex + 1] || "")
    : "";
  if (!artifact || !existsSync(artifact) || !artifact.endsWith(".deb")) {
    fail("usage: node scripts/linux-packaged-smoke.mjs <x86_64 .deb> [--receipt <path>]");
  }
  if (receiptArgumentIndex >= 0 && !requestedReceipt) {
    fail("--receipt requires an output path.");
  }

  const work = mkdtempSync(join(tmpdir(), "civicdesk-linux-smoke-"));
  const profile = join(work, "app-data");
  mkdirSync(profile);
  let child;
  let installed = false;

  try {
    const architecture = command("dpkg-deb", ["-f", artifact, "Architecture"]);
    if (architecture !== "amd64") {
      fail(`expected Debian amd64 package; found '${architecture}'`);
    }
    const packageContents = command("dpkg-deb", ["-c", artifact]);
    if (!/\.\/usr\/bin\/civicnews\s*$/.test(packageContents)) {
      fail("the DEB does not contain /usr/bin/civicnews.");
    }
    if (!/browser-extension\/chromium\/manifest\.json/.test(packageContents)) {
      fail("browser-extension manifest is missing from the DEB.");
    }
    const promptFileCount = packageContents
      .split("\n")
      .filter((line) => /\/prompts\/[^/]+$/.test(line))
      .length;
    if (promptFileCount === 0) {
      fail("prompt resources are missing from the DEB.");
    }

    command("sudo", ["apt-get", "install", "-y", artifact], { stdio: "inherit" });
    installed = true;
    if (!existsSync("/usr/bin/civicnews")) {
      fail("the installed DEB did not provide /usr/bin/civicnews.");
    }
    const executableInspection = command("file", ["--brief", "/usr/bin/civicnews"]);
    if (!/ELF 64-bit.*x86-64/.test(executableInspection)) {
      fail(`expected x86_64 Linux executable; found '${executableInspection}'`);
    }

    const logPath = join(work, "packaged-launch.log");
    const logChunks = [];
    child = spawn("xvfb-run", ["-a", "/usr/bin/civicnews"], {
      detached: true,
      env: {
        ...process.env,
        CIVICNEWS_APP_DATA_DIR: profile,
        CIVICNEWS_OLLAMA_BASE_URL: "http://127.0.0.1:1",
      },
      stdio: ["ignore", "pipe", "pipe"],
    });
    child.stdout.on("data", (chunk) => logChunks.push(chunk));
    child.stderr.on("data", (chunk) => logChunks.push(chunk));
    await delay(5000);
    if (child.exitCode !== null) {
      fail(`packaged app exited during launch smoke with code ${child.exitCode}`);
    }
    process.kill(-child.pid, "SIGTERM");
    await Promise.race([
      new Promise((resolveExit) => child.once("exit", resolveExit)),
      delay(5000),
    ]);
    if (child.exitCode === null) process.kill(-child.pid, "SIGKILL");
    writeFileSync(logPath, Buffer.concat(logChunks));

    const databasePath = join(profile, "civicdesk.db");
    if (!existsSync(databasePath)) {
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
      package_format: "deb",
      architecture: "x86_64",
      minimum_ubuntu: "22.04",
      installed_package: true,
      executable_inspection: executableInspection,
      browser_extension_manifest: true,
      prompt_file_count: promptFileCount,
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
      `linux-packaged-smoke-${stamp}`,
      "linux-packaged-smoke-receipt.json",
    );
    mkdirSync(resolve(receiptPath, ".."), { recursive: true });
    writeFileSync(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
    console.log(`OK: Linux packaged smoke passed for ${basename(artifact)}`);
    console.log(`SHA256: ${artifactHash}`);
    console.log(`Receipt: ${receiptPath}`);
  } finally {
    if (child?.exitCode === null) {
      try {
        process.kill(-child.pid, "SIGKILL");
      } catch {
        // The process group may already have exited.
      }
    }
    if (installed) {
      try {
        command("sudo", ["apt-get", "remove", "-y", "civicnews"], { stdio: "inherit" });
      } catch {
        console.error("WARN: could not remove the smoke-test package.");
      }
    }
    rmSync(work, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(`FAIL: ${error.message}`);
  process.exit(1);
});
