# Architecture Overview

This document provides a detailed overview of the `react-native-rs` architecture and design decisions.

## 🏗 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    React Native App                         │
├─────────────────────────────────────────────────────────────┤
│                TypeScript Bridge Layer                     │
│  • Type-safe command interface                             │
│  • Automatic serialization/deserialization                │
│  • Error handling and panic recovery                       │
├─────────────────────────────────────────────────────────────┤
│                 Platform Native Layer                      │
│  iOS (Objective-C++)        │        Android (Java/JNI)   │
│  • TurboModule integration  │        • TurboModule support │
│  • Background thread exec   │        • Native library load │
├─────────────────────────────────────────────────────────────┤
│                    C++ Bridge Layer                        │
│  • Cross-platform C interface                             │
│  • Memory management                                       │
│  • String marshaling                                       │
├─────────────────────────────────────────────────────────────┤
│                     Rust Core                              │
│  • High-performance computation                            │
│  • Async/await support                                     │
│  • Parallel processing with Rayon                          │
│  • Comprehensive error handling                            │
│  • Structured logging                                      │
└─────────────────────────────────────────────────────────────┘
```

## 🔄 Data Flow

### 1. Command Execution Flow

```typescript
// 1. TypeScript call
const result = await RustBridge.execute({
  cmd: 'fibonacci',
  params: { n: 40 }
});

// 2. Serialization
const cmdString = JSON.stringify(command);

// 3. Native module call (iOS/Android)
const resultString = await ReactNativeRs.execute(cmdString);

// 4. C++ bridge
const char* rust_execute(const char* cmd);

// 5. Rust execution
pub extern "C" fn rust_execute(raw_cmd: *const c_char) -> *const c_char

// 6. Command parsing and execution
let cmd = serde_json::from_str::<Command>(cmd_str)?;
let result = execute_cmd(cmd).await?;

// 7. Result serialization and return
serde_json::to_string(&result)?
```

### 2. Error Handling Flow

```rust
// Rust panic recovery
let exec_res = panic::catch_unwind(|| {
    RUNTIME.block_on(async move {
        execute_cmd(cmd).await
    })
});

match exec_res {
    Ok(res) => res,
    Err(panic) => handle_panic(panic, cmd),
}
```

## 🛠 Key Design Decisions

### 1. Command Pattern Architecture

**Decision**: Use a centralized command enum with tagged unions.

**Rationale**:
- Type safety across the Rust/TypeScript boundary
- Easy to extend with new operations
- Centralized error handling and logging
- Automatic TypeScript type generation

```rust
#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "cmd", content = "params")]
pub enum Command {
    Fibonacci(FibonacciInput),
    HashData(HashDataInput),
    // ... more commands
}
```

### 2. Async Runtime Management

**Decision**: Use a global Tokio runtime with multi-threading.

**Rationale**:
- Non-blocking operations for React Native UI
- Efficient resource utilization
- Platform-specific thread pool sizing

```rust
lazy_static! {
    pub static ref RUNTIME: Arc<Runtime> =
        Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());
}
```

### 3. Memory Management Strategy

**Decision**: Use owned strings with explicit memory management.

**Rationale**:
- Avoid memory leaks across FFI boundary
- Clear ownership semantics
- Safe string marshaling

```rust
pub extern "C" fn rust_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { let _ = CString::from_raw(ptr); };
    }
}
```

### 4. Type Generation System

**Decision**: Automatic TypeScript type generation from Rust structs.

**Rationale**:
- Eliminates type mismatches at compile time
- Reduces maintenance overhead
- Ensures API consistency

```rust
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FibonacciInput {
    pub n: u32,
}
```

## 🚀 Performance Optimizations

### 1. Parallel Processing

- **Rayon**: Used for CPU-intensive operations like sorting and prime generation
- **Thread Pool**: Optimized for each platform (Android uses N-1 cores)
- **Work Stealing**: Efficient load balancing across cores

### 2. Memory Efficiency

- **Zero-Copy**: Where possible, avoid unnecessary data copying
- **String Interning**: Efficient string handling across FFI
- **Stack Allocation**: Prefer stack over heap for small data structures

### 3. Build Optimizations

- **LTO**: Link-time optimization enabled for release builds
- **Embed Bitcode**: iOS compatibility for App Store submissions
- **Target-Specific**: Platform-specific optimizations

## 🔒 Safety & Error Handling

### 1. Panic Recovery

All Rust code is wrapped in `panic::catch_unwind` to prevent crashes:

```rust
let exec_res = panic::catch_unwind(|| {
    // Rust execution code
});

match exec_res {
    Ok(res) => res,
    Err(panic) => handle_panic_gracefully(panic),
}
```

### 2. Input Validation

- JSON schema validation at the Rust boundary
- Type-safe deserialization with `serde`
- Graceful error messages for invalid inputs

### 3. Resource Management

- Automatic cleanup of native resources
- Proper thread lifecycle management  
- Memory leak prevention with RAII patterns

## 📱 Platform Integration

### iOS Implementation

- **TurboModule**: Full new architecture support with fallback
- **XCFramework**: Universal binary for device and simulator
- **CocoaPods**: Seamless integration with existing projects

### Android Implementation

- **JNI**: Efficient Java-to-native communication
- **CMake**: Cross-compilation build system
- **NDK**: Multiple architecture support (ARM64, x86_64)

## 🔧 Build System

### Cross-Platform Build Script

The `build.sh` script handles:
- Platform detection and configuration
- Target installation and compilation
- Library packaging and distribution
- Error handling and cleanup

### Dependency Management

- **Cargo**: Rust dependency management with workspaces
- **npm/yarn**: JavaScript package management
- **CocoaPods**: iOS native dependency management
- **Gradle**: Android build system integration

## 📊 Monitoring & Debugging

### Structured Logging

- **Tracing**: Hierarchical structured logging
- **Log Collection**: In-memory log storage for debugging
- **Performance Metrics**: Automatic timing and profiling

### Development Tools

- **Type Checking**: Automatic verification of Rust/TS type sync
- **Testing**: Comprehensive unit and integration tests
- **Linting**: Code quality enforcement across all languages

## 🔮 Future Extensibility

The architecture is designed for easy extension:

1. **New Commands**: Add to the `Command` enum and implement handler
2. **New Platforms**: Extend the build system and native modules
3. **New Features**: Leverage the existing infrastructure
4. **Performance**: Profile and optimize individual components

This architecture provides a solid foundation for high-performance React Native applications while maintaining type safety, error resilience, and cross-platform compatibility.
