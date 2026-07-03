import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, test } from "vitest";

describe("layout CSS guardrails", () => {
  test("keeps the full-width mobile navigation out of medium desktop webviews", () => {
    const css = readFileSync(join(process.cwd(), "src", "App.css"), "utf8");

    expect(css).not.toContain("@media (max-width: 980px)");
    expect(css).toMatch(/@media\s*\(max-width:\s*700px\)\s*{[\s\S]*?\.sidebar\s*{[\s\S]*?width:\s*100%\s*!important;/);
  });
});
