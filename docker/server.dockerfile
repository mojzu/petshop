FROM petshop/rust-cache:latest as build

# DEPEND: docker pull alpine:3.13.5
# <https://hub.docker.com/_/alpine>
FROM alpine:3.13.5

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

# Add opencontainers annotations
#
# created, revision, source and version are added by Makefile.toml tasks
# <https://github.com/opencontainers/image-spec/blob/master/annotations.md>
LABEL org.opencontainers.image.title "Petshop Server"
LABEL org.opencontainers.image.authors "Sam Ward <mail@mojzu.net>"
LABEL org.opencontainers.image.url "https://github.com/mojzu/petshop"
LABEL org.opencontainers.image.documentation "https://github.com/mojzu/petshop"
LABEL org.opencontainers.image.source "https://github.com/mojzu/petshop"

CMD ["petshop_server", "-c", "/config/config.toml"]
