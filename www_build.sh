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

TEMPDIR=`mktemp -d`

cargo build --release --target wasm32-unknown-unknown "$@" -Z unstable-options --out-dir "$TEMPDIR"

wasm-bindgen --out-dir ./target/www/pkg --target web --reference-types --no-typescript --omit-default-module-path "$TEMPDIR"/*.wasm

rm -rf "$TEMPDIR"

BIN_NAME=`echo target/www/pkg/*.js | sed 's@.*/@@;s@\.js@@'`
sed 's/_BIN_NAME_/'"$BIN_NAME"'/g' index.html.template > ./target/www/index.html

echo 'Build assets output to `./target/www` directory.'
