use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

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
    /// Generate Swift and Kotlin bindings for app-mobile
    GenerateBindings,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateBindings => generate_bindings(),
    }
}

fn generate_bindings() -> Result<()> {
    let ios_device = format!("target/aarch64-apple-ios/debug/{LIB}.a");
    let ios_sim = format!("target/aarch64-apple-ios-sim/debug/{LIB}.a");

    // Build for host (dylib for Kotlin binding generation)
    run("cargo", &["build", "-p", "app-mobile"])?;

    // Build for iOS device
    run(
        "cargo",
        &["build", "-p", "app-mobile", "--target", "aarch64-apple-ios"],
    )?;

    // Build for iOS simulator
    run(
        "cargo",
        &[
            "build",
            "-p",
            "app-mobile",
            "--target",
            "aarch64-apple-ios-sim",
        ],
    )?;

    // Generate Kotlin bindings from host dylib
    let kotlin_out = format!("{PKG}/android/src/main/java");
    run(
        "cargo",
        &[
            "uniffi-bindgen",
            "generate",
            "--library",
            &format!("target/debug/{LIB}.dylib"),
            "--language",
            "kotlin",
            "--out-dir",
            &kotlin_out,
            "--no-format",
        ],
    )?;

    // Clean old artifacts
    let swift_out = format!("{PKG}/ios/uniffi");
    let _ = std::fs::remove_dir_all(&swift_out);

    // Generate Swift bindings from iOS simulator library
    run(
        "cargo",
        &[
            "uniffi-bindgen",
            "generate",
            "--library",
            &ios_sim,
            "--language",
            "swift",
            "--out-dir",
            &swift_out,
        ],
    )?;

    // Create xcframework combining device and simulator libraries
    run(
        "xcodebuild",
        &[
            "-create-xcframework",
            "-library",
            &ios_device,
            "-library",
            &ios_sim,
            "-output",
            &format!("{PKG}/ios/uniffi/AppMobile.xcframework"),
        ],
    )?;

    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("failed to execute: {cmd} {}", args.join(" ")))?;

    if !status.success() {
        bail!("{cmd} {} exited with {status}", args.join(" "));
    }

    Ok(())
}
