# Build AetherDraw for Windows (native).
# Produces: target\release\aetherdraw.exe plus portable .zip and installer (NSIS/Inno).

$ErrorActionPreference = "Stop"

Write-Host "==> Checking toolchain"
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo not found. Install Rust: https://rustup.rs"
    exit 1
}

Write-Host "==> Building (release)"
cargo build --release

$bin = "target\release\aetherdraw.exe"
Write-Host "==> Binary: $bin"

# Portable zip
New-Item -ItemType Directory -Force -Path dist | Out-Null
Compress-Archive -Path $bin -DestinationPath "dist\aetherdraw-windows-x86_64.zip" -Force

# NSIS / Inno Setup: optional. Example with Inno Setup:
# & "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss

Write-Host "==> Done. See dist\"