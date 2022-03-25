dev:
    cargo check
    cargo clippy
    cargo build --release
    cargo test

install:
    cargo install --path crates/tlaplus-cli --offline
