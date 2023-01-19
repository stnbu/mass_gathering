#!/bin/sh -ue

NICKNAMES="$@"

for nick in $NICKNAMES ; do
    cargo run -p client -- --nickname "$nick" &
done
cargo run -p server -- --speed 1 --system old_rando

