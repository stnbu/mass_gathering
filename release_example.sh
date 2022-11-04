#!/bin/sh -ue

EXAMPLE="$1"

function mg_build() {
    cargo build --release --target wasm32-unknown-unknown --example $EXAMPLE
    wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/release/examples/"$EXAMPLE".wasm
}

MG_DEST="pu:/var/www/unintuitive.org/mass_gathering/examples/$EXAMPLE"

mg_build
mv -f pkg/"$EXAMPLE".js pkg/index.js
mv -f pkg/"$EXAMPLE"_bg.wasm pkg/index_bg.wasm
rsync -xva ./pkg ./index.html "$MG_DEST"/

# omg
sed -i '' 's/stereo_enabled: false/stereo_enabled: true/g' examples/"$EXAMPLE".rs
mg_build
mv -f pkg/"$EXAMPLE".js pkg/index.js
mv -f pkg/"$EXAMPLE"_bg.wasm pkg/index_bg.wasm
rsync -xva ./pkg ./index.html "$MG_DEST"-3d/
# omg
git checkout examples/"$EXAMPLE".rs

echo "done"
