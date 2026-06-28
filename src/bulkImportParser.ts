// src/bulkImportParser.ts
// Shared source-list parser for pasted rows and extracted file text.

export const VALID_SOURCE_TYPES = [
  "primary_record",
  "official_comm",
  "community_signal",
  "media_lead",
] as const;

export interface ParsedImportLine {
  name: string;
  url: string;
  type: string;
}

export interface ImportReviewItem extends ParsedImportLine {
  id: string;
  row: number;
  tier: string;
  credibility: string;
  review_note: string;
  selected: boolean;
}

export interface RejectedImportLine {
  row: number;
  text: string;
  reason: string;
}

export interface BulkImportReview {
  accepted: ImportReviewItem[];
  rejected: RejectedImportLine[];
  duplicates: RejectedImportLine[];
}

const HTTP_URL_RE = /https?:\/\/[^\s<>"')\]]+/i;
const HTTP_URL_GLOBAL_RE = /https?:\/\/[^\s<>"')\]]+/gi;
const KNOWN_OFFICIAL_HOSTS = [
  "denvergov.org",
  "denver.legistar.com",
  "dpsk12.org",
];

function cleanField(value: string): string {
  return value
    .trim()
    .replace(/^["'`]+|["'`]+$/g, "")
    .trim();
}

export function normalizeImportUrl(value: string): string {
  let url = cleanField(value).replace(/[.,;:!?]+$/g, "");
  const unbalancedPairs: Array<[string, string]> = [
    ["(", ")"],
    ["[", "]"],
    ["{", "}"],
  ];

  let changed = true;
  while (changed) {
    changed = false;
    for (const [open, close] of unbalancedPairs) {
      if (!url.endsWith(close)) continue;
      const opens = [...url].filter((ch) => ch === open).length;
      const closes = [...url].filter((ch) => ch === close).length;
      if (closes > opens) {
        url = url.slice(0, -1).replace(/[.,;:!?]+$/g, "");
        changed = true;
      }
    }
  }

  return url;
}

function looksLikeHttpUrl(value: string): boolean {
  return /^https?:\/\//i.test(cleanField(value));
}

function isValidSourceType(value: string): boolean {
  return VALID_SOURCE_TYPES.includes(value as (typeof VALID_SOURCE_TYPES)[number]);
}

function explicitSourceType(fields: string[] | null): string | null {
  return fields?.find((field) => isValidSourceType(field)) ?? null;
}

function deriveNameFromUrl(url: string): string {
  try {
    const parsedUrl = new URL(url);
    return parsedUrl.hostname.replace(/^www\./i, "");
  } catch {
    return url;
  }
}

export function tierForSourceType(type: string): string {
  if (type === "primary_record" || type === "official_comm") return "official_record";
  if (type === "media_lead") return "news_reporting";
  return "community_signal";
}

function inferSourceType(name: string, url: string, fallbackType: string): string {
  let host = "";
  try {
    host = new URL(url).hostname.toLowerCase().replace(/^www\./, "");
  } catch {
    return isValidSourceType(fallbackType) ? fallbackType : "primary_record";
  }
  const label = `${name} ${url}`.toLowerCase();
  if (
    host.includes("reddit.com")
    || host.includes("facebook.com")
    || host.includes("nextdoor.com")
    || host.includes("youtube.com")
    || label.includes("reddit")
    || label.includes("facebook")
  ) {
    return "community_signal";
  }
  if (
    host.includes("longmontleader.com")
    || host.includes("timescall.com")
    || host.includes("dailycamera.com")
    || label.includes("local news")
    || label.includes("newspaper")
    || label.includes("media")
  ) {
    return "media_lead";
  }
  if (
    host.endsWith(".gov")
    || host.includes(".gov.")
    || host.includes("legistar.com")
    || host.includes("publicnotice")
    || label.includes("agenda")
    || label.includes("minutes")
    || label.includes("public notice")
    || label.includes("budget")
  ) {
    return "primary_record";
  }
  if (
    label.includes("press release")
    || label.includes("public safety")
    || label.includes("police")
    || label.includes("fire")
  ) {
    return "official_comm";
  }
  return isValidSourceType(fallbackType) ? fallbackType : "primary_record";
}

export function credibilityForSource(source: ParsedImportLine): { credibility: string; note: string; selected: boolean } {
  let host = "";
  try {
    host = new URL(source.url).hostname.toLowerCase();
  } catch {
    return { credibility: "Invalid URL", note: "The URL could not be parsed.", selected: false };
  }
  const comparableHost = host.replace(/^www\./, "");

  if (/google\.[^/]+$|bing\.com$|duckduckgo\.com$|search\.yahoo\.com$/.test(comparableHost)) {
    return {
      credibility: "Search helper",
      note: "Use this link to find a real source, but do not import it as a watched feed.",
      selected: false,
    };
  }
  if (comparableHost.includes("facebook.com") || comparableHost.includes("reddit.com") || comparableHost.includes("nextdoor.com")) {
    return {
      credibility: "Community signal",
      note: "Useful for leads, but verify against public records before publishing.",
      selected: false,
    };
  }
  if (
    comparableHost.endsWith(".gov")
    || comparableHost.includes(".gov.")
    || KNOWN_OFFICIAL_HOSTS.includes(comparableHost)
    || comparableHost.endsWith(".legistar.com")
  ) {
    return {
      credibility: "Official record",
      note: "Likely a primary civic source. Confirm it is the right department/feed.",
      selected: true,
    };
  }
  if (source.type === "media_lead") {
    return {
      credibility: "News source",
      note: "Good for leads and context; verify facts against primary records.",
      selected: false,
    };
  }
  return {
    credibility: "Needs review",
    note: "Check ownership and usefulness before importing.",
    selected: false,
  };
}

function parseDelimitedLine(line: string, delimiter: string): string[] {
  const fields: string[] = [];
  let current = "";
  let inQuotes = false;

  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    const next = line[i + 1];

    if (ch === '"' && inQuotes && next === '"') {
      current += '"';
      i++;
      continue;
    }

    if (ch === '"') {
      inQuotes = !inQuotes;
      continue;
    }

    if (ch === delimiter && !inQuotes) {
      fields.push(cleanField(current));
      current = "";
      continue;
    }

    current += ch;
  }

  fields.push(cleanField(current));
  return fields;
}

function findDelimitedFields(line: string): string[] | null {
  for (const delimiter of ["\t", ",", "|"]) {
    if (!line.includes(delimiter)) continue;
    const fields = parseDelimitedLine(line, delimiter).filter(Boolean);
    if (fields.length >= 2 && fields.some(looksLikeHttpUrl)) {
      return fields;
    }
  }
  return null;
}

function parseLinkedText(line: string): { name: string; url: string } | null {
  const markdownMatch = line.match(/\[([^\]]+)\]\((https?:\/\/[^)\s]+)\)/i);
  if (markdownMatch) {
    return {
      name: cleanField(markdownMatch[1]),
      url: normalizeImportUrl(markdownMatch[2]),
    };
  }

  const htmlMatch = line.match(/<a\b[^>]*href=["'](https?:\/\/[^"']+)["'][^>]*>(.*?)<\/a>/i);
  if (htmlMatch) {
    return {
      name: cleanField(htmlMatch[2].replace(/<[^>]*>/g, "")),
      url: normalizeImportUrl(htmlMatch[1]),
    };
  }

  return null;
}

function nameNearUrl(line: string, url: string): string {
  const urlIndex = line.indexOf(url);
  const before = cleanField(cleanField(line.slice(0, urlIndex)).replace(/[-:|,]+$/g, ""));
  if (before) return before;

  const after = cleanField(cleanField(line.slice(urlIndex + url.length)).replace(/^[-:|,]+/g, ""));
  if (after && !looksLikeHttpUrl(after)) return after;

  return deriveNameFromUrl(url);
}

/**
 * Parse a single import row into a source record.
 *
 * Accepts bare URLs, CSV/TSV/pipe rows, markdown links, HTML links, and plain
 * text rows containing an http(s) URL near their label. Returns null for rows
 * that do not contain an http(s) URL.
 */
export function parseBulkImportLine(
  rawLine: string,
  defaultType: string
): ParsedImportLine | null {
  const line = rawLine.trim();
  if (!line) return null;

  let name = "";
  let url = "";
  let type = defaultType;

  const linked = parseLinkedText(line);
  if (linked) {
    name = linked.name;
    url = linked.url;
  } else {
    const fields = findDelimitedFields(line);
    if (fields) {
      const urlFieldIndex = fields.findIndex(looksLikeHttpUrl);
      url = normalizeImportUrl(fields[urlFieldIndex]);
      const typeField = explicitSourceType(fields);
      if (typeField) type = typeField;

      const nameField = fields.find((field, index) => {
        return index !== urlFieldIndex && !isValidSourceType(field) && !looksLikeHttpUrl(field);
      });
      name = nameField ? cleanField(nameField) : deriveNameFromUrl(url);
    } else {
      const urlMatch = line.match(HTTP_URL_RE);
      if (!urlMatch) return null;
      url = normalizeImportUrl(urlMatch[0]);
      name = nameNearUrl(line, url);
    }
  }

  if (!looksLikeHttpUrl(url)) {
    return null;
  }

  if (!isValidSourceType(type)) {
    type = defaultType;
  }
  if (type === defaultType && defaultType === "primary_record") {
    type = inferSourceType(name, url, defaultType);
  }

  return {
    name: name || deriveNameFromUrl(url),
    url,
    type,
  };
}

export function buildBulkImportReview(
  text: string,
  defaultType: string,
  existingUrls: string[] = []
): BulkImportReview {
  const existing = new Set(existingUrls.map((url) => url.trim().toLowerCase()).filter(Boolean));
  const seen = new Set<string>();
  const accepted: ImportReviewItem[] = [];
  const rejected: RejectedImportLine[] = [];
  const duplicates: RejectedImportLine[] = [];

  const rows = text.split(/\r?\n/).flatMap((rawLine) => {
    const matches = [...rawLine.matchAll(HTTP_URL_GLOBAL_RE)];
    if (matches.length <= 1 || findDelimitedFields(rawLine)) {
      return [rawLine];
    }
    return matches.map((match, idx) => {
      const url = match[0];
      const start = match.index ?? rawLine.indexOf(url);
      const nextStart = idx + 1 < matches.length ? matches[idx + 1].index ?? rawLine.length : rawLine.length;
      const before = rawLine.slice(Math.max(0, rawLine.lastIndexOf(" ", start - 2)), start).trim();
      const after = rawLine.slice(start + url.length, nextStart).trim();
      const label = before && !looksLikeHttpUrl(before) ? before : after;
      return label ? `${label} ${url}` : url;
    });
  });

  rows.forEach((rawLine, index) => {
    const row = index + 1;
    const line = rawLine.trim();
    if (!line) return;
    const parsed = parseBulkImportLine(line, defaultType);
    if (!parsed) {
      rejected.push({ row, text: line, reason: "No valid http(s) URL found." });
      return;
    }
    const key = parsed.url.toLowerCase();
    if (existing.has(key) || seen.has(key)) {
      duplicates.push({ row, text: line, reason: "Duplicate URL already in this import or source list." });
      return;
    }
    seen.add(key);
    const credibility = credibilityForSource(parsed);
    accepted.push({
      ...parsed,
      id: `${row}-${key}`,
      row,
      tier: tierForSourceType(parsed.type),
      credibility: credibility.credibility,
      review_note: credibility.note,
      selected: credibility.selected,
    });
  });

  return { accepted, rejected, duplicates };
}
