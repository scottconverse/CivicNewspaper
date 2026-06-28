import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { chromium } from "playwright";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const stamp = new Date().toISOString().replace(/[:.]/g, "-");
const runDir = path.join(root, ".agent-runs", `ui-smoke-${stamp}`);
const baseUrl = process.env.CIVICNEWS_UI_URL || "http://127.0.0.1:1420";

async function waitForServer(url, timeoutMs = 30000) {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    try {
      const res = await fetch(url);
      if (res.ok) return;
    } catch {
      // server still starting
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function main() {
  await mkdir(runDir, { recursive: true });
  const devCommand = process.platform === "win32" ? (process.env.ComSpec || "cmd.exe") : "npm";
  const devArgs = process.platform === "win32"
    ? ["/d", "/s", "/c", "npm run dev -- --host 127.0.0.1"]
    : ["run", "dev", "--", "--host", "127.0.0.1"];
  const dev = spawn(devCommand, devArgs, {
    cwd: root,
    shell: false,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const logs = [];
  dev.stdout.on("data", (chunk) => logs.push(chunk.toString()));
  dev.stderr.on("data", (chunk) => logs.push(chunk.toString()));

  const results = [];
  let browser;
  try {
    await waitForServer(baseUrl);
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1366, height: 900 } });
    const consoleErrors = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`${baseUrl}/?firstRun=1`);
    await page.getByText("Step 1 of 5").waitFor({ timeout: 10000 });
    await page.screenshot({ path: path.join(runDir, "first-run-step-1.png"), fullPage: true });
    results.push({ name: "forced-browser-first-run", ok: true });

    await page.goto(baseUrl);
    await page.getByRole("heading", { name: "Story Queue" }).waitFor({ timeout: 10000 });
    const navTargets = [
      ["dailyScan", "Daily Scan"],
      ["darkSignals", "Dark Signal Desk"],
      ["verification", "Verification Queue"],
      ["workbench", /Workbench|No draft selected|Select a story/i],
      ["sources", "Sources"],
      ["onboarding", "Set up your private AI"],
      ["publish", "Publishing"],
      ["pairing", "Browser pairing"],
      ["settings", "Ethics & Backups"],
      ["system", /System|Status|Diagnostics/i],
    ];
    for (const [navId, expected] of navTargets) {
      await page.locator(`#nav-tab-${navId}`).click();
      if (expected instanceof RegExp) {
        await page.getByText(expected).first().waitFor({ timeout: 5000 });
      } else {
        await page.getByRole("heading", { name: expected }).waitFor({ timeout: 5000 });
      }
      results.push({ name: `nav-${navId}`, ok: true });
    }
    await page.screenshot({ path: path.join(runDir, "main-navigation.png"), fullPage: true });
    if (consoleErrors.length) {
      throw new Error(`Browser console errors: ${consoleErrors.join(" | ")}`);
    }
  } finally {
    if (browser) await browser.close();
    if (process.platform === "win32" && dev.pid) {
      spawnSync("taskkill", ["/pid", String(dev.pid), "/T", "/F"], { stdio: "ignore" });
    } else {
      dev.kill("SIGTERM");
    }
    await writeFile(path.join(runDir, "vite.log"), logs.join(""));
    await writeFile(path.join(runDir, "ui-smoke-receipt.json"), JSON.stringify({
      generated_at: new Date().toISOString(),
      base_url: baseUrl,
      results,
    }, null, 2));
  }

  console.log(`UI smoke receipt: ${path.join(runDir, "ui-smoke-receipt.json")}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
