# Stuck-input recovery rerun report for 30035ac

Status: **FAIL - model setup gate blocked after recovery**

Directive: `test-comms/directives/20260628-rerun-stuck-input-recovery-after-30035ac.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `4a76f8ec173e141a4218602c6ff6be1f084b6967`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `30035aca074222950f2a5af6c9b9a70c0bc13b2a` (`Add stuck-input recovery for first-run setup`)

## Result

The stuck-input recovery path worked: the packaged app advanced from Step 1 to Step 2 by itself and displayed the recovery notice. The visible notice said the setup screen did not receive input events, so The Civic Desk continued with a starter profile and identity can be edited later in Settings.

However, the run stopped at the next setup/model gate. After recovery reached Step 2, the UI still did not respond to normal scroll or keyboard navigation. I could not scroll the Step 2 body down to the action row or no-model prompt, and I could not reach or activate the `Download qwen2.5:7b` control from the UI.

Because the directive says to report the next exact blocker if controls are still not responding, I stopped there. I did not continue to model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `10A19060494961BE9E85E0FA07E011232A68A9C1CFDE0C4FD733DD7BD20D3296` | `10A19060494961BE9E85E0FA07E011232A68A9C1CFDE0C4FD733DD7BD20D3296` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `E6CB22A236E018FD414B494CCADFFD697A819D44D56488195F897643199F241E` | `E6CB22A236E018FD414B494CCADFFD697A819D44D56488195F897643199F241E` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-stuck-input-recovery-rerun-30035ac\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

The running processes/listeners were:

| Item | Observed |
| --- | --- |
| `civicnews` | Running, window title `The Civic Desk`, responding |
| `ollama` | Running, started by app-managed setup |
| App backend listener | `127.0.0.1:12053` |
| Ollama listener | `127.0.0.1:11434` |
| `GET http://127.0.0.1:11434/api/tags` | HTTP 200, `{"models":[]}` |

No helper terminal/console appeared before the blocker.

## Required setup/model checklist

| Gate | Result |
| --- | --- |
| Pull/reread active directive | PASS |
| Verify artifact hashes | PASS |
| Clean reset | PASS |
| Install artifact | PASS |
| Launch The Civic Desk | PASS |
| Set window to 1280x720 | PASS |
| Publication Name has initial visible focus | NOT CAPTURED - recovery fired before first manual input attempt |
| Try typing `ABC` and record whether it appears | NOT CAPTURED on Step 1 - recovery fired first |
| Try clicking visible `Longmont` starter profile link | NOT CAPTURED on Step 1 - recovery fired first |
| Stuck-input recovery notice appears and advances to Step 2 | PASS |
| No helper/terminal/console during runtime setup | PASS |
| App-managed runtime setup starts Ollama | PASS |
| `GET http://127.0.0.1:11434/api/tags` reachable | PASS |
| Reach `Download qwen2.5:7b` at 1280x720 or by normal scrolling | FAIL - Step 2 did not respond to scroll/key navigation |
| Click body `Download qwen2.5:7b` | NOT REACHED |
| Model download completes | NOT REACHED |
| `/api/tags` lists downloaded model | NOT REACHED |

## Longmont E2E / publish / soak

Not reached because the model setup gate was blocked after recovery.

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

- `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/01-first-run-1280.png` - first captured screen after launch; app had already advanced to Step 2 and showed the stuck-input recovery notice.
- `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/02-step2-ollama-tags-empty.png` - Step 2 with recovery notice after confirming Ollama was reachable and tags were empty.
- `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/03-step2-scrolled-bottom.png` - after normal mouse-wheel scroll attempt; viewport did not move.
- `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/04-step2-keyboard-navigation-attempt.png` - after PageDown/Tab/Enter navigation attempt; viewport did not move and no action advanced.

## Exact breakpoint

`30035ac` successfully recovers past first-run identity setup and starts Ollama, but the packaged Windows app still does not receive enough input events to scroll/reach/activate the model-download controls on Step 2. The next fix should make Step 2 no-model/model setup reachable without relying on WebView input events that are currently not delivered on this tester machine.
