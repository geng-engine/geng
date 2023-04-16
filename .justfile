check *OPTIONS:
    cargo clippy --all-targets {{OPTIONS}}
    cargo clippy --all-targets --target wasm32-unknown-unknown {{OPTIONS}}

prepare *OPTIONS:
    cargo clippy --all-targets {{OPTIONS}}
    cargo build --all-targets {{OPTIONS}}

prepare-all:
    just prepare
    just prepare --target wasm32-unknown-unknown
    just prepare --release
    just prepare --release --target wasm32-unknown-unknown
