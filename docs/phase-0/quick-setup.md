# Quick Setup Guide

Get your zkvm-from-scratch development environment up and running in minutes.

## Prerequisites

- Git
- A Unix-like system (Linux, macOS, or WSL on Windows)

## One-Command Setup

Run this single command to set up everything:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    source ~/.cargo/env && \
    rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi && \
    cargo install just wasm-pack cargo-watch cargo-audit
    ```

## Verify Installation

Check that everything is working:

    ```bash
    just verify-env
    ```

## Next Steps

1. **Initialize the project**:
   
    ```bash
    just setup
    ```

2. **Run all checks locally** (simulates CI):
   
    ```bash
    just ci
    ```

3. **Start development with auto-reload**:
   
    ```bash
    just dev
    ```

4. **View all available commands**:
   
    ```bash
    just --list
    ```

## Common Development Commands

    ```bash
    # Build for all targets
    just build
    
    # Run tests
    just test
    
    # Build for WebAssembly
    just build-wasm
    
    # Format and lint code
    just fmt
    just lint
    
    # Generate documentation
    just docs
    
    # Run benchmarks
    just bench
    ```

## Troubleshooting

If you encounter issues:

1. **Check Rust version**: `rustc --version` (should show nightly)
2. **Check targets**: `rustup target list --installed` 
3. **Update toolchain**: `just update-rust`
4. **Clean build**: `just clean && just build`

For detailed troubleshooting, see [dev-environment.md](./dev-environment.md).

## IDE Setup

### VS Code (Recommended)

Install these extensions:
- `rust-analyzer`
- `CodeLLDB`
- `WebAssembly`

The project includes pre-configured settings in `.vscode/settings.json`.

### Other IDEs

The project uses standard Rust tooling, so any Rust-compatible IDE should work with the `rust-toolchain.toml` configuration.

## Ready to Code!

Your development environment is now ready. The CI pipeline will automatically run the same checks you can run locally with `just ci`.

Happy coding! 🚀