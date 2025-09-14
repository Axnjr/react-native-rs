# Contributing to react-native-rs

Thank you for your interest in contributing to react-native-rs! This guide will help you get started.

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Axnjr/react-native-rs
   cd react-native-rs
   ```

2. **Install dependencies:**
   ```bash
   yarn install
   ```

3. **Set up Rust environment:**
   ```bash
   yarn setup-rust
   ```

4. **Build the project:**
   ```bash
   yarn build-all
   ```

## Project Structure

```
react-native-rs/
├── src/                    # TypeScript bridge code
├── rust/                   # Rust implementation
│   ├── src/
│   │   ├── commands/      # Individual command implementations
│   │   ├── lib.rs         # Main library entry point
│   │   └── cmd.rs         # Command definitions
│   ├── build.sh           # Cross-platform build script
│   └── Cargo.toml         # Rust dependencies
├── ios/                    # iOS native module
├── android/                # Android native module
├── cpp/                    # C++ bridge layer
└── example/                # Example React Native app
```

## Adding New Commands

To add a new command to the bridge:

1. **Create a new command module in Rust:**
   ```rust
   // rust/src/commands/my_command.rs
   use serde::{Deserialize, Serialize};
   use eyre::Result;
   
   #[cfg(feature = "ts-rs")]
   use ts_rs::TS;
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[cfg_attr(feature = "ts-rs", derive(TS))]
   #[cfg_attr(feature = "ts-rs", ts(export))]
   pub struct MyCommandInput {
       pub value: i32,
   }
   
   #[derive(Debug, Serialize, Deserialize)]
   #[cfg_attr(feature = "ts-rs", derive(TS))]
   #[cfg_attr(feature = "ts-rs", ts(export))]
   pub struct MyCommandResult {
       pub result: i32,
   }
   
   pub async fn my_command(input: &MyCommandInput) -> Result<MyCommandResult> {
       // Your implementation here
       Ok(MyCommandResult {
           result: input.value * 2,
       })
   }
   ```

2. **Add to the command enum:**
   ```rust
   // rust/src/cmd.rs
   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[cfg_attr(feature = "ts-rs", derive(TS))]
   #[cfg_attr(feature = "ts-rs", ts(export))]
   #[serde(tag = "cmd", content = "params", rename_all = "snake_case")]
   pub enum Command {
       // ... existing commands
       MyCommand(MyCommandInput),
   }
   ```

3. **Add to the executor:**
   ```rust
   // rust/src/cmd.rs
   pub async fn execute_cmd(cmd: Arc<Command>, logs: &'static Mutex<Vec<String>>) -> Result<String, eyre::Error> {
       match &*cmd {
           // ... existing cases
           Command::MyCommand(input) => parse_result(my_command(input).await?),
       }
   }
   ```

4. **Generate TypeScript types:**
   ```bash
   yarn generate-types
   ```

5. **Use in React Native:**
   ```typescript
   const result = await RustBridge.execute({
     cmd: 'my_command',
     params: { value: 42 }
   });
   ```

## Testing

### Rust Tests
```bash
cd rust
cargo test
```

### TypeScript Tests
```bash
yarn test
```

### Example App
```bash
yarn example:ios
# or
yarn example:android
```

## Code Style

- **Rust**: Follow standard Rust formatting with `cargo fmt`
- **TypeScript**: We use Prettier for formatting
- **Commit Messages**: Use conventional commit format

## Performance Guidelines

1. **Use appropriate algorithms**: Choose the right algorithm for the task
2. **Leverage parallelism**: Use `rayon` for CPU-intensive tasks when beneficial
3. **Memory management**: Be mindful of memory allocations in hot paths
4. **Error handling**: Use `eyre` for comprehensive error context

## Submitting Changes

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run the test suite**: `yarn test && cd rust && cargo test`
6. **Generate types**: `yarn generate-types`
7. **Commit your changes**: `git commit -m "feat: add my feature"`
8. **Push to your fork**: `git push origin feature/my-feature`
9. **Create a Pull Request**

## Pull Request Guidelines

- **Description**: Clearly describe what your PR does and why
- **Tests**: Include tests for new functionality
- **Documentation**: Update documentation if needed
- **Types**: Ensure TypeScript types are generated and committed
- **Breaking Changes**: Clearly mark any breaking changes

## Release Process

1. Update version in `package.json` and `rust/Cargo.toml`
2. Update `CHANGELOG.md`
3. Generate and commit TypeScript types
4. Create a release PR
5. After merge, tag the release: `git tag v0.x.x`
6. Push tags: `git push --tags`
7. Publish to npm: `npm publish`

## Getting Help

- **Issues**: Open an issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Discord**: Join our Discord server for real-time help

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

Thank you for contributing! 🚀
