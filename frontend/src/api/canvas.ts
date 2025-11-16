import { invoke } from '@tauri-apps/api/tauri';
import type { CommandResult, CanvasState } from './types';

export async function getCanvasState(
  projectId: number
): Promise<CanvasState | null> {
  const result = await invoke<CommandResult<CanvasState | null>>('get_canvas_state', {
    projectId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to get canvas state');
  }
  return result.data || null;
}

export async function saveCanvasState(
  projectId: number,
  state: CanvasState
): Promise<void> {
  const result = await invoke<CommandResult<void>>('save_canvas_state', {
    projectId,
    state,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to save canvas state');
  }
}
