# Native starter rerun report for f4147af

Status: **FAIL - setup gate blocked**

Directive: `test-comms/directives/20260628-rerun-native-starter-after-f4147af.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `e9126055b8e6584409abc61f48eedec0df4d6053`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `f4147af6f43ddf01d6cad685e0325845325d08d2` (`Add native starter profile navigation fallback`)

## Result

The rerun stopped at required setup retest step 9/10.

The Civic Desk launched at 1280x720. The starter profiles were now visibly rendered as underlined native-style links, and the Publication Name field showed visible initial focus with a caret. Keyboard typing still failed: `ABC` did not appear in Publication Name.

Per directive, I continued and tried to activate the visible `Longmont` native starter profile link. The app did not advance to Step 2 and did not visibly fill Longmont identity. I tried:

1. Clicking the visible `Longmont` native link.
2. Trying keyboard focus traversal/activation from the focused field (`Shift+Tab` sequence, then `Enter`).

In both cases, the app remained on Step 1 and the identity fields remained empty. Because the directive says to stop if the native starter link cannot advance to Step 2, I did not continue to runtime setup, Ollama startup, model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `504D62730E3C3332F52185CF3D728FC460FA8ED5B55A07B8EBF8299CFA1447CF` | `504D62730E3C3332F52185CF3D728FC460FA8ED5B55A07B8EBF8299CFA1447CF` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `6F76F944E975C4358CA3CE4B908C7941DCEE1B635C6219D8EE07DBDF610F105B` | `6F76F944E975C4358CA3CE4B908C7941DCEE1B635C6219D8EE07DBDF610F105B` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-native-starter-rerun-f4147af\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
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
| Click visible `Longmont` starter profile link | FAIL - did not advance |
| Confirm app applies Longmont identity and enters Step 2 | FAIL |
| First-run action row visible/reachable while body scrolls | NOT REACHED |
| No helper/terminal/console during runtime setup | NOT REACHED |
| App-managed runtime setup starts Ollama | NOT REACHED |
| `GET http://127.0.0.1:11434/api/tags` reachable | NOT REACHED |
| Body `Download qwen2.5:7b` starts pull | NOT REACHED |
| Footer `Next` starts pull from clean no-model state | NOT REACHED |
| Model download completes | NOT REACHED |
| `/api/tags` lists downloaded model | NOT REACHED |

## Longmont E2E / publish / soak

Not reached because the native Longmont starter link did not advance setup.

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

- `test-comms/artifacts/20260628-native-starter-rerun-f4147af/01-first-run-1280.png` - 1280x720 first-run screen with native-style starter links visible and Publication Name focused.
- `test-comms/artifacts/20260628-native-starter-rerun-f4147af/02-publication-abc-attempt.png` - after typing `ABC`; field still empty.
- `test-comms/artifacts/20260628-native-starter-rerun-f4147af/03-longmont-native-link-click.png` - after clicking the visible Longmont native link; app remained on Step 1.
- `test-comms/artifacts/20260628-native-starter-rerun-f4147af/04-longmont-native-link-keyboard-fallback.png` - after keyboard focus/Enter fallback; app remained on Step 1.

## Exact breakpoint

The product still cannot progress through first-run identity setup on this cleanroom machine. Keyboard entry into Publication Name still fails, and the native Longmont starter link did not advance to Step 2 or fill identity values in the packaged Windows app.
