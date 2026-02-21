# Project Context

## Rust
- Rust version: **1.93.1** (pinned in `rust-toolchain.toml`)
- Prefer `std::sync::LazyLock` or `std::sync::OnceLock` over `lazy_static` or `once_cell`

## Workspace
- Libraries live under `crates/` (`cargo new --lib crates/<name>`)
- Binaries live under `bins/` (`cargo new --bin bins/<name>`)

## Verification
After making code changes run `just verify`, which runs in order:
1. `cargo check --all-targets`
2. `cargo clippy --all-targets`
3. `cargo fmt --check`
4. `cargo nextest run`
