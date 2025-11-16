import { invoke } from '@tauri-apps/api/tauri';
import type { CommandResult, MaskedProviderConfig } from './types';

export interface UpdateProviderRequest {
  provider_id: string;
  api_key?: string;
  base_url?: string;
  default_model?: string;
  enabled?: boolean;
}

export async function getProviders(): Promise<MaskedProviderConfig[]> {
  const result = await invoke<CommandResult<MaskedProviderConfig[]>>('get_providers');
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to get providers');
  }
  return result.data;
}

export async function updateProvider(request: UpdateProviderRequest): Promise<void> {
  const result = await invoke<CommandResult<void>>('update_provider', { request });
  if (!result.success) {
    throw new Error(result.error || 'Failed to update provider');
  }
}

export async function deleteProvider(providerId: string): Promise<void> {
  const result = await invoke<CommandResult<void>>('delete_provider', {
    providerId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to delete provider');
  }
}

export async function testProviderConnection(providerId: string): Promise<string> {
  const result = await invoke<CommandResult<string>>('test_provider_connection', {
    providerId,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to test connection');
  }
  return result.data;
}
