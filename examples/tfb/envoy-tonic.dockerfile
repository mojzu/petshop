# DEPEND: docker pull envoyproxy/envoy-alpine:v1.17.1
# <https://hub.docker.com/r/envoyproxy/envoy-alpine>
FROM envoyproxy/envoy-alpine:v1.17.1

# Create configuration directory
RUN mkdir -p /config
WORKDIR /config

COPY ./envoy.yml /config/envoy.yaml
COPY ./api.pb /data/api.pb

CMD ["envoy", "-c", "/config/envoy.yaml"]
