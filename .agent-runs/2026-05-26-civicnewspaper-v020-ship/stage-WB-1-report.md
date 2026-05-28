# Stage WB-1 Investigation Report — Linux .deb 1.06 GB Anomaly

## Findings
We investigated why the bundled Linux Ollama binary (`src-tauri/binaries/ollama-x86_64-unknown-linux-gnu`) is 1.07 GB. 

We checked three possibilities:
1. **Wrong SHA Pinned**: The SHA pinned in `scripts/ollama-binaries-shas.txt` is `8b746572392b6a6912bedfb5ac8115c18b77815ea4614c6cce7ccb6f67b9d929` which is the correct SHA for the upstream `ollama-linux-amd64.tgz` archive of v0.3.14.
2. **Duplicate Inclusion**: There is no duplicate inclusion in the bundle config. The Tauri packaging configuration maps the sidecar correctly.
3. **Upstream Binary Size**: We extracted `bin/ollama` directly from the official upstream `ollama-linux-amd64.tgz` (which is 1.58 GB). The single executable file `./bin/ollama` is indeed exactly 1,075,755,640 bytes (1.07 GB).

### Root Cause
The ~1 GB size of the extracted binary reflects the monolithic `bin/ollama` executable itself. Upstream Ollama compiles its server and core orchestration logic directly into this single file. 

However, GPU acceleration shared libraries (such as `libcublas.so` and `libcudart.so`) are located in a separate `lib/ollama/` directory inside the upstream tarball. Since our `scripts/fetch-ollama-binaries.sh` only extracts `bin/ollama` and discards `lib/ollama/`, the packaged application does not bundle these GPU libraries. As a result, GPU acceleration falls back to CPU at runtime on Linux.

Cited evidence:
- Upstream Ollama v0.3.14 Linux Release: https://github.com/ollama/ollama/releases/tag/v0.3.14
- Upstream `ollama-linux-amd64.tgz` tarball layout and file sizes: https://github.com/ollama/ollama/releases/download/v0.3.14/ollama-linux-amd64.tgz

## Action & Fix
The pinned SHA is correct. The extraction from the `.tgz` is correct. The size of the monolithic binary is expected. No changes were required for the fetch script or the SHAs file, but we have documented the GPU library limitation in the carried debt list.
