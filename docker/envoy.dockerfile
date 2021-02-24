# DEPEND: docker pull envoyproxy/envoy-alpine:v1.17.0
# <https://hub.docker.com/r/envoyproxy/envoy-alpine>
FROM envoyproxy/envoy-alpine:v1.17.0

# Create configuration directory
RUN mkdir -p /config
WORKDIR /config

COPY ./docker/envoy/envoy.yml /config/envoy/envoy.yaml
COPY ./dist/api.pb /config/api.pb

CMD ["/usr/local/bin/envoy", "-c", "/config/envoy/envoy.yaml"]
