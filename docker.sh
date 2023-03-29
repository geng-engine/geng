#!/usr/bin/env bash

set -e; # Exit if any command has non-zero exit status
set -o pipefail; # Prevend pipelines masking errors
set -u; # Error on usage of undefined variables
set -x; # Print all commands as they are executed

function setup() {
    apt-get update
    function install() {
        apt-get install -y --no-install-recommends "$@"
    }

    install ca-certificates # Dont want no certificate errors
    install curl # Need to install rust
    install build-essential # This is essential

    curl https://sh.rustup.rs -sSf | sh -s - -y --no-modify-path --profile minimal

    for target in ${TARGETS:-} linux; do
    case $target in
    linux)
        install libasound2-dev
        install pkg-config
        install libudev-dev
        ;;
    windows)
        install mingw-w64
        rustup target add x86_64-pc-windows-gnu
        printf "[target.x86_64-pc-windows-gnu]\nlinker = \"x86_64-w64-mingw32-gcc\"\n" >> $CARGO_HOME/config
        ;;
    armv7)
        install gcc-arm-linux-gnueabihf
        install libc6-dev-armhf-cross
        rustup target add armv7-unknown-linux-gnueabihf
        printf "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"\n" >> $CARGO_HOME/config
        ;;
    web)
        rustup target add wasm32-unknown-unknown
        ;;
    *)
        echo "$target is not supported"
        exit 1
        ;;
    esac
    done

    # Cleanup
    rm -rf /var/lib/apt/lists/*
}

function run_tests() {
    cargo build \
        --workspace \
        --all-targets
    for target in ${TARGETS:-}; do
    case $target in
    windows)
        cargo build \
            --workspace \
            --all-targets \
            --target x86_64-pc-windows-gnu
        ;;
    armv7)
        cargo build \
            --workspace \
            --all-targets \
            --target armv7-unknown-linux-gnueabihf
        ;;
    web)
        cargo build \
            --workspace \
            --exclude cargo-geng \
            --all-targets \
            --target wasm32-unknown-unknown
        ;;
    *)
        echo "$target is not supported"
        exit 1
        ;;
    esac
    done
}

case ${1:-} in
test)
    run_tests;;
setup)
    setup;;
*)
    echo "This is used by Dockerfile";;
esac