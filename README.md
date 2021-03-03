# petshop

[![licence](https://img.shields.io/github/license/mojzu/petshop?label=licence)](https://github.com/mojzu/petshop/blob/master/LICENCE)
[![CI](https://github.com/mojzu/petshop/workflows/CI/badge.svg?branch=main)](https://github.com/mojzu/petshop/actions/workflows/ci.yml)

Template for Rust API server with gRPC and OpenAPI (V2) interfaces

## Features

- Rust gRPC server using [tonic](https://github.com/hyperium/tonic)
- Envoy proxy with [gRPC-JSON transcoder](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/grpc_json_transcoder_filter)
- Builds [Docker](https://docs.docker.com/reference/) images for gRPC server and Envoy proxy
- [Generated OpenAPI (V2) definitions](https://github.com/grpc-ecosystem/grpc-gateway) from gRPC `.proto` files
- Generated TypeScript [axios](https://github.com/axios/axios) and [gRPC Web](https://github.com/grpc/grpc-web) clients
- Client playground for browser with [Parcel](https://v2.parceljs.org/)
- Integration tests with [Jasmine](https://jasmine.github.io/)
- Continuous integration using [GitHub Actions](https://github.com/features/actions) and [cargo-make](https://github.com/sagiegurari/cargo-make)
- Example proto definitions based (loosely) on [OpenAPI (V2) Petstore](https://petstore.swagger.io/#/)
- [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for multiple crates
- Configuration from file and/or environment variables using [config](https://github.com/mehcode/config-rs)
- Logs and panic output to `stderr` are optionally formatted as single line JSON objects with [tracing](https://tracing.rs/tracing/)
- Request validation with [validator](https://github.com/Keats/validator)
- Postgres connection pool with [Deadpool](https://github.com/bikeshedder/deadpool) and [tokio-postgres](https://crates.io/crates/tokio-postgres)
- [Prometheus metrics](https://prometheus.io/) endpoint
- [Kubernetes liveness and readiness](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) endpoints
- Authentication example with [OAuth2 Proxy](https://oauth2-proxy.github.io/oauth2-proxy/) and [Envoy External Authorization](https://www.envoyproxy.io/docs/envoy/latest/api-v2/config/filter/http/ext_authz/v2/ext_authz.proto)
- [TechEmpower Benchmark Framework](https://www.techempower.com/benchmarks/) example ([2021-03-02 results](https://www.techempower.com/benchmarks/#section=test&shareid=980a0c18-35c7-4a79-9199-f79a5a19eeee))

## Quickstart

```shell
# Dependencies
# Install Docker, Docker Compose, Rust stable
docker --version
docker-compose --version
cargo --version
# DEPEND: Install cargo-make
# <https://crates.io/crates/cargo-make>
cargo install --force cargo-make --version "~0.32"
cargo make --version
# Run distribution tasks
cargo make dist-build
# ---

# Run on host
# Run envoy and postgres docker images in host networking mode
cargo make dev-envoy
cargo make dev-postgres

# Run server binary with cargo or docker image
cargo make dev-server
cargo make dev-server-release

# Run client playground in development mode
cargo make dev-client-playground
# ---

# Or run on docker
# Run envoy and server using docker-compose
cargo make compose build
cargo make compose up
cargo make compose down
# ---

# Open client playground at http://localhost:1234
```

## Notes

How to use this template:

- Clone this repository
- Find and replace `petshop` and `Petshop` to rename outputs
- Change `LICENCE` file

How to change the API:

- Update `proto/proto/api.proto`
- Implement aysnc trait methods in `server` crate (run `cargo build` to check)
- Restart `dev-*` tasks or run `cargo make dist-build` and `cargo make compose build` to update everything

The following labels are used:

- `TODO: ...` Something to fix/feature to add/ideas/etc.
- `FIXME: ...` Something that has been fixed in an unintuitive way/may require manual intervention
- `DEPEND: ...` A project dependency that needs updating occasionally
