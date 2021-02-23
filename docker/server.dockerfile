FROM petshop/rust-cache:latest as build

# DEPEND: docker pull debian:10.8
# <https://hub.docker.com/_/debian>
FROM debian:10.8

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y && apt-get clean \
    && rm -rf /tmp/* /var/tmp/* \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=build /build/target/release/petshop_server /usr/local/bin/petshop_server
RUN chmod +x /usr/local/bin/petshop_server

# Copy wait-for-it script
COPY ./docker/server/wait-for-it.sh /usr/local/bin/wait-for-it
RUN chmod +x /usr/local/bin/wait-for-it

# Create configuration directory
RUN mkdir -p /config
COPY ./docker/server/config.toml /config/config.toml
WORKDIR /config

# Log information by default
ENV RUST_LOG="info"

EXPOSE 5000
EXPOSE 5501

# Create system user
RUN groupadd -r petshop && useradd --no-log-init -r -g petshop petshop
USER petshop:petshop

CMD ["petshop_server", "-c", "/config/config.toml"]