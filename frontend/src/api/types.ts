// Common types used across API calls

export interface CommandResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export interface ChatResponse {
  content: string;
  model: string;
  finish_reason?: string;
  usage?: {
    prompt_tokens: number;
    completion_tokens: number;
    total_tokens: number;
  };
}

export interface MaskedProviderConfig {
  provider_id: string;
  has_api_key: boolean;
  base_url?: string;
  default_model?: string;
  enabled: boolean;
}

export interface Project {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
  canvas_state?: string;
}

export interface Document {
  id: number;
  project_id: number;
  name: string;
  source_path?: string;
  created_at: string;
}

export interface Chunk {
  id: number;
  document_id: number;
  project_id: number;
  content: string;
  chunk_index: number;
}

export interface ChunkMatch {
  chunk: Chunk;
  similarity: number;
  document_name: string;
}

export interface RagChatResponse {
  response: string;
  sources: ChunkMatch[];
  model: string;
}

export interface CanvasState {
  nodes: CanvasNode[];
  edges: CanvasEdge[];
}

export interface CanvasNode {
  id: string;
  node_type: string;
  position: { x: number; y: number };
  data: any;
}

export interface CanvasEdge {
  id: string;
  source: string;
  target: string;
}
