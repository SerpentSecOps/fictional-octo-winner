# LLM Workbench - Desktop Application

A secure, native desktop application for working with multiple LLM providers (DeepSeek, Gemini, Claude) featuring chat, RAG, canvas workspace, and code editing capabilities.

## Features

- **🔐 Secure API Key Storage**: API keys are encrypted using ChaCha20Poly1305 and stored in OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **💬 Multi-Provider Chat**: Supports DeepSeek, Google Gemini, and Anthropic Claude with streaming responses
- **📚 RAG (Retrieval-Augmented Generation)**: Per-project document indexing, vector search, and context injection
- **🎨 Canvas Workspace**: Visual notebook for organizing conversations, notes, and code snippets
- **💻 Code Editor**: Monaco-powered editor with syntax highlighting (ESLint integration TODO)
- **⚡ Native Performance**: Built with Tauri (Rust backend, React frontend) for fast, lightweight execution

## Architecture

### Stack

- **Desktop Framework**: Tauri 1.5
- **Backend**: Rust
  - Encryption: ChaCha20Poly1305
  - Database: SQLite (via sqlx)
  - HTTP Client: reqwest
  - Keychain: keyring
- **Frontend**: React 18 + TypeScript + Vite
  - State: Zustand
  - Styling: TailwindCSS
  - Editor: Monaco Editor
  - Canvas: React Flow

### Security Model

1. **Master Key Generation**: On first run, a random 256-bit key is generated and stored in OS keychain
2. **Config Encryption**: Provider configs (including API keys) are serialized and encrypted with ChaCha20Poly1305
3. **Storage**: Encrypted config saved to `~/.config/llm-workbench/config.enc`
4. **Runtime**: API keys decrypted only in Rust backend, never exposed to frontend

## Prerequisites

### System Requirements

- **Operating System**: Windows, macOS, or Linux
- **Node.js**: >= 18.0.0
- **pnpm**: >= 8.0.0
- **Rust**: >= 1.70 (installed automatically by Tauri)

### macOS Additional Requirements

```bash
xcode-select --install
```

### Linux Additional Requirements

```bash
# Debian/Ubuntu
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsqlite3-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    sqlite-devel

# Arch
sudo pacman -Syu
sudo pacman -S webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    sqlite
```

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd fictional-octo-winner
```

### 2. Install Dependencies

```bash
# Install pnpm if not already installed
npm install -g pnpm

# Install all dependencies (frontend + Tauri CLI)
pnpm install

# Install frontend dependencies
cd frontend
pnpm install
cd ..
```

### 3. Development Build

```bash
# Run in development mode (hot reload enabled)
pnpm tauri dev
```

This will:
- Compile the Rust backend
- Start the Vite dev server
- Launch the desktop application with dev tools enabled

### 4. Production Build

```bash
# Build production bundle
pnpm tauri build
```

Output locations:
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## Usage Guide

### First Launch

1. **Configure Providers**: Go to Settings → Select a provider (Claude, DeepSeek, or Gemini)
2. **Enter API Key**: Paste your API key (it will be encrypted immediately)
3. **Set Base URL & Model**: Configure endpoint and default model
4. **Test Connection**: Click "Test Connection" to verify setup

### Chat View

1. Select provider and model from the header
2. Adjust temperature (0-2)
3. Type message and press Enter (Shift+Enter for new line)
4. Streaming responses appear in real-time
5. Toggle RAG mode to use document context (requires project selection)

### RAG Projects

1. **Create Project**: Canvas view → "New Project"
2. **Add Documents**: Use Chat view with RAG enabled
   ```typescript
   // In future UI: Upload .txt/.md files via file picker
   // For now: Programmatically via rag_commands
   ```
3. **Query with Context**: Enable RAG toggle in Chat, select project, and ask questions

### Canvas Workspace

1. Select or create a project
2. Add nodes with "+ Add Node"
3. Drag nodes to arrange
4. Connect nodes by dragging from edge
5. Save canvas with "Save" button

### Code Lab

1. Select language (JavaScript, TypeScript, Python, Rust, JSON)
2. Write code in Monaco editor
3. Syntax highlighting and IntelliSense enabled
4. **TODO**: Linting integration (requires ESLint Tauri command)

## Configuration Files

- **Config Location**: `~/.config/llm-workbench/` (macOS/Linux) or `%APPDATA%\llm-workbench\` (Windows)
- **config.enc**: Encrypted provider configurations
- **rag.db**: SQLite database for RAG projects/documents/chunks

## API Keys Required

Obtain API keys from:

- **Anthropic Claude**: https://console.anthropic.com/
- **DeepSeek**: https://platform.deepseek.com/
- **Google Gemini**: https://aistudio.google.com/app/apikey

## Development

### Project Structure

```
fictional-octo-winner/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── security/       # Encryption + keychain
│   │   ├── config/         # Config store
│   │   ├── llm_providers/  # LLM clients
│   │   ├── rag/            # RAG system
│   │   └── commands/       # Tauri IPC handlers
│   ├── Cargo.toml
│   └── tauri.conf.json
├── frontend/               # React app
│   ├── src/
│   │   ├── views/          # Main views
│   │   ├── components/     # Reusable components
│   │   ├── store/          # Zustand state
│   │   └── api/            # Tauri command wrappers
│   ├── package.json
│   └── vite.config.ts
└── package.json            # Workspace root
```

### Adding a New LLM Provider

1. Create `src-tauri/src/llm_providers/your_provider.rs`
2. Implement `LlmProvider` trait
3. Add to `create_provider()` match in `llm_providers/mod.rs`
4. Update frontend Settings with new provider metadata

### Running Tests

```bash
# Rust backend tests
cd src-tauri
cargo test

# Frontend tests (if configured)
cd frontend
pnpm test
```

### Debugging

- **Backend Logs**: Use `RUST_LOG=debug pnpm tauri dev` for verbose Rust logging
- **Frontend DevTools**: Press `Cmd+Opt+I` (macOS) or `Ctrl+Shift+I` (Windows/Linux) in dev mode
- **IPC Inspection**: Check browser console for Tauri command invocations

## Troubleshooting

### "Failed to get master key" Error

- **macOS**: Ensure Keychain Access is not blocking the app
- **Linux**: Install `gnome-keyring` or `kwallet`
- **Windows**: Run as user with access to Credential Manager

### Build Failures

- Ensure all system dependencies are installed (see Prerequisites)
- Clear Tauri cache: `pnpm tauri clean`
- Rebuild: `pnpm tauri build`

### Streaming Not Working

- Check provider API key is valid
- Verify network connectivity
- Review Rust logs for HTTP errors

## Roadmap / TODOs

- [ ] **Advanced RAG**: HNSW index for large document sets (currently in-memory cosine similarity)
- [ ] **Code Linting**: ESLint integration via Tauri command + Node.js subprocess
- [ ] **Multi-modal**: Image input for Gemini/Claude vision models
- [ ] **Export**: Conversation export as Markdown/PDF
- [ ] **Prompt Library**: Save and reuse prompt templates
- [ ] **Cost Tracking**: Monitor API usage and costs
- [ ] **Conversation History**: Persistent chat storage in SQLite
- [ ] **Token Counting**: Accurate token estimation per provider
- [ ] **Dark Mode Toggle**: UI theme switcher

## Security Considerations

### What This App Protects Against

- ✅ Casual disk inspection (config encrypted)
- ✅ Config file theft (requires OS keychain access)
- ✅ Accidental exposure in backups (keys not in plaintext)

### What This App Does NOT Protect Against

- ❌ Memory dumps of running process
- ❌ OS-level keyloggers
- ❌ Malicious code execution on the same machine

### Best Practices

- Keep OS and keychain software updated
- Use full-disk encryption (FileVault, BitLocker, LUKS)
- Don't share API keys in config files or screenshots
- Rotate API keys regularly

## License

MIT License - see LICENSE file for details

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit a pull request

## Support

- **Issues**: https://github.com/yourusername/llm-workbench/issues
- **Discussions**: https://github.com/yourusername/llm-workbench/discussions

## Acknowledgments

- **Tauri**: For excellent Rust + Web framework
- **Anthropic, DeepSeek, Google**: For LLM APIs
- **Open Source Community**: For all the amazing libraries used

---

Built with ❤️ using Tauri, Rust, and React
