#!/bin/sh -ue

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path \
	     target/wasm32-unknown-unknown/release/index.wasm

rsync -xva ./pkg ./index.html "$1"

echo "done"
