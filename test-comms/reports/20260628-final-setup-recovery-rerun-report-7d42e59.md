# Final setup recovery rerun report for 7d42e59

Status: **FAIL - onboarding still does not complete after recovered model install**

Directive: `test-comms/directives/20260628-rerun-final-setup-recovery-after-7d42e59.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `a2234ac674ef594b00e0377faaec8edfd71586bb`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `7d42e59f9c9db17217c7eca0701e5749f13c0578` (`Complete onboarding after recovered model install`)

## Result

The setup recovery flow still did not reach the usable main application.

The packaged app launched at 1280x720, recovered past stuck identity input, started the app-managed Ollama runtime, auto-started the recommended model pull, and `/api/tags` listed `qwen2.5:7b`.

However, after more than 3 minutes with `qwen2.5:7b` listed, the app remained on the Step 3 `Download AI Model` screen with the recovery notice. It did not mark onboarding complete or enter the application. I did not manually install Ollama, manually pull a model, use a helper terminal, repair PATH, use developer tools, or bypass setup.

Because setup still blocked, I stopped before the Longmont E2E workflow, here.now publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `45F0B044688E3C7F68FBCC35BB70DC31C06600AF7B4CED41EC9CE1A7714AD418` | `45F0B044688E3C7F68FBCC35BB70DC31C06600AF7B4CED41EC9CE1A7714AD418` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `62400F326942EF9E1196059F5523DB4CBE5531C7B9D59F5E8E8B38CFFFFC0778` | `62400F326942EF9E1196059F5523DB4CBE5531C7B9D59F5E8E8B38CFFFFC0778` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-final-setup-recovery-rerun-7d42e59\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

The running processes/listeners were:

| Item | Observed |
| --- | --- |
| `civicnews` | Running, window title `The Civic Desk`, responding |
| `ollama` | Running, app-managed startup |
| App backend listener | `127.0.0.1:12053` |
| Ollama listener | `127.0.0.1:11434` |
| `/api/tags` initially | HTTP 200, `{"models":[]}` |
| `/api/tags` after recovery pull | HTTP 200, listed `qwen2.5:7b` |

No helper terminal/console appeared before the blocker.

## Required setup checklist

| Gate | Result |
| --- | --- |
| Pull/reread active directive | PASS |
| Verify artifact hashes | PASS |
| Clean reset | PASS |
| Install artifact | PASS |
| Launch app at 1280x720 | PASS |
| Complete first-run setup only through app UI/recovery paths | FAIL |
| Identity stuck-input recovery advances to Step 2 | PASS |
| App starts its own AI runtime | PASS |
| Auto-start model pull | PASS |
| `/api/tags` lists `qwen2.5:7b` | PASS |
| Final setup recovery marks onboarding complete and enters main application | FAIL |

## Longmont E2E / publish / soak

Not reached because onboarding never completed.

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

- `test-comms/artifacts/20260628-final-setup-recovery-rerun-7d42e59/01-after-launch-recovery-state.png` - after launch; identity recovery advanced to Step 2.
- `test-comms/artifacts/20260628-final-setup-recovery-rerun-7d42e59/02-final-recovery-monitor-80s.png` - Step 3 model download recovery screen.
- `test-comms/artifacts/20260628-final-setup-recovery-rerun-7d42e59/03-final-recovery-monitor-220s.png` - after more than 3 minutes with `qwen2.5:7b` listed; app still remained on Step 3.

## Exact breakpoint

`7d42e59` still does not complete onboarding after recovered model install. The model is installed and visible in `/api/tags`, but the packaged app remains on Step 3 instead of saving the selected model/default folders, marking onboarding complete, and entering the main application.
