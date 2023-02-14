#!/bin/sh -ue

# RUSTFLAGS=-Zmacro-backtrace
# RUST_BACKTRACE=full

NICKNAMES="${NICKNAMES:=bob jim tim}"
SERVER_ARGS="${SERVER_ARGS:=}"
SYSTEM_NAME="${SYSTEM_NAME:=rando_calrissian}"
CARGO_ARGS="${CARGO_ARGS:=--color always}"

LOG_DIR="/tmp/logs/$(date +%s)"
mkdir -p "$LOG_DIR"

LOG_FILENAME="build.out"
cargo build -p '*' $CARGO_ARGS --lib --bins 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME" 

for nick in $NICKNAMES ; do
    LOG_FILENAME="client-${nick}.out"
    ./target/debug/client --nickname "$nick" 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"  &
done

LOG_FILENAME="server.out"
./target/debug/server $SERVER_ARGS --system $SYSTEM_NAME 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"

echo "Log dir: ${LOG_DIR}"
