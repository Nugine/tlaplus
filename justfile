dev:
    cargo fmt
    cargo check
    cargo clippy
    cargo build --release
    cargo test
    ls -lh target/release/tlaplus-cli

install:
    cargo install --path crates/tlaplus-cli --offline
