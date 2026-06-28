# Publication Name rerun report for 5bdd7a8

Status: **FAIL - setup gate blocked**

Directive: `test-comms/directives/20260628-rerun-publication-name-after-5bdd7a8.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `dada36379864ffa889a959228768024b03434a6a`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `5bdd7a8dd51852a59050bd95b91aa903b74cb59e` (`Fix first-run publication name typing`)

## Result

The rerun stopped at required setup retest step 8.

The Civic Desk launched at 1280x720 and the Publication Name field showed visible initial focus. However, `ABC` did not appear in the Publication Name field after normal keyboard entry attempts. I tried:

1. Typing `ABC` with the field visibly focused immediately after launch.
2. Clicking inside Publication Name, confirming the text caret appeared, then typing `ABC`.
3. Clicking inside Publication Name again and injecting `A`, `B`, `C` as virtual key events.

In all three cases, the field remained empty. Because the directive says to stop and report the exact failure if any setup/model gate fails, I did not continue to identity completion, runtime setup, Ollama startup, model download, Longmont E2E, publish, or soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `1619FBA57C1300DCFB8673C3BFE44BF81F39FC6043B9ACC602AD79A3825B958E` | `1619FBA57C1300DCFB8673C3BFE44BF81F39FC6043B9ACC602AD79A3825B958E` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `58DE67CBE45517E1757F9806A7C9ACBE0ED4500B7AA6C84ACFB076F25DC99995` | `58DE67CBE45517E1757F9806A7C9ACBE0ED4500B7AA6C84ACFB076F25DC99995` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-publication-name-rerun-5bdd7a8\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
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

- `test-comms/artifacts/20260628-publication-name-rerun-5bdd7a8/01-first-run-1280.png` - 1280x720 first-run screen; Publication Name has visible focus.
- `test-comms/artifacts/20260628-publication-name-rerun-5bdd7a8/02-publication-abc.png` - after typing `ABC` while initial focus was visible; field still empty.
- `test-comms/artifacts/20260628-publication-name-rerun-5bdd7a8/03-publication-abc-after-click.png` - after clicking in the field and typing `ABC`; caret visible, field still empty.
- `test-comms/artifacts/20260628-publication-name-rerun-5bdd7a8/04-publication-abc-keybd-event.png` - after clicking in the field and sending virtual key events for `ABC`; field still empty.

## Exact breakpoint

The product still presents the same release-blocking behavior as the previous cleanroom run: Publication Name visually receives focus/caret but does not accept text entry. The next coder fix should start at first-run Publication Name input handling before any further runtime/model/E2E testing can proceed.
