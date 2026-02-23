setup:
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim

run *args:
    cargo run -p server {{ args }}

verify:
    cargo check --workspace --all-targets
    cargo clippy --workspace --all-targets
    cargo fmt --check
    cargo nextest run --workspace

verify-js:
    npm --workspaces run lint
    CI=true npm --workspaces run test

generate-bindings:
    cargo xtask generate-bindings
