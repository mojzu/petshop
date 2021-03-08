# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-   Add ngx-grpc client generation
-   Add release checklist to README
-   Add more opencontainers annotations to output server and envoy images
-   Remove clients package, copying the generated files would work better/be less complex. Planning to replace this with an archive of client source files for each tag
-   Add `ghcr` example
-   Fix `tracing::instrument` not working in tonic async trait

## [0.2.2] - 2021-03-06

-   Include generated `api.swagger.json` file in clients package
-   Add `cron` example for periodic jobs
-   Fix using `target-cpu=native` option in docker image builds caused runtime failures

## [0.2.1] - 2021-03-05

-   Tag and push to test CI workflow produces packages with the expected versions
