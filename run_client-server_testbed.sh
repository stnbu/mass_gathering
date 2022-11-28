#!/bin/sh -u

for ID in 49 ; do
    echo ">>> Running client ${ID} \"in the background\"..." >&2
    cargo run --bin client -- --id $ID &
done &
echo ">>> Running server \"in the forground\"..." >&2
cargo run --bin server