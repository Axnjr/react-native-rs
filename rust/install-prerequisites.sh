#!/usr/bin/env bash

set -euo pipefail

echo "Installing Rust prerequisites for React Native development..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Install cargo-ndk for Android builds
if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
else
    echo "cargo-ndk is already installed"
fi

# Install required Rust targets
echo "Installing Rust targets..."

# iOS targets
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

# Android targets
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android

echo "Prerequisites installed successfully!"
echo ""
echo "Next steps:"
echo "1. Make sure you have Xcode installed (for iOS builds)"
echo "2. Make sure you have Android NDK installed and ANDROID_HOME set (for Android builds)"
echo "3. Run 'bash build.sh' to build the Rust libraries"
