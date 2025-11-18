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
): Promise<() => void> {
  let unlisten1: (() => void) | null = null;
  let unlisten2: (() => void) | null = null;

  const cleanup = () => {
    if (unlisten1) {
      unlisten1();
      unlisten1 = null;
    }
    if (unlisten2) {
      unlisten2();
      unlisten2 = null;
    }
  };

  try {
    // Listen for chunks
    unlisten1 = await listen<ChatChunk>('chat-chunk', (event) => {
      if (event.payload.request_id === requestId) {
        onChunk(event.payload);
      }
    });

    // Listen for completion
    unlisten2 = await listen<string>('chat-complete', (event) => {
      if (event.payload === requestId) {
        onComplete();
        cleanup();
      }
    });

    // Start streaming
    const result = await invoke<CommandResult<void>>('send_chat_message_stream', {
      request,
      requestId,
    });

    if (!result.success) {
      cleanup();
      throw new Error(result.error || 'Failed to start streaming');
    }

    // Return cleanup function for caller to use if component unmounts
    return cleanup;
  } catch (error) {
    cleanup();
    throw error;
  }
}
