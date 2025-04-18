#!/bin/bash

# rustup target add x86_64-pc-windows-gnu
# rustup target add x86_64-unknown-linux-musl
# rustup target add aarch64-apple-darwin
# rustup target add x86_64-apple-darwin

# ~/.cargo/config.toml
#   [target.x86_64-unknown-linux-musl]
#   linker = "x86_64-linux-musl-gcc"

cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
