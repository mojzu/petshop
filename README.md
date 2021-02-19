# petshop

[![licence](https://img.shields.io/github/license/mojzu/petshop?label=licence)](https://github.com/mojzu/petshop/blob/master/LICENCE)
[![CI](https://github.com/mojzu/petshop/workflows/CI/badge.svg?branch=main)](https://github.com/mojzu/petshop/actions/workflows/ci.yml)

Template for Rust API server with gRPC and OpenAPI (V2) interfaces

## Features

-   Rust gRPC server using [tonic](https://github.com/hyperium/tonic)
-   Envoy proxy with [gRPC-JSON transcoder](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/grpc_json_transcoder_filter)
-   Builds [Docker](https://docs.docker.com/reference/) images for gRPC server and Envoy proxy
-   [Generated OpenAPI (V2) definitions](https://github.com/grpc-ecosystem/grpc-gateway) from gRPC `.proto` files
-   Generated TypeScript [axios](https://github.com/axios/axios) and [gRPC Web](https://github.com/grpc/grpc-web) clients
-   Continuous integration using [GitHub Actions](https://github.com/features/actions) and [cargo-make](https://github.com/sagiegurari/cargo-make)
-   Example proto definitions based (loosely) on [OpenAPI (V2) Petstore](https://petstore.swagger.io/#/)
-   [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for multiple crates

## Template

How to use this template:

-   Clone this repository
-   Find and replace `petshop` and `Petshop` to rename outputs
-   Change `LICENCE` file
-   Update `proto/proto/api.proto` to change API
-   Implement aysnc trait methods in `server` crate
-   Run `cargo make dist-flow` to build everything

## Quickstart

```shell
# Dependencies
# Install Docker, Docker Compose, Rust stable
docker --version
docker-compose --version
cargo --version
# depend: Install cargo-make
# <https://crates.io/crates/cargo-make>
cargo install --force cargo-make --version "~0.32"
cargo make --version

# Run distribution tasks
cargo make dist-flow

# Run development tasks
cargo make dev-envoy
cargo make dev-server

# Make HTTP requests
curl -v --header "Content-Type: application/json" \
  -XPOST --data '{id:32,name:"Name1",category:{id:23,name:"Cat1"},photoUrls:["Photo1"],tags:[{id:45,name:"Tag1"}],status:"PENDING"}' \
  localhost:10000/petshop.Petshop/PetPut
```

## Notes

The following labels are used:

- `TODO: ...` Something to fix/feature to add/ideas/etc.
- `FIXME: ...` Something that has been fixed in an unintuitive way/may require manual intervention
- `DEPEND: ...` A project dependency that needs updating occasionally
