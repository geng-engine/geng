FROM rust:slim-stretch
RUN apt-get update && apt-get install -y \
    ssh git curl wget zip unzip pkg-config \
    libssl-dev make cmake libasound2-dev libgtk-3-dev jq \
    mingw-w64 clang libxml2-dev

# Install cargo-geng
COPY . /tmp/cargo-geng
RUN cargo install --path /tmp/cargo-geng

# Windows
RUN rustup target add x86_64-pc-windows-gnu

# MacOS
COPY install-osxcross.sh /tmp
RUN sh /tmp/install-osxcross.sh
ENV PATH=$PATH:/tmp/osxcross/target/bin
ENV LIBZ_SYS_STATIC=1
ENV CC=o64-clang
ENV CXX=o64-clang++
RUN rustup target add x86_64-apple-darwin

# Web
RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

COPY cargo.config $HOME/.cargo/config