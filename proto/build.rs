fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/api.proto")?;
    Ok(())
}

// TODO: Github actions to build docker images with versions (cd.yml?)
// TODO: Docker compose test suite (for dev and CI?)
// TODO: Way of testing typescript clients, parcel for open in browser/run tests using TS?
// TODO: Change envoy config to support grpc-web and transcoding? Test this works
// TODO: Prometheus, Kubernetes endpoints, other best practices?
// TODO: Auth integrations, mtls and other options using envoy?
// TODO: Database integrations, migrations crate/binary?
// TODO: Rust docs output in dist? cargo make --no-workspace docs-flow
// TODO: Use alpine image for server docker image? Research podman?
// TODO: Running in k8s/nomad examples?
// TODO: Openapi doc generator from specification?
// TODO: Add examples for rust tests/examples/benches?
// TODO: More comments/explanation in proto and server files?
