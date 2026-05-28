# Stage 07 Onboarding Model Pull Wizard Fixes Report

The onboarding wizard model-pull step and associated deep-linking have been implemented and verified.

## Changes Made
- **tauri_cmds.rs**: Added `pull_ollama_model(model_id: String)` Tauri command. This command makes a POST request to `/api/pull` on the local Ollama sidecar, streams the response chunks, parses progress status/percentages, and emits event `"ollama-pull-progress"` with `{model, status, completed, total}` payload. Registered it in `lib.rs`.
- **ipc.ts**: Added TypeScript wrappers `ollamaHealth` and `pullOllamaModel` to handle communication with the backend.
- **OnboardingWizard.tsx**: 
  - Updated the "Pull Model" step to "Download AI Model" pointing specifically to `gemma2:9b`.
  - Bound the progress bar to the `"ollama-pull-progress"` event stream.
  - Added a clear warning about skipping: *"Warning: You can skip this download, but you will be unable to run a Daily Scan until the model is downloaded later."*
  - Supported `initialStep` prop for deep-linking.
- **useApp.ts**:
  - Implemented `onboardingStep` state.
  - Exposed `onboardingStep` and `setOnboardingStep` in the `useApp` return hook value.
  - Updated `handleDailyScan` to query `ollamaHealth()` and verify if `gemma2:9b` is present. If missing, it alerts the user, sets `onboardingStep = 3`, and deep-links to `"onboarding"`.
- **AppContent.tsx**: Passed `initialStep={app.onboardingStep}` to `OnboardingWizard`.
- **OnboardingWizard.test.tsx**: Added a new vitest case `"Model Pull: streams progress and completes"` to mock progress event emissions and assert that the progress bar updates from 0% to 100% and displays success.

## Verification
- Frontend vitest tests passed: 20 passed.
- Backend cargo tests passed.
