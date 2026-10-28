# Dependencies

## Rust toolchain
Install via rustup: https://rustup.rs

## Linux (Ubuntu/Debian)
```bash
sudo apt-get install -y \
  build-essential pkg-config \
  libasound2-dev libudev-dev \
  libx11-dev libxi-dev libxrandr-dev libxinerama-dev libxcursor-dev \
  libgl1-mesa-dev libvulkan-dev
```
Tablet support (optional): `libinput-dev`, `libevdev-dev`.

## Windows
Install Visual Studio Build Tools (C++ workload) and the Vulkan SDK (optional, wgpu can use DX12).
Rustup handles the rest.

## Rust crates
All listed in Cargo.toml. Install automatically via `cargo build`.