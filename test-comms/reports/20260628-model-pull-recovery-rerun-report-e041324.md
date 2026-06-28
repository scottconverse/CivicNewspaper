# Model-pull recovery rerun report for e041324

Status: **FAIL - setup cannot continue after automatic model pull**

Directive: `test-comms/directives/20260628-rerun-model-pull-recovery-after-e041324.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `4051b1ea10e3ac7d509e26917bd0e21a2c8078c6`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `e0413247945afe5c0072be6b31b39b6c186da774` (`Auto-start model pull when setup input is stuck`)

## Result

The new model-pull recovery partially worked.

The packaged app launched at 1280x720, recovered past stuck identity input, started the app-managed Ollama runtime, detected the no-model state, advanced to Step 3, and automatically started the recommended `qwen2.5:7b` model pull. After monitoring, `GET http://127.0.0.1:11434/api/tags` listed `qwen2.5:7b`.

The run stopped because the app UI remained on the Step 3 `Download AI Model` screen after the model appeared in `/api/tags`, and the packaged UI still did not receive enough input/scroll events to continue setup or proceed into the Longmont E2E workflow. I did not manually pull models, manually install Ollama, use developer tools, or bypass setup.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `CF0348593E4E98530D24AE1E449F9DF27FB165E1449BA7B37451B21B39BA4333` | `CF0348593E4E98530D24AE1E449F9DF27FB165E1449BA7B37451B21B39BA4333` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `845514BEE27E0B594DA0EE791F1AB8C8327B26BD055B9159B774DD8A02F5F2F7` | `845514BEE27E0B594DA0EE791F1AB8C8327B26BD055B9159B774DD8A02F5F2F7` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-model-pull-recovery-rerun-e041324\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

The running processes/listeners were:

| Item | Observed |
| --- | --- |
| `civicnews` | Running, window title `The Civic Desk`, responding |
| `ollama` | Running, app-managed startup |
| App backend listener | `127.0.0.1:12053` |
| Ollama listener | `127.0.0.1:11434` |
| `GET http://127.0.0.1:11434/api/tags` before pull recovery | HTTP 200, `{"models":[]}` |
| `GET http://127.0.0.1:11434/api/tags` after pull recovery | HTTP 200, listed `qwen2.5:7b` |

No helper terminal/console appeared before the blocker.

## Required setup/model checklist

| Gate | Result |
| --- | --- |
| Pull/reread active directive | PASS |
| Verify artifact hashes | PASS |
| Clean reset | PASS |
| Install artifact | PASS |
| Launch app at 1280x720 | PASS |
| Complete first-run setup only through app UI/recovery paths | PARTIAL PASS |
| Step 1 stuck-input identity recovery advances setup | PASS |
| App starts or installs its own AI runtime | PASS |
| `/api/tags` reachable and initially no models | PASS |
| Step 2 auto-starts recommended model pull when input remains stuck | PASS |
| Step 3 appears and `qwen2.5:7b` starts downloading from inside app | PASS |
| Model appears in `/api/tags` | PASS |
| App proceeds beyond model setup into usable workflow | FAIL - UI remained on Step 3 and input/scroll path remained unusable |

## Longmont E2E / publish / soak

Not reached because setup could not continue after automatic model pull.

| Item | Count / value |
| --- | --- |
| AI-generated drafts from real Longmont source material | 0 |
| Sources | 0 |
| Leads | 0 |
| Drafts | 0 |
| Approved stories/briefs | 0 |
| Held/sent-back items | 0 |
| Cut/killed items | 0 |
| here.now URL | Not reached |
| Local output folder | Not reached |
| Exported ZIP path | Not reached |

## Artifacts

Screenshots are clean app-window captures with no helper windows visible:

- `test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/01-after-launch-recovery-state.png` - after launch; Step 1 identity recovery already advanced to Step 2.
- `test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/02-after-model-pull-recovery-wait.png` - Step 3 appears with automatic model-download recovery notice.
- `test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/03-model-pull-progress-monitor.png` - monitored Step 3 while `/api/tags` listed `qwen2.5:7b`.
- `test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/04-model-listed-still-step3.png` - after an additional wait, `/api/tags` still listed `qwen2.5:7b`, but app remained on Step 3.

## Exact breakpoint

`e041324` successfully starts the recommended model pull automatically and Ollama lists `qwen2.5:7b`, but the packaged app does not advance beyond Step 3 into the usable application/workflow. The next fix should auto-detect the model listed in `/api/tags` and continue setup without requiring WebView scroll/click/key input.
