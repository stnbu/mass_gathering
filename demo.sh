#!/bin/sh -ue

NICKNAMES="$@"

cargo run &
for nick in $NICKNAMES ; do
    cargo run --bin client -- --nickname "$nick" &
done

