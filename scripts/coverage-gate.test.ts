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

  test("signs every shipped Windows executable during Tauri bundling", () => {
    const release = read(".github/workflows/release.yml");
    const smoke = read(".github/workflows/signing-smoke.yml");
    const tauriConfig = JSON.parse(read("src-tauri/tauri.windows-signing.conf.json"));
    const signer = read("scripts/sign-windows-artifact.ps1");

    expect(tauriConfig.bundle.windows.signCommand).toEqual({
      cmd: "powershell.exe",
      args: [
        "-NoLogo",
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        "scripts\\sign-windows-artifact.ps1",
        "-File",
        "%1",
      ],
    });
    expect(release).toContain("Install-Module -Name ArtifactSigning -RequiredVersion 0.1.8");
    expect(smoke).toContain("Install-Module -Name ArtifactSigning -RequiredVersion 0.1.8");
    expect(release).toContain("tauri.windows-signing.conf.json");
    expect(smoke).toContain("tauri.windows-signing.conf.json");
    expect(release).toContain("release-signature-proof-${{ github.sha }}");
    expect(signer).toContain("Invoke-ArtifactSigning @params");
    expect(signer).toContain("ExcludeAzureCliCredential = $true");
    expect(signer).not.toContain("az login");
    expect(release).not.toContain("artifact-signing-cli");
    expect(smoke).not.toContain("artifact-signing-cli");
    expect(release).toContain("AZURE_CLIENT_SECRET");
    expect(smoke).toContain("AZURE_CLIENT_SECRET");
    expect(release).not.toContain('client_secret = "${{ secrets.AZURE_CLIENT_SECRET }}"');
    expect(smoke).not.toContain('client_secret = "${{ secrets.AZURE_CLIENT_SECRET }}"');
    expect(release).toContain("client_secret = $env:AZURE_CLIENT_SECRET");
    expect(smoke).toContain("client_secret = $env:AZURE_CLIENT_SECRET");
    expect(release).toContain("oauth2/v2.0/token");
    expect(release).toContain("src-tauri\\target\\${{ matrix.target }}\\release\\bundle\\nsis");
    expect(release).not.toContain("azure/login@");
    expect(smoke).not.toContain("azure/login@");
  });

  test("provides a non-publishing manual signing smoke workflow", () => {
    const smoke = read(".github/workflows/signing-smoke.yml");
    const verifier = read("scripts/verify-windows-signatures.ps1");

    expect(smoke).toContain("workflow_dispatch:");
    expect(smoke).toContain("scripts/verify-windows-signatures.ps1");
    expect(smoke).toContain("actions/upload-artifact@v4");
    expect(smoke).not.toContain("gh release");
    expect(smoke).not.toContain("azure/login@");
    expect(verifier).toContain("Get-AuthenticodeSignature -LiteralPath $Path");
    expect(verifier).toContain('Get-SignatureRecord -Name "installer"');
    expect(verifier).toContain('Get-SignatureRecord -Name "application"');
    expect(verifier).toContain('Get-SignatureRecord -Name "uninstaller"');
    expect(verifier).toContain("TimeStamperCertificate");
    expect(verifier).toContain('Status -eq "Valid"');
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

  test("keeps the proved local candidate distinct from the published asset", () => {
    const policy = read("scripts/policy/check_release_docs_consistency.py");
    const landing = read("docs/index.html");

    expect(policy).toContain("LOCAL_CANDIDATE_COMMIT");
    expect(policy).toContain("PUBLISHED_COMMIT");
    expect(landing).toContain("has not been uploaded");
    expect(landing).toContain("do not use its hash for the current download");
  });

  test("renders the nested evidence report with publish-root navigation", () => {
    const renderer = read("scripts/render_public_docs.py");
    const reportPage = read("docs/release-evidence/v0.3.2-local-isolated-package-report.html");
    const docCss = read("docs/doc-page.css");
    const siteCss = read("docs/style.css");

    expect(renderer).toContain('"release-evidence/v0.3.2-local-isolated-package-report.md"');
    expect(renderer).toContain('page_prefix = "../" *');
    expect(reportPage).toContain('href="../style.css"');
    expect(reportPage).toContain('href="../index.html#top"');
    expect(docCss).toContain("overflow-wrap: anywhere");
    expect(siteCss).toContain(".download-card code");
  });

  test("keeps the direct desktop IPC draft regression in the Rust suite", () => {
    const commands = read("src-tauri/src/tauri_cmds.rs");

    expect(commands).toContain("registered_ipc_command_generates_and_reloads_linked_draft");
    expect(commands).toContain("tauri::generate_handler![generate_and_save_draft]");
    expect(commands).toContain("tauri::test::get_ipc_response");
    expect(commands).toContain("Arc<dyn LlmClient>");
    expect(commands).toContain("get_draft(&db.lock().unwrap(), draft.id.unwrap())");
  });
});
