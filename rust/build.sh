#!/usr/bin/env bash

set -euxo pipefail

cleanup() {
  rm -rf dist target react-native-rs.xcframework
}
trap cleanup EXIT

EAS_BUILD_PLATFORM=${EAS_BUILD_PLATFORM:-""}

# Full clean build if requested
if [[ "$@" == *"--full-clean"* ]]; then
  echo "Doing a clean build..."
  (
    set +e
    set -x
    cargo clean
    set -e
  )
  rm -rf react-native-rs.xcframework
  rm -rf ../cpp/target
fi

# Optimize for all builds
export RUSTFLAGS='-C embed-bitcode=yes -C lto=on'

# iOS Build
if [[ "$EAS_BUILD_PLATFORM" == "ios" || -z "$EAS_BUILD_PLATFORM" ]]; then
  echo "Building for iOS..."
  
  # Install iOS targets if not present
  rustup target add aarch64-apple-ios
  rustup target add aarch64-apple-ios-sim
  rustup target add x86_64-apple-ios
  
  # Build for iOS device (ARM64)
  IPHONEOS_DEPLOYMENT_TARGET=13.4 cargo build --release --target aarch64-apple-ios
  
  # Build for iOS simulator (ARM64)
  IPHONEOS_DEPLOYMENT_TARGET=13.4 cargo build --release --target aarch64-apple-ios-sim
  
  # Build for iOS simulator (x86_64) - for Intel Macs
  IPHONEOS_DEPLOYMENT_TARGET=13.4 cargo build --release --target x86_64-apple-ios
  
  # Create XCFramework
  mkdir -p ios-device ios-simulator
  
  cp target/aarch64-apple-ios/release/libreact_native_rs.a ios-device/
  
  # Create universal simulator binary
  lipo -create \
    target/aarch64-apple-ios-sim/release/libreact_native_rs.a \
    target/x86_64-apple-ios/release/libreact_native_rs.a \
    -output ios-simulator/libreact_native_rs.a
  
  xcodebuild -create-xcframework \
    -library ios-device/libreact_native_rs.a \
    -library ios-simulator/libreact_native_rs.a \
    -output react-native-rs.xcframework
  
  # Copy to target directory
  mkdir -p ../cpp/target
  cp -rf react-native-rs.xcframework ../cpp/target/
fi

# Android Build
if [[ "$EAS_BUILD_PLATFORM" == "android" || -z "$EAS_BUILD_PLATFORM" ]]; then
  echo "Building for Android..."
  
  # Check for Android NDK
  ANDROID_HOME=${ANDROID_HOME:-""}
  if [[ -z "${ANDROID_HOME}" ]]; then
    echo "ANDROID_HOME not set. Aborting..."
    exit 1
  fi
  
  if [[ ! -d "${ANDROID_HOME}" ]]; then
    echo "ANDROID_HOME directory does not exist. Aborting..."
    exit 1
  fi
  
  # Find latest NDK version
  LATEST_NDK_VERSION=$(ls -f "${ANDROID_HOME}"/ndk | sort -V | tail -n 1)
  echo "Latest NDK version = ${LATEST_NDK_VERSION}"
  
  ANDROID_NDK_HOME="${ANDROID_HOME}/ndk/${LATEST_NDK_VERSION}"
  export ANDROID_NDK_HOME
  
  if [[ ! -d "${ANDROID_NDK_HOME}" ]]; then
    echo "No NDK installation found. Aborting..."
    exit 1
  fi
  
  # Install cargo-ndk if not present
  if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
  fi
  
  # Install Android targets
  rustup target add aarch64-linux-android
  rustup target add x86_64-linux-android
  
  # Set minimum SDK version
  ANDROID_MIN_SDK_VERSION=21
  
  # Build for Android ARM64
  cargo ndk --target aarch64-linux-android --platform ${ANDROID_MIN_SDK_VERSION} -- build --release
  
  # Build for Android x86_64 (emulator)
  cargo ndk --target x86_64-linux-android --platform ${ANDROID_MIN_SDK_VERSION} -- build --release
  
  # Copy libraries to target directory
  mkdir -p ../cpp/target/aarch64-linux-android/release
  mkdir -p ../cpp/target/x86_64-linux-android/release
  
  cp target/aarch64-linux-android/release/libreact_native_rs.a ../cpp/target/aarch64-linux-android/release/
  cp target/x86_64-linux-android/release/libreact_native_rs.a ../cpp/target/x86_64-linux-android/release/
fi

echo "Build completed successfully!"
