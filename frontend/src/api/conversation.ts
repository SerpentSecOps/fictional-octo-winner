import { invoke } from '@tauri-apps/api/tauri';
import type {
  CommandResult,
  Conversation,
  Message,
  ConversationWithMessages,
} from './types';

export interface CreateConversationRequest {
  title: string;
  provider_id: string;
  model: string;
}

export interface AddMessageRequest {
  conversation_id: number;
  role: string;
  content: string;
}

export async function createConversation(
  request: CreateConversationRequest
): Promise<Conversation> {
  const result = await invoke<CommandResult<Conversation>>('create_conversation', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to create conversation');
  }
  return result.data;
}

export async function listConversations(): Promise<Conversation[]> {
  const result = await invoke<CommandResult<Conversation[]>>('list_conversations');
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to list conversations');
  }
  return result.data;
}

export async function getConversationWithMessages(
  conversationId: number
): Promise<ConversationWithMessages> {
  const result = await invoke<CommandResult<ConversationWithMessages>>(
    'get_conversation_with_messages',
    {
      conversationId,
    }
  );
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to get conversation');
  }
  return result.data;
}

export async function updateConversationTitle(
  conversationId: number,
  title: string
): Promise<void> {
  const result = await invoke<CommandResult<void>>('update_conversation_title', {
    conversationId,
    title,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to update conversation title');
  }
}

export async function deleteConversation(conversationId: number): Promise<void> {
  const result = await invoke<CommandResult<void>>('delete_conversation', {
    conversationId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to delete conversation');
  }
}

export async function addMessage(request: AddMessageRequest): Promise<Message> {
  const result = await invoke<CommandResult<Message>>('add_message', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to add message');
  }
  return result.data;
}

export async function getConversationMessages(
  conversationId: number
): Promise<Message[]> {
  const result = await invoke<CommandResult<Message[]>>(
    'get_conversation_messages',
    {
      conversationId,
    }
  );
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to get messages');
  }
  return result.data;
}

export async function deleteMessage(messageId: number): Promise<void> {
  const result = await invoke<CommandResult<void>>('delete_message', {
    messageId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to delete message');
  }
}
