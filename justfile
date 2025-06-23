# zkvm-from-scratch justfile
# Task automation for the zkvm project

# Default recipe to display available commands
default:
    @just --list

# Install development dependencies
setup:
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown
    rustup target add wasm32-wasip1
    cargo install cargo-watch
    cargo install wasm-pack
    cargo install cargo-expand
    cargo install cargo-audit

# Build the project
build:
    cargo build

# Build with release optimizations
build-release:
    cargo build --release

# Build for WebAssembly target
build-wasm:
    cargo build --target wasm32-unknown-unknown
    cargo build --target wasm32-wasip1

# Build WebAssembly with wasm-pack
build-wasm-pack:
    wasm-pack build --target web --out-dir pkg

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run tests for specific target
test-wasm:
    cargo test --target wasm32-unknown-unknown

# Run benchmarks
bench:
    cargo bench

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Run linter
lint:
    cargo clippy -- -D warnings

# Run linter with all targets
lint-all:
    cargo clippy --all-targets -- -D warnings

# Fix linter issues automatically
lint-fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Security audit
audit:
    cargo audit

# Clean build artifacts
clean:
    cargo clean
    rm -rf pkg/

# Generate documentation
docs:
    cargo doc --no-deps --open

# Generate documentation for all features
docs-all:
    cargo doc --all-features --no-deps

# Check for updates
update:
    cargo update

# Expand macros (useful for debugging)
expand *ARGS:
    cargo expand {{ARGS}}

# Watch for changes and run tests
watch-test:
    cargo watch -x test

# Watch for changes and run checks
watch-check:
    cargo watch -x check

# Watch for changes and run clippy
watch-lint:
    cargo watch -x clippy

# Development server with auto-reload
dev:
    cargo watch -x "build --target wasm32-unknown-unknown"

# Run continuous integration checks locally
ci: fmt-check lint test build-wasm

# Full CI pipeline including benchmarks
ci-full: fmt-check lint-all test test-wasm build build-wasm bench audit

# Profile build (for performance analysis)
profile:
    cargo build --release
    @echo "Built with release profile for performance analysis"

# Size optimization build
build-size:
    RUSTFLAGS="-C opt-level=z -C lto=fat -C codegen-units=1" cargo build --release --target wasm32-unknown-unknown

# Generate flamegraph (requires perf and flamegraph)
flamegraph *ARGS:
    cargo flamegraph {{ARGS}}

# Install pre-commit hooks
install-hooks:
    @echo "Installing pre-commit hooks..."
    @echo '#!/bin/sh\nexec just ci' > .git/hooks/pre-commit
    @chmod +x .git/hooks/pre-commit
    @echo "Pre-commit hooks installed"

# Remove pre-commit hooks
remove-hooks:
    rm -f .git/hooks/pre-commit
    @echo "Pre-commit hooks removed"

# Check dependencies for known vulnerabilities
check-deps:
    cargo audit

# Update Rust toolchain
update-rust:
    rustup update

# Show project statistics
stats:
    @echo "Project Statistics:"
    @echo "=================="
    @echo "Lines of code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo "Number of files:"
    @find src -name "*.rs" | wc -l
    @echo "Cargo dependencies:"
    @cargo tree --depth 1 | wc -l

# Create a new release build with size optimization
release: clean
    @echo "Building optimized release..."
    RUSTFLAGS="-C opt-level=z" cargo build --release
    RUSTFLAGS="-C opt-level=z" cargo build --release --target wasm32-unknown-unknown
    @echo "Release build complete"
    ls -lh target/release/ target/wasm32-unknown-unknown/release/

# Verify the development environment is properly set up
verify-env:
    @echo "Verifying development environment..."
    @echo "Rust version:"
    @rustc --version
    @echo "Cargo version:"
    @cargo --version
    @echo "Installed targets:"
    @rustup target list --installed
    @echo "Just version:"
    @just --version
    @echo "Environment verification complete ✓"