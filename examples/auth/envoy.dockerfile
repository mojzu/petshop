FROM petshop/envoy:latest

COPY ./envoy.yml /config/envoy.yaml

# Enable for debug output
#CMD ["envoy", "-c", "/config/envoy.yaml", "--log-level", "debug"]
