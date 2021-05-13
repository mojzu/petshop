# DEPEND: docker pull rust:1.52.1-buster
# <https://hub.docker.com/_/rust>
FROM rust:1.52.1-buster

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates gcc libc6-dev \
    && apt-get -y install lldb python3-minimal libpython3.? \
	&& rm -rf /var/lib/apt/lists/*

# Install Rust features
RUN rustup update 2>&1 \
    && rustup component add rls rust-analysis rust-src rustfmt clippy 2>&1

# DEPEND: Install cargo-make
# <https://crates.io/crates/cargo-make>
RUN cargo install --force cargo-make --version "~0.33"

RUN mkdir -p /src
WORKDIR /src
CMD ["/bin/bash"]
