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

const HTTP_URL_RE = /https?:\/\/[^\s<>"')\]]+/i;

function cleanField(value: string): string {
  return value
    .trim()
    .replace(/^["'`]+|["'`]+$/g, "")
    .trim();
}

function cleanUrl(value: string): string {
  return cleanField(value).replace(/[.,;:!?]+$/g, "");
}

function looksLikeHttpUrl(value: string): boolean {
  return /^https?:\/\//i.test(cleanField(value));
}

function isValidSourceType(value: string): boolean {
  return VALID_SOURCE_TYPES.includes(value as (typeof VALID_SOURCE_TYPES)[number]);
}

function deriveNameFromUrl(url: string): string {
  try {
    const parsedUrl = new URL(url);
    return parsedUrl.hostname.replace(/^www\./i, "");
  } catch {
    return url;
  }
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
      url: cleanUrl(markdownMatch[2]),
    };
  }

  const htmlMatch = line.match(/<a\b[^>]*href=["'](https?:\/\/[^"']+)["'][^>]*>(.*?)<\/a>/i);
  if (htmlMatch) {
    return {
      name: cleanField(htmlMatch[2].replace(/<[^>]*>/g, "")),
      url: cleanUrl(htmlMatch[1]),
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
      url = cleanUrl(fields[urlFieldIndex]);
      const typeField = fields.find((field) => isValidSourceType(field));
      if (typeField) type = typeField;

      const nameField = fields.find((field, index) => {
        return index !== urlFieldIndex && !isValidSourceType(field) && !looksLikeHttpUrl(field);
      });
      name = nameField ? cleanField(nameField) : deriveNameFromUrl(url);
    } else {
      const urlMatch = line.match(HTTP_URL_RE);
      if (!urlMatch) return null;
      url = cleanUrl(urlMatch[0]);
      name = nameNearUrl(line, url);
    }
  }

  if (!looksLikeHttpUrl(url)) {
    return null;
  }

  if (!isValidSourceType(type)) {
    type = defaultType;
  }

  return {
    name: name || deriveNameFromUrl(url),
    url,
    type,
  };
}
