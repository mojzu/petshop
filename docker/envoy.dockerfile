# DEPEND: docker pull envoyproxy/envoy-alpine:v1.18.3
# <https://hub.docker.com/r/envoyproxy/envoy-alpine>
FROM envoyproxy/envoy-alpine:v1.18.3

# Create configuration directory
RUN mkdir -p /config
WORKDIR /config

# Copy default configuration and protobuf definition file
COPY ./docker/envoy/envoy.yaml /config/envoy.yaml
COPY ./dist/data /data

# Add opencontainers annotations
#
# created, revision, source and version are added by Makefile.toml tasks
# <https://github.com/opencontainers/image-spec/blob/master/annotations.md>
LABEL org.opencontainers.image.title "Petshop Envoy"
LABEL org.opencontainers.image.authors "Sam Ward <mail@mojzu.net>"
LABEL org.opencontainers.image.url "https://github.com/mojzu/petshop"
LABEL org.opencontainers.image.documentation "https://github.com/mojzu/petshop"
LABEL org.opencontainers.image.source "https://github.com/mojzu/petshop"

CMD ["envoy", "-c", "/config/envoy.yaml"]

# Enable for debug output
#CMD ["envoy", "-c", "/config/envoy.yaml", "--log-level", "debug"]
