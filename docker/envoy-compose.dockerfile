FROM petshop/envoy:latest

COPY ./docker/envoy/envoy-compose.yaml /config/envoy/envoy.yaml
