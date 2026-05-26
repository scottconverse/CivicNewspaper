# Phase R - Research

## Inventory of `call_local_ollama` Call Sites
Based on `grep` of the repository, the `call_local_ollama` function is directly invoked in the following places:

1. **`src-tauri/src/core/daily_scan.rs`**
   - Inside `run_daily_scan` (Line 81)
   - `let llm_res = llm::call_local_ollama("gemma2:9b", &final_prompt, &prompt_template).await;`
2. **`src-tauri/src/core/server.rs`**
   - Inside `handle_api_request` or similar (Line 224)
   - `match call_local_ollama(model, &payload.prompt, &payload.system).await {`
3. **`src-tauri/src/tauri_cmds.rs`**
   - Inside `plain_language_rewrite` (Line 313)
   - `llm::call_local_ollama(&model, &prompt, &sys).await.map_err(|e| e.to_string())`
   - Inside `llm_task` (Line 348)
   - `llm::call_local_ollama(&model, &prompt, &system).await.map_err(|e| e.to_string())`

To satisfy D1, the Tauri application must hold an `Arc<dyn LlmClient>` in its managed state. These functions will need to be updated to accept `tauri::State<'_, Arc<dyn LlmClient>>` (or extract it from `AppHandle` in the case of non-command functions like `server.rs` where the state might need to be passed down or retrieved differently).

## Inventory of `CommunityProfile` / `get_setting` Flow
To address P4-005 ("handleDailyScan hardcodes Brighton, CO"), we examined the settings flow.

1. **Backend Types and Commands:**
   - `CommunityProfile` is defined in `src-tauri/src/tauri_cmds.rs` (with `city` and `state` fields).
   - `get_community_profile` and `save_community_profile` are exposed as Tauri commands.
2. **Frontend State (`src/useApp.ts`):**
   - The React state hook `const [communityProfile, setCommunityProfile] = useState<CommunityProfile | null>(null);` caches the profile.
   - It is fetched on load (`await getCommunityProfile()`) and populated into the state.
3. **Daily Scan Usage:**
   - In `useApp.ts` `handleDailyScan`, it currently passes `"Brighton", "CO"` directly to `runDailyScan`.
   - **Fix:** We can utilize the existing `communityProfile` state in `useApp.ts` before calling `runDailyScan`.
   - `if (!communityProfile || !communityProfile.city || !communityProfile.state) { /* alert user or show error */ } else { await runDailyScan(communityProfile.city, communityProfile.state, 24); }`

## D1 Feasibility
- We can define `pub trait LlmClient: Send + Sync` in `llm.rs`.
- We can define `pub struct OllamaClient;` implementing it.
- `call_local_ollama` will be moved/wrapped inside `OllamaClient::call`.
- The `tauri::Builder` in `main.rs` (or `setup` function) needs to `manage(Arc::new(OllamaClient {}) as Arc<dyn LlmClient>)`.
- `daily_scan.rs` and `tauri_cmds.rs` can inject `llm_client: tauri::State<'_, Arc<dyn LlmClient>>`.
- `server.rs` handler context can extract the state.
