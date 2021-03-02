fn main() {
    tonic_build::configure()
        .compile(&["proto/api.proto"], &["proto"])
        .expect("tonic_build failed");
}
