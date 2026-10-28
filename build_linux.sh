#!/usr/bin/env bash
# Build AetherDraw for Linux (native).
# Produces: target/release/aetherdraw, plus .tar.gz and (optionally) AppImage/.deb.
set -euo pipefail

echo "==> Checking toolchain"
command -v cargo >/dev/null 2>&1 || { echo "cargo not found. Install Rust: https://rustup.rs"; exit 1; }

echo "==> Building (release)"
cargo build --release

BIN="target/release/aetherdraw"
echo "==> Binary: $BIN"

# --- tar.gz ---
echo "==> Packaging .tar.gz"
mkdir -p dist
tar -czf dist/aetherdraw-linux-x86_64.tar.gz -C target/release aetherdraw

# --- .desktop file ---
mkdir -p dist/aetherdraw
cat > dist/aetherdraw/aetherdraw.desktop <<'EOF'
[Desktop Entry]
Name=AetherDraw
Exec=aetherdraw
Icon=aetherdraw
Type=Application
Categories=Graphics;2DGraphics;3DGraphics;Animation;
EOF

echo "==> Done. See dist/"
echo "For AppImage/.deb, install appimagetool / dpkg-deb and follow docs."