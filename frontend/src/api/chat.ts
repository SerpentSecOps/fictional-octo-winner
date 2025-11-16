import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import type { CommandResult, ChatMessage, ChatResponse } from './types';

export interface SendChatRequest {
  provider_id: string;
  model: string;
  messages: ChatMessage[];
  temperature?: number;
  max_tokens?: number;
  top_p?: number;
  stream: boolean;
}

export interface ChatChunk {
  request_id: string;
  delta: string;
  finish_reason?: string;
}

export async function sendChatMessage(request: SendChatRequest): Promise<ChatResponse> {
  const result = await invoke<CommandResult<ChatResponse>>('send_chat_message', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to send chat message');
  }
  return result.data;
}

export async function sendChatMessageStream(
  request: SendChatRequest,
  requestId: string,
  onChunk: (chunk: ChatChunk) => void,
  onComplete: () => void
): Promise<void> {
  // Listen for chunks
  const unlisten1 = await listen<ChatChunk>('chat-chunk', (event) => {
    if (event.payload.request_id === requestId) {
      onChunk(event.payload);
    }
  });

  // Listen for completion
  const unlisten2 = await listen<string>('chat-complete', (event) => {
    if (event.payload === requestId) {
      onComplete();
      unlisten1();
      unlisten2();
    }
  });

  // Start streaming
  const result = await invoke<CommandResult<void>>('send_chat_message_stream', {
    request,
    requestId,
  });

  if (!result.success) {
    unlisten1();
    unlisten2();
    throw new Error(result.error || 'Failed to start streaming');
  }
}
