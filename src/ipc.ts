// src/ipc.ts
import { invoke } from "@tauri-apps/api/core";

// QA-003: every IPC call goes through invokeGuarded, which checks isTauri()
// once before touching @tauri-apps. In a plain browser (no Tauri runtime)
// window.__TAURI_INTERNALS__ is absent and calling invoke would throw an
// opaque "Cannot read properties of undefined (reading 'invoke')". The guard
// converts that into a single, explained failure instead of a cryptic crash
// per call.
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

let warnedIpcUnavailable = false;

function ipcUnavailable(command: string): never {
  if (!warnedIpcUnavailable) {
    warnedIpcUnavailable = true;
    console.warn(
      "Desktop bridge unavailable: IPC commands cannot run outside The Civic Desk desktop app. " +
        "This is expected in a browser preview or test harness."
    );
  }
  throw new Error(`The desktop bridge is unavailable, so "${command}" could not run.`);
}

async function invokeGuarded<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) return ipcUnavailable(command);
  // Match raw invoke's call shape: omit the second argument entirely when there
  // are no args, rather than passing `undefined` (keeps call-site assertions and
  // Tauri's own arg handling identical to a direct invoke).
  return args === undefined ? invoke<T>(command) : invoke<T>(command, args);
}

// UX-2: one place that turns any thrown value into plain-language, action-oriented
// copy. Components surface toUserMessage(e) instead of e.toString(), which used to
// leak Rust Debug strings and "[object Object]" to end users.
export function toUserMessage(e: unknown): string {
  const raw =
    e instanceof Error
      ? e.message
      : typeof e === "string"
        ? e
        : e == null
          ? ""
          : (() => {
              try {
                return JSON.stringify(e);
              } catch {
                return String(e);
              }
            })();

  const lower = raw.toLowerCase();

  if (!raw.trim()) {
    return "Something went wrong, but no details were reported. Please try again.";
  }

  // QA-R2-mn1 / QA-R2-mn2: the backend tags some conditions with a typed
  // `UPPER_SNAKE:` prefix (e.g. `NO_EVIDENCE:`, `MODEL_NOT_INSTALLED:`). Strip the
  // machine token and translate to plain-language guidance, rather than leaking
  // the raw "Something went wrong: NO_EVIDENCE: …" debug string to the user.
  const typedPrefix = raw.match(/^([A-Z][A-Z0-9_]+):\s*(.*)$/s);
  if (typedPrefix) {
    const [, token, rest] = typedPrefix;
    const message = rest.trim();
    switch (token) {
      case "NO_EVIDENCE":
        return "There's nothing to scan yet — run Scrape & Detect first to collect evidence, then try again.";
      case "MODEL_NOT_INSTALLED":
        return "The selected AI model isn't installed yet. Open AI Model to download it, then try again.";
      default:
        // Unknown typed prefix: surface the human-readable remainder without the
        // raw token, falling back to the whole message if there's nothing after it.
        return message || raw;
    }
  }

  if (lower.includes("desktop bridge is unavailable") || lower.includes("__tauri")) {
    return "This action needs The Civic Desk desktop app. It can't run in a browser preview.";
  }
  // QA-C1: a missing model surfaces from Ollama as e.g. `model "qwen3:8b" not
  // found`. Catch it before the generic connection / not-found branches and give
  // a plain-language remedy that points at the model download.
  if (
    lower.includes("model") &&
    (lower.includes("not found") || lower.includes("try pulling") || lower.includes("no such model"))
  ) {
    return "The selected AI model isn't downloaded yet. Open AI Model to download it, then try again.";
  }
  if (
    lower.includes("ollama") ||
    lower.includes("connection refused") ||
    lower.includes("10061") ||
    lower.includes("connect") && lower.includes("refused")
  ) {
    return "Couldn't reach the local AI model service (Ollama). Make sure it's installed and running, then try again.";
  }
  if (lower.includes("permission") || lower.includes("denied") || lower.includes("access is denied")) {
    return "The Civic Desk didn't have permission to complete this. Check the file or folder permissions and try again.";
  }
  if (lower.includes("not found") || lower.includes("no such file")) {
    return "The requested item couldn't be found. It may have been moved or deleted.";
  }
  return `Something went wrong: ${raw}`;
}


export interface Source {
  id?: number;
  name: string;
  url: string;
  type: string; // 'primary_record', 'official_comm', 'community_signal', 'media_lead'
  status: string; // 'online', 'offline'
  tier: string; // 'official_record', 'news_reporting', 'community_signal'
  last_success_at?: string;
  last_failed_at?: string;
  last_scraped?: string;
}

export interface EvidenceItem {
  id?: number;
  source_id: number;
  url?: string;
  fetched_at: string;
  excerpt: string;
  content_hash: string;
  entities: string; // JSON array
}

export interface Lead {
  id?: number;
  detector_name: string;
  why: string;
  confidence: string; // 'low', 'med', 'high'
  risk_level: string; // 'low', 'med', 'high'
  confirmation_checklist: string; // JSON array
  from_scan_lead_id?: number;
  created_at: string;
}

export interface Draft {
  id?: number;
  lead_id?: number;
  format: string; // 'brief', 'watch', 'explainer', 'investigation', 'opinion'
  title: string;
  content: string;
  status: string; // 'lead', 'draft_generated', 'ready_to_review', ...
  verification_checklist: string; // JSON array
  missing_evidence_notes?: string;
  correction_note?: string;
  created_at?: string;
  updated_at?: string;
}

export interface PairedClient {
  id?: number;
  token: string;
  label: string;
  pairing_pin?: string;
  pin_expires_at?: string;
  created_at: string;
  last_used_at?: string;
  revoked: boolean;
}

export interface CommunityProfile {
  site_title: string;
  site_subtitle: string;
  about_text: string;
  ethics_text: string;
  how_we_report_text: string;
  money_threshold: number;
  watchlist: string[];
  city: string;
  state: string;
}

export interface DailyScanLead {
  id?: number;
  scan_id: number;
  title: string;
  summary: string;
  source_id?: number;
  original_url: string;
  why_flagged?: string;
  source_name?: string;
  source_type?: string;
  priority?: string;
  suggested_next_step?: string;
}

export interface QueueData {
  leads: Lead[];
  drafts: Draft[];
}

export interface GuardrailsIssue {
  category: string;
  message: string;
  severity: string; // 'error', 'warning'
  line_number?: number;
}

export interface GuardrailsReport {
  is_clean: boolean;
  issues: GuardrailsIssue[];
}

export interface CompiledArticle {
  title: string;
  format: string;
  relative_path: string;
  updated_at: string;
}

export interface PublishResult {
  output_dir: string;
  generated_at: string;
  article_count: number;
  skipped_count: number;
  files_written: number;
  index_path: string;
  rss_path: string;
  newsletter_path: string;
  share_package_path: string;
  manifest_path: string;
  zip_path: string;
  articles: CompiledArticle[];
}

// IPC wrappers
export async function getSources(): Promise<Source[]> {
  return invokeGuarded<Source[]>("get_sources");
}

export async function addSource(name: string, url: string, type: string, tier: string): Promise<number> {
  return invokeGuarded<number>("add_source", { name, url, type, tier });
}

export async function deleteSource(id: number): Promise<void> {
  return invokeGuarded<void>("delete_source", { id });
}

export async function generatePairingPin(label: string): Promise<string> {
  return invokeGuarded<string>("generate_pairing_pin", { label });
}

export async function listPairedClients(): Promise<PairedClient[]> {
  return invokeGuarded<PairedClient[]>("list_paired_clients");
}

export async function getBrowserExtensionPath(): Promise<string> {
  return invokeGuarded<string>("get_browser_extension_path");
}

export async function revokePairing(id: number): Promise<void> {
  return invokeGuarded<void>("revoke_pairing", { id });
}

export async function getCommunityProfile(): Promise<CommunityProfile> {
  return invokeGuarded<CommunityProfile>("get_community_profile");
}

export async function saveCommunityProfile(profile: CommunityProfile): Promise<void> {
  return invokeGuarded<void>("save_community_profile", { profile });
}

export async function ingest(): Promise<number> {
  return invokeGuarded<number>("ingest");
}

export async function getQueue(): Promise<QueueData> {
  return invokeGuarded<QueueData>("get_queue");
}

export async function getEvidence(leadId: number): Promise<EvidenceItem[]> {
  return invokeGuarded<EvidenceItem[]>("get_evidence", { leadId });
}

export async function saveDraft(draft: Draft): Promise<number> {
  return invokeGuarded<number>("save_draft", { draft });
}

export async function deleteDraft(id: number): Promise<void> {
  return invokeGuarded<void>("delete_draft", { id });
}

export async function storyDecision(
  id: number,
  decision: string,
  overrideReason?: string
): Promise<void> {
  return invokeGuarded<void>(
    "story_decision",
    overrideReason === undefined ? { id, decision } : { id, decision, overrideReason }
  );
}

// GG-C1: record a human attestation that the editor verified the draft against
// its cited evidence. Required before a draft can be approved for publishing.
export async function attestDraft(id: number, editor: string): Promise<void> {
  return invokeGuarded<void>("attest_draft", { id, editor });
}

export async function generateDraft(leadId: number, format: string, systemPrompt?: string): Promise<string> {
  return invokeGuarded<string>("generate_draft", { leadId, format, systemPrompt });
}

export async function llmTask(prompt: string, system: string): Promise<string> {
  return invokeGuarded<string>("llm_task", { prompt, system });
}

export async function guardrailsCheck(draftId: number): Promise<GuardrailsReport> {
  return invokeGuarded<GuardrailsReport>("guardrails_check", { draftId });
}

// Editor-editable guardrail word lists (per newsroom). Words warn by default;
// only words in `blocking` hard-stop publishing.
export interface GuardrailConfig {
  accusatory: string[];
  legal: string[];
  blocking: string[];
}

export async function getGuardrailTerms(): Promise<GuardrailConfig> {
  return invokeGuarded<GuardrailConfig>("get_guardrail_terms");
}

export async function setGuardrailTerms(config: GuardrailConfig): Promise<void> {
  return invokeGuarded<void>("set_guardrail_terms", { config });
}

export async function publish(outputDir: string): Promise<PublishResult> {
  return invokeGuarded<PublishResult>("publish", { outputDir });
}

export async function registerCorrection(draftId: number, correctionNote: string): Promise<void> {
  return invokeGuarded<void>("register_correction", { draftId, correctionNote });
}

export async function backupSave(destPath: string): Promise<void> {
  return invokeGuarded<void>("backup_save", { destPath });
}

export async function backupRestore(backupPath: string): Promise<void> {
  return invokeGuarded<void>("backup_restore", { backupPath });
}

export async function checkOllama(): Promise<boolean> {
  return invokeGuarded<boolean>("check_ollama");
}

export interface OllamaState {
  reachable: boolean;
  models: string[];
  version: string | null;
}

export async function ollamaHealth(): Promise<OllamaState> {
  return invokeGuarded<OllamaState>("ollama_health");
}

export async function pullOllamaModel(modelId: string): Promise<void> {
  return invokeGuarded<void>("pull_ollama_model", { modelId });
}

export async function getSystemRam(): Promise<number> {
  return invokeGuarded<number>("get_system_ram");
}

export interface DiscoveredSource {
  name: string;
  url: string;
  type: string;
}

export interface DiscoveredSourceCategory {
  category_name: string;
  type: string;
  candidates: DiscoveredSource[];
}

export async function discoverSources(city: string, state: string): Promise<DiscoveredSourceCategory[]> {
  return invokeGuarded<DiscoveredSourceCategory[]>("discover_sources", { city, state });
}

export async function extractSourceImportText(path: string): Promise<string> {
  return invokeGuarded<string>("extract_source_import_text", { path });
}

export async function openLocalPath(path: string): Promise<void> {
  return invokeGuarded<void>("open_local_path", { path });
}

export async function openExternalUrl(url: string): Promise<void> {
  return invokeGuarded<void>("open_external_url", { url });
}

export async function runDailyScan(city: string, state: string, sinceHours: number): Promise<number> {
  return invokeGuarded<number>("run_daily_scan", { city, state, sinceHours });
}

export async function listDailyScanLeads(scanId: number): Promise<DailyScanLead[]> {
  return invokeGuarded<DailyScanLead[]>("list_daily_scan_leads", { scanId });
}

export async function plainLanguageRewrite(text: string, draftFormat: string): Promise<string> {
  return invokeGuarded<string>("plain_language_rewrite", { text, draftFormat });
}

// Settings + onboarding + diagnostics: previously called via raw invoke() inside
// components, which bypassed the isTauri() guard. Centralized here so every IPC
// path is guarded and discoverable.
export async function getSetting(key: string): Promise<string | null> {
  return invokeGuarded<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invokeGuarded<void>("set_setting", { key, value });
}

export async function isOnboardingComplete(): Promise<boolean> {
  return invokeGuarded<boolean>("is_onboarding_complete");
}

export async function setOnboardingComplete(value: boolean): Promise<void> {
  return invokeGuarded<void>("set_onboarding_complete", { value });
}

export async function exportDiagnostics(path: string): Promise<void> {
  return invokeGuarded<void>("export_diagnostics", { path });
}

export async function cancelOllamaPull(model: string): Promise<void> {
  return invokeGuarded<void>("cancel_ollama_pull", { model });
}
