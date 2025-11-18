import { invoke } from '@tauri-apps/api/tauri';
import type {
  CommandResult,
  Project,
  Document,
  ChunkMatch,
  RagChatResponse,
} from './types';

export interface AddDocumentRequest {
  project_id: number;
  name: string;
  content: string;
  provider_id: string;
}

export interface AddDocumentResponse {
  document_id: number;
  chunks_created: number;
}

export interface RagSearchRequest {
  project_id: number;
  query: string;
  provider_id: string;
  top_k: number;
}

export interface RagChatRequest {
  project_id: number;
  query: string;
  provider_id: string;
  model: string;
  top_k: number;
  temperature?: number;
  max_tokens?: number;
}

export async function createProject(name: string): Promise<Project> {
  const result = await invoke<CommandResult<Project>>('create_project', { name });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to create project');
  }
  return result.data;
}

export async function listProjects(): Promise<Project[]> {
  const result = await invoke<CommandResult<Project[]>>('list_projects');
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to list projects');
  }
  return result.data;
}

export async function deleteProject(projectId: number): Promise<void> {
  const result = await invoke<CommandResult<void>>('delete_project', {
    projectId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to delete project');
  }
}

export async function listDocuments(projectId: number): Promise<Document[]> {
  const result = await invoke<CommandResult<Document[]>>('list_documents', {
    projectId,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to list documents');
  }
  return result.data;
}

export async function deleteDocument(documentId: number): Promise<void> {
  const result = await invoke<CommandResult<void>>('delete_document', {
    documentId,
  });
  if (!result.success) {
    throw new Error(result.error || 'Failed to delete document');
  }
}

export async function addDocument(
  request: AddDocumentRequest
): Promise<AddDocumentResponse> {
  const result = await invoke<CommandResult<AddDocumentResponse>>('add_document', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to add document');
  }
  return result.data;
}

export async function ragSearch(request: RagSearchRequest): Promise<ChunkMatch[]> {
  const result = await invoke<CommandResult<ChunkMatch[]>>('rag_search', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to search');
  }
  return result.data;
}

export async function ragChat(request: RagChatRequest): Promise<RagChatResponse> {
  const result = await invoke<CommandResult<RagChatResponse>>('rag_chat', {
    request,
  });
  if (!result.success || !result.data) {
    throw new Error(result.error || 'Failed to RAG chat');
  }
  return result.data;
}
