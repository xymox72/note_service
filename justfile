build:
    cargo build --release

test:
    cargo nextest run --all-targets

lint:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all -- --check