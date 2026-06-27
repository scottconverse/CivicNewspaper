import { describe, expect, test } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { buildBulkImportReview } from "./bulkImportParser";

const extractedDir = process.env.CIVICNEWS_IMPORT_EXTRACTED_DIR;

describe.skipIf(!extractedDir)("local bulk-import fixture review", () => {
  const cases = [
    ["colorado-source-list-clean.csv.txt", 35],
    ["colorado-source-list-messy.xlsx.txt", 20],
    ["colorado-source-list-human-notes.txt.txt", 25],
    ["colorado-source-list-briefing.docx.txt", 25],
    ["colorado-source-list-exported.pdf.txt", 20],
    ["colorado-source-list-edge-cases.xlsx.txt", 10],
  ] as const;

  test("fixture extraction directory contains the expected files", () => {
    const files = readdirSync(extractedDir!);
    for (const [name] of cases) {
      expect(files).toContain(name);
    }
  });

  test.each(cases)("%s produces a reviewable candidate set", (name, minAccepted) => {
    const text = readFileSync(join(extractedDir!, name), "utf8");
    const review = buildBulkImportReview(text, "primary_record", []);
    const totalParsed = review.accepted.length + review.duplicates.length;

    expect(totalParsed).toBeGreaterThanOrEqual(minAccepted);
    expect(review.accepted.length).toBeGreaterThan(0);
    expect(review.accepted.every((item) => /^https?:\/\//i.test(item.url))).toBe(true);
  });
});
