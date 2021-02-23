FROM petshop/rust-tools:latest

RUN mkdir -p /build
WORKDIR /build

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
