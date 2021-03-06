# DEPEND: docker pull envoyproxy/envoy-alpine:v1.17.1
# <https://hub.docker.com/r/envoyproxy/envoy-alpine>
FROM envoyproxy/envoy-alpine:v1.17.1

# Create configuration directory
RUN mkdir -p /config
WORKDIR /config

COPY ./docker/envoy/envoy.yml /config/envoy.yaml
COPY ./dist/data /data

LABEL org.opencontainers.image.source https://github.com/mojzu/petshop

CMD ["envoy", "-c", "/config/envoy.yaml"]

# Enable for debug output
#CMD ["envoy", "-c", "/config/envoy.yaml", "--log-level", "debug"]
