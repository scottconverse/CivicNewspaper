import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { performance } from "node:perf_hooks";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outputDir = path.join(root, ".agent-runs");
const defaultModels = ["qwen2.5:7b", "gemma4:e4b", "phi4-mini:latest", "llama3.2:3b"];
const timeoutMs = Number(process.env.MODEL_BAKEOFF_TIMEOUT_MS || 180000);
const ollamaUrl = process.env.OLLAMA_URL || "http://127.0.0.1:11434";
const models = (process.argv.slice(2).join(",") || process.env.MODEL_BAKEOFF_MODELS || "")
  .split(",")
  .map((model) => model.trim())
  .filter(Boolean);

const selectedModels = models.length ? models : defaultModels;

const evidence = [
  {
    source_id: 12,
    url: "https://example.gov/council/agendas/2026-06-25",
    excerpt:
      "City Council agenda item 7B schedules a public hearing on a zoning map amendment for 45 acres near the highway interchange. Staff recommends approval. Written comments close July 12.",
  },
  {
    source_id: 12,
    url: "https://example.gov/council/agendas/2026-06-25#contracts",
    excerpt:
      "Consent agenda includes a $1.8 million contract amendment for wastewater lift station repairs after emergency pump failures in May.",
  },
  {
    source_id: 18,
    url: "https://example.edu/board/2026-06-24",
    excerpt:
      "The school board will vote on a bus route consolidation plan expected to affect three elementary schools and save $420,000 next year.",
  },
  {
    source_id: 21,
    url: "https://example.gov/planning/notices",
    excerpt:
      "Planning Commission notice: variance request for reduced parking at a proposed 96-unit affordable housing project. Hearing set for July 9.",
  },
];

const emptyEvidence = [
  {
    source_id: 44,
    url: "https://example.gov/parks/news",
    excerpt:
      "The parks department reminds residents that picnic shelters are available by reservation during regular summer hours.",
  },
];

function buildPrompt(city, state, items) {
  const context = items
    .map(
      (item, idx) =>
        `Evidence 1.${idx + 1}\nSource ID: ${item.source_id}\nOriginal URL: ${item.url}\nExcerpt: ${item.excerpt}`
    )
    .join("\n\n");

  return `City: ${city}, State: ${state}
Batch: 1

Evidence Context:
${context}

Return ONLY valid JSON. No markdown. No prose. No code fence.
Schema: {"leads":[{"title":"short civic lead title","summary":"1-2 evidence-grounded sentences","original_url":"source URL from evidence or empty string"}]}
Include at most 3 leads. Use an empty leads array if nothing deserves an editor's look.`;
}

function extractJsonObject(text) {
  const trimmed = text.trim().replace(/^```(?:json)?\s*/i, "").replace(/```\s*$/i, "").trim();
  const start = trimmed.indexOf("{");
  const end = trimmed.lastIndexOf("}");
  if (start < 0 || end <= start) {
    throw new Error("no JSON object found");
  }
  return trimmed.slice(start, end + 1);
}

function validateResult(text) {
  const hasThinkTag = /<\/?think>/i.test(text);
  const jsonText = extractJsonObject(text);
  const parsed = JSON.parse(jsonText);
  if (!parsed || !Array.isArray(parsed.leads)) {
    throw new Error("missing leads array");
  }
  for (const lead of parsed.leads) {
    if (
      typeof lead.title !== "string" ||
      typeof lead.summary !== "string" ||
      typeof lead.original_url !== "string"
    ) {
      throw new Error("lead has invalid shape");
    }
  }
  return { hasThinkTag, leadCount: parsed.leads.length, parsed };
}

function buildRepairPrompt(raw) {
  return `The previous local model output was not valid JSON for CivicNewspaper Daily Scan.

Return ONLY valid JSON with this exact shape:
{"leads":[{"title":"...","summary":"...","original_url":"..."}]}

Rules:
- Keep at most 3 leads.
- Use {"leads":[]} if the output contains no real civic lead.
- Do not add markdown, explanations, code fences, comments, or trailing text.

Output to repair:
${raw}`;
}

async function generate(model, prompt, system) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  const start = performance.now();
  try {
    const response = await fetch(`${ollamaUrl}/api/generate`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        model,
        prompt,
        system,
        stream: false,
        format: "json",
        options: {
          temperature: 0,
          num_ctx: 4096,
        },
      }),
      signal: controller.signal,
    });
    const elapsedMs = Math.round(performance.now() - start);
    const body = await response.text();
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${body.slice(0, 300)}`);
    }
    const envelope = JSON.parse(body);
    return {
      elapsedMs,
      response: envelope.response || "",
      evalCount: envelope.eval_count,
      evalDuration: envelope.eval_duration,
    };
  } finally {
    clearTimeout(timer);
  }
}

async function main() {
  await mkdir(outputDir, { recursive: true });
  const system = await readFile(path.join(root, "src-tauri", "prompts", "aggregator.md"), "utf8");
  const cases = [
    ["civic-signals", buildPrompt("Brighton", "CO", evidence)],
    ["empty-noise", buildPrompt("Brighton", "CO", emptyEvidence)],
  ];
  const results = [];

  for (const model of selectedModels) {
    for (const [caseName, prompt] of cases) {
      const result = {
        model,
        case: caseName,
        ok: false,
        elapsedMs: null,
        leadCount: null,
        hasThinkTag: null,
        error: null,
        repairAttempted: false,
        repaired: false,
        responsePreview: "",
      };
      try {
        const generated = await generate(model, prompt, system);
        result.elapsedMs = generated.elapsedMs;
        result.responsePreview = generated.response.slice(0, 500);
        let validation;
        try {
          validation = validateResult(generated.response);
        } catch (firstError) {
          result.repairAttempted = true;
          const repaired = await generate(model, buildRepairPrompt(generated.response), system);
          result.elapsedMs += repaired.elapsedMs;
          result.responsePreview = repaired.response.slice(0, 500);
          validation = validateResult(repaired.response);
          result.repaired = true;
          result.repairError = firstError instanceof Error ? firstError.message : String(firstError);
        }
        result.ok = true;
        result.leadCount = validation.leadCount;
        result.hasThinkTag = validation.hasThinkTag;
      } catch (error) {
        result.error = error instanceof Error ? error.message : String(error);
      }
      results.push(result);
      console.log(
        `${result.ok ? "PASS" : "FAIL"} ${model} ${caseName} ${result.elapsedMs ?? "-"}ms leads=${result.leadCount ?? "-"} think=${result.hasThinkTag ?? "-"} ${result.error ?? ""}`
      );
    }
  }

  const stamp = new Date().toISOString().replace(/[:.]/g, "-");
  const outPath = path.join(outputDir, `model-bakeoff-${stamp}.json`);
  await writeFile(outPath, JSON.stringify({ timeoutMs, models: selectedModels, results }, null, 2));
  console.log(`Wrote ${outPath}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
