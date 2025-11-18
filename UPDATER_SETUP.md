# Auto-Update Setup Guide

The application is configured to support automatic updates via Tauri's updater system.

## Setup Steps

### 1. Generate Update Keys

First, generate a public/private key pair for signing updates:

```bash
cd src-tauri
cargo install tauri-cli
cargo tauri signer generate
```

This will output:
- A **public key** (to be added to `tauri.conf.json`)
- A **private key** (keep this SECRET - needed for signing releases)

### 2. Configure tauri.conf.json

Replace `YOUR_PUBLIC_KEY_HERE` in `src-tauri/tauri.conf.json` with your public key:

```json
"updater": {
  "active": true,
  "endpoints": [
    "https://releases.myapp.com/{{target}}/{{current_version}}"
  ],
  "dialog": true,
  "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6..."
}
```

### 3. Update the Endpoint URL

Replace `https://releases.myapp.com` with your actual release server URL. Common options:

- **GitHub Releases**: `https://github.com/username/repo/releases/latest/download`
- **Custom Server**: Your own S3, CDN, or file server

The `{{target}}` and `{{current_version}}` placeholders are automatically replaced.

### 4. Set Up Release Workflow

Add this to `.github/workflows/release.yml`:

```yaml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies (Ubuntu only)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install app dependencies
        run: |
          cd frontend && npm install

      - name: Build and sign
        env:
          TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
          TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
        run: |
          cd src-tauri
          cargo tauri build

      - name: Upload release
        uses: softprops/action-gh-release@v1
        with:
          files: src-tauri/target/release/bundle/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 5. Add Secrets to GitHub

In your GitHub repository settings, add:

- `TAURI_PRIVATE_KEY`: Your private key from step 1
- `TAURI_KEY_PASSWORD`: Password for the private key (if you set one)

### 6. Create a Release

```bash
git tag v0.2.0
git push origin v0.2.0
```

The workflow will automatically build, sign, and upload the release artifacts.

## Testing

To test the updater locally:

1. Build version 0.1.0
2. Install it on your system
3. Update `package.json` version to 0.2.0
4. Create a release
5. Run the app - it should detect and offer the update

## Disabling Auto-Update

To disable auto-updates, set `"active": false` in the updater config.

## Resources

- [Tauri Updater Guide](https://tauri.app/v1/guides/distribution/updater)
- [Tauri Release Workflow Example](https://github.com/tauri-apps/tauri-action)
