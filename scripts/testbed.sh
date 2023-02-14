#!/bin/sh -ue

# RUSTFLAGS=-Zmacro-backtrace
# RUST_BACKTRACE=full

NICKNAMES="${NICKNAMES:=bob jim tim}"
SERVER_ARGS="${SERVER_ARGS:=}"
SYSTEM_NAME="${SYSTEM_NAME:=rando_calrissian}"
CARGO_ARGS="${CARGO_ARGS:=--color always}"
PROFILE="${PROFILE:=debug}" # must be 'debug' or 'release'

###

LOG_DIR="/tmp/logs/$(date +%s)"
mkdir -p "$LOG_DIR"

RELEASE_FLAG="--release"
if [ "$PROFILE" != "release" ] ; then
    RELEASE_FLAG=""
fi

LOG_FILENAME="build.out"
cargo build -p '*' $CARGO_ARGS $RELEASE_FLAG --lib --bins 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"

for nick in $NICKNAMES ; do
    LOG_FILENAME="client-${nick}.out"
    ./target/"$PROFILE"/client --nickname "$nick" 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"  &
done

LOG_FILENAME="server.out"
./target/"$PROFILE"/server $SERVER_ARGS --system $SYSTEM_NAME 2>&1 | tee "$LOG_DIR"/"$LOG_FILENAME"

echo "Log dir: ${LOG_DIR}"
