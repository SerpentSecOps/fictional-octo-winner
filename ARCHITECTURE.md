# Desktop LLM Workbench - Architecture

## Overview

A secure, native desktop application for working with multiple LLM providers (DeepSeek, Gemini, Claude) featuring chat, RAG, canvas workspace, and code editing capabilities.

## Technology Stack

### Backend (Rust via Tauri)
- **Tauri 1.x**: Desktop framework providing native window + webview
- **Security**:
  - `keyring`: OS-level keychain integration (macOS Keychain, Windows Credential Manager, Linux Secret Service)
  - `chacha20poly1305`: Authenticated encryption for API keys at rest
  - `rand`: Cryptographically secure random number generation
- **LLM Integration**:
  - `reqwest`: Async HTTP client for provider APIs
  - `tokio`: Async runtime
  - `serde` + `serde_json`: JSON serialization
- **RAG Storage**:
  - `sqlx`: Async SQLite with compile-time checked queries
  - Vector embeddings stored as BLOBs
- **Utilities**:
  - `async-trait`: Async trait support
  - `thiserror`: Error handling
  - `base64`: Encoding

### Frontend (React + TypeScript)
- **Build**: Vite for fast dev + optimized production builds
- **UI Framework**: React 18 with TypeScript
- **State Management**: Zustand (lightweight, no boilerplate)
- **Styling**: TailwindCSS
- **Key Libraries**:
  - `@tauri-apps/api`: IPC communication with Rust backend
  - `@monaco-editor/react`: VS Code-powered code editor
  - `reactflow`: Canvas/node-based workspace
  - `react-markdown`: Markdown rendering
  - `lucide-react`: Icons

## Project Structure

```
fictional-octo-winner/
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── security/             # Encryption + keychain
│   │   │   ├── mod.rs
│   │   │   ├── encryption.rs     # ChaCha20Poly1305 encryption
│   │   │   └── keychain.rs       # OS keychain integration
│   │   ├── config/               # Encrypted config storage
│   │   │   ├── mod.rs
│   │   │   └── store.rs
│   │   ├── llm_providers/        # LLM provider implementations
│   │   │   ├── mod.rs
│   │   │   ├── traits.rs         # Common trait
│   │   │   ├── deepseek.rs
│   │   │   ├── gemini.rs
│   │   │   └── claude.rs
│   │   ├── rag/                  # RAG subsystem
│   │   │   ├── mod.rs
│   │   │   ├── database.rs       # SQLite schema + queries
│   │   │   ├── embeddings.rs     # Embedding generation
│   │   │   ├── chunking.rs       # Text chunking
│   │   │   └── search.rs         # Vector similarity search
│   │   ├── linting/              # Code linting
│   │   │   ├── mod.rs
│   │   │   └── eslint.rs
│   │   └── commands/             # Tauri command handlers
│   │       ├── mod.rs
│   │       ├── config_commands.rs
│   │       ├── chat_commands.rs
│   │       ├── rag_commands.rs
│   │       ├── canvas_commands.rs
│   │       └── lint_commands.rs
│   └── icons/
├── frontend/                      # React app
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── index.html
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── api/                   # Tauri command wrappers
│   │   │   ├── config.ts
│   │   │   ├── chat.ts
│   │   │   ├── rag.ts
│   │   │   ├── canvas.ts
│   │   │   └── lint.ts
│   │   ├── store/                 # Zustand state
│   │   │   ├── appStore.ts
│   │   │   ├── chatStore.ts
│   │   │   └── canvasStore.ts
│   │   ├── views/                 # Main views/pages
│   │   │   ├── Chat.tsx
│   │   │   ├── Canvas.tsx
│   │   │   ├── CodeLab.tsx
│   │   │   └── Settings.tsx
│   │   ├── components/            # Reusable components
│   │   │   ├── Sidebar.tsx
│   │   │   ├── ChatMessage.tsx
│   │   │   ├── ProviderConfig.tsx
│   │   │   └── ...
│   │   └── styles/
│   │       └── globals.css
├── package.json                   # Workspace root
└── README.md
```

## Security Model

### API Key Protection

1. **Master Key Generation**:
   - On first run, generate a random 256-bit master key
   - Store in OS keychain using `keyring` crate
   - Service name: `llm_workbench_master_key`
   - Account: `master` or machine identifier

2. **Config Encryption**:
   - Provider configs (including API keys) serialized to JSON
   - Encrypted using ChaCha20Poly1305 with random nonce
   - Format: `[12-byte nonce][ciphertext][16-byte tag]`
   - Stored as base64 in `~/.config/llm-workbench/config.enc`

3. **Runtime Handling**:
   - API keys decrypted only in Rust backend, never sent to frontend
   - Used directly in HTTP requests, then dropped
   - Frontend only sees masked indicators ("API key configured")

### Threat Model

- **Protects against**: Casual disk inspection, config file theft
- **Doesn't protect against**: Memory dumps of running process, OS-level keyloggers
- **Assumptions**: OS keychain is trusted and secure

## Data Flow

### Chat Flow
```
User types message in Chat UI
  → Frontend calls tauri.invoke('send_chat_message', {...})
    → Rust: Retrieves encrypted API key
    → Rust: Decrypts key in memory
    → Rust: Calls LLM provider HTTP API
    → Rust: Streams response chunks via tauri.event.emit('chat-chunk')
      → Frontend: Listens to 'chat-chunk' events
      → Frontend: Updates UI with streamed text
```

### RAG Flow
```
User adds document to project
  → Frontend calls tauri.invoke('add_document', {project_id, file_path})
    → Rust: Reads file from disk
    → Rust: Chunks text (e.g., 512 token chunks, 50 token overlap)
    → Rust: Generates embeddings via LLM provider
    → Rust: Stores chunks + embeddings in SQLite

User sends RAG-enabled chat message
  → Frontend calls tauri.invoke('rag_chat', {project_id, query, ...})
    → Rust: Generates query embedding
    → Rust: Searches similar chunks (cosine similarity)
    → Rust: Constructs prompt with top-k context chunks
    → Rust: Calls LLM with augmented prompt
    → Rust: Returns response + source chunks
      → Frontend: Displays response + "Sources used" panel
```

### Canvas Persistence
```
User drags nodes, edits content in Canvas
  → On change: Frontend debounces and calls tauri.invoke('save_canvas_state', {...})
    → Rust: Serializes canvas state to JSON
    → Rust: Stores in SQLite (projects table has canvas_state column)

User opens Canvas
  → Frontend calls tauri.invoke('load_canvas_state', {project_id})
    → Rust: Queries SQLite
    → Rust: Returns canvas state JSON
      → Frontend: Reactflow renders nodes/edges
```

## LLM Provider Abstraction

### Common Trait (Rust)

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    async fn stream_chat(
        &self,
        request: ChatRequest,
        tx: tokio::sync::mpsc::Sender<ChatChunk>,
    ) -> Result<()>;

    async fn embed(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>>;
}
```

### Provider-Specific Implementations

- **DeepSeek**: `https://api.deepseek.com/v1/chat/completions`
- **Gemini**: `https://generativelanguage.googleapis.com/v1/models/{model}:generateContent`
- **Claude**: `https://api.anthropic.com/v1/messages`

Each provider:
- Handles its own auth header format
- Transforms to/from common `ChatRequest`/`ChatResponse` types
- Implements streaming via SSE (DeepSeek, Claude) or HTTP/2 streaming (Gemini)

## RAG Implementation

### Text Chunking Strategy
- **Method**: Sliding window with overlap
- **Default size**: 512 tokens (~2048 chars)
- **Overlap**: 50 tokens (~200 chars)
- **Rationale**: Ensures context continuity across chunks

### Embedding Storage
- **SQLite schema**:
  ```sql
  CREATE TABLE chunks (
      id INTEGER PRIMARY KEY,
      document_id INTEGER NOT NULL,
      project_id INTEGER NOT NULL,
      content TEXT NOT NULL,
      embedding BLOB NOT NULL,  -- Serialized Vec<f32>
      chunk_index INTEGER NOT NULL,
      FOREIGN KEY (document_id) REFERENCES documents(id),
      FOREIGN KEY (project_id) REFERENCES projects(id)
  );
  ```

### Similarity Search
- **Method**: Cosine similarity
- **Implementation**: In-memory (load all project embeddings, compute similarity)
- **TODO**: For large projects (>10k chunks), use HNSW or FAISS via Rust bindings

## Development Workflow

### Development Mode
```bash
cd fictional-octo-winner
pnpm install          # Install all deps (frontend + Tauri CLI)
pnpm tauri dev        # Launches dev window with hot reload
```

### Production Build
```bash
pnpm tauri build      # Creates native installer in src-tauri/target/release/bundle/
```

### Debugging
- **Frontend**: Open DevTools via Tauri menu or `Ctrl+Shift+I` (enabled in dev builds)
- **Backend**: Rust logs via `env_logger`, output to terminal
- **IPC**: Use `tauri dev --verbose` to see command invocations

## TODOs / Future Enhancements

- [ ] **Advanced RAG**: Implement HNSW index for large document sets
- [ ] **Embedding providers**: Support dedicated embedding APIs (OpenAI, Cohere)
- [ ] **Code execution**: Sandboxed code execution for code generation workflows
- [ ] **Multi-modal**: Image input support for Gemini/Claude vision models
- [ ] **Prompt library**: Save/share prompt templates
- [ ] **Export**: Export conversations as Markdown/PDF
- [ ] **Themes**: Dark/light mode toggle
- [ ] **Keyboard shortcuts**: Global shortcuts for common actions
- [ ] **Conversation search**: Full-text search across all chats
- [ ] **Token counting**: Accurate token counting per provider
- [ ] **Cost tracking**: Track API usage and estimated costs
