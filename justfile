pkg := "packages/expo-app-mobile"
lib := "libapp_mobile"
ios_device := "target/aarch64-apple-ios/debug/" + lib + ".a"
ios_sim := "target/aarch64-apple-ios-sim/debug/" + lib + ".a"

setup:
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim

run *args:
    cargo run -p server {{ args }}

verify:
    cargo check --all-targets
    cargo clippy --all-targets
    cargo fmt --check
    cargo nextest run

generate-bindings:
    cargo build -p app-mobile
    cargo build -p app-mobile --target aarch64-apple-ios
    cargo build -p app-mobile --target aarch64-apple-ios-sim
    cargo run -p uniffi-bindgen -- generate --library target/debug/{{ lib }}.dylib \
        --language kotlin --out-dir {{ pkg }}/android/src/main/java
    cargo run -p uniffi-bindgen -- generate --library {{ ios_sim }} \
        --language swift --out-dir {{ pkg }}/ios
    rm -f {{ pkg }}/ios/{{ lib }}.a
    rm -rf {{ pkg }}/ios/AppMobile.xcframework
    xcodebuild -create-xcframework \
        -library {{ ios_device }} \
        -library {{ ios_sim }} \
        -output {{ pkg }}/ios/AppMobile.xcframework
