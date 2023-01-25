#!/bin/sh -ux

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## clone...
#git clone https://github.com/stnbu/mass_gathering.git ./mass_gathering
#cd mass_gathering

## or update...
git update -f

time cargo build -p client --release

echo "Yaay! Next time will be way faster."
