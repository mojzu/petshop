# petshop

[![licence](https://img.shields.io/github/license/mojzu/petshop?label=licence)](https://github.com/mojzu/petshop/blob/master/LICENCE)
[![CI](https://github.com/mojzu/petshop/workflows/CI/badge.svg?branch=main)](https://github.com/mojzu/petshop/actions/workflows/ci.yml)

Template for Rust API server with gRPC and OpenAPI (V2) interfaces

## Features

-   Rust gRPC server using [tonic](https://github.com/hyperium/tonic)
-   Envoy proxy with [gRPC-JSON transcoder](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/grpc_json_transcoder_filter)
-   Builds [Docker](https://docs.docker.com/reference/) images for gRPC server and Envoy proxy based on [Alpine Linux](https://alpinelinux.org/)
-   [Generated OpenAPI (V2) definitions](https://github.com/grpc-ecosystem/grpc-gateway) from gRPC `.proto` files
-   Generated TypeScript [axios](https://github.com/axios/axios), [gRPC Web](https://github.com/grpc/grpc-web) and [Angular](https://github.com/cyclosproject/ng-swagger-gen) clients
-   Client playground for browser with [Parcel](https://v2.parceljs.org/)
-   Integration tests with [Jasmine](https://jasmine.github.io/)
-   Continuous integration using [GitHub Actions](https://github.com/features/actions) and [cargo-make](https://github.com/sagiegurari/cargo-make)
-   Docker image and Node.js package publishing to [GitHub Packages](https://github.com/features/packages)
-   Example proto definitions based (loosely) on [OpenAPI (V2) Petstore](https://petstore.swagger.io/#/)
-   [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for multiple crates
-   Configuration from file and/or environment variables using [config](https://github.com/mehcode/config-rs)
-   Logs and panic output to `stderr` are optionally formatted as single line JSON objects with [tracing](https://tracing.rs/tracing/)
-   Request validation with [validator](https://github.com/Keats/validator)
-   Postgres connection pool with [Deadpool](https://github.com/bikeshedder/deadpool) and [tokio-postgres](https://crates.io/crates/tokio-postgres)
-   [Prometheus metrics](https://prometheus.io/) endpoint
-   [Kubernetes liveness and readiness](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) endpoints
-   Authentication example with [OAuth2 Proxy](https://oauth2-proxy.github.io/oauth2-proxy/) and [Envoy External Authorization](https://www.envoyproxy.io/docs/envoy/latest/api-v2/config/filter/http/ext_authz/v2/ext_authz.proto)
-   [TechEmpower Benchmark Framework](https://www.techempower.com/benchmarks/) example ([2021-03-04 results](https://www.techempower.com/benchmarks/#section=test&shareid=e8cbd6d2-802d-4e44-9537-d6328dff022f))

## Quickstart

Install dependencies

-   [Docker](https://docs.docker.com/)
-   [Docker Compose](https://docs.docker.com/compose/)
-   [Rust](https://www.rust-lang.org/)

```shell
# Check dependencies are installed
docker --version
docker-compose --version
cargo --version
# DEPEND: Install cargo-make
# <https://crates.io/crates/cargo-make>
cargo install --force cargo-make --version "~0.32"
cargo make --version
```

Build all docker images and generated outputs

```shell
cargo make dist-build
```

Run using docker-compose

```shell
cargo make compose build
cargo make compose up
cargo make compose down
```

Open the client playground at <http://localhost:1234>

## Developer

How to use this repository as a template

-   Clone this repository
-   Find and replace `petshop`, `Petshop` and `mojzu` to rename outputs
-   Change `LICENCE` file

How to change the API

-   Update protobuf definitions in `proto/proto/api.proto` file
-   Implement methods in `server/src/api.rs` file (run `cargo build` to check)
-   Restart `dev-*` tasks or run `cargo make dist-build` to rebuild

How to run development tasks

```shell
# Run envoy and postgres docker images in host networking mode
cargo make dev-envoy
cargo make dev-postgres

# Run server binary with cargo or docker image
cargo make dev-server
cargo make dev-server-release

# Run job of name with cargo
cargo make dev-job $NAME

# Run client playground in development mode
cargo make dev-client-playground

# Run integration test in development mode
cargo make dev-integration-test

# Set version numbers, tag versions in format `vX.Y.Z`
cargo make dist-version $VERSION
git tag v$VERSION
```

These labels are used in the source code

-   `TODO: ...` Something to fix/feature to add/ideas/etc.
-   `FIXME: ...` Something that has been fixed in an unintuitive way/may require manual intervention
-   `DEPEND: ...` A project dependency that needs updating occasionally
