#!/bin/sh -ue

#
# A thick wrapper to `cargo build` a web-server-deployable (albiet primitive)
# WASM build (JavaScript bindings included.)
#
# You may pass in whatever arguments you like: `--example game` or nothing at all, in which case `src/main.rs` is built.
#
# One could `my-server ./target/www/` or `rsync -a ./target/www/ web-server:/html/mygame/`
#

rm -rf ./target/www
mkdir -p ./target/www

cargo build --release --target wasm32-unknown-unknown --bin "index" "$@"
wasm-bindgen --out-dir ./target/www/pkg --target web --reference-types --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/release/index.wasm

cp -af index.html ./target/www/

echo "Build assets output to `./target/www` directory."
