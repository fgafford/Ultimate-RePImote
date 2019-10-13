#!/bin/bash
# set -x

setup="" # "RUST_BACKTRACE=1"
remotePath='~'
executable='Ultimate-RePImote'
# Commands to run on remote 
cmd="$setup $remotePath/$executable;"


while getopts ":acvrs" opt; do
  case ${opt} in
    a ) # Do it all
      COMPILE=1
      COPY=1
      RUN=1
      ;;
    c ) # process option c
      COMPILE=1
      ;;
    v ) # process for x (copy)
      COPY=1
      ;;
    r ) # process option r
      RUN=1
      ;;
    s ) # Setup database
      SETUP=1
      ;;
    \? ) 
cat << EOF
Synopsis: 
  cmd [-cxra] file.rs

Describe:
  -a : Do full program run (compile, copy, and run)
  -c : compile 
  -x : copy 
  -r : run
  -s : setup (setup database on server)
EOF
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
  ssh -t pi-zero $cmd
fi

# Setup DB on server
if [ "$SETUP" == "1" ]; then
  scp ./setup.sql pi-zero:~
  
  # Run Script
  setup_cmd="sqlite3 UltimateRePImote.db < ~/setup.sql"
  ssh -t pi-zero $setup_cmd

  # Remove Script
  # ssh -t pi-zero "rm ~/.setup.sql"
fi