#!/bin/bash

CURRENT_VER=$(head -n4 Cargo.toml | grep version | cut -f2 -d'=' | cut -f2 -d\")

cargo b --release --target aarch64-apple-darwin
cargo b --release --target x86_64-apple-darwin
cargo b --release --target aarch64-unknown-linux-gnu
cargo b --release --target x86_64-unknown-linux-gnu
cargo b --release --target x86_64-pc-windows-gnu

# remove existing files
rm -rf dist
# make the folder again
mkdir -p dist

# copy files to the dist folder
# macos
cp target/aarch64-apple-darwin/release/api-server dist/api-server_macos_aarch64_v"$CURRENT_VER"
cp target/x86_64-apple-darwin/release/api-server dist/api-server_macos_x86-64_v"$CURRENT_VER"
# linux
cp target/aarch64-unknown-linux-gnu/release/api-server dist/api-server_linux_aarch64_v"$CURRENT_VER"
cp target/x86_64-unknown-linux-gnu/release/api-server dist/api-server_linux_x86-64_v"$CURRENT_VER"
# windows
cp target/x86_64-pc-windows-gnu/release/api-server.exe dist/api-server_win_x86-64_v"$CURRENT_VER".exe
