FROM petshop/rust-cache:latest as build

# DEPEND: docker pull alpine:3.12
# <https://hub.docker.com/_/alpine>
FROM alpine:3.12

# Copy binaries
COPY --from=build /build/target/release/petshop_server /usr/local/bin/petshop_server
RUN chmod +x /usr/local/bin/petshop_server

# Create configuration directory
RUN mkdir -p /config
COPY ./docker/server/config.toml /config/config.toml
WORKDIR /config

# Log information by default
ENV RUST_LOG="info"

EXPOSE 5000
EXPOSE 5501

# Create system user
RUN addgroup -S petshop && adduser -S petshop -G petshop
USER petshop:petshop

CMD ["petshop_server", "-c", "/config/config.toml"]
