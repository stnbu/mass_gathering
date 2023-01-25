#!/bin/sh -ue

# RUSTFLAGS=-Zmacro-backtrace
# RUST_BACKTRACE=full

_REL="${_REL:=}"
_NICKS="${_NICKS:=bob}"
_CARGS="${_CARGS:=}"
_SARGS="${_SARGS:=--speed 1}"
_SYS="${_SYS:=demo_2m1i}"
_CRUN="${_CRUN:=--color always}"

LOG_DIR="/tmp/logs/$(date +%s)"
mkdir -p "$LOG_DIR"

for nick in $_NICKS ; do
    LOG_FILENAME="client-${nick}.out"
    cargo run -p client $_CRUN $_REL -- --nickname "$nick" $_CARGS 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"  &
done

LOG_FILENAME="server.out"
cargo run -p server $_CRUN $_REL -- $_SARGS --system $_SYS 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"

echo "Log dir: ${LOG_DIR}"

# echo ">>> HERE COME INTERESTING LOG MESSAGES..."
# grep -rvE 'DEBUG|Blocking waiting for file lock on package cache|Radeon|Blocking waiting for file lock on build directory|  Finished dev \[| Running `' $LOG_DIR