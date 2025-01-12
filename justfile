dev:
    cargo fmt
    cargo check
    cargo clippy
    cargo build --release
    cargo test --release
    ls -lh target/release/tlaplus-cli

install:
    cargo install --path . --offline
