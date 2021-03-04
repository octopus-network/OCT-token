#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/cdot_token.wasm ./out/main.wasm