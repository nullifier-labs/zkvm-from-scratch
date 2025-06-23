# Development Environment Setup

This document describes the development environment setup for the zkvm-from-scratch project.

## Prerequisites

- **Rust**: Nightly toolchain required
- **WebAssembly targets**: For compilation to WASM
- **Just**: Command runner for task automation
- **Git**: Version control

## Quick Setup

Run the following commands to set up your development environment:

    ```bash
    # Install Rust if not already installed
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
    
    # Install nightly toolchain and set as default
    rustup toolchain install nightly
    rustup default nightly
    
    # Add WebAssembly targets
    rustup target add wasm32-unknown-unknown
    rustup target add wasm32-wasip1
    
    # Install Just command runner
    cargo install just
    
    # Install additional development tools
    cargo install cargo-watch
    cargo install wasm-pack
    ```

## Rust Toolchain Configuration

The project uses Rust nightly for:
- Latest language features
- Experimental WASM optimizations
- Advanced const evaluation features

## WebAssembly Targets

We support two WASM targets:
- `wasm32-unknown-unknown`: For browser and standalone WASM
- `wasm32-wasip1`: For WASI-compatible environments

## Project Structure

    ```
    zkvm-from-scratch/
    ├── src/                # Core library source
    ├── examples/           # Example implementations
    ├── tests/              # Test suites
    ├── benches/           # Benchmarks
    ├── docs/              # Documentation
    ├── .github/           # CI/CD workflows
    ├── justfile           # Task automation
    └── Cargo.toml         # Project configuration
    ```

## Development Workflow

Use the `justfile` for common development tasks:

    ```bash
    # See all available tasks
    just --list
    
    # Run tests
    just test
    
    # Build all targets
    just build
    
    # Build for WASM
    just build-wasm
    
    # Run benchmarks
    just bench
    
    # Format code
    just fmt
    
    # Run linter
    just lint
    
    # Run continuous integration checks
    just ci
    ```

## IDE Setup

### VS Code
Install the following extensions:
- `rust-analyzer`: Rust language server
- `CodeLLDB`: Debugging support
- `WebAssembly`: WASM file support

### Configuration
Create `.vscode/settings.json`:

    ```json
    {
        "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
        "rust-analyzer.checkOnSave.command": "clippy",
        "rust-analyzer.cargo.features": "all"
    }
    ```

## Troubleshooting

### Common Issues

1. **Nightly toolchain not found**
   - Ensure nightly is installed: `rustup toolchain install nightly`
   - Set as default: `rustup default nightly`

2. **WASM target missing**
   - Add target: `rustup target add wasm32-unknown-unknown`

3. **Just command not found**
   - Install via cargo: `cargo install just`
   - Or via package manager (Ubuntu): `sudo apt install just`

### Environment Verification

Run the following to verify your setup:

    ```bash
    # Check Rust version (should show nightly)
    rustc --version
    
    # Check available targets
    rustup target list --installed
    
    # Check Just installation
    just --version
    
    # Verify WASM compilation
    echo 'fn main() {}' > test.rs
    rustc --target wasm32-unknown-unknown test.rs
    rm test.rs test.wasm
    ```