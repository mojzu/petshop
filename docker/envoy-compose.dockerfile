FROM petshop/envoy:latest

COPY ./docker/envoy/envoy-compose.yml /config/envoy/envoy.yaml

# Enable for debug output
#CMD ["/usr/local/bin/envoy", "-c", "/config/envoy/envoy.yaml", "--log-level", "debug"]
