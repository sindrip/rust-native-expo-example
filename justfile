verify:
    cargo check --all-targets
    cargo clippy --all-targets
    cargo fmt --check
    cargo nextest run
