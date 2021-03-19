# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-   Add unit test examples to `proto` and `server` crates
-   Add `swagger-ui` task to makefile, runs Swagger UI with the generated API definitions
-   Refactoring proto crate to support multiple gRPC services
-   Add Sphinx documentation builder
-   Start Helm chart and MicroK8s example development
-   Use Minikube for Helm example, use headless service instead of istio

## [0.2.4] - 2021-03-11

-   Hide internal error details from responses, log them as warnings
-   Add metrics_name config option and error counters
-   Upgrade dependencies
-   Add fortunes test to `tfb` example, re-run benchmarks
-   Add placeholder step in CI action for release artifacts
-   Add allow_origin/allow_origins config option and source origin checking in csrf module

## [0.2.3] - 2021-03-10

-   Add ngx-grpc client generation
-   Add release checklist to README
-   Add more opencontainers annotations to output server and envoy images
-   Remove clients package, copying the generated files would work better/be less complex. Planning to replace this with an archive of client source files for each tag
-   Add `ghcr` example
-   Fix `tracing::instrument` not working in tonic async trait
-   Refactor envoy images to make published images easier to use in example
-   Add CSRF module, service and example
-   Remove `x-xsrf-token` from envoy CORS exposed headers
-   Modify `tfb` example to disable CSRF

## [0.2.2] - 2021-03-06

-   Include generated `api.swagger.json` file in clients package
-   Add `cron` example for periodic jobs
-   Fix using `target-cpu=native` option in docker image builds caused runtime failures

## [0.2.1] - 2021-03-05

-   Tag and push to test CI workflow produces packages with the expected versions
