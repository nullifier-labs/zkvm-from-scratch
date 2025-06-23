# Survey of State-of-the-Art Zero-Knowledge Virtual Machines (zkVMs)

*A Comparative Analysis of Leading zkVM Implementations*

## Executive Summary

This report provides a comprehensive survey of the current state-of-the-art in Zero-Knowledge Virtual Machines (zkVMs), analyzing five major implementations: RISC-Zero, zkSync Era VM, SP1, Jolt, and Scroll. Each represents a distinct approach to enabling verifiable computation at scale, with varying trade-offs in performance, compatibility, and architectural design.

## 1. Introduction

Zero-Knowledge Virtual Machines represent a paradigm shift in verifiable computation, allowing developers to prove correct execution of arbitrary programs without revealing private inputs. The field has rapidly evolved, with multiple teams pursuing different architectural approaches to address the fundamental challenges of proving general computation efficiently.

### Key Evaluation Criteria

- **Architecture & Design Philosophy**
- **Performance Characteristics**
- **Developer Experience & Compatibility**
- **Security & Formal Verification**
- **Ecosystem Maturity**
- **Innovation & Differentiators**

## 2. zkVM Landscape Overview

```markdown

    ┌─────────────────────────────────────────────────────────────┐
    │                    zkVM Architectural Spectrum              │
    │                                                             │
    │  RISC-V Native     ◄────────────────────► EVM Native       │
    │  ┌─────────────┐   ┌──────────┐   ┌─────────────────────┐  │
    │  │  RISC-Zero  │   │    SP1   │   │     zkSync Era      │  │
    │  │     Jolt    │   │          │   │       Scroll        │  │
    │  └─────────────┘   └──────────┘   └─────────────────────┘  │
    │                                                             │
    │  ◄── Specialized Proving ──────── General Purpose ────────►│
    └─────────────────────────────────────────────────────────────┘

```

## 3. Detailed zkVM Analysis

### 3.1 RISC-Zero (R0VM 2.0)

**Overview**: RISC-Zero implements a RISC-V based zkVM using STARK proofs, recently launching R0VM 2.0 with significant performance improvements.

#### Architecture

    - **Instruction Set**: RISC-V rv32im
    - **Proof System**: STARK-based with FRI
    - **Execution Model**: RISC-V processor emulation
    - **Language Support**: Rust (primary), C/C++ via compilation

#### Key Features

    - Native Rust development experience via `sp1_derive`
    - Formal verification partnerships with Veridise
    - Universal verifier supporting multiple blockchains
    - Bonsai proving network for scalable proof generation

#### Performance Characteristics

    - **Proving Speed**: Sub-12 second real-time Ethereum proofs (claimed)
    - **Memory Requirements**: ~16GB RAM minimum for local proving
    - **Proof Size**: Large STARK proofs (requires recursion for on-chain verification)
    - **Verification Time**: Fast (milliseconds for STARK verification)

#### Strengths

    - Mature ecosystem with extensive documentation
    - Strong Rust integration and developer tooling
    - Enterprise-grade security with formal verification
    - Universal verifier design for multi-chain deployment

#### Limitations

    - Large proof sizes without recursion
    - High memory requirements for proving
    - Limited to RISC-V instruction set

### 3.2 zkSync Era VM (EraVM + EVM Interpreter)

**Overview**: zkSync Era features a dual-VM approach with native EraVM and a newly introduced EVM bytecode interpreter for full EVM compatibility.

#### Architecture

    - **Primary VM**: EraVM (register-based, ZK-optimized)
    - **Compatibility Layer**: EVM Interpreter (added in v27 upgrade)
    - **Proof System**: Boojum (custom STARK implementation)
    - **Language Support**: Solidity, Vyper (native), Cairo planned

#### Key Features

    - Full EVM bytecode compatibility via interpreter
    - Account abstraction built-in
    - Native fee abstraction and paymasters
    - Upcoming Boojum 2.0 for native EVM execution

#### Performance Characteristics

    - **Throughput**: High transaction throughput on L2
    - **Cost**: EVM interpreter has higher gas costs than native EraVM
    - **Finality**: Fast finality (minutes vs. days for optimistic rollups)
    - **Scalability**: Supports complex DeFi applications

#### Strengths

    - Production-ready with billions in TVL
    - Excellent EVM compatibility via interpreter
    - Strong ecosystem with major DeFi deployments
    - Innovative hybrid VM approach

#### Limitations

    - EVM interpreter performance penalty
    - Complex dual-VM architecture
    - Some EVM features not supported in interpreter mode

### 3.3 SP1 (Succinct Labs)

**Overview**: SP1 is a performant, open-source RISC-V zkVM focused on developer experience and proving speed.

#### Architecture

    - **Instruction Set**: RISC-V rv32im
    - **Proof System**: STARK-based with Plonky3
    - **Execution Model**: RISC-V emulation with precompiles
    - **Language Support**: Rust, C/C++, Go (via RISC-V compilation)

#### Key Features

    - Fastest proving speeds in benchmarks (5x faster than RISC-Zero)
    - Extensive precompile system for acceleration
    - Formal verification with Lean theorem prover
    - Small codebase (under 25k lines) for auditability

#### Performance Characteristics

    - **Proving Speed**: State-of-the-art performance (2x faster than competitors)
    - **Cycle Efficiency**: Optimized instruction execution
    - **Memory Usage**: Efficient memory utilization
    - **Precompiles**: Hardware-accelerated cryptographic operations

#### Strengths

    - Superior proving performance
    - Strong formal verification efforts
    - Active open-source development
    - Modular precompile architecture

#### Limitations

    - Newer project with smaller ecosystem
    - Limited to RISC-V instruction set
    - No native EVM compatibility

### 3.4 Jolt (a16z Crypto)

**Overview**: Jolt represents a novel approach to zkVM design, utilizing lookup tables and the Lasso lookup argument for efficient proving.

#### Architecture

    - **Design Philosophy**: "Lookup Singularity" - everything as lookups
    - **Instruction Set**: RISC-V rv32im
    - **Proof System**: Lasso lookup argument with sumcheck
    - **Execution Model**: Decomposition into lookup table operations

#### Key Features

    - Radical lookup-based architecture
    - Simplest zkVM implementation (under 25k lines)
    - Novel approach to instruction decomposition
    - Emphasis on auditability and simplicity

#### Performance Characteristics

    - **Proving Speed**: Competitive with state-of-the-art
    - **Implementation Complexity**: Minimal (50-100 LOC per instruction)
    - **Memory Efficiency**: Structured lookup tables avoid linear scaling
    - **Verification**: Fast due to sumcheck-based proofs

#### Strengths

    - Revolutionary architectural approach
    - Extremely simple implementation
    - Strong theoretical foundations
    - Fast development cycle for new instructions

#### Limitations

    - Alpha stage, not production-ready
    - Limited ecosystem and tooling
    - Theoretical approach requires more real-world validation

### 3.5 Scroll (zkEVM)

**Overview**: Scroll is a Layer 2 zkEVM solution focused on bytecode-level EVM compatibility and decentralized proving.

#### Architecture

    - **VM Type**: zkEVM (Type 2/3 in Vitalik's classification)
    - **Proof System**: Halo2-based ZK circuits
    - **Compatibility**: EVM bytecode level compatibility
    - **Network**: Layer 2 rollup with decentralized provers

#### Key Features

    - True EVM bytecode compatibility
    - Decentralized proving network ("Rollers")
    - Three-layer architecture (Settlement, Sequencing, Proving)
    - Live mainnet with substantial TVL

#### Performance Characteristics

    - **Compatibility**: Seamless Ethereum migration
    - **Transaction Cost**: Significantly lower than Ethereum L1
    - **Finality**: Fast finality vs. optimistic rollups
    - **Throughput**: High transaction throughput

#### Strengths

    - Production deployment with real users
    - Excellent EVM compatibility
    - Decentralized proving architecture
    - Strong ecosystem adoption

#### Limitations

    - Layer 2 specific, not general-purpose zkVM
    - Complex multi-layer architecture
    - Proving costs still significant

## 4. Comparative Analysis

### 4.1 Performance Comparison

```markdown

| zkVM        | Proving Speed | Memory Req | Proof Size | Verification |
|-------------|---------------|------------|------------|--------------|
| RISC-Zero   | Good          | High       | Large      | Fast         |
| zkSync Era  | Very Good     | Medium     | Medium     | Fast         |
| SP1         | Excellent     | Medium     | Large      | Fast         |
| Jolt        | Good          | Low        | Medium     | Fast         |
| Scroll      | Good          | Medium     | Medium     | Fast         |

```

### 4.2 Compatibility Matrix

```markdown

| zkVM        | RISC-V | EVM Compat | Rust Native | Production |
|-------------|---------|-------------|-------------|------------|
| RISC-Zero   | ✓       | ✗           | ✓           | ✓          |
| zkSync Era  | ✗       | ✓           | ✗           | ✓          |
| SP1         | ✓       | ✗           | ✓           | ✓          |
| Jolt        | ✓       | ✗           | ✓           | ✗          |
| Scroll      | ✗       | ✓           | ✗           | ✓          |

```

### 4.3 Innovation Highlights

#### RISC-Zero: Formal Verification Pioneer
- First zkVM with extensive formal verification
- Universal verifier architecture
- Enterprise-focused approach

#### zkSync Era: EVM Compatibility Leader  
- Dual-VM approach with EVM interpreter
- Account abstraction integration
- Real-world scaling for Ethereum

#### SP1: Performance Champion
- Fastest proving speeds in benchmarks
- Lean architecture and codebase
- Strong open-source community

#### Jolt: Architectural Innovation
- Revolutionary lookup-based design
- Simplest implementation approach
- Theoretical breakthrough potential

#### Scroll: Production zkEVM
- Live Layer 2 with significant adoption
- Decentralized proving network
- True EVM bytecode compatibility

## 5. Ecosystem Maturity Assessment

### Production Readiness

    **Tier 1 (Production Ready)**
    - zkSync Era: Billions in TVL, major DeFi protocols
    - Scroll: Live mainnet, growing ecosystem
    - RISC-Zero: Enterprise deployments, formal verification

    **Tier 2 (Developer Preview)**
    - SP1: Feature complete, active development
    
    **Tier 3 (Alpha/Research)**
    - Jolt: Alpha implementation, research stage

### Developer Ecosystem

```markdown

| zkVM        | Documentation | Tooling | Community | Examples |
|-------------|---------------|---------|-----------|----------|
| RISC-Zero   | Excellent     | Mature  | Large     | Many     |
| zkSync Era  | Good          | Good    | Large     | Many     |
| SP1         | Good          | Growing | Medium    | Good     |
| Jolt        | Limited       | Basic   | Small     | Few      |
| Scroll      | Good          | Good    | Medium    | Good     |

```

## 6. Security Analysis

### Formal Verification Status

- **RISC-Zero**: Extensive formal verification with Veridise, continuous security audits
- **SP1**: Formal verification with Lean, multiple security audits
- **zkSync Era**: Audited production system, formal verification planned
- **Scroll**: Audited circuits, production validation
- **Jolt**: Research stage, limited security review

### Threat Model Considerations

All zkVMs share common security challenges:
- Trusted setup requirements (where applicable)
- Circuit implementation bugs
- Prover/verifier soundness
- Side-channel attacks on proving systems

## 7. Future Outlook and Trends

### Emerging Patterns

1. **Convergence on RISC-V**: Growing adoption of RISC-V as standard ISA
2. **EVM Compatibility Focus**: Increasing emphasis on Ethereum compatibility
3. **Formal Verification**: Industry-wide move toward formal proofs
4. **Hardware Acceleration**: GPU/FPGA proving becoming standard
5. **Decentralized Proving**: Move away from centralized proving services

### Technical Evolution

- **Proof Systems**: Migration to newer, more efficient proof systems
- **Hardware Optimization**: Specialized proving hardware development
- **Language Support**: Expansion beyond Rust to broader language ecosystems
- **Integration**: Better tooling and framework integration

## 8. Recommendations

### For Developers

#### Choose RISC-Zero if:
- Enterprise security requirements are paramount
- Formal verification is essential
- Universal verifier functionality is needed

#### Choose zkSync Era if:
- EVM compatibility is critical
- Layer 2 scaling is the primary use case
- Production deployment is immediate priority

#### Choose SP1 if:
- Proving performance is the top priority
- Open-source development model preferred
- RISC-V compatibility sufficient

#### Choose Jolt if:
- Simplicity and auditability are key requirements
- Working on research/experimental projects
- Contributing to architectural innovation

#### Choose Scroll if:
- Full EVM bytecode compatibility required
- Layer 2 deployment with decentralized proving
- Production Ethereum scaling solution needed

### For Researchers

The zkVM space presents numerous research opportunities:
- Optimization of proving systems for specific workloads
- Hardware acceleration techniques
- Novel instruction set architectures for ZK-friendliness
- Formal verification methodologies
- Decentralized proving network designs

## 9. Conclusion

The zkVM landscape represents one of the most rapidly evolving areas in cryptography and blockchain technology. Each implementation examined offers unique advantages:

- **RISC-Zero** leads in formal verification and enterprise adoption
- **zkSync Era** dominates in production EVM scaling
- **SP1** excels in proving performance and open development
- **Jolt** pioneers revolutionary architectural approaches
- **Scroll** provides practical zkEVM Layer 2 solutions

The field is moving toward greater standardization around RISC-V while maintaining diverse approaches to EVM compatibility. Formal verification is becoming standard practice, and hardware acceleration is increasingly important for production deployments.

For developers entering the space, the choice of zkVM should align with specific requirements around compatibility, performance, security, and production readiness. The rapid pace of innovation suggests that hybrid approaches and cross-platform compatibility will become increasingly important.

## 10. References and Further Reading

### Primary Sources
- [RISC-Zero Documentation](https://dev.risczero.com/)
- [zkSync Era Documentation](https://docs.zksync.io/)
- [SP1 Documentation](https://docs.succinct.xyz/)
- [Jolt Paper](https://eprint.iacr.org/2023/1217)
- [Scroll Technical Documentation](https://scroll.io/blog/architecture)

### Academic Papers
- Jolt: SNARKs for Virtual Machines via Lookups (2023)
- Lasso: Zero-Knowledge Arguments for Machine Learning (2023)
- Various formal verification papers from zkVM teams

### Performance Benchmarks
- a16z zkVM Benchmarks Repository
- SP1 vs RISC-Zero Performance Comparisons
- Independent zkVM Performance Studies

---

*This report represents the state of zkVM technology as of January 2025. Given the rapid pace of development in this field, readers should verify current capabilities and performance claims with the respective project teams.*