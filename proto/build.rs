fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/api.proto")?;
    Ok(())
}

// TODO: Run binary in docker image as user, allow override to local
// TODO: Github actions to build docker images with versions (cd.yml?)
// TODO: Docker compose test suite (for dev and CI?)
