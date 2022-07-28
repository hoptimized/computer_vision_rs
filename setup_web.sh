#!/usr/bin/env bash
set -eu

# Pre-requisites:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version $(cargo pkgid wasm-bindgen | grep -Po '(?<=#wasm-bindgen[@,:])(.*)')

# For local tests with `./start_server`:
cargo install basic-http-server
