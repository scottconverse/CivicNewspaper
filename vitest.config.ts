process.env.FORCE_COLOR = '0';
process.env.NO_COLOR = '1';

import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    globals: true,
    coverage: {
      provider: 'v8',
      reportsDirectory: 'coverage/frontend',
      reporter: ['text', 'json-summary', 'lcov'],
      thresholds: {
        statements: 60,
        branches: 65,
        functions: 55,
        lines: 60,
        "src/ipc.ts": { statements: 50, branches: 50, functions: 45, lines: 50 },
        "src/useApp.ts": { statements: 42, branches: 38, functions: 35, lines: 43 },
        "src/components/DailyScanPage.tsx": { statements: 60, branches: 75, functions: 95, lines: 60 },
        "src/components/LeadQueue.tsx": { statements: 74, branches: 70, functions: 70, lines: 73 },
        "src/components/OnboardingWizard.tsx": { statements: 78, branches: 72, functions: 75, lines: 78 },
        "src/components/PublishPanel.tsx": { statements: 67, branches: 80, functions: 50, lines: 66 },
        "src/components/Workbench.tsx": { statements: 78, branches: 77, functions: 72, lines: 78 },
      },
    },
  },
});
