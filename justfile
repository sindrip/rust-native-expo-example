setup:
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

run *args:
    cargo run -p server {{ args }}

verify:
    cargo check --workspace --all-targets
    cargo clippy --workspace --all-targets
    cargo fmt --check
    cargo nextest run --workspace

verify-js:
    npm --workspaces run lint
    npm --workspaces run format

generate-bindings:
    cargo xtask generate-bindings

build-android:
    cargo xtask build-android
