# Tester Report - Cleanroom First-Run Blocked By Missing MSVC Linker

Date: 2026-06-28T01:25:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop, 16 GB RAM class
Repo: https://github.com/scottconverse/CivicNewspaper.git
Product branch: stable-readiness-local-gates
Product commit: e423f5f
Directive:

- test-comms/directives/20260627-1905-coder-directive-cleanroom-first-run.md
- test-comms/directives/20260627-1910-coder-directive-product-commit-update.md

## Environment

- Windows version: Microsoft Windows 11 Home, version 10.0.26200, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16,474,668 KB visible physical memory, about 16 GB class
- GPU: Intel(R) UHD Graphics, 2,147,479,552 bytes adapter RAM, driver 32.0.101.5972
- GPU: NVIDIA GeForce RTX 4050 Laptop GPU, 4,293,918,720 bytes adapter RAM, driver 32.0.15.8129
- Disk free: C: 381,117,636,608 bytes free at start of run
- Node: no system node on PATH; used bundled Codex Node v24.14.0
- Rust: no system Rust/Cargo on PATH; installed isolated workspace Rust stable-x86_64-pc-windows-msvc, rustc 1.96.0, under work/tools
- npm: no system npm on PATH; used bundled pnpm 11.7.0 and a local workspace npm.cmd shim so Tauri's configured beforeDevCommand could resolve
- Ollama installed/running: ollama command not found on PATH; not running for this test
- Models present: not checked through Ollama because ollama command was absent

## Steps Run

1. Cloned the repo and checked out the comms branch:

```powershell
git clone https://github.com/scottconverse/CivicNewspaper.git work\CivicNewspaper
git checkout test-comms/cleanroom-coder-tester
git status --short --branch
```

2. Read the required comms files and directives:

```powershell
Get-Content test-comms\README.md
Get-Content test-comms\protocol.md
Get-Content test-comms\prompts\tester-codex-desktop-prompt.md
Get-ChildItem test-comms\directives -Force
Get-Content test-comms\directives\20260627-1905-coder-directive-cleanroom-first-run.md
Get-Content test-comms\directives\20260627-1910-coder-directive-product-commit-update.md
```

3. Switched to the product branch and verified the directive commit:

```powershell
git fetch origin
git switch stable-readiness-local-gates
git pull --ff-only origin stable-readiness-local-gates
git rev-parse --short HEAD
git merge-base --is-ancestor e423f5f HEAD
```

Result: HEAD was e423f5f, so the branch was at the requested commit.

4. Checked tool availability:

```powershell
node --version
npm --version
rustc --version
cargo --version
ollama --version
where.exe rustc
where.exe cargo
where.exe node
where.exe npm
where.exe pnpm
```

Result: system Node, npm, Rust/Cargo, and Ollama were absent. Bundled Codex Node and pnpm were available.

5. Installed missing tools where reasonable without changing the normal user PATH:

```powershell
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile work\tools\rustup-init.exe
$env:RUSTUP_HOME = "<workspace>\work\tools\rustup"
$env:CARGO_HOME = "<workspace>\work\tools\cargo"
.\work\tools\rustup-init.exe -y --no-modify-path --profile minimal --default-toolchain stable
pnpm install --lockfile=false
```

Result: Rust installed successfully into the workspace. JS dependencies installed successfully.

6. Prepared clean app-data/profile redirection:

```powershell
$env:APPDATA = "<workspace>\work\clean-profile\AppData\Roaming"
$env:LOCALAPPDATA = "<workspace>\work\clean-profile\AppData\Local"
$env:TEMP = "<workspace>\work\clean-profile\Temp"
$env:TMP = $env:TEMP
```

7. Tried the real Tauri desktop path, not browser-only Vite preview:

```powershell
pnpm tauri dev
```

Result: Vite started on http://localhost:1420, Cargo downloaded Rust crates, then the Tauri build failed before a desktop window launched because the MSVC linker was absent.

## Results

- Real Tauri app launch: Blocked. The build reached Cargo compilation but failed before the desktop app launched.
- Clean app-data/profile state: Partially prepared. APPDATA, LOCALAPPDATA, TEMP, and TMP were redirected to an isolated workspace clean-profile folder before attempting Tauri launch.
- App-data/database/settings location known and isolated: Blocked. The isolated roots were known, but the app never launched far enough to create or inspect product app data.
- First-run state not forced by query string or dev-only URL: Blocked. No real desktop window launched.
- Onboarding from first launch through workspace: Blocked. No real desktop window launched.
- Missing Ollama running behavior: Blocked. Ollama was absent, but the app did not launch far enough to observe the UI state.
- Ollama running but selected model missing: Blocked. Ollama was absent and app launch was blocked.
- Model available: Not attempted; blocked before app launch.
- Daily Scan degraded behavior without a model: Blocked. No real desktop window launched.
- Draft/Workbench degraded behavior without a model: Blocked. No real desktop window launched.
- Narrow-window navigation for Sources, Publishing, Workbench, and System: Blocked. No real desktop window launched.

## Evidence

Local-only evidence paths, not committed:

- work/evidence/tauri-dev-stdout.log
- work/evidence/tauri-dev-stderr.log
- work/clean-profile/AppData/Roaming
- work/clean-profile/AppData/Local
- work/clean-profile/Temp

Relevant stderr excerpt:

```text
error: linker `link.exe` not found
= note: program not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.
note: VS Code is a different product, and is not sufficient.
error: could not compile `quote` (build script) due to 1 previous error
[ELIFECYCLE] Command failed with exit code 101.
```

No screenshots were captured because no targetable Civic/Tauri desktop app window appeared.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Windows MSVC linker missing prevents real Tauri launch

- Observed: `pnpm tauri dev` started the Vite dev server and Cargo crate download/compilation, then failed with `linker link.exe not found`.
- Expected: The cleanroom machine can compile and launch the real Tauri desktop app, allowing first-run, dependency-absent, onboarding, degraded-state, and narrow-window behavior to be validated.
- Impact: No directive UI requirements can be truthfully verified from the real desktop app on this machine. Browser-only preview would not satisfy the directive.
- Repro: On this clean machine, run `pnpm tauri dev` after installing Rust but without Visual Studio Build Tools / Visual C++ build tools on PATH.

## Request For Coder

Smallest human action needed: provide a Windows cleanroom image or machine with Visual Studio 2017 or later / Build Tools for Visual Studio installed with the Visual C++ option so `link.exe` is available to the Rust MSVC toolchain.

Alternative coder action: provide a signed or unsigned Windows desktop build artifact/installer for commit e423f5f, if validating a prebuilt artifact is acceptable for this directive. Without one of those, tester cannot produce the required real Tauri first-run proof.
