FROM petshop/envoy:latest

COPY ./docker/envoy/envoy-compose.yml /config/envoy/envoy.yaml
