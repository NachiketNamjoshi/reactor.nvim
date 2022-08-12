#!/usr/bin/env bash

# Stolen from :: https://github.com/shaunsingh/oxocarbon.nvim/main/install.sh

set -e

cargo build --release 

# Place the compiled library where Neovim can find it.
mkdir -p lua

if [ "$(uname)" == "Darwin" ]; then
    mv target/release/libreactor.dylib lua/reactor.so
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    mv target/release/libreactor.so lua/reactor.so
elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
    mv target/release/reactor.dll lua/reactor.dll
fi

