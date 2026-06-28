# WebView-safe identity rerun report for 65d97f4

Status: **FAIL - setup gate blocked**

Directive: `test-comms/directives/20260628-rerun-webview-safe-identity-after-65d97f4.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `d3b0977e959e8e56edf082c18906b6b7bdd841ec`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `65d97f4b517e6bac2a1f77b603d9916534b1ce32` (`Make first-run identity fields WebView-safe`)

## Result

The rerun stopped at required setup retest step 8.

The Civic Desk launched at 1280x720 and the Publication Name field showed visible initial focus with a caret. However, `ABC` did not appear in the Publication Name field after keyboard entry attempts. I tried:

1. Typing `ABC` with the field visibly focused immediately after launch.
2. Clicking inside Publication Name and sending `A`, `B`, `C` as virtual key events.

In both cases, the field remained empty. Because the directive says to stop and report the exact failure if any setup/model gate fails, I did not continue to identity completion, runtime setup, Ollama startup, model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `439288E91E78F65783D450C03F2A001D3B4E973EDBEE41FE4F6A5DE76C6C374B` | `439288E91E78F65783D450C03F2A001D3B4E973EDBEE41FE4F6A5DE76C6C374B` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `D828D3C5B43047ED8B86F7EB2AB5F197BE0747708DF2C5D07216005C7BA00B5A` | `D828D3C5B43047ED8B86F7EB2AB5F197BE0747708DF2C5D07216005C7BA00B5A` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-webview-safe-identity-rerun-65d97f4\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

The running app process was:

| Process | Window title | Responding | Executable |
| --- | --- | --- | --- |
| `civicnews` | `The Civic Desk` | True | `%LOCALAPPDATA%\The Civic Desk\civicnews.exe` |

No helper terminal/console appeared before the setup gate blocker. Runtime setup was not reached.

## Required setup/model checklist

| Gate | Result |
| --- | --- |
| Pull/reread active directive | PASS |
| Verify artifact hashes | PASS |
| Clean reset | PASS |
| Install artifact | PASS |
| Launch The Civic Desk | PASS |
| Set window to 1280x720 | PASS |
| Publication Name has initial visible focus | PASS |
| Type `ABC` into Publication Name and confirm it appears | FAIL |
| Fill Publication Name, Editor Name, Publisher Type, City, State | NOT REACHED |
| Click Next and verify identity values persist | NOT REACHED |
| First-run action row visible/reachable while body scrolls | NOT REACHED |
| No helper/terminal/console during runtime setup | NOT REACHED |
| App-managed runtime setup starts Ollama | NOT REACHED |
| `GET http://127.0.0.1:11434/api/tags` reachable | NOT REACHED |
| Body `Download qwen2.5:7b` starts pull | NOT REACHED |
| Footer `Next` starts pull from clean no-model state | NOT REACHED |
| Model download completes | NOT REACHED |
| `/api/tags` lists downloaded model | NOT REACHED |

## Longmont E2E / publish / soak

Not reached because setup gate step 8 failed.

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

- `test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/01-first-run-1280.png` - 1280x720 first-run screen; Publication Name has visible focus and caret.
- `test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/02-publication-abc.png` - after typing `ABC` while initial focus was visible; field still empty.
- `test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/03-publication-abc-click-keybd-event.png` - after clicking in the field and sending virtual key events for `ABC`; field still empty.

## Exact breakpoint

The product still presents the release-blocking first-run behavior: Publication Name visually receives focus/caret but does not accept/display typed text. The new WebView-safe identity changes in `65d97f4` did not resolve the cleanroom desktop input blocker on this tester machine.
