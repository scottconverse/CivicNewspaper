import '@testing-library/jest-dom';
import { vi } from 'vitest';

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
vi.mock('@tauri-apps/plugin-updater', () => ({
  check: vi.fn(),
}));
vi.mock('@tauri-apps/plugin-opener', () => ({
  open: vi.fn(),
}));
