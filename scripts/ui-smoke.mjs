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

async function assertKeyboardAndAccessibility(page, results) {
  await page.keyboard.press("Tab");
  const focusInfo = await page.evaluate(() => {
    const element = document.activeElement;
    if (!element || element === document.body) return null;
    const rect = element.getBoundingClientRect();
    const style = window.getComputedStyle(element);
    return {
      tag: element.tagName.toLowerCase(),
      id: element.id || "",
      text: (element.textContent || "").trim().slice(0, 80),
      visible: rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none",
    };
  });
  if (!focusInfo?.visible) {
    throw new Error(`Keyboard Tab did not land on a visible control: ${JSON.stringify(focusInfo)}`);
  }
  results.push({ name: "keyboard-tab-focus-visible", ok: true, details: focusInfo });

  const unnamedControls = await page.evaluate(() => {
    const labelText = (node) => {
      if (node.id) {
        const label = document.querySelector(`label[for="${CSS.escape(node.id)}"]`);
        if (label?.textContent?.trim()) return label.textContent.trim();
      }
      return "";
    };
    return Array.from(document.querySelectorAll("button, input, select, textarea, a[href]"))
      .filter((node) => {
        const rect = node.getBoundingClientRect();
        const style = window.getComputedStyle(node);
        return rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none";
      })
      .map((node) => ({
        tag: node.tagName.toLowerCase(),
        id: node.id || "",
        text: (node.textContent || "").trim(),
        aria: node.getAttribute("aria-label") || "",
        title: node.getAttribute("title") || "",
        placeholder: node.getAttribute("placeholder") || "",
        label: labelText(node),
      }))
      .filter((item) => ![item.text, item.aria, item.title, item.placeholder, item.label].some((value) => value.trim().length > 0))
      .slice(0, 10);
  });
  if (unnamedControls.length) {
    throw new Error(`Visible controls without accessible names: ${JSON.stringify(unnamedControls)}`);
  }
  results.push({ name: "visible-controls-have-accessible-names", ok: true });

  const contrastInfo = await page.evaluate(() => {
    const parseRgb = (value) => {
      const trimmed = value.trim();
      const rgbMatch = trimmed.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/i);
      if (rgbMatch) return [Number(rgbMatch[1]), Number(rgbMatch[2]), Number(rgbMatch[3])];
      const hexMatch = trimmed.match(/^#([0-9a-f]{3}|[0-9a-f]{6})$/i);
      if (!hexMatch) return null;
      const hex = hexMatch[1].length === 3
        ? hexMatch[1].split("").map((ch) => ch + ch).join("")
        : hexMatch[1];
      return [0, 2, 4].map((offset) => parseInt(hex.slice(offset, offset + 2), 16));
    };
    const luminance = ([r, g, b]) => {
      const convert = (channel) => {
        const srgb = channel / 255;
        return srgb <= 0.03928 ? srgb / 12.92 : Math.pow((srgb + 0.055) / 1.055, 2.4);
      };
      return 0.2126 * convert(r) + 0.7152 * convert(g) + 0.0722 * convert(b);
    };
    const ratio = (a, b) => {
      const l1 = luminance(a);
      const l2 = luminance(b);
      return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);
    };
    const root = window.getComputedStyle(document.documentElement);
    const text = parseRgb(root.getPropertyValue("--text-primary"));
    const background = parseRgb(root.getPropertyValue("--bg-app"));
    if (!text || !background) return { ratio: 0, text: null, background: null };
    return { ratio: ratio(text, background), text, background };
  });
  if (contrastInfo.ratio < 4.5) {
    throw new Error(`Primary text contrast is below 4.5:1: ${JSON.stringify(contrastInfo)}`);
  }
  results.push({ name: "primary-text-contrast-aa", ok: true, details: contrastInfo });
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
    await assertKeyboardAndAccessibility(page, results);
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
      const headingTop = await target.evaluate((node) => node.getBoundingClientRect().top);
      const viewportHeight = page.viewportSize()?.height ?? 844;
      if (headingTop > viewportHeight * 0.7) {
        throw new Error(`Mobile nav-${navId} active content starts too low in the first viewport: ${headingTop}px`);
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
