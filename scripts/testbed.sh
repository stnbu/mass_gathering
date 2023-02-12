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
cargo build -p '*' $CARGO_ARGS --lib --bins 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"  &

for nick in $NICKNAMES ; do
    LOG_FILENAME="client-${nick}.out"
    cargo run -p client $CARGO_ARGS -- --nickname "$nick" 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"  &
done

LOG_FILENAME="server.out"
cargo run -p server $CARGO_ARGS --features windows -- $SERVER_ARGS --system $SYSTEM_NAME 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"

echo "Log dir: ${LOG_DIR}"

# echo ">>> HERE COME INTERESTING LOG MESSAGES..."
# grep -rvE 'DEBUG|Blocking waiting for file lock on package cache|Radeon|Blocking waiting for file lock on build directory|  Finished dev \[| Running `' $LOG_DIR