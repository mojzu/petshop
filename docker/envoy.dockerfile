# DEPEND: docker pull envoyproxy/envoy-alpine:v1.17.1
# <https://hub.docker.com/r/envoyproxy/envoy-alpine>
FROM envoyproxy/envoy-alpine:v1.17.1

# Create configuration directory
RUN mkdir -p /config
WORKDIR /config

COPY ./docker/envoy/envoy.yml /config/envoy/envoy.yaml
COPY ./dist/api.pb /config/api.pb

LABEL org.opencontainers.image.source https://github.com/mojzu/petshop

CMD ["/usr/local/bin/envoy", "-c", "/config/envoy/envoy.yaml"]

# Enable for debug output
#CMD ["/usr/local/bin/envoy", "-c", "/config/envoy/envoy.yaml", "--log-level", "debug"]
