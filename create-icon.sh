#!/bin/bash

# Create a professional LLM Workbench icon
cat > /tmp/llm-workbench-icon.svg << 'EOF'
<svg width="1024" height="1024" xmlns="http://www.w3.org/2000/svg">
  <!-- Gradient background -->
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#0ea5e9;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#0369a1;stop-opacity:1" />
    </linearGradient>
    <filter id="shadow">
      <feDropShadow dx="0" dy="4" stdDeviation="8" flood-opacity="0.3"/>
    </filter>
  </defs>

  <!-- Rounded rectangle background -->
  <rect width="1024" height="1024" rx="200" fill="url(#grad)"/>

  <!-- Chat bubble icon -->
  <g filter="url(#shadow)">
    <!-- Main chat bubble -->
    <path d="M 256 384 Q 256 256 384 256 L 640 256 Q 768 256 768 384 L 768 512 Q 768 640 640 640 L 448 640 L 320 736 L 320 640 Q 256 640 256 512 Z"
          fill="white" opacity="0.95"/>

    <!-- AI brain/chip symbol inside -->
    <circle cx="512" cy="448" r="96" fill="#0369a1" opacity="0.8"/>
    <circle cx="476" cy="420" r="12" fill="white"/>
    <circle cx="548" cy="420" r="12" fill="white"/>
    <path d="M 476 476 Q 512 496 548 476" stroke="white" stroke-width="8" fill="none" stroke-linecap="round"/>

    <!-- Circuit pattern -->
    <circle cx="384" cy="384" r="16" fill="white" opacity="0.6"/>
    <circle cx="640" cy="384" r="16" fill="white" opacity="0.6"/>
    <circle cx="384" cy="512" r="16" fill="white" opacity="0.6"/>
    <circle cx="640" cy="512" r="16" fill="white" opacity="0.6"/>
    <line x1="400" y1="384" x2="460" y2="420" stroke="white" stroke-width="4" opacity="0.6"/>
    <line x1="564" y1="420" x2="624" y2="384" stroke="white" stroke-width="4" opacity="0.6"/>
  </g>
</svg>
EOF

echo "Icon SVG created at /tmp/llm-workbench-icon.svg"

# Check if ImageMagick is available
if command -v convert &> /dev/null; then
    echo "Converting SVG to PNG..."
    convert -background none /tmp/llm-workbench-icon.svg -resize 1024x1024 /tmp/llm-workbench-icon.png
    echo "PNG created at /tmp/llm-workbench-icon.png"

    # Use Tauri icon generator if in the project
    if [ -d "/home/user/fictional-octo-winner" ]; then
        echo "Generating Tauri icons..."
        cd /home/user/fictional-octo-winner

        # Generate icons (this requires @tauri-apps/cli to be installed)
        if command -v pnpm &> /dev/null; then
            pnpm tauri icon /tmp/llm-workbench-icon.png
        else
            echo "pnpm not found. Please run manually: pnpm tauri icon /tmp/llm-workbench-icon.png"
        fi
    fi
else
    echo "ImageMagick not installed. Install with:"
    echo "  macOS: brew install imagemagick"
    echo "  Ubuntu: sudo apt install imagemagick"
    echo "  Fedora: sudo dnf install ImageMagick"
    echo ""
    echo "After installing, run: pnpm tauri icon /tmp/llm-workbench-icon.png"
fi
