# React Native Rust Bridge Example

This example app demonstrates the capabilities of `react-native-rs` with various heavy computation tasks implemented in Rust.

## Features Demonstrated

- 🔢 **Fibonacci Calculation**: Iterative algorithm with performance timing
- 🔐 **SHA-256 Hashing**: Cryptographic hashing with input validation  
- 🔄 **Parallel Sorting**: Multi-threaded sorting using Rayon
- 🔢 **Prime Number Generation**: Sieve of Eratosthenes with parallel collection
- ⚡ **Performance Benchmarking**: Concurrent execution of multiple operations

## Getting Started

1. **Install dependencies:**
   ```bash
   yarn install
   ```

2. **Build the Rust bridge:**
   ```bash
   cd ..
   yarn setup-rust
   yarn build-rust
   cd example
   ```

3. **Run on iOS:**
   ```bash
   yarn ios
   ```

4. **Run on Android:**
   ```bash
   yarn android
   ```

## What to Try

### Basic Operations
- Calculate Fibonacci numbers (try n=35 for noticeable computation time)
- Hash different text inputs and observe the SHA-256 output
- Sort arrays of various sizes with comma-separated input
- Find prime numbers up to different limits

### Performance Testing
- Use the benchmark button to run multiple operations concurrently
- Compare computation times between different operations
- Try large inputs to see Rust's performance advantages

### Error Handling
- Try invalid inputs to see graceful error handling
- The bridge catches Rust panics and reports them safely

## Architecture

The example demonstrates the complete flow:

1. **React Native UI** → User interaction and result display
2. **TypeScript Bridge** → Command serialization and type safety
3. **Native Modules** → iOS (Objective-C++) and Android (Java/JNI)
4. **C++ Layer** → Cross-platform bridge to Rust
5. **Rust Implementation** → High-performance computation with error handling

## Performance Notes

- All operations run on background threads to keep the UI responsive
- Rust leverages multiple CPU cores for parallel operations
- Memory management is handled efficiently with zero-copy where possible
- Error handling includes panic recovery without crashing the app

## Extending the Example

To add your own commands:

1. Implement the command in the main `react-native-rs` package
2. Generate TypeScript types: `yarn generate-types`
3. Add UI elements in `App.tsx` to call your new command
4. Test with various inputs and edge cases

This example serves as both a demonstration and a testing ground for the bridge functionality.
