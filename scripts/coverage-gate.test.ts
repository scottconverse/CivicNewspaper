import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

const root = resolve(import.meta.dirname, "..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");

describe("release coverage gate", () => {
  test("uses pinned coverage tooling and enforceable measured floors", () => {
    const packageJson = JSON.parse(read("package.json"));
    const config = read("vitest.config.ts");

    expect(packageJson.devDependencies["@vitest/coverage-v8"]).toBe("4.1.9");
    expect(packageJson.scripts["test:coverage"]).toBe("vitest run --coverage");
    expect(config).toContain("statements: 60");
    expect(config).toContain("branches: 65");
    expect(config).toContain("functions: 55");
    expect(config).toContain("lines: 60");
    expect(config).toContain('"src/components/OnboardingWizard.tsx"');
    expect(config).toContain('"src/components/PublishPanel.tsx"');
  });

  test("makes coverage failures block CI and tag releases", () => {
    const ci = read(".github/workflows/ci.yml");
    const release = read(".github/workflows/release.yml");
    const coverageJob = ci.slice(ci.indexOf("\n  coverage:"));

    expect(coverageJob).not.toContain("continue-on-error: true");
    expect(coverageJob).toContain("--fail-under-lines 72");
    expect(coverageJob).toContain("npm run test:coverage");
    expect(coverageJob).toContain("actions/upload-artifact@v4");
    expect(release).toContain("--fail-under-lines 72");
    expect(release).toContain("npm run test:coverage");
  });

  test("requires accessible packaged first-run and live core-flow proof", () => {
    const walkthrough = read("scripts/packaged-first-run-walkthrough.ps1");
    const webviewDriver = read("scripts/packaged-webview-driver.mjs");
    const releaseSmoke = read("scripts/release-smoke.ps1");
    const rcEvidence = read("scripts/prepare-rc-evidence.ps1");

    expect(walkthrough).toContain("packaged-webview-driver.mjs");
    expect(walkthrough).toContain("--expected-build-id $($receipt.commit)");
    expect(walkthrough).toContain("core-flow-database-persistence");
    expect(walkthrough).toContain('"$receiptPath.sha256"');
    expect(walkthrough).not.toContain("SendKeys");
    expect(walkthrough).not.toContain("Click-WindowPoint");
    expect(webviewDriver).toContain('locator("#civicnews-build-id")');
    expect(webviewDriver).toContain('check("build-id-matches-commit"');
    expect(releaseSmoke).toContain("packaged-first-run-and-core-flow");
    expect(rcEvidence).toContain("packagedWalkthrough.installer_sha256");
    expect(rcEvidence).toContain("packagedWalkthrough.installer_size");
    expect(rcEvidence).toContain("does not match the packaged walkthrough receipt");
    expect(rcEvidence).toContain("core-draft-reloaded-in-workbench");
  });

  test("makes any model bakeoff case failure fail the command", () => {
    const bakeoff = read("scripts/model-bakeoff.mjs");

    expect(bakeoff).toContain("results.some((result) => !result.ok)");
    expect(bakeoff).toContain("process.exitCode = 1");
  });
});
