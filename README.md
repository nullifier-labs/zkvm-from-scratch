# zkVM from Scratch

A minimal implementation of a zero-knowledge virtual machine (zkVM) targeting the RISC-V instruction set architecture. This project demonstrates how to build a zkVM from first principles, making zero-knowledge proofs accessible and understandable.

## Overview

This zkVM implementation provides:
- **RISC-V Execution**: Interpreter for RV32IM (32-bit base integer + multiplication/division)
- **Zero-Knowledge Proofs**: Prove computation without revealing inputs
- **Modular Architecture**: Trait-based design supporting multiple proof systems
- **WebAssembly Support**: Full browser compatibility

## Features

- ✅ RISC-V instruction execution (RV32IM subset)
- ✅ Memory management with flat addressing
- ✅ Cryptographic primitives (hashing, Merkle trees)
- ✅ WebAssembly compilation target
- 🚧 Trace generation and constraint systems (Phase 2)
- 🚧 Proof generation (Phase 3)
- 🚧 Proof verification (Phase 4)

## Quick Start

### Prerequisites

- Rust 1.70+ (nightly recommended)
- `wasm-pack` (for WebAssembly builds)

### Installation

```bash
# Clone the repository
git clone https://github.com/nullifier-labs/zkvm-from-scratch
cd zkvm-from-scratch

# Build the project
cargo build --release

# Run tests
cargo test

# Build for WebAssembly
wasm-pack build --target web --out-dir pkg
```

### Usage

```rust
use zkvm_from_scratch::{encode_hex, decode_hex};

// Example: Hex encoding/decoding
let data = vec![1, 2, 3, 4];
let encoded = encode_hex(&data);
let decoded = decode_hex(&encoded).unwrap();
assert_eq!(data, decoded);
```

## Architecture

The project follows a modular architecture:

```
src/
├── vm/          # Virtual machine core
│   ├── execution.rs    # RISC-V instruction execution
│   ├── memory.rs       # Memory management
│   └── opcodes.rs      # Instruction decoding
├── zkp/         # Zero-knowledge proof system
│   ├── prover.rs       # Proof generation
│   └── verifier.rs     # Proof verification
├── crypto/      # Cryptographic primitives
│   ├── hash.rs         # Hash functions
│   └── merkle.rs       # Merkle tree implementation
└── utils/       # Utility functions
```

## Development Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| 0 | ✅ Complete | Research & Environment Setup |
| 1 | 🚧 In Progress | RISC-V Execution Model |
| 2 | 📋 Planned | Trace & Constraint System Design |
| 3 | 📋 Planned | Prover Implementation |
| 4 | 📋 Planned | Verifier Implementation |
| 5 | 📋 Planned | Proof Aggregation & Recursion |
| 6 | 📋 Planned | Host Integration & API |
| 7 | 📋 Planned | Security Audit & Optimization |
| 8 | 📋 Planned | Documentation & Release |

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

### Development Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --no-deps --open

# Security audit
cargo audit
```

## Documentation

- [Contributing Guidelines](CONTRIBUTING.md) - How to contribute to the project
- [Architecture Docs](docs/) - Detailed architecture and design documents
- [API Documentation](https://docs.rs/zkvm-from-scratch) - Generated from code

## License

This project is dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

This project builds upon foundational work in:
- RISC-V specification and tooling
- Zero-knowledge proof systems research
- Rust cryptographic libraries

## Contact

- **Maintainer**: Nullifier Labs
- **Email**: nullifier-labs@proton.me
- **GitHub**: [@nullifier-labs](https://github.com/nullifier-labs)

---

Built with ❤️ by the Nullifier Labs team
