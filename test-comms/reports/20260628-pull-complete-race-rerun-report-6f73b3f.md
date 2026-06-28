# Pull-complete race rerun report for 6f73b3f

Status: **FAIL - setup completes, Longmont E2E blocked by main-app input/navigation**

Directive: `test-comms/directives/20260628-rerun-pull-complete-race-after-6f73b3f.md`  
Coordination branch: `test-comms/cleanroom-coder-tester`  
Coordination HEAD tested: `16712482d86cbf7a6302b7cae1f1ad13044023fb`  
Product branch: `stable-readiness-local-gates`  
Product commit tested: `6f73b3fb43e58d18ee3fef663ebaaee49f97a758` (`Finish setup after recovered pull completion`)

## Result

The pull-complete recovery fix worked. The packaged app launched at 1280x720, recovered past stuck identity input, started its own Ollama runtime, auto-started the recommended `qwen2.5:7b` model pull, detected the model in `/api/tags`, completed onboarding, and entered the usable main application without manual clicks.

The run stopped at the next gate: the main app still did not respond to normal click or keyboard navigation. I could not navigate from `Story Queue` to `Daily Scan`, so I could not add/discover Longmont sources, run intake/discovery, generate leads/stories, export, publish, or begin soak.

## Hash verification

Both installer artifacts matched the directive before install.

| Artifact | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `C9A584326FD3E3A8825A9BB7275F6F90A218FE3AF7C9C7F3C246904E0D1F5CC1` | `C9A584326FD3E3A8825A9BB7275F6F90A218FE3AF7C9C7F3C246904E0D1F5CC1` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `39C8C7FE35614AD71FBD63D40C4474D4725B033850A5D1AE2635FF563AECBE8E` | `39C8C7FE35614AD71FBD63D40C4474D4725B033850A5D1AE2635FF563AECBE8E` | PASS |

## Clean reset and install

Performed a clean reset within the directive boundary: stopped CivicNewspaper/Ollama-related processes and removed CivicNewspaper app data, Ollama data, local models, and prior local app install folders. Windows, the user account, browser, Git, and Codex tester environment were left intact.

Installed as an end user using:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-pull-complete-race-rerun-6f73b3f\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
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
| Complete first-run setup only through app UI/recovery paths | PASS |
| Identity stuck-input recovery advances to Step 2 | PASS |
| App starts its own AI runtime | PASS |
| Auto-start model pull | PASS |
| `/api/tags` lists `qwen2.5:7b` | PASS |
| Final setup recovery enters main application | PASS |
| Navigate main app to Daily Scan / source workflow | FAIL |

## Longmont E2E / publish / soak

Not reached because main-app navigation/input remained blocked after setup completed.

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

## Main-app input attempts

After the app entered the main `Story Queue` screen, I tried:

1. Normal click on `Daily Scan` in the left navigation.
2. A DPI-adjusted click attempt intended to hit `Daily Scan`.
3. Keyboard navigation with `Tab`, `Tab`, `Enter`.

The screen remained on `Story Queue` after all attempts.

## Artifacts

Screenshots are clean app-window captures with no helper windows visible:

- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/01-after-launch-recovery-state.png` - after launch; identity recovery advanced to Step 2.
- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/02-recovery-monitor-85s.png` - main app visible after setup recovery.
- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/03-recovery-monitor-240s.png` - main app still visible after extended monitor.
- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/04-main-app-click-daily-scan.png` - after normal Daily Scan click; still on Story Queue.
- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/05-main-app-scaled-click-daily-scan.png` - after adjusted click attempt; still on Story Queue.
- `test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/06-main-app-keyboard-navigation-attempt.png` - after keyboard navigation attempt; still on Story Queue.

## Exact breakpoint

`6f73b3f` fixes setup recovery enough to enter the main app. The next release blocker is that the packaged Windows main app still does not receive usable click/key input, preventing navigation into Daily Scan/source intake and blocking the Longmont publication workflow.
