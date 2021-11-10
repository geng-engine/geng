FROM debian:stretch-slim
SHELL ["/bin/bash", "-c"]

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        netbase \
        ssh \
        git \
        curl \
        wget \
        zip \
        unzip \
        pkg-config \
        libssl-dev \
        gcc \
        make \
        cmake \
        libasound2-dev \
        libgtk-3-dev \
        jq \
        mingw-w64 \
        libxml2-dev \
        gcc-arm-linux-gnueabihf \
        libc6-dev-armhf-cross \
    ; \
    rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    curl https://sh.rustup.rs -sSf | sh -s - -y --no-modify-path --profile minimal; \
    rustup target add x86_64-pc-windows-gnu; \
    printf "[target.x86_64-pc-windows-gnu]\nlinker = \"x86_64-w64-mingw32-gcc\"\n" >> $CARGO_HOME/config; \
    rustup target add armv7-unknown-linux-gnueabihf; \
    printf "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"\n" >> $CARGO_HOME/config; \
    rustup target add wasm32-unknown-unknown; \
    cargo install wasm-bindgen-cli; \
    printf "[net]\ngit-fetch-with-cli = true\n" >> $CARGO_HOME/config;
    
COPY . /src
RUN cargo install --path /src/crates/cargo-geng
RUN rm -rf /src