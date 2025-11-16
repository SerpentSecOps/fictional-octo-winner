# TODO List for LLM Workbench

## 🚨 Critical (Must Do Before First Run)

- [x] **Create Application Icons** ✅
  - Script created: `create-icon.sh`
  - Generates SVG icon and converts to all required formats
  - User needs to install ImageMagick and run script

- [ ] **Verify Compilation**
  ```bash
  cd fictional-octo-winner
  pnpm install
  cd frontend && pnpm install && cd ..
  pnpm tauri dev  # Test if it compiles and runs
  ```

- [ ] **Add File System Permissions to Tauri Config**
  - Currently limited to APPDATA - need to allow document uploads
  - Update `src-tauri/tauri.conf.json` allowlist

- [ ] **Test End-to-End Flow**
  - [ ] Configure at least one provider (Claude or Gemini with real API key)
  - [ ] Send a chat message
  - [ ] Create a RAG project
  - [ ] Test canvas save/load

## 🔧 High Priority (Core Functionality)

- [x] **Document Upload UI** ✅
  - File picker dialog implemented in RAG view
  - Supports .txt and .md formats
  - Shows upload progress with spinner
  - **Location**: `frontend/src/views/RAG.tsx`

- [x] **Conversation Persistence** ✅
  - SQLite tables for conversations and messages
  - Auto-save all messages to DB
  - Conversation list sidebar with history
  - Load/delete/rename functionality
  - **Location**: `frontend/src/views/ChatV2.tsx`

- [x] **Better Error Handling** ✅
  - Toast notifications implemented (react-hot-toast)
  - Replaced all alert() calls
  - User-friendly error messages
  - **Location**: `frontend/src/utils/toast.ts`

- [x] **Loading States** ✅
  - Spinners for all long operations
  - Disabled buttons during operations
  - Progress indicators in Canvas, RAG, Settings
  - Initial app loading screen

- [ ] **Add Document Management UI**
  - View list of documents in project
  - Delete documents
  - Re-index documents
  - Show chunk count per document

## ⚡ Medium Priority (Polish & UX)

- [ ] **Model Presets**
  - Dropdown with common models per provider:
    - Claude: claude-3-5-sonnet-20241022, claude-3-opus-20240229
    - DeepSeek: deepseek-chat, deepseek-coder
    - Gemini: gemini-1.5-pro, gemini-1.5-flash

- [ ] **Token Counter**
  - Estimate tokens before sending (use tiktoken or similar)
  - Show token usage after response
  - Track costs per provider

- [ ] **Dark Mode**
  - Add theme toggle
  - Persist preference
  - Update TailwindCSS classes

- [ ] **Keyboard Shortcuts**
  - Cmd/Ctrl+K: Focus chat input
  - Cmd/Ctrl+,: Open settings
  - Cmd/Ctrl+N: New conversation
  - Cmd/Ctrl+Shift+P: Toggle RAG

- [ ] **Export Conversations**
  - Export as Markdown
  - Export as PDF
  - Export as JSON

- [ ] **Conversation Search**
  - Full-text search across all chats
  - Filter by provider, date, project

## 🎨 Low Priority (Nice to Have)

- [ ] **ESLint Integration**
  - Implement `lint_code` Tauri command
  - Run ESLint via Node.js subprocess
  - Parse and display diagnostics in Code Lab

- [ ] **Advanced RAG Features**
  - HNSW index for better performance with large datasets
  - Hybrid search (keyword + semantic)
  - Re-ranking of results
  - Chunk preview on hover

- [ ] **Multi-Modal Support**
  - Image upload for Claude/Gemini vision models
  - Image in chat bubbles
  - Screenshot capture tool

- [ ] **Prompt Library**
  - Save frequently used prompts
  - Template variables
  - Share prompts across conversations

- [ ] **Canvas Enhancements**
  - More node types (code, image, table)
  - Node grouping
  - Export canvas as image
  - Collaborative editing (WebRTC)

- [ ] **Code Execution**
  - Sandboxed code execution for generated code
  - Output display in Code Lab
  - Security warnings

## 🧪 Testing & Quality

- [ ] **Unit Tests**
  - Rust: Test encryption, config, RAG modules
  - Frontend: Test API wrappers, state management

- [ ] **Integration Tests**
  - Test full chat flow (mock LLM responses)
  - Test RAG workflow
  - Test canvas persistence

- [ ] **Error Handling Tests**
  - Invalid API keys
  - Network failures
  - Malformed responses

- [ ] **Security Audit**
  - Review key storage
  - Check for injection vulnerabilities
  - Validate user inputs

## 📚 Documentation

- [ ] **API Key Acquisition Guide**
  - Step-by-step for each provider
  - Screenshots of console/dashboard

- [ ] **Video Tutorial**
  - Quick start guide (5 min)
  - Full walkthrough (15 min)

- [ ] **Troubleshooting Guide**
  - Common errors and solutions
  - Platform-specific issues

- [ ] **Architecture Diagram**
  - Visual representation of data flow
  - Component interaction diagram

## 🚀 Deployment & Distribution

- [ ] **CI/CD Pipeline**
  - GitHub Actions for builds
  - Automated testing
  - Release artifacts

- [ ] **Code Signing**
  - macOS: Apple Developer certificate
  - Windows: Code signing certificate

- [ ] **Auto-Update**
  - Tauri updater integration
  - Release notifications

- [ ] **Analytics (Optional)**
  - Telemetry for crash reports
  - Usage analytics (opt-in)

## 🐛 Known Issues to Fix

1. **CORS/CSP**: Currently set to `null` - should be configured properly
2. **Type Safety**: Some `any` types in TypeScript should be properly typed
3. **Gemini Streaming**: Simplified implementation - needs proper SSE parsing
4. **Error Messages**: Using `alert()` - should use toast notifications
5. **No Input Validation**: Need to validate user inputs (project names, etc.)
6. **RAG Document Limit**: No pagination for large document sets
7. **Memory Leaks**: Event listeners in streaming chat may not clean up properly
8. **Hardcoded Values**: Some magic numbers should be configurable

## 📝 Code Quality Improvements

- [ ] Add Rust linting rules (clippy)
- [ ] Add frontend ESLint config
- [ ] Add Prettier for code formatting
- [ ] Add pre-commit hooks (husky)
- [ ] Add commit message linting (commitlint)
- [ ] Document all public APIs with JSDoc/rustdoc
- [ ] Add logging levels (debug, info, warn, error)

---

## Current Status

**Last Updated**: 2025-11-16

**Completion**: ~75% (core features implemented, conversation persistence done, UX polish pending)

**Recently Completed**:
- ✅ Conversation Persistence with full UI
- ✅ Toast notifications for errors
- ✅ Comprehensive loading states
- ✅ Icon generation script
- ✅ Document upload UI

**Next Steps**:
1. Add document delete functionality
2. Verify filesystem permissions for document uploads
3. Test end-to-end with real API keys
4. Add model presets dropdown
5. Implement dark mode toggle
