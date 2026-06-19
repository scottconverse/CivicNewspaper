// src/bulkImportParser.test.ts
// TEST-Mn4: unit coverage for the extracted pure bulk-import line parser —
// well-formed lines, malformed/skippable lines, type handling, and an
// imported-count check that mirrors how useApp.handleBulkImport loops the parser.
import { describe, test, expect } from "vitest";
import { parseBulkImportLine } from "./bulkImportParser";

describe("parseBulkImportLine", () => {
  test("parses a bare URL and derives the hostname (stripping www.)", () => {
    expect(parseBulkImportLine("https://www.example.com/foo", "media_lead")).toEqual({
      name: "example.com",
      url: "https://www.example.com/foo",
      type: "media_lead",
    });
  });

  test("parses `name, url` CSV form", () => {
    expect(parseBulkImportLine("City Records, https://records.gov", "primary_record")).toEqual({
      name: "City Records",
      url: "https://records.gov",
      type: "primary_record",
    });
  });

  test("parses `url, name` CSV form (url-first detection)", () => {
    expect(parseBulkImportLine("https://records.gov, City Records", "primary_record")).toEqual({
      name: "City Records",
      url: "https://records.gov",
      type: "primary_record",
    });
  });

  test("honors an explicit valid third-field type", () => {
    expect(parseBulkImportLine("Name, https://x.com, official_comm", "media_lead")).toEqual({
      name: "Name",
      url: "https://x.com",
      type: "official_comm",
    });
  });

  test("falls back to default type when the third field is not a recognized type", () => {
    expect(parseBulkImportLine("Name, https://x.com, bogus_type", "community_signal")).toEqual({
      name: "Name",
      url: "https://x.com",
      type: "community_signal",
    });
  });

  test.each([
    ["blank line", "   "],
    ["empty string", ""],
    ["non-URL bare token", "just-some-text"],
    ["ftp scheme (not http/https)", "ftp://files.example.com"],
    ["single CSV field", "OnlyOneField"],
    ["CSV with no valid URL in either slot", "Name, also-not-a-url"],
  ])("returns null for malformed/skippable input: %s", (_label, input) => {
    expect(parseBulkImportLine(input, "primary_record")).toBeNull();
  });

  test("imported-count: only well-formed http(s) lines are counted (mirrors handleBulkImport loop)", () => {
    const text = [
      "https://a.com",            // ok
      "",                          // skip (blank)
      "   ",                       // skip (whitespace)
      "not-a-url",                 // skip (no scheme)
      "Name B, https://b.com",     // ok
      "ftp://c.com",               // skip (wrong scheme)
      "https://d.com, D Name, media_lead", // ok
      "OnlyOne",                   // skip (single field, no url)
    ];

    let importedCount = 0;
    const parsed = [];
    for (const rawLine of text) {
      const p = parseBulkImportLine(rawLine, "community_signal");
      if (!p) continue;
      parsed.push(p);
      importedCount++;
    }

    expect(importedCount).toBe(3);
    expect(parsed.map((p) => p.url)).toEqual([
      "https://a.com",
      "https://b.com",
      "https://d.com",
    ]);
    expect(parsed[2].type).toBe("media_lead");
  });
});
