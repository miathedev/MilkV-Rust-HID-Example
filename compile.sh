#!/bin/bash
set -e -x

# Install SDK if riscv64-lp64d--musl--stable-2024.05-1 folder doesnt exist
if [ -d "toolchain" ]; then
    echo "SDK already installed"
else
    echo "Installing SDK..."
    curl https://toolchains.bootlin.com/downloads/releases/toolchains/riscv64-lp64d/tarballs/riscv64-lp64d--musl--stable-2024.05-1.tar.xz -o /tmp/toolchain.tar.xz
    mkdir toolchain
    tar -xf /tmp/toolchain.tar.xz -C toolchain --strip-components=1
    rm /tmp/toolchain.tar.xz
fi

#Add Compiler to PATH
export PATH=${PWD}/toolchain/bin:$PATH
export PKG_CONFIG_SYSROOT_DIR=${PWD}/toolchain/riscv64-buildroot-linux-musl/sysroot
export CC=${PWD}/toolchain/bin/riscv64-buildroot-linux-musl-gcc
export CXX=${PWD}/toolchain/bin/riscv64-buildroot-linux-musl-g++
#Install nightly rust
rustup toolchain install nightly
#Install std
rustup target add riscv64gc-unknown-linux-musl
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly

echo "Build for riscv"
cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std

#echo "Build for x86"
#unset CC CXX
#cargo +nightly build --target x86_64-unknown-linux-gnu -Zbuild-std