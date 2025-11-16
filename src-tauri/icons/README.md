# Application Icons

This directory should contain the application icons for different platforms.

## Required Icons

For a full Tauri build, you need:

- `32x32.png` - Windows small icon
- `128x128.png` - macOS/Linux icon
- `128x128@2x.png` - macOS Retina icon
- `icon.icns` - macOS bundle icon
- `icon.ico` - Windows icon

## Generating Icons

You can use Tauri's icon generator:

```bash
pnpm tauri icon path/to/your/icon.png
```

This will generate all required formats from a single source image (preferably 1024x1024 PNG).

## Temporary Workaround

For development, Tauri will use default icons if these files are missing.

To generate icons now:

1. Create a 1024x1024 PNG icon
2. Run: `pnpm tauri icon your-icon.png`
3. Icons will be placed in this directory

## Icon Design Guidelines

- Use a simple, recognizable symbol
- Ensure it looks good at small sizes (32x32)
- Use transparent background for better OS integration
- Follow platform-specific guidelines (macOS Human Interface, Windows Fluent, etc.)
