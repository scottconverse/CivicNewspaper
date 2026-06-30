import { mkdir, writeFile } from "node:fs/promises";
import net from "node:net";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { chromium } from "playwright";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const stamp = new Date().toISOString().replace(/[:.]/g, "-");
const runDir = path.join(root, ".agent-runs", `ui-smoke-${stamp}`);
const attachUrl = process.env.CIVICNEWS_UI_URL || "";

function commandOutput(command, args) {
  const result = spawnSync(command, args, { cwd: root, encoding: "utf8" });
  return result.status === 0 ? result.stdout.trim() : "";
}

async function freePort() {
  return await new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => resolve(address.port));
    });
  });
}

async function waitForServer(url, dev, logs, timeoutMs = 30000) {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    if (dev?.exitCode !== null) {
      throw new Error(`Managed Vite process exited before smoke could connect. Log: ${logs.join("")}`);
    }
    const logText = logs.join("");
    if (/port .* is already in use/i.test(logText) || /EADDRINUSE/i.test(logText)) {
      throw new Error(`Managed Vite server failed to bind its port. Log: ${logText}`);
    }
    try {
      const res = await fetch(url);
      if (res.ok) return;
    } catch {
      // server still starting
    }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error(`Timed out waiting for ${url}. Log: ${logs.join("")}`);
}

async function expectBuildMarker(page, expectedBuildId, attachMode) {
  await page.locator("#civicnews-build-id").waitFor({ state: "attached", timeout: 10000 });
  const actual = await page.locator("#civicnews-build-id").getAttribute("data-build-id");
  if (!attachMode && actual !== expectedBuildId) {
    throw new Error(`UI smoke reached build id ${actual}, expected managed build id ${expectedBuildId}`);
  }
  return actual;
}

async function assertNav(page, navId, expected) {
  const tab = page.locator(`#nav-tab-${navId}`);
  await tab.click();
  if (expected instanceof RegExp) {
    await page.getByText(expected).first().waitFor({ timeout: 5000 });
  } else {
    await page.getByRole("heading", { name: expected }).waitFor({ timeout: 5000 });
  }
  const ariaCurrent = await tab.getAttribute("aria-current");
  if (ariaCurrent !== "page") {
    throw new Error(`nav-${navId} content loaded but aria-current was ${ariaCurrent ?? "missing"}`);
  }
}

async function main() {
  await mkdir(runDir, { recursive: true });
  const attachMode = Boolean(attachUrl);
  const port = attachMode ? Number(new URL(attachUrl).port || 80) : await freePort();
  const baseUrl = attachMode ? attachUrl.replace(/\/$/, "") : `http://127.0.0.1:${port}`;
  const gitSha = commandOutput("git", ["rev-parse", "HEAD"]);
  const gitStatus = commandOutput("git", ["status", "--porcelain"]);
  const expectedBuildId = `${gitSha || "unknown"}:${gitStatus ? "dirty" : "clean"}:${stamp}`;

  let dev = null;
  const logs = [];
  if (!attachMode) {
    const devCommand = process.platform === "win32" ? (process.env.ComSpec || "cmd.exe") : "npm";
    const devArgs = process.platform === "win32"
      ? ["/d", "/s", "/c", "npm run dev -- --host 127.0.0.1 --port " + port + " --strictPort"]
      : ["run", "dev", "--", "--host", "127.0.0.1", "--port", String(port), "--strictPort"];
    dev = spawn(devCommand, devArgs, {
      cwd: root,
      shell: false,
      stdio: ["ignore", "pipe", "pipe"],
      env: {
        ...process.env,
        CIVICNEWS_UI_PORT: String(port),
        VITE_CIVICNEWS_BUILD_ID: expectedBuildId,
      },
    });
    dev.stdout.on("data", (chunk) => logs.push(chunk.toString()));
    dev.stderr.on("data", (chunk) => logs.push(chunk.toString()));
  }

  const results = [];
  let browser;
  let actualBuildId = null;
  try {
    await waitForServer(baseUrl, dev, logs);
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1366, height: 900 } });
    const consoleErrors = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
    });

    await page.goto(`${baseUrl}/?firstRun=1`);
    actualBuildId = await expectBuildMarker(page, expectedBuildId, attachMode);
    await page.getByText(/Step 1 of 5|desktop app required/i).waitFor({ timeout: 10000 });
    await page.screenshot({ path: path.join(runDir, "first-run-step-1.png"), fullPage: true });
    results.push({ name: "forced-browser-first-run", ok: true });

    await page.goto(baseUrl);
    await expectBuildMarker(page, expectedBuildId, attachMode);
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
      await assertNav(page, navId, expected);
      results.push({ name: `nav-${navId}`, ok: true });
    }
    await page.screenshot({ path: path.join(runDir, "main-navigation.png"), fullPage: true });

    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto(baseUrl);
    await expectBuildMarker(page, expectedBuildId, attachMode);
    const mobileTargets = [
      ["sources", "Sources"],
      ["publish", "Publishing"],
      ["workbench", /Workbench|No draft selected|Select a story/i],
      ["system", /System|Status|Diagnostics/i],
    ];
    for (const [navId, expected] of mobileTargets) {
      await assertNav(page, navId, expected);
      const target = expected instanceof RegExp
        ? page.getByText(expected).first()
        : page.getByRole("heading", { name: expected });
      const isVisibleInViewport = await target.evaluate((node) => {
        const rect = node.getBoundingClientRect();
        return rect.bottom > 0 && rect.top < window.innerHeight && rect.right > 0 && rect.left < window.innerWidth;
      });
      if (!isVisibleInViewport) {
        throw new Error(`Mobile nav-${navId} content is not visible in the viewport`);
      }
      results.push({ name: `mobile-nav-${navId}`, ok: true });
    }
    await page.screenshot({ path: path.join(runDir, "mobile-navigation.png"), fullPage: true });
    if (consoleErrors.length) {
      throw new Error(`Browser console errors: ${consoleErrors.join(" | ")}`);
    }
  } finally {
    if (browser) await browser.close();
    if (dev) {
      if (process.platform === "win32" && dev.pid) {
        spawnSync("taskkill", ["/pid", String(dev.pid), "/T", "/F"], { stdio: "ignore" });
      } else {
        dev.kill("SIGTERM");
      }
    }
    await writeFile(path.join(runDir, "vite.log"), logs.join(""));
    await writeFile(path.join(runDir, "ui-smoke-receipt.json"), JSON.stringify({
      generated_at: new Date().toISOString(),
      mode: attachMode ? "attach" : "managed",
      base_url: baseUrl,
      managed_pid: dev?.pid ?? null,
      managed_port: attachMode ? null : port,
      git_sha: gitSha,
      dirty: Boolean(gitStatus),
      expected_build_id: attachMode ? null : expectedBuildId,
      actual_build_id: actualBuildId,
      results,
    }, null, 2));
  }

  console.log(`UI smoke receipt: ${path.join(runDir, "ui-smoke-receipt.json")}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
