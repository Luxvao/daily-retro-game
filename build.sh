#! /bin/sh

# Build for x86-64 linux
cargo build --release

# Build for windows
RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-pc-windows-gnu --release
