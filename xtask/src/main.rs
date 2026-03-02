use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

static ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    #[allow(clippy::expect_used)]
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be in a subdirectory of the workspace root")
        .to_path_buf()
});

const PKG: &str = "packages/app-mobile";
const LIB: &str = "libapp_mobile";

#[derive(Parser)]
#[command(name = "xtask", about = "Development automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Swift and Kotlin bindings from the host library (works on Linux)
    GenerateBindings,
    /// Build iOS libraries and create xcframework (requires macOS)
    BuildIos {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Build Android shared libraries via cargo-ndk
    BuildAndroid {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateBindings => generate_bindings(),
        Commands::BuildIos { release } => build_ios(release),
        Commands::BuildAndroid { release } => build_android(release),
    }
}

fn generate_bindings() -> Result<()> {
    let host_lib = ROOT.join(format!("target/debug/{LIB}"));

    // Build for host (staticlib for Swift, dylib for Kotlin)
    run(Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("app-mobile"))?;

    // Generate Swift bindings from host staticlib
    let swift_out = ROOT.join(format!("{PKG}/ios/uniffi"));
    let _ = std::fs::remove_dir_all(&swift_out);
    run(Command::new("cargo")
        .arg("uniffi-bindgen-swift")
        .arg(host_lib.with_extension("a"))
        .arg(&swift_out)
        .arg("--swift-sources")
        .arg("--headers"))?;

    // Generate Kotlin bindings from host dylib
    let kotlin_out = ROOT.join(format!("{PKG}/android/src/main/java"));
    run(Command::new("cargo")
        .arg("uniffi-bindgen")
        .arg("generate")
        .arg("--library")
        .arg(host_lib.with_extension(std::env::consts::DLL_EXTENSION))
        .arg("--language")
        .arg("kotlin")
        .arg("--out-dir")
        .arg(&kotlin_out)
        .arg("--no-format"))?;

    Ok(())
}

fn build_ios(release: bool) -> Result<()> {
    let profile = if release { "release" } else { "debug" };
    let ios_device = ROOT.join(format!("target/aarch64-apple-ios/{profile}/{LIB}.a"));
    let ios_sim = ROOT.join(format!("target/aarch64-apple-ios-sim/{profile}/{LIB}.a"));

    // Build for iOS device and simulator
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("-p")
        .arg("app-mobile")
        .arg("--target")
        .arg("aarch64-apple-ios")
        .arg("--target")
        .arg("aarch64-apple-ios-sim");
    if release {
        cmd.arg("--release");
    }
    run(&mut cmd)?;

    // Create xcframework combining device and simulator libraries
    let xcframework = ROOT.join(format!("{PKG}/ios/AppMobile.xcframework"));
    let _ = std::fs::remove_dir_all(&xcframework);
    run(Command::new("xcodebuild")
        .arg("-create-xcframework")
        .arg("-library")
        .arg(&ios_device)
        .arg("-library")
        .arg(&ios_sim)
        .arg("-output")
        .arg(&xcframework))?;

    Ok(())
}

fn build_android(release: bool) -> Result<()> {
    let jni_libs = ROOT.join(format!("{PKG}/android/src/main/jniLibs"));
    let _ = std::fs::remove_dir_all(&jni_libs);

    let mut cmd = Command::new("cargo");
    cmd.arg("ndk")
        .args(["--output-dir", &jni_libs.to_string_lossy()])
        .args(["--target", "aarch64-linux-android"])
        .args(["--target", "armv7-linux-androideabi"])
        .args(["--target", "x86_64-linux-android"])
        .args(["--target", "i686-linux-android"])
        .args(["build", "--package", "app-mobile"]);
    if release {
        cmd.arg("--release");
    }
    run(&mut cmd)?;

    Ok(())
}

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("failed to execute: {cmd:?}"))?;

    if !status.success() {
        bail!("{cmd:?} exited with {status}");
    }

    Ok(())
}
