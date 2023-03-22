FROM debian:buster-slim AS base
SHELL ["/bin/bash", "-c"]
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
COPY docker.sh /
ARG TARGETS
RUN ["/bin/bash", "/docker.sh", "setup"]

FROM base AS intermediate
COPY . /src
RUN cargo install --path /src/crates/cargo-geng

FROM base AS final
COPY --from=intermediate /usr/local/cargo/bin/cargo-geng /usr/local/cargo/bin/cargo-geng
