#!/bin/sh

echo "Compiling submodules..."
rustc rust-pcre/pcre.rc
echo "OK"

echo "Compiling main crate..."
rustc -L rust-pcre hello_world.rc
echo "OK"