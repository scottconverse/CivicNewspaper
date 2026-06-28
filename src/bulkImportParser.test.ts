// src/bulkImportParser.test.ts
// TEST-Mn4: unit coverage for the extracted pure bulk-import line parser —
// well-formed lines, malformed/skippable lines, type handling, and an
// imported-count check that mirrors how useApp.handleBulkImport loops the parser.
import { describe, test, expect } from "vitest";
import { buildBulkImportReview, parseBulkImportLine } from "./bulkImportParser";

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

  test("parses quoted CSV rows with commas in the source name", () => {
    expect(parseBulkImportLine('"City Records, Clerk", "https://records.gov/rss", primary_record', "media_lead")).toEqual({
      name: "City Records, Clerk",
      url: "https://records.gov/rss",
      type: "primary_record",
    });
  });

  test("parses TSV and pipe-separated exports", () => {
    expect(parseBulkImportLine("Library Events\thttps://library.gov/events\tcommunity_signal", "primary_record")).toEqual({
      name: "Library Events",
      url: "https://library.gov/events",
      type: "community_signal",
    });
    expect(parseBulkImportLine("https://paper.example/news | Local Paper | media_lead", "primary_record")).toEqual({
      name: "Local Paper",
      url: "https://paper.example/news",
      type: "media_lead",
    });
  });

  test("parses markdown and HTML links", () => {
    expect(parseBulkImportLine("[Council Agendas](https://city.gov/agendas)", "primary_record")).toEqual({
      name: "Council Agendas",
      url: "https://city.gov/agendas",
      type: "primary_record",
    });
    expect(parseBulkImportLine('<a href="https://city.gov/notices">Public Notices</a>', "official_comm")).toEqual({
      name: "Public Notices",
      url: "https://city.gov/notices",
      type: "official_comm",
    });
  });

  test("parses label plus URL rows from copied documents", () => {
    expect(parseBulkImportLine("City Council Agendas - https://city.gov/agendas.", "primary_record")).toEqual({
      name: "City Council Agendas",
      url: "https://city.gov/agendas",
      type: "primary_record",
    });
  });

  test("trims unbalanced trailing punctuation from extracted spreadsheet and document URLs", () => {
    expect(parseBulkImportLine("Denver Legistar, https://denver.legistar.com/Calendar.aspx)", "primary_record")).toEqual({
      name: "Denver Legistar",
      url: "https://denver.legistar.com/Calendar.aspx",
      type: "primary_record",
    });
    expect(parseBulkImportLine("Map notes, https://city.gov/maps/site(plan)", "primary_record")?.url).toBe("https://city.gov/maps/site(plan)");
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

  test("builds an import review with credibility labels, duplicates, and skipped rows", () => {
    const review = buildBulkImportReview([
      "Council, https://city.gov/agendas",
      "Search, https://www.google.com/search?q=city+agenda",
      "Forum, https://reddit.com/r/city",
      "Duplicate, https://city.gov/agendas",
      "not a url",
    ].join("\n"), "primary_record", ["https://existing.gov/rss"]);

    expect(review.accepted).toHaveLength(3);
    expect(review.accepted[0]).toMatchObject({
      name: "Council",
      credibility: "Official record",
      selected: true,
      tier: "official_record",
    });
    expect(review.accepted[1]).toMatchObject({
      credibility: "Search helper",
      selected: false,
    });
    expect(review.accepted[2]).toMatchObject({
      credibility: "Community signal",
      selected: false,
    });
    expect(review.duplicates).toHaveLength(1);
    expect(review.rejected).toHaveLength(1);
  });

  test("recovers multiple labeled URLs from a single extracted document line", () => {
    const review = buildBulkImportReview(
      "City of Longmont https://longmontcolorado.gov/ City Council https://longmontcolorado.gov/city-council Local News https://www.longmontleader.com/local-news",
      "primary_record",
      []
    );

    expect(review.accepted.map((item) => item.url)).toEqual([
      "https://longmontcolorado.gov/",
      "https://longmontcolorado.gov/city-council",
      "https://www.longmontleader.com/local-news",
    ]);
    expect(review.accepted).toHaveLength(3);
  });

  test("treats known civic portal hosts as official records even when they are not dot-gov", () => {
    const review = buildBulkImportReview([
      "Denver official website, https://www.denvergov.org/",
      "Denver City Council agendas, https://denver.legistar.com/Calendar.aspx",
    ].join("\n"), "primary_record", []);

    expect(review.accepted).toHaveLength(2);
    expect(review.accepted.every((item) => item.credibility === "Official record")).toBe(true);
    expect(review.accepted.every((item) => item.selected)).toBe(true);
  });
});
