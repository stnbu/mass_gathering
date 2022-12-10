#!/bin/sh

cargo run &
cargo run --bin client -- --nickname NICK &
cargo run --bin client -- --nickname KNOCK &
cargo run --bin client -- --nickname KNACK &
