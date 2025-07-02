# zkVM from Scratch

A minimal implementation of a zero-knowledge virtual machine (zkVM) targeting the RISC-V instruction set architecture. This project demonstrates how to build a zkVM from first principles, making zero-knowledge proofs accessible and understandable.

## Overview

This zkVM implementation provides:
- **RISC-V Execution**: Interpreter for RV32IM (32-bit base integer + multiplication/division)
- **🔒 Zero-Knowledge Proofs**: Prove computation without revealing inputs using STARK proofs
- **⚡ Fast Verification**: Verify proofs faster than re-executing programs
- **🎭 Privacy Preservation**: Keep private inputs secret while proving correctness
- **Modular Architecture**: Trait-based design supporting multiple proof systems
- **WebAssembly Support**: Full browser compatibility

## 🚀 Try It Now!

```bash
git clone https://github.com/nullifier-labs/zkvm-from-scratch
cd zkvm-from-scratch
cargo run --bin zkvm
```

**Watch the magic happen**: See execution traces, zero-knowledge proof generation, and verification in action!

## Features

- ✅ RISC-V instruction execution (RV32IM subset)
- ✅ Memory management with flat addressing
- ✅ Cryptographic primitives (hashing, Merkle trees)
- ✅ WebAssembly compilation target
- ✅ **Execution trace generation** - Complete VM state recording (Phase 2)
- ✅ **Arithmetic constraint systems** - RISC-V to circuit conversion (Phase 2)
- ✅ **STARK proof generation** - Zero-knowledge proofs (Phase 3)
- ✅ **Proof verification** - Fast ZK verification (Phase 4)
- ✅ **End-to-end zkVM** - Prove computation without revealing secrets!

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

#### Running the Zero-Knowledge Demo

```bash
# Run the full zkVM demo (includes zero-knowledge proof generation!)
cargo run --bin zkvm

# Run with verbose output
RUST_LOG=debug cargo run --bin zkvm
```

**What you'll see:**
- 📟 Basic VM execution with RISC-V instructions
- 🔍 **Execution trace generation** capturing every VM state
- ⚡ **Zero-knowledge proof creation** (STARK-based)
- 🔐 **Proof verification** without re-execution
- 🎭 **Zero-knowledge properties** - private inputs stay hidden!

#### Library Usage

```rust
use zkvm_from_scratch::{
    VmState, StarkProver, StarkVerifier, Prover, Verifier, 
    encode_hex, decode_hex
};

// Basic VM Usage
let mut vm = VmState::new(1024 * 1024); // 1MB memory
let program = vec![0x33, 0x01, 0x10, 0x00]; // ADD instruction
vm.memory.load_program(&program, 0).unwrap();
vm.run(100).unwrap(); // Execute normally

// Zero-Knowledge Proof Usage
let stark_prover = StarkProver::default();
let prover = Prover::new(stark_prover);

// Generate execution trace with private inputs
vm.registers[1] = 42; // Secret value (won't be revealed!)
let trace = prover.generate_execution_trace(&mut vm, 10).unwrap();

// Create zero-knowledge proof
let proof = prover.prove_execution(&trace).unwrap();

// Verify proof without seeing private inputs
let stark_verifier = StarkVerifier::default();
let verifier = Verifier::new(stark_verifier);
let is_valid = verifier.verify(&proof, &[]).unwrap();

println!("Proof valid: {} (without revealing secret 42!)", is_valid);
```

## Architecture

The project follows a modular architecture:

```
src/
├── vm/          # Virtual machine core
│   ├── execution.rs    # RISC-V instruction execution + trace generation
│   ├── memory.rs       # Memory management
│   ├── opcodes.rs      # Instruction decoding
│   └── mod.rs          # Module exports
├── zkp/         # Zero-knowledge proof system 🔒
│   ├── prover.rs       # STARK proof generation
│   ├── verifier.rs     # STARK proof verification  
│   ├── constraints.rs  # Arithmetic constraint system
│   ├── stark.rs        # STARK protocol + FRI
│   └── mod.rs          # Module exports
├── crypto/      # Cryptographic primitives
│   ├── hash.rs         # Hash functions
│   ├── merkle.rs       # Merkle tree implementation
│   └── mod.rs          # Module exports
├── utils/       # Utility functions
│   ├── encoding.rs     # Hex encoding/decoding
│   └── mod.rs          # Module exports
├── bin/         # Binary executables
│   └── main.rs         # Zero-knowledge demo application 🎯
└── lib.rs       # Library root
```

## Development Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| 0 | ✅ Complete | Research & Environment Setup |
| 1 | ✅ **Complete** | **RISC-V Execution Model** |
| 2 | ✅ **Complete** | **Trace & Constraint System Design** |
| 3 | ✅ **Complete** | **Prover Implementation (STARK)** |
| 4 | ✅ **Complete** | **Verifier Implementation** |
| 5 | 📋 Future | Proof Aggregation & Recursion |
| 6 | 📋 Future | Host Integration & API |
| 7 | 📋 Future | Security Audit & Optimization |
| 8 | 📋 Future | Documentation & Release |

🎉 **Current Status**: **Fully functional zkVM with zero-knowledge proofs!**

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

### Development Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --no-deps --open

# Security audit
cargo audit

# Run the demo
cargo run --bin zkvm
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
