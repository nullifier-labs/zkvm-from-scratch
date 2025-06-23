# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development Workflow
- **Build**: `cargo build` (development) or `cargo build --release` (optimized)
- **Test**: `cargo test` (all tests) or `cargo test -- --nocapture` (with output)
- **Lint**: `cargo clippy -- -D warnings` (strict linting)
- **Format**: `cargo fmt` (code formatting)
- **CI Checks**: `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test`

### WebAssembly Builds
- **WASM**: `cargo build --target wasm32-unknown-unknown`
- **WASM Pack**: `wasm-pack build --target web --out-dir pkg`

### Additional Commands
- **Benchmarks**: `cargo bench`
- **Documentation**: `cargo doc --no-deps --open`
- **Security Audit**: `cargo audit`
- **Expand Macros**: `cargo expand`

## Project Architecture

This is a zero-knowledge virtual machine (zkVM) implementation targeting a subset of the RISC-V instruction set architecture.

### High-Level Components

#### Virtual Machine Core (`src/vm/`)
- **Execution Engine**: RISC-V instruction interpretation and execution
- **Memory Management**: VM memory model with load/store operations
- **State Management**: Program counter, registers, and execution state
- **Target ISA**: RV32IM (32-bit base integer + multiplication/division)

#### Zero-Knowledge Proof System (`src/zkp/`)
- **Proof Generation**: Creates ZK proofs for VM execution traces
- **Verification**: Verifies proofs without revealing execution details
- **Circuit Construction**: Converts VM traces to constraint systems
- **Witness Generation**: Creates circuit witnesses from execution

#### Cryptographic Primitives (`src/crypto/`)
- **Hash Functions**: Cryptographic hashing for commitments
- **Merkle Trees**: Efficient data structures for batch proofs
- **Field Arithmetic**: Mathematical operations in finite fields

#### Utility Functions (`src/utils/`)
- **Encoding/Decoding**: Hex string conversions and data serialization
- **Helper Functions**: Common operations used across modules

### Development Phases

The project follows an 8-phase development plan:
1. **Phase 0**: Research & Environment Setup (COMPLETED)
2. **Phase 1**: RISC-V Execution Model (IN PROGRESS)
3. **Phase 2**: Trace & Constraint System Design
4. **Phase 3**: Prover Implementation
5. **Phase 4**: Verifier Implementation
6. **Phase 5**: Proof Aggregation & Recursion
7. **Phase 6**: Host Integration & API
8. **Phase 7**: Security Audit & Optimization
9. **Phase 8**: Documentation & Release

### Key Architectural Decisions

#### RISC-V ISA Subset
- **Current Target**: RV32IM (32-bit base + multiplication/division)
- **Future Extensions**: Compressed instructions (C), Bit manipulation (B)
- **Rationale**: Balance between functionality and circuit complexity

#### Proof System Integration
- **Modular Design**: Trait-based approach supports multiple proof systems
- **Planned Systems**: STARK, SNARK, potentially others
- **WebAssembly Support**: Full browser compatibility with WASM targets

#### Memory Model
- **Flat Memory**: Simple linear memory addressing
- **Constraint Optimization**: Efficient permutation checks for memory operations
- **Range Checks**: Bounds validation for memory access

### Testing Strategy

#### Unit Tests
- Located in same files as implementation (`#[cfg(test)]`)
- Test individual functions and components
- Focus on correctness of RISC-V instruction semantics

#### Integration Tests
- Located in `tests/` directory
- Test complete execution flows
- Verify end-to-end proof generation and verification

#### Benchmarks
- Located in `benches/` directory using Criterion
- Performance testing for critical paths
- Proof generation and verification timing

#### Property-Based Testing
- Use for cryptographic components
- Verify mathematical properties (commutativity, associativity)
- Test with randomized inputs

### Code Style and Conventions

#### Rust Standards
- **Edition**: Rust 2021
- **Toolchain**: Nightly (required for some features)
- **Linting**: Clippy with strict warnings
- **Formatting**: rustfmt with default settings

#### Naming Conventions
- Functions/variables: `snake_case`
- Types/structs: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

#### Error Handling
- Use `Result<T, E>` for fallible operations
- Custom error types for domain-specific errors
- Avoid `unwrap()` and `expect()` in production code

#### Documentation
- All public items must have documentation
- Include examples in documentation when helpful
- Use triple-slash comments (`///`) for public items

### WebAssembly Integration

The project has full WebAssembly support for browser deployment:
- **Memory Allocator**: `wee_alloc` for size optimization
- **Error Handling**: `console_error_panic_hook` for debugging
- **Bindings**: `wasm-bindgen` for JavaScript interop
- **Build Targets**: Both `wasm32-unknown-unknown` and `wasm32-wasi`

### Security Considerations

#### Cryptographic Code
- Constant-time operations for sensitive functions
- Secure random number generation
- Never log or expose private keys
- All cryptographic code requires thorough review

#### Memory Safety
- Minimize `unsafe` code
- Document safety requirements when `unsafe` is necessary
- Prefer owned types in public APIs
- Use `Arc<T>` and `Rc<T>` judiciously

### Performance Guidelines

#### Optimization Priorities
1. Correctness first - never sacrifice correctness for performance
2. Profile before optimizing - use benchmarks and profiling tools
3. Algorithmic improvements - focus on Big-O complexity first
4. Memory usage - minimize allocations in hot paths

#### Circuit Optimization
- Each RISC-V instruction maps to specific constraint gates
- Multiplication/division circuits are performance-critical
- Memory operations require efficient permutation arguments
- Proof generation time scales with circuit complexity

### Common Development Tasks

#### Adding New RISC-V Instructions
1. Update instruction decoder in `src/vm/opcodes.rs`
2. Implement execution logic in `src/vm/execution.rs`
3. Add constraint gates in `src/circuit/constraints.rs`
4. Update witness generation in `src/circuit/witness.rs`
5. Write unit tests for instruction behavior
6. Update integration tests for full execution

#### Integrating New Proof Systems
1. Implement `ProofSystem` trait in `src/zkp/`
2. Add feature flag in `Cargo.toml`
3. Update circuit construction for new system
4. Add benchmarks for performance comparison
5. Update documentation and examples

#### WebAssembly Development
1. Build with `cargo build --target wasm32-unknown-unknown`
2. Test with `cargo test --target wasm32-unknown-unknown`
3. Package with `wasm-pack build --target web`
4. Deploy from `pkg/` directory

When implementing new features, always consider the impact on circuit complexity and proof generation time. The zkVM's performance characteristics are fundamentally tied to the underlying zero-knowledge proof system's efficiency.