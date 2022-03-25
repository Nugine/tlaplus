dev:
    cargo check
    cargo clippy -- -D warnings
    cargo build --release
    cargo test

install:
    cargo install --path crates/tlaplus-cli --offline
