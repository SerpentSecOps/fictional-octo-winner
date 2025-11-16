# Quick Start Guide

## Prerequisites Check

Before running, ensure you have:

```bash
# Check Node.js version (need >= 18)
node --version

# Check pnpm (install if missing)
pnpm --version || npm install -g pnpm

# Check Rust (Tauri will install if missing)
rustc --version
```

## Installation (5 minutes)

### 1. Install Dependencies

```bash
cd fictional-octo-winner

# Install root dependencies (Tauri CLI)
pnpm install

# Install frontend dependencies
cd frontend
pnpm install
cd ..
```

### 2. Handle Missing Icons (Optional for Dev)

**Option A: Skip (dev mode works without icons)**
- Just run `pnpm tauri dev` - Tauri uses default icons

**Option B: Generate Icons**
```bash
# Create a simple 1024x1024 PNG image (any tool)
# Then run:
pnpm tauri icon path/to/your/icon.png
```

### 3. Run Development Build

```bash
pnpm tauri dev
```

This will:
- Compile Rust backend (first time takes 3-5 min)
- Start Vite dev server
- Launch the desktop app window

**Expected output:**
```
    Finished dev [unoptimized + debuginfo] target(s) in 2m 34s
    Running `target/debug/llm-workbench`
```

## First Use (2 minutes)

### 1. Configure a Provider

1. Click **Settings** icon (bottom left)
2. Select **Anthropic Claude** (or Gemini/DeepSeek)
3. Enter your API key (get from https://console.anthropic.com/)
4. Set model: `claude-3-5-sonnet-20241022`
5. Click **Save Configuration**
6. Click **Test Connection** to verify

### 2. Start Chatting

1. Click **Chat** icon (top left)
2. Ensure provider is selected in dropdown
3. Type a message
4. Press Enter
5. Watch streaming response appear

### 3. Try RAG (Optional)

1. Go to **Canvas** view
2. Click **New Project** → enter "My Notes"
3. Go back to **Chat**
4. Toggle **RAG** button ON
5. Select project "My Notes"
6. (Note: Document upload UI not yet implemented - see below)

## Current Limitations

### ⚠️ What's Not Yet Implemented

1. **Document Upload UI**
   - RAG documents must be added via direct Tauri commands
   - File picker dialog TODO

2. **Conversation History**
   - Chats are not saved between sessions
   - Refresh = lose conversation

3. **Code Linting**
   - Code Lab editor works, but linting is TODO

4. **Error Handling**
   - Some errors show as browser alerts (not polished)

### ✅ What Works Now

- ✅ Secure API key storage (encrypted + OS keychain)
- ✅ Multi-provider chat (DeepSeek, Gemini, Claude)
- ✅ Streaming responses
- ✅ RAG vector search (if documents added programmatically)
- ✅ Canvas workspace (create, save, load)
- ✅ Monaco code editor with syntax highlighting

## Troubleshooting

### "Failed to get master key"

**macOS:**
```bash
# Grant keychain access
# You may see a popup - click "Always Allow"
```

**Linux:**
```bash
# Install keyring backend
sudo apt install gnome-keyring  # Ubuntu/Debian
# OR
sudo pacman -S gnome-keyring    # Arch
```

**Windows:**
- Run as administrator if issues persist

### Build Errors

```bash
# Clean and rebuild
pnpm tauri clean
rm -rf node_modules frontend/node_modules
pnpm install
cd frontend && pnpm install && cd ..
pnpm tauri dev
```

### Port 5173 Already in Use

```bash
# Kill process on port 5173
lsof -ti:5173 | xargs kill -9  # macOS/Linux
# OR change port in frontend/vite.config.ts
```

### Rust Compilation Slow

First build takes 3-5 minutes (compiling dependencies).
Subsequent builds: 5-10 seconds.

### API Connection Fails

- Verify API key is correct
- Check internet connection
- Review backend logs in terminal
- Some providers have rate limits

## Advanced: Programmatically Add RAG Documents

Since the UI isn't implemented yet, you can add documents via browser console:

```typescript
// In DevTools console (Cmd+Opt+I / Ctrl+Shift+I)
const { invoke } = window.__TAURI__.tauri;

// 1. Create project (if not exists)
const project = await invoke('create_project', { name: 'Test Project' });

// 2. Add document
await invoke('add_document', {
  request: {
    project_id: project.data.id,
    name: 'sample.txt',
    content: 'Your document content here...',
    provider_id: 'gemini'  // Use Gemini for embeddings
  }
});

// 3. Now use RAG in Chat with this project
```

## Next Steps

See `TODO.md` for the full roadmap. Priority improvements:

1. Add document upload UI
2. Persist conversation history
3. Better error notifications
4. Model preset dropdowns

## Getting Help

- **Issues**: Check `TODO.md` for known issues
- **Documentation**: Read `ARCHITECTURE.md` for technical details
- **Logs**: Backend logs appear in the terminal where you ran `pnpm tauri dev`

---

**Estimated Time to Working App**: 7 minutes (5 min install + 2 min config)
