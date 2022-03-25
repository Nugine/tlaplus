dev:
    cargo check
    cargo clippy
    cargo build --release
    cargo test

install:
    cargo install --path crates/tlaplus-cli --offline

run:
    cargo run -p tlaplus-cli --release -q
