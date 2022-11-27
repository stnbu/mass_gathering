#!/bin/sh -u

echo ">>> Running client \"in the background\"..." >&2
cargo run --bin client &
echo ">>> Running server \"in the forground\"..." >&2
cargo run --bin server