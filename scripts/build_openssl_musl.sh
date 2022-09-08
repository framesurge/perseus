#!/bin/bash
# This script build OpenSSL from source for musl cross-compilation, without which the Perseus CLI can't be built for Alpine Linux and the like.
# WARNING: this is for execution on CI, not on your local system!
# Source: https://github.com/dobefore/cross-compile-openssl-musl/blob/main/d.sh (used under MIT license)

wget --no-check-certificate https://www.openssl.org/source/old/1.1.1/openssl-1.1.1f.tar.gz
tar -zxvf openssl-1.1.1f.tar.gz  -C .

export PATH="$HOME/aarch64-linux-musl-cross/bin:$PATH"
export MACHINE=aarch64
export ARCH=arm
export CC=aarch64-linux-musl-gcc
mkdir -p $HOME/openssl
cd openssl-1.1.1f
./config shared --prefix=$HOME/openssl && make
make install
cd ..
