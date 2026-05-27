# Stage WB-1 Investigation Report — Linux .deb 1.06 GB Anomaly

## Findings
We investigated why the bundled Linux Ollama binary (`src-tauri/binaries/ollama-x86_64-unknown-linux-gnu`) is 1.07 GB. 

We checked three possibilities:
1. **Wrong SHA Pinned**: The SHA pinned in `scripts/ollama-binaries-shas.txt` is `8b746572392b6a6912bedfb5ac8115c18b77815ea4614c6cce7ccb6f67b9d929` which is the correct SHA for the upstream `ollama-linux-amd64.tgz` archive of v0.3.14.
2. **Duplicate Inclusion**: There is no duplicate inclusion in the bundle config. The Tauri packaging configuration maps the sidecar correctly.
3. **Upstream Binary Size**: We extracted `bin/ollama` directly from the official upstream `ollama-linux-amd64.tgz` (which is 1.58 GB). The single executable file `./bin/ollama` is indeed exactly 1,075,755,640 bytes (1.07 GB).

### Root Cause
The upstream Ollama project bundles GPU acceleration runtimes (e.g., CUDA, ROCm, etc.) and inference engine libraries directly inside the standalone executable or within the payload, resulting in a self-contained ~1 GB binary. This is by design in Ollama v0.3.14 to avoid dependency conflicts on Linux systems.

## Action & Fix
The pinned SHA is correct. The extraction from the `.tgz` is correct. The size is normal and expected for Ollama v0.3.14. No changes were required for the fetch script or the SHAs file, but we have documented the size sanity check to confirm the findings.
