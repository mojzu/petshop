FROM petshop/rust-tools:latest

WORKDIR /build

COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
COPY ./proto /build/proto
COPY ./server/Cargo.toml /build/server/Cargo.toml
RUN mkdir .cargo \
    && cargo vendor > .cargo/config

COPY ./server /build/server
RUN cargo build --release
