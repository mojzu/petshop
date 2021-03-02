# tfb

- <https://github.com/TechEmpower/FrameworkBenchmarks>

```shell
cargo make dist-build

# Clone TFB repository
git clone https://github.com/TechEmpower/FrameworkBenchmarks.git
# Copy this directory to FrameworkBenchmarks/frameworks/Rust

# Start server service in background
cd examples/tfb
docker-compose up

# Verify and run TFB tests
cd FrameworkBenchmarks
./tfb --mode verify --test envoy-tonic
./tfb --test envoy-tonic hyper warp-rust [django ...]

# Upload results.json at <https://tfb-status.techempower.com/share>
```
