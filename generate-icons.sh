#!/bin/bash
# Generate placeholder icons for Tauri

# Create a simple SVG icon
cat > /tmp/icon.svg << 'EOF'
<svg width="1024" height="1024" xmlns="http://www.w3.org/2000/svg">
  <rect width="1024" height="1024" fill="#0ea5e9" rx="200"/>
  <text x="512" y="600" font-family="Arial" font-size="400" fill="white" text-anchor="middle" font-weight="bold">LW</text>
  <circle cx="512" cy="300" r="80" fill="white"/>
</svg>
EOF

# Convert to PNG using ImageMagick (if available) or use the SVG directly
if command -v convert &> /dev/null; then
    convert /tmp/icon.svg -resize 1024x1024 /tmp/icon.png
    echo "Created icon.png"
else
    echo "ImageMagick not found. You'll need to manually create icons."
    echo "Install with: brew install imagemagick (macOS) or apt install imagemagick (Linux)"
fi

# Tauri can generate icons from a single PNG
if [ -f /tmp/icon.png ]; then
    cd /home/user/fictional-octo-winner
    pnpm tauri icon /tmp/icon.png 2>&1 || echo "Tauri icon command not available yet (install deps first)"
fi
