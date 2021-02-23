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
- Continuous integration using [GitHub Actions](https://github.com/features/actions)
  and [cargo-make](https://github.com/sagiegurari/cargo-make)
- Example proto definitions based (loosely) on [OpenAPI (V2) Petstore](https://petstore.swagger.io/#/)
- [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for multiple crates
- Configuration from file and/or environment variables using [config](https://github.com/mehcode/config-rs)
- [Prometheus metrics](https://prometheus.io/) endpoint
- [Kubernetes liveness and readiness](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) endpoints
- Logs and panic output to `stderr` are optionally formatted as single line JSON objects

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
cargo make dist-flow
# ---

# Run on host
# Run envoy docker image in host networking mode
cargo make dev-envoy

# Run server binary with cargo or docker image
cargo make dev-server
cargo make dev-server-release

# Run client playground in development mode
cargo make dev-client-playground
# ---

# Or run on docker
# Run envoy and server using docker-compose
cargo make compose-build
docker-compose up
docker-compose down
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
- Restart `dev-*` tasks or run `cargo make compose-build` task and restart docker containers to update everything

The following labels are used:

- `TODO: ...` Something to fix/feature to add/ideas/etc.
- `FIXME: ...` Something that has been fixed in an unintuitive way/may require manual intervention
- `DEPEND: ...` A project dependency that needs updating occasionally
