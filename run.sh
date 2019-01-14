#!/bin/bash
# set -x

executable='Ultimate-RePImote'
remotePath='~'

# Compile
cargo build --bins --target=arm-unknown-linux-gnueabihf

# Copy compiled files over 
scp ./target/arm-unknown-linux-gnueabihf/debug/Ultimate-RePImote pi-zero:$remotePath

# Commands to run on remote 
cmd="$remotePath/$executable;"

# Copy over and run
ssh pi-zero $cmd
