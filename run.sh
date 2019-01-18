#!/bin/bash
# set -x

executable='Ultimate-RePImote'
remotePath='~'
# Commands to run on remote 
cmd="$remotePath/$executable;"


while getopts ":acxr" opt; do
  case ${opt} in
    a ) # Do it all
      COMPILE=1
      COPY=1
      RUN=1
      ;;
    c ) # process option c
      COMPILE=1
      ;;
    x ) # process for x (copy)
      COPY=1
      ;;
    r ) # process option r
      RUN=1
      ;;
    \? ) 
      echo /
      "Usage: cmd [-cxr] /
      -c : compile /
      -x : copy /
      -r : run"
      ;;
  esac
done

# Compile
if [ "$COMPILE" == "1" ]; then
  cargo build --bins --target=arm-unknown-linux-gnueabihf
fi

# Copy compiled files over 
if [ "$COPY" == "1" ]; then
  scp ./target/arm-unknown-linux-gnueabihf/debug/Ultimate-RePImote pi-zero:$remotePath
fi

# Run on remote
if [ "$RUN" == "1" ]; then
  ssh pi-zero $cmd
fi