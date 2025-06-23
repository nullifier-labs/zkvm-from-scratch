# Contributing to zkvm-from-scratch

Thank you for your interest in contributing to zkvm-from-scratch! This document outlines the coding conventions, development guidelines, and contribution process for this project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Coding Conventions](#coding-conventions)
- [Code Structure](#code-structure)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Standards](#documentation-standards)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Security Considerations](#security-considerations)
- [Performance Guidelines](#performance-guidelines)

## Getting Started

### Prerequisites

Ensure you have the following tools installed:

- **Rust** (latest stable version)
- **Git**
- **Cargo** (comes with Rust)
- **rustfmt** for code formatting
- **clippy** for linting

### Setting up the Development Environment

1. Fork the repository
2. Clone your fork:

    ```bash
    git clone https://github.com/your-username/zkvm-from-scratch.git
    cd zkvm-from-scratch
    ```

3. Install dependencies:

    ```bash
    cargo build
    ```

4. Install development tools:

    ```bash
    rustup component add rustfmt clippy
    ```

## Coding Conventions

### General Guidelines

- **Language**: Primary development language is Rust
- **Edition**: Use Rust 2021 edition
- **Formatting**: Use `rustfmt` with default settings
- **Linting**: Code must pass `clippy` checks without warnings

### Naming Conventions

- **Functions and variables**: Use `snake_case`
- **Types and structs**: Use `PascalCase`
- **Constants**: Use `SCREAMING_SNAKE_CASE`
- **Modules**: Use `snake_case`
- **Files**: Use `snake_case` with `.rs` extension

### Code Style

#### Function Signatures

    ```rust
    /// Brief description of what the function does.
    ///
    /// # Arguments
    ///
    /// * `param1` - Description of parameter 1
    /// * `param2` - Description of parameter 2
    ///
    /// # Returns
    ///
    /// Description of return value
    ///
    /// # Examples
    ///
    /// ```
    /// let result = my_function(arg1, arg2);
    /// assert_eq!(result, expected_value);
    /// ```
    pub fn my_function(param1: Type1, param2: Type2) -> ReturnType {
        // Implementation
    }
    ```

#### Error Handling

- Use `Result<T, E>` for fallible operations
- Create custom error types for domain-specific errors
- Use `?` operator for error propagation
- Avoid `unwrap()` and `expect()` in production code

    ```rust
    pub fn zkvm_operation() -> Result<Output, ZkvmError> {
        let intermediate = risky_operation()?;
        let result = process_data(intermediate)?;
        Ok(result)
    }
    ```

#### Memory Safety

- Prefer owned types over borrowed types in public APIs
- Use `Arc<T>` and `Rc<T>` judiciously
- Minimize `unsafe` code and document thoroughly when used
- Use `Box<T>` for heap allocation when stack allocation is insufficient

### Documentation

- All public functions, structs, and modules must have documentation
- Use triple-slash comments (`///`) for public items
- Use double-slash comments (`//`) for implementation details
- Include examples in documentation when helpful

## Code Structure

### Module Organization

    ```
    src/
    ├── lib.rs              # Main library entry point
    ├── vm/                 # Virtual machine core
    │   ├── mod.rs
    │   ├── execution.rs    # Execution engine
    │   ├── memory.rs       # Memory management
    │   └── opcodes.rs      # Instruction definitions
    ├── circuit/            # Circuit construction
    │   ├── mod.rs
    │   ├── constraints.rs  # Constraint generation
    │   └── witness.rs      # Witness generation
    ├── prover/             # Zero-knowledge proof generation
    │   ├── mod.rs
    │   └── stark.rs        # STARK proof system
    ├── verifier/           # Proof verification
    │   ├── mod.rs
    │   └── verify.rs       # Verification logic
    └── utils/              # Utility functions
        ├── mod.rs
        ├── field.rs        # Field arithmetic
        └── hash.rs         # Cryptographic hashing
    ```

### Import Organization

Order imports as follows:

1. Standard library imports
2. External crate imports  
3. Internal crate imports
4. Local module imports

    ```rust
    use std::collections::HashMap;
    use std::sync::Arc;
    
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    
    use crate::vm::execution::ExecutionEngine;
    use crate::circuit::constraints::ConstraintSystem;
    
    use super::types::ZkvmState;
    ```

## Testing Guidelines

### Test Organization

- Unit tests: Place in the same file as the code being tested using `#[cfg(test)]`
- Integration tests: Place in `tests/` directory
- Benchmarks: Place in `benches/` directory

### Test Naming

- Test function names should clearly describe what is being tested
- Use descriptive assertions with custom messages

    ```rust
    #[test]
    fn test_vm_executes_add_instruction_correctly() {
        let mut vm = VirtualMachine::new();
        vm.load_program(&[OpCode::Add, OpCode::Halt]);
        
        let result = vm.execute();
        assert!(result.is_ok(), "VM execution should succeed");
        assert_eq!(vm.get_register(0), 42, "Register 0 should contain expected sum");
    }
    ```

### Property-Based Testing

Use property-based testing for complex mathematical operations:

    ```rust
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn field_addition_is_commutative(a in any::<FieldElement>(), b in any::<FieldElement>()) {
            prop_assert_eq!(a + b, b + a);
        }
    }
    ```

### Test Coverage

- Aim for high test coverage, especially for critical cryptographic components
- Test both success and failure paths
- Include edge cases and boundary conditions
- Test with various input sizes and types

## Documentation Standards

### Code Documentation

- Use rustdoc syntax for all public APIs
- Include usage examples in documentation
- Document safety requirements for `unsafe` code
- Explain complex algorithms and mathematical concepts

### Architecture Documentation

- Keep high-level architecture documentation in `docs/`
- Use Mermaid diagrams for visual representations
- Document design decisions and trade-offs
- Maintain up-to-date API references

### Examples

Provide working examples in the `examples/` directory:

    ```rust
    // examples/simple_program.rs
    use zkvm_from_scratch::{VirtualMachine, Program};
    
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let program = Program::from_file("examples/fibonacci.zkasm")?;
        let mut vm = VirtualMachine::new();
        
        let proof = vm.execute_and_prove(program)?;
        println!("Generated proof: {:?}", proof);
        
        Ok(())
    }
    ```

## Pull Request Process

### Before Submitting

1. **Format your code**: Run `cargo fmt`
2. **Lint your code**: Run `cargo clippy` and fix all warnings
3. **Run tests**: Ensure `cargo test` passes
4. **Update documentation**: Add or update relevant documentation
5. **Performance check**: Run benchmarks if your changes affect performance

### PR Requirements

- **Title**: Use a clear, descriptive title
- **Description**: Explain what changes you made and why
- **Tests**: Include tests for new functionality
- **Documentation**: Update documentation as needed
- **Changelog**: Add entry to CHANGELOG.md if applicable

### PR Template

When creating a PR, include:

    ```markdown
    ## Summary
    Brief description of changes
    
    ## Changes Made
    - [ ] Added new feature X
    - [ ] Fixed bug Y
    - [ ] Updated documentation
    
    ## Testing
    - [ ] Unit tests pass
    - [ ] Integration tests pass
    - [ ] Manual testing completed
    
    ## Performance Impact
    - [ ] No performance impact
    - [ ] Performance improved
    - [ ] Performance regression (justified)
    
    ## Breaking Changes
    - [ ] No breaking changes
    - [ ] Breaking changes (listed below)
    ```

## Issue Reporting

### Bug Reports

Include the following information:

- **Environment**: OS, Rust version, dependencies
- **Steps to reproduce**: Clear, minimal reproduction steps
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Error messages**: Full error output if applicable

### Feature Requests

- **Use case**: Explain why this feature is needed
- **Proposed solution**: Describe how it should work
- **Alternatives**: Other approaches you've considered
- **Implementation notes**: Technical considerations if any

## Security Considerations

### Cryptographic Code

- **Constant-time operations**: Use constant-time implementations for sensitive operations
- **Side-channel resistance**: Consider timing and power analysis attacks
- **Randomness**: Use cryptographically secure random number generators
- **Key management**: Never log or expose private keys

### Code Review

- All cryptographic code requires thorough review
- Security-sensitive changes need approval from maintainers
- Consider formal verification for critical components

### Reporting Security Issues

**Do not open public issues for security vulnerabilities.** Instead:

1. Email security issues to [security@project.org]
2. Include detailed description and reproduction steps
3. Allow reasonable time for response before public disclosure

## Performance Guidelines

### Optimization Priorities

1. **Correctness first**: Never sacrifice correctness for performance
2. **Profile before optimizing**: Use `cargo bench` and profiling tools
3. **Big-O complexity**: Focus on algorithmic improvements first
4. **Memory usage**: Minimize allocations in hot paths

### Benchmarking

- Include benchmarks for performance-critical code
- Use `criterion` for statistical analysis
- Compare before and after performance in PRs
- Document performance characteristics in code

### Common Optimizations

    ```rust
    // Prefer iterators over index-based loops
    let sum: u64 = data.iter().map(|x| x.value).sum();
    
    // Use Vec::with_capacity when size is known
    let mut results = Vec::with_capacity(expected_size);
    
    // Avoid unnecessary clones
    fn process_data(data: &[u8]) -> Result<Output, Error> {
        // Process without cloning
    }
    ```

## Getting Help

- **Documentation**: Check existing docs in `docs/` directory
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions
- **Code review**: Don't hesitate to ask for review feedback

## License

By contributing to zkvm-from-scratch, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to zkvm-from-scratch! Your efforts help advance zero-knowledge virtual machine technology.