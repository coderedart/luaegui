#!/bin/bash
# this script is used to build and copy the files into a directory called dist.
set -ex

echo "building for emscripten target"
cargo build --example=basic --target=wasm32-unknown-emscripten --release

echo "copying files to dist directory"
mkdir -p dist
cp target/wasm32-unknown-emscripten/release/examples/basic.wasm dist
cp target/wasm32-unknown-emscripten/release/examples/basic.js dist
cp examples/index.html dist