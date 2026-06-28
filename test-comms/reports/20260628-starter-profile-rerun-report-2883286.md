# Starter profile rerun report for 2883286

Status: **FAIL - setup gate blocked**

Directive: `test-comms/directives/20260628-rerun-starter-profile-after-2883286.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `6557afb710fe1add0205b67ba0fc9e6ed7d00971`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `2883286e599117bdf749b944818f0fb2d8bb2619` (`Add starter identity profiles for first-run setup`)

## Result

The rerun stopped at the starter profile setup gate.

The Civic Desk launched at 1280x720. The new starter profile buttons were visible, and the Publication Name field showed visible initial focus with a caret. Keyboard typing still failed: `ABC` did not appear in Publication Name.

Per directive, I continued and tried to use the `Longmont` starter profile. The Longmont profile did not fill Publication Name, Editor Name, City, or State. I tried:

1. Clicking the visible `Longmont` starter profile button.
2. Clicking the visible `Longmont` button again after refocusing the app.
3. Retrying with adjusted coordinates after observing WebView/screen scaling.
4. Trying a keyboard fallback from the focused field (`Shift+Tab`, then `Enter`).

In all cases, the identity fields remained empty. Because the directive says to stop if the starter profile cannot fill fields, I did not continue to Next, runtime setup, Ollama startup, model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `37219037C8CFC5399F19872BC9A1CE313452DBA54E6E2AB11F83CA8FCAA1FC2A` | `37219037C8CFC5399F19872BC9A1CE313452DBA54E6E2AB11F83CA8FCAA1FC2A` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `7F1BAB8868364A9D733583615FACC520DA0960DD90AE98068819839467A8D2D1` | `7F1BAB8868364A9D733583615FACC520DA0960DD90AE98068819839467A8D2D1` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-starter-profile-rerun-2883286\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
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
| Try typing `ABC` and record whether it appears | FAIL - still did not appear |
| Click `Longmont` starter profile button | FAIL - button did not fill fields |
| Confirm Publication Name, Editor Name, City, State visibly fill | FAIL |
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

Not reached because the starter profile setup gate failed.

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

- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/01-first-run-1280.png` - 1280x720 first-run screen with starter profile buttons visible and Publication Name focused.
- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/02-publication-abc-attempt.png` - after typing `ABC`; field still empty.
- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/03-longmont-profile-clicked.png` - after first visible Longmont click; fields still empty.
- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/04-longmont-profile-second-click.png` - after second visible Longmont click; fields still empty.
- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/05-longmont-profile-scaled-click.png` - after scaled-coordinate Longmont click; fields still empty.
- `test-comms/artifacts/20260628-starter-profile-rerun-2883286/06-longmont-profile-keyboard-fallback.png` - after keyboard fallback; fields still empty.

## Exact breakpoint

The product still cannot progress through first-run identity setup on this cleanroom machine. Keyboard entry into Publication Name still fails, and the new Longmont starter profile does not fill the identity fields. The next fix should verify that starter profile button activation works in the packaged Windows WebView, not only in browser/dev preview.
