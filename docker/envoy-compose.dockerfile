FROM petshop/envoy:latest

COPY ./docker/envoy/envoy-compose.yml /config/envoy.yaml

# Enable for debug output
#CMD ["envoy", "-c", "/config/envoy.yaml", "--log-level", "debug"]
