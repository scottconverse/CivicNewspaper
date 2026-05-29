import '@testing-library/jest-dom';
import { vi } from 'vitest';

// ipc.ts's invokeGuarded() short-circuits when window.__TAURI_INTERNALS__ is
// absent. jsdom has no Tauri runtime, so without this the guard would throw
// before reaching the mocked invoke below. Make the test env "look like Tauri"
// so guarded IPC calls hit the mock instead of the unavailable path.
(window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {};

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
  transformCallback: vi.fn(),
}));
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => vi.fn()),
}));
vi.mock('@tauri-apps/api/path', () => ({
  documentDir: vi.fn(),
  appDataDir: vi.fn(),
  join: vi.fn(),
}));
vi.mock('@tauri-apps/api/app', () => ({
  getVersion: vi.fn(() => Promise.resolve('0.2.6')),
}));
vi.mock('@tauri-apps/plugin-opener', () => ({
  open: vi.fn(),
}));
