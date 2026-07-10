import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { chromium } from "playwright";

function option(name, fallback = "") {
  const index = process.argv.indexOf(`--${name}`);
  return index >= 0 ? process.argv[index + 1] : fallback;
}

const cdpUrl = option("cdp-url");
const mode = option("mode");
const outputDir = path.resolve(option("output-dir"));
const model = option("model", "phi4-mini:latest");
const expectedBuildId = option("expected-build-id");
const outputPath = path.join(outputDir, `${mode}-webview-result.json`);

if (!cdpUrl || !mode || !option("output-dir") || !expectedBuildId) {
  throw new Error("Usage: node packaged-webview-driver.mjs --cdp-url URL --mode first-run|core-flow --output-dir DIR --expected-build-id SHA [--model MODEL]");
}

await mkdir(outputDir, { recursive: true });
const result = { mode, cdp_url: cdpUrl, checks: [], screenshots: [], details: {} };
const check = (name, ok, details = {}) => {
  result.checks.push({ name, ok: Boolean(ok), details });
  if (!ok) throw new Error(`Check failed: ${name}`);
};

async function screenshot(page, name) {
  const target = path.join(outputDir, `${name}.png`);
  await page.screenshot({ path: target, fullPage: true });
  result.screenshots.push(target);
}

async function invoke(page, command, args = {}) {
  return page.evaluate(
    ({ command, args }) => window.__TAURI_INTERNALS__.invoke(command, args),
    { command, args },
  );
}

async function runFirstRun(page) {
  await page.getByRole("heading", { name: "Workspace Setup" }).waitFor();
  await screenshot(page, "01-first-run-identity");
  await page.getByLabel("Publication Name").fill("Local Beta Desk");
  await page.getByLabel("Editor Name").fill("Packaged Walkthrough");
  await screenshot(page, "02-first-run-identity-filled");
  await page.getByRole("button", { name: /Continue setup/ }).click();

  await page.getByText("Step 2 of 5").waitFor();
  await page.getByRole("button", { name: "Install local AI runtime" }).waitFor();
  await page.getByRole("button", { name: "Skip for now" }).waitFor();
  check("dependency-absent-choice-visible", true);
  await screenshot(page, "03-first-run-ai-unavailable");
  await page.getByRole("button", { name: "Skip for now" }).click();
  await page.getByRole("button", { name: "Continue without AI" }).waitFor();
  await screenshot(page, "04-first-run-skip-confirmation");
  await page.getByRole("button", { name: "Continue without AI" }).click();

  await page.getByText("Step 4 of 5").waitFor();
  await screenshot(page, "05-first-run-defaults");
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByText("Step 5 of 5").waitFor();
  await screenshot(page, "06-first-run-done");
  await page.getByRole("button", { name: "Finish Onboarding" }).click();

  await page.locator("#nav-tab-dailyScan").waitFor({ timeout: 30_000 });
  check("workspace-reached", true);
  await screenshot(page, "07-first-run-workspace");
  await page.locator("#nav-tab-dailyScan").click();
  await page.getByRole("heading", { name: "Daily Scan" }).waitFor();
  await page.getByRole("button", { name: "Add Sources First" }).waitFor();
  await page.getByRole("button", { name: "Go to Sources" }).waitFor();
  check("zero-source-guidance-usable", true);
  await screenshot(page, "08-first-run-zero-source-guidance");
}

async function runCoreFlow(page) {
  for (const [key, value] of [
    ["identity.newsroom_name", "Brighton Local Beta Desk"],
    ["identity.editor_name", "Packaged Core Flow"],
    ["identity.city", "Brighton"],
    ["identity.state", "CO"],
    ["model.selected", model],
  ]) {
    await invoke(page, "set_setting", { key, value });
  }
  await invoke(page, "set_onboarding_complete", { value: true });
  await page.reload();
  await page.locator("#nav-tab-queue").waitFor({ timeout: 30_000 });

  const health = await invoke(page, "ollama_health");
  check("live-model-ready", health.reachable && health.models.includes(model), health);
  const sources = [
    ["Brighton City Council Agenda Center", "https://www.brightonco.gov/AgendaCenter/City-Council-3"],
    ["Denver Council Legistar", "https://denver.legistar.com/"],
  ];
  for (const [name, url] of sources) {
    await invoke(page, "add_source", { name, url, type: "primary_record", tier: "official_record" });
  }
  check("controlled-sources-added", true, { count: sources.length, urls: sources.map(([, url]) => url) });

  const scanId = await invoke(page, "run_daily_scan", { city: "Brighton", state: "CO", sinceHours: 168 });
  const scanLeads = await invoke(page, "list_daily_scan_leads", { scanId });
  const queue = await invoke(page, "get_queue");
  let selected = null;
  let selectedEvidence = [];
  for (const lead of queue.leads.filter((lead) => lead.from_scan_lead_id)) {
    const evidence = await invoke(page, "get_evidence", { leadId: lead.id });
    if (evidence.length > 0) {
      selected = lead;
      selectedEvidence = evidence;
      break;
    }
  }
  check("daily-scan-created-linked-lead", scanLeads.length > 0 && selected, {
    scan_id: scanId,
    scan_leads: scanLeads.length,
    queue_leads: queue.leads.length,
    selected_lead_id: selected?.id,
    evidence_count: selectedEvidence.length,
  });

  await page.reload();
  await page.locator("#nav-tab-queue").waitFor({ timeout: 30_000 });
  await page.locator("#nav-tab-queue").click();
  const draftLeadButton = page.getByTestId(`btn-draft-lead-${selected.id}`);
  await draftLeadButton.waitFor({ timeout: 30_000 });
  await screenshot(page, "01-core-flow-linked-lead");
  await draftLeadButton.click();
  await page.getByRole("heading", { name: "Drafting Article" }).waitFor();
  const generate = page.locator("#btn-generate-draft-top");
  await generate.waitFor();
  await generate.click({ timeout: 60_000 });
  await page.locator("#btn-save-draft").waitFor({ timeout: 300_000 });
  await screenshot(page, "02-core-flow-generated-draft");

  const afterGenerate = await invoke(page, "get_queue");
  const draft = afterGenerate.drafts.find((candidate) => candidate.lead_id === selected.id);
  check("draft-persisted-through-command", draft?.id && draft.content?.trim(), {
    draft_id: draft?.id,
    lead_id: selected.id,
    title: draft?.title,
    content_length: draft?.content?.length ?? 0,
  });

  await page.reload();
  await page.locator("#nav-tab-queue").waitFor({ timeout: 30_000 });
  await page.locator("#nav-tab-queue").click();
  await page.locator("#queue-tab-drafts").click();
  await page.locator(`#btn-open-workbench-${draft.id}`).click();
  await page.getByLabel("Story Title").waitFor();
  check("draft-reloaded-in-workbench", (await page.getByLabel("Story Title").inputValue()) === draft.title, {
    draft_id: draft.id,
  });
  await screenshot(page, "03-core-flow-reloaded-workbench");
  result.details = { scan_id: scanId, lead_id: selected.id, draft_id: draft.id, evidence_count: selectedEvidence.length };
}

let browser;
try {
  browser = await chromium.connectOverCDP(cdpUrl);
  const context = browser.contexts()[0];
  const page = context.pages()[0];
  await page.waitForLoadState("domcontentloaded");
  const embeddedBuildId = await page.locator("#civicnews-build-id").getAttribute("data-build-id");
  check("build-id-matches-commit", embeddedBuildId === expectedBuildId, {
    expected: expectedBuildId,
    actual: embeddedBuildId,
  });
  if (mode === "first-run") await runFirstRun(page);
  else if (mode === "core-flow") await runCoreFlow(page);
  else throw new Error(`Unknown mode: ${mode}`);
  result.ok = true;
} catch (error) {
  result.ok = false;
  result.error = error instanceof Error ? error.stack || error.message : String(error);
  throw error;
} finally {
  await writeFile(outputPath, JSON.stringify(result, null, 2), "utf8");
  await browser?.close();
}

console.log(outputPath);
