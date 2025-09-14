# react-native-rs

A high-performance React Native TurboModule bridge for integrating Rust code with heavy computation logic. This package provides a seamless way to offload CPU-intensive tasks to Rust while maintaining the React Native developer experience.

## ✨ Features

- 🚀 **High Performance**: Leverage Rust's zero-cost abstractions for heavy computations
- 🔄 **Async/Await Support**: Non-blocking operations with proper React Native threading
- 🎯 **Type Safety**: Automatic TypeScript type generation from Rust structs
- 📱 **Cross-Platform**: Seamless iOS and Android support
- 🛠 **Easy Integration**: Simple command-based API
- 🔧 **Production Ready**: Robust error handling and panic recovery
- 📦 **TurboModule**: Built on React Native's new architecture (backwards compatible)

## 🚀 Quick Start

### Installation

```bash
npm install react-native-rs
# or
yarn add react-native-rs
```

### iOS Setup

```bash
cd ios && pod install
```

### Android Setup

No additional setup required - auto-linking handles everything!

### Build Rust Code

```bash
# Install Rust prerequisites (first time only)
yarn setup-rust

# Build Rust libraries for all platforms
yarn build-rust
```

## 📖 Usage

### Basic Example

```typescript
import { RustBridge } from 'react-native-rs';

// Execute heavy computation in Rust
const result = await RustBridge.execute({
  cmd: 'fibonacci',
  params: { n: 40 }
});

console.log('Fibonacci result:', result);
```

### Advanced Example

```typescript
import { RustBridge } from 'react-native-rs';

// Image processing example
const processedImage = await RustBridge.execute({
  cmd: 'image_filter',
  params: {
    data: base64ImageData,
    filter: 'gaussian_blur',
    radius: 5.0
  }
});

// Cryptographic operations
const signature = await RustBridge.execute({
  cmd: 'sign_data',
  params: {
    data: 'message to sign',
    private_key: privateKeyHex
  }
});
```

## 🏗 Architecture

### Command Pattern

All operations use a command-based architecture:

```rust
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Command {
    Fibonacci(FibonacciInput),
    ImageFilter(ImageFilterInput),
    SignData(SignDataInput),
    // Add your custom commands here
}
```

### Type Safety

Types are automatically generated from Rust to TypeScript:

```rust
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FibonacciInput {
    pub n: u32,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FibonacciResult {
    pub value: u64,
    pub computation_time_ms: u64,
}
```

### Error Handling

Robust error handling with panic recovery:

```typescript
try {
  const result = await RustBridge.execute(command);
  // Handle success
} catch (error) {
  // Handle Rust panics and errors gracefully
  console.error('Rust execution failed:', error);
}
```

## 🛠 Development

### Adding New Commands

1. **Define Rust types:**

```rust
// rust/src/fibonacci.rs
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FibonacciInput {
    pub n: u32,
}

pub async fn fibonacci(input: &FibonacciInput) -> Result<u64, eyre::Error> {
    // Your implementation here
}
```

2. **Add to command enum:**

```rust
// rust/src/cmd.rs
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Command {
    Fibonacci(FibonacciInput),
    // ... other commands
}
```

3. **Generate TypeScript types:**

```bash
yarn generate-types
```

4. **Use in React Native:**

```typescript
const result = await RustBridge.execute({
  cmd: 'fibonacci',
  params: { n: 40 }
});
```

### Building

```bash
# Build everything
yarn build-all

# Build only Rust
yarn build-rust

# Build only TypeScript
yarn build
```

### Testing

```bash
# Run TypeScript tests
yarn test

# Check type synchronization
yarn check-types
```

## 📱 Example App

Check out the `example/` directory for a complete React Native app showcasing various use cases:

- Fibonacci computation
- Image processing
- Cryptographic operations
- Real-time data processing

```bash
yarn example:ios
# or
yarn example:android
```

## 🏗 Build System

### iOS

- Uses `cargo-pod` for seamless CocoaPods integration
- Builds universal frameworks for device and simulator
- Supports both old and new React Native architectures

### Android

- Uses `cargo-ndk` for Android NDK integration
- Supports multiple architectures (arm64-v8a, x86_64)
- CMake integration for native library linking

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Clone the repository
2. Install dependencies: `yarn install`
3. Set up Rust: `yarn setup-rust`
4. Build everything: `yarn build-all`
5. Run example: `yarn example:ios`

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Inspired by production React Native + Rust integrations and the need for high-performance mobile computing.

---
