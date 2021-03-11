# DEPEND: docker pull rust:1.50.0-alpine3.12
# <https://hub.docker.com/_/rust>
FROM rust:1.50.0-alpine3.12

# Install dependencies
RUN apk add --no-cache musl-dev protoc \
    && rustup component add rustfmt 2>&1

RUN mkdir -p /build
WORKDIR /build

# FIXME: Removed `RUSTFLAGS="-C target-cpu=native"` from cargo build
# Otherwise docker images may fail to run on some architectures (illegal instruction)
# May be useful in some cases where the target cpu is known

# Vendor and build minimum dependencies to avoid recompilation
# If more crates are added they should be added here too
RUN cargo new --lib proto \
    && cargo new --bin server
COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
COPY ./proto/Cargo.toml /build/proto/Cargo.toml
COPY ./server/Cargo.toml /build/server/Cargo.toml
RUN mkdir .cargo \
    && cargo vendor > .cargo/config \
    && cargo build --release

COPY ./proto /build/proto
COPY ./server /build/server
RUN cargo build --release
