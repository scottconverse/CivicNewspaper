import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
function run(command, options = {}) {
  const result = spawnSync(command, {
    stdio: "inherit",
    shell: true,
    ...options,
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function output(command, args) {
  const result = spawnSync(command, args, {
    encoding: "utf8",
  });
  if (result.status !== 0) return "";
  return result.stdout.trim();
}

const buildId =
  process.env.VITE_CIVICNEWS_BUILD_ID ||
  process.env.CIVICNEWS_BUILD_ID ||
  output("git", ["rev-parse", "HEAD"]) ||
  "unknown";

const env = {
  ...process.env,
  VITE_CIVICNEWS_BUILD_ID: buildId,
};

run("npx tsc", { env });
run("npx vite build", { env });

mkdirSync("dist", { recursive: true });
writeFileSync(join("dist", "build-id.txt"), `${buildId}\n`, "utf8");
console.log(`CivicNewspaper frontend build id: ${buildId}`);
