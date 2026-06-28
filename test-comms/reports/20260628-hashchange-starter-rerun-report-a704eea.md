# Hashchange starter rerun report for a704eea

Status: **FAIL - setup gate blocked**

Directive: `test-comms/directives/20260628-rerun-hashchange-starter-after-a704eea.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `7ec529afc1899a0cd8af1bd01c2070e62a0df8fd`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `a704eea7036caecea36da67b8f02e9aeb7db7265` (`Handle starter profile hash navigation`)

## Result

The rerun stopped at required setup retest step 9/10.

The Civic Desk launched at 1280x720. The starter profiles were visibly rendered as underlined native-style links, and the Publication Name field showed visible initial focus with a caret. Keyboard typing still failed: `ABC` did not appear in Publication Name.

Per directive, I continued and tried to activate the visible `Longmont` hash starter link. The app did not advance to Step 2 and did not visibly fill Longmont identity. I tried:

1. Clicking the visible `Longmont` hash starter link.
2. Retrying the click with DPI-adjusted coordinates that landed on the visible link position.
3. Trying keyboard focus traversal/activation from the focused field (`Shift+Tab` sequence, then `Enter`).

In all cases, the app remained on Step 1 and the identity fields remained empty. Because the directive says to stop if the Longmont starter link cannot advance to Step 2, I did not continue to runtime setup, Ollama startup, model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `D509873B95B8E72F6851C2524B04AB69C5A8042AA0E97B07BA2B4A9F69249C5A` | `D509873B95B8E72F6851C2524B04AB69C5A8042AA0E97B07BA2B4A9F69249C5A` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `CC4D177A0F2B895C8264AB05603B55FDA712017D364CFB0EB2290CE3A1768CDD` | `CC4D177A0F2B895C8264AB05603B55FDA712017D364CFB0EB2290CE3A1768CDD` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-hashchange-starter-rerun-a704eea\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
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

Not reached because the Longmont hash starter link did not advance setup.

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

- `test-comms/artifacts/20260628-hashchange-starter-rerun-a704eea/01-first-run-1280.png` - 1280x720 first-run screen with hash starter links visible and Publication Name focused.
- `test-comms/artifacts/20260628-hashchange-starter-rerun-a704eea/02-publication-abc-attempt.png` - after typing `ABC`; field still empty.
- `test-comms/artifacts/20260628-hashchange-starter-rerun-a704eea/03-longmont-hash-link-click.png` - after clicking the visible Longmont hash link; app remained on Step 1.
- `test-comms/artifacts/20260628-hashchange-starter-rerun-a704eea/04-longmont-hash-link-scaled-click.png` - after DPI-adjusted click landing on the visible Longmont link; app remained on Step 1.
- `test-comms/artifacts/20260628-hashchange-starter-rerun-a704eea/05-longmont-hash-link-keyboard-fallback.png` - after keyboard focus/Enter fallback; app remained on Step 1.

## Exact breakpoint

The product still cannot progress through first-run identity setup on this cleanroom machine. Keyboard entry into Publication Name still fails, and the Longmont hash starter link did not advance to Step 2 or fill identity values in the packaged Windows app.
