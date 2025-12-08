#! /bin/sh

# Check and test the library.

cargo check && cargo check --features metrics && cargo check --features optimizer && cargo check --features draws && cargo check --features draws,metrics;
cargo test && cargo test --features metrics && cargo test --features optimizer && cargo test --features draws && cargo test --features draws,metrics;
