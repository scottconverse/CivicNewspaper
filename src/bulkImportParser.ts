// src/bulkImportParser.ts
// TEST-Mn4: the bulk-import line parser, extracted as a pure function so it can
// be unit-tested independently of useApp's React/IPC machinery. handleBulkImport
// in useApp.ts delegates per-line parsing here; behavior is identical to the
// previous inline implementation.

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

/**
 * Parse a single bulk-import line into a source record.
 *
 * Accepts either:
 *   - a bare URL (`https://example.com`) — name is derived from the hostname
 *   - a CSV-ish line (`name, url[, type]` or `url, name[, type]`) — the first
 *     field is treated as the URL when it looks like one, otherwise the second
 *     field is the URL.
 *
 * Returns `null` for lines that should be skipped: blank lines, malformed CSV
 * lines (fewer than two fields), or any line that does not resolve to an
 * http(s) URL. An unrecognized type falls back to `defaultType`.
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

  if (line.includes(",")) {
    const parts = line.split(",").map((p) => p.trim());
    if (parts.length >= 2) {
      if (parts[0].startsWith("http://") || parts[0].startsWith("https://")) {
        url = parts[0];
        name = parts[1];
        if (parts.length >= 3 && parts[2]) {
          type = parts[2];
        }
      } else {
        name = parts[0];
        url = parts[1];
        if (parts.length >= 3 && parts[2]) {
          type = parts[2];
        }
      }
    }
  } else {
    url = line;
    try {
      const parsedUrl = new URL(url);
      name = parsedUrl.hostname.replace("www.", "");
    } catch {
      name = url;
    }
  }

  if (url.startsWith("http://") || url.startsWith("https://")) {
    if (!VALID_SOURCE_TYPES.includes(type as (typeof VALID_SOURCE_TYPES)[number])) {
      type = defaultType;
    }
    return { name, url, type };
  }

  return null;
}
