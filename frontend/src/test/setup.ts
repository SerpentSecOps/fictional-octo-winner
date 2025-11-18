import '@testing-library/jest-dom';
import { expect, afterEach, vi } from 'vitest';
import { cleanup } from '@testing-library/react';

// Cleanup after each test case
afterEach(() => {
  cleanup();
});

// Mock Tauri API
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
  emit: vi.fn(),
}));

vi.mock('@tauri-apps/api/dialog', () => ({
  open: vi.fn(),
  save: vi.fn(),
}));

vi.mock('@tauri-apps/api/fs', () => ({
  readTextFile: vi.fn(),
  writeTextFile: vi.fn(),
}));
