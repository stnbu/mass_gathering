#!/bin/sh -ue

NICKNAMES="$@"

for nick in $NICKNAMES ; do
    cargo run --bin client -- --nickname "$nick" &
done
cargo run -- --speed 1 --system testing_no_unhinhabited

