#!/bin/bash

set -ex

# build the c lib, let's just do it statically for now
C_LIB_DIR=./c_lib
C_LIB_BUILD_DIR=$C_LIB_DIR/build
rm -rf $C_LIB_BUILD_DIR
mkdir -p $C_LIB_BUILD_DIR
gcc -c -o $C_LIB_BUILD_DIR/libdumb.o $C_LIB_DIR/libdumb.c
ar rcs $C_LIB_BUILD_DIR/libdumb.a $C_LIB_BUILD_DIR/libdumb.o

CWD=`pwd`
# build the rust thing and run it
cargo clean --manifest-path ./ffi_exp/Cargo.toml
RUSTFLAGS="-L $CWD/c_lib/build/" cargo build --manifest-path ./ffi_exp/Cargo.toml
ffi_exp/target/debug/ffi_exp