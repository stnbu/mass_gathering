#!/bin/sh -ue

function mg_build() {
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/release/index.wasm
}

MG_DEST="pu:/var/www/unintuitive.org/mass_gathering"

mg_build
rsync -xva ./pkg ./index.html "$MG_DEST"/

# omg
sed -i '' 's/stereo_enabled: false/stereo_enabled: true/g' src/main.rs
mg_build
rsync -xva ./pkg ./index.html "$MG_DEST"-3d/
# omg
git checkout ./src/main.rs

echo "done"
