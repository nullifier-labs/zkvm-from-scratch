# RISC-V ISA Subset Selection

## Overview

This document outlines the decision process and rationale for selecting the target subset of the RISC-V Instruction Set Architecture (ISA) for our zkVM implementation.

## Recommended Target: RV32IM

We recommend starting with **RV32IM** as the initial target subset for the following reasons:

### Base Integer Instruction Set (RV32I)
- **Foundation**: RV32I provides the core 32-bit integer instruction set that forms the foundation of all RISC-V implementations
- **Completeness**: Contains all essential instructions for basic computation including:
  - Integer arithmetic and logical operations
  - Load/store instructions for memory access
  - Control flow instructions (branches, jumps)
  - System instructions
- **Simplicity**: Well-defined and stable specification with clear semantics

### Integer Multiplication and Division Extension (M)
- **Practical Necessity**: Most real-world programs require multiplication and division operations
- **Performance**: Hardware multiplication/division is significantly faster than software emulation
- **Completeness**: Provides full set of multiply/divide instructions:
  - `MUL`, `MULH`, `MULHSU`, `MULHU` (multiplication variants)
  - `DIV`, `DIVU`, `REM`, `REMU` (division and remainder)

## Future Extensions to Consider

### Compressed Instructions (C Extension)
- **Benefits**: 
  - Reduces code size by ~25-30%
  - Improves instruction fetch efficiency
  - Maintains full compatibility with base ISA
- **Implementation Complexity**: Moderate - requires instruction decompression logic
- **Recommendation**: Add after RV32IM is stable and tested

### Bit Manipulation (B Extension)
- **Benefits**:
  - Efficient cryptographic operations
  - Optimized bit manipulation for zkVM circuits
  - Better performance for hash functions and merkle tree operations
- **Status**: Recently ratified (Zba, Zbb, Zbc, Zbs subsets)
- **Recommendation**: Consider after C extension, particularly relevant for zero-knowledge applications

## Implementation Phases

### Phase 1: RV32I
1. Implement base integer instruction set
2. Build basic interpreter/executor
3. Implement memory model
4. Add system call interface

### Phase 2: RV32IM
1. Add multiplication instructions
2. Add division and remainder instructions
3. Optimize for circuit generation
4. Performance testing and validation

### Phase 3: RV32IMC (Future)
1. Add compressed instruction support
2. Instruction decoder updates
3. Code density improvements
4. Compatibility testing

### Phase 4: RV32IMCB (Future)
1. Evaluate bit manipulation subset needs
2. Implement relevant B extension instructions
3. Optimize for cryptographic workloads
4. Performance benchmarking

## Design Considerations for zkVM

### Circuit Complexity
- Each instruction type adds constraints to the zero-knowledge circuit
- Start minimal to validate the core architecture
- Gradual expansion allows for optimization at each step

### Verification Efficiency
- RV32IM provides good balance of functionality vs. circuit size
- Multiplication circuits are well-understood and optimizable
- Division can be implemented efficiently using inverse operations

### Compatibility
- RV32IM is widely supported by existing toolchains
- Large ecosystem of compatible software
- Clear upgrade path to additional extensions

## Rationale Summary

Starting with RV32IM provides:
- **Solid Foundation**: Complete basic computation capabilities
- **Manageable Complexity**: Well-scoped for initial implementation
- **Practical Utility**: Can run meaningful programs and benchmarks
- **Clear Growth Path**: Natural progression to C and B extensions
- **Toolchain Support**: Excellent compiler and debugging tool support

This approach allows us to build a functional zkVM while maintaining the flexibility to add more sophisticated instruction set features as the project matures.