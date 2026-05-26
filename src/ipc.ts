// src/ipc.ts
import { invoke } from "@tauri-apps/api/core";
import { openPath, openUrl } from "@tauri-apps/plugin-opener";


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

// IPC wrappers
export async function getSources(): Promise<Source[]> {
  return invoke<Source[]>("get_sources");
}

export async function addSource(name: string, url: string, type: string, tier: string): Promise<number> {
  return invoke<number>("add_source", { name, url, type, tier });
}

export async function deleteSource(id: number): Promise<void> {
  return invoke<void>("delete_source", { id });
}

export async function generatePairingPin(label: string): Promise<string> {
  return invoke<string>("generate_pairing_pin", { label });
}

export async function listPairedClients(): Promise<PairedClient[]> {
  return invoke<PairedClient[]>("list_paired_clients");
}

export async function revokePairing(id: number): Promise<void> {
  return invoke<void>("revoke_pairing", { id });
}

export async function getCommunityProfile(): Promise<CommunityProfile> {
  return invoke<CommunityProfile>("get_community_profile");
}

export async function saveCommunityProfile(profile: CommunityProfile): Promise<void> {
  return invoke<void>("save_community_profile", { profile });
}

export async function ingest(): Promise<number> {
  return invoke<number>("ingest");
}

export async function getQueue(): Promise<QueueData> {
  return invoke<QueueData>("get_queue");
}

export async function getEvidence(leadId: number): Promise<EvidenceItem[]> {
  return invoke<EvidenceItem[]>("get_evidence", { leadId });
}

export async function saveDraft(draft: Draft): Promise<number> {
  return invoke<number>("save_draft", { draft });
}

export async function deleteDraft(id: number): Promise<void> {
  return invoke<void>("delete_draft", { id });
}

export async function storyDecision(id: number, decision: string): Promise<void> {
  return invoke<void>("story_decision", { id, decision });
}

export async function generateDraft(leadId: number, format: string, systemPrompt?: string): Promise<string> {
  return invoke<string>("generate_draft", { leadId, format, systemPrompt });
}

export async function llmTask(prompt: string, system: string): Promise<string> {
  return invoke<string>("llm_task", { prompt, system });
}

export async function guardrailsCheck(draftId: number): Promise<GuardrailsReport> {
  return invoke<GuardrailsReport>("guardrails_check", { draftId });
}

export async function publish(outputDir: string): Promise<void> {
  return invoke<void>("publish", { outputDir });
}

export async function registerCorrection(draftId: number, correctionNote: string): Promise<void> {
  return invoke<void>("register_correction", { draftId, correctionNote });
}

export async function backupSave(destPath: string): Promise<void> {
  return invoke<void>("backup_save", { destPath });
}

export async function backupRestore(backupPath: string): Promise<void> {
  return invoke<void>("backup_restore", { backupPath });
}

export async function checkOllama(): Promise<boolean> {
  return invoke<boolean>("check_ollama");
}

export async function pullModel(model: string): Promise<void> {
  return invoke<void>("pull_model", { model });
}

export async function getSystemRam(): Promise<number> {
  return invoke<number>("get_system_ram");
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
  return invoke<DiscoveredSourceCategory[]>("discover_sources", { city, state });
}

export async function openLocalPath(path: string): Promise<void> {
  await openPath(path);
}

export async function openExternalUrl(url: string): Promise<void> {
  await openUrl(url);
}

export async function runDailyScan(city: string, state: string, sinceHours: number): Promise<number> {
  return invoke<number>("run_daily_scan", { city, state, sinceHours });
}

export async function listDailyScanLeads(scanId: number): Promise<DailyScanLead[]> {
  return invoke<DailyScanLead[]>("list_daily_scan_leads", { scanId });
}

export async function plainLanguageRewrite(text: string, draftFormat: string): Promise<string> {
  return invoke<string>("plain_language_rewrite", { text, draftFormat });
}
