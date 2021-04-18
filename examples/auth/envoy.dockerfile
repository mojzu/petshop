FROM petshop/envoy:latest

COPY ./envoy.yaml /config/envoy.yaml

# Enable for debug output
#CMD ["envoy", "-c", "/config/envoy.yaml", "--log-level", "debug"]
