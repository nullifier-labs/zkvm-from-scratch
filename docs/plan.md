# zkVM from Scratch – Comprehensive Project Plan

## 0. Introduction

This document describes, in depth, the steps required to design and implement a zero-knowledge virtual machine (zkVM) whose execution semantics follow a subset of the RISC-V instruction-set architecture (ISA).  The plan is technology-agnostic in places (e.g. choice of specific polynomial-commitment scheme) and prescriptive where architectural commitments are beneficial (e.g. the exact RISC-V base and extensions we target).

The project is organised into **eight phases**.  Each phase lists:
* Objectives & learning outcomes
* Detailed tasks (with deliverables)
* Recommended tools & libraries
* Acceptance criteria / completion checklist

Inter-phase dependencies are explicit so you can track critical-path items on your timeline.

---

## 1. Phase Breakdown

### Phase 0 – Research & Environment Setup

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 0.0.1 | Survey state-of-the-art zkVMs (RISC-Zero, zkSync VM, SP1, Jolt, Scroll, etc.) | Comparative report |
| 0.0.2 | Select proving system (e.g. Halo 2, Plonky2, STARK-based) | Rationale document |
| 0.0.3 | Decide on target subset of RISC-V (recommend **RV32IM** to start; add C/B later) | ISA spec markdown |
| 0.0.4 | Stand up dev environment (Rust + Nightly, Wasm targets, CI) | `justfile`, CI workflow |
| 0.0.5 | Define coding conventions / contribution guidelines | `CONTRIBUTING.md` |

**Acceptance Criteria**
1. You can build & test a "hello-world" Rust crate in CI.
2. All architectural decisions captured in docs.

---

### Phase 1 – RISC-V Execution Model

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 1.1 | Formal ISA subset spec (instruction encoding, semantics) | `isa_spec.md` |
| 1.2 | Bytecode loader & ELF parser (optional) | Loader crate |
| 1.3 | In-memory representation of program state (regs, pc, memory) | `core/src/state.rs` |
| 1.4 | Interpreter (non-zk) with step tracing | Passes ISA tests |
| 1.5 | Unit-test harness with official RISC-V tests | Green tests |

**Checklist**
* All integer instructions produce correct traces.

---

### Phase 2 – Trace & Constraint System Design

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 2.1 | Choose trace layout (rows, columns, advice/fixed) | Design doc |
| 2.2 | Map each instruction to constraint gates | Gate specs |
| 2.3 | Memory model: permutation / range checks | Memory gadget spec |
| 2.4 | Implement witness generator (circuit builder) | `circuit/src/lib.rs` |
| 2.5 | Reproduce interpreter trace as circuit witness | Passing integration test |

---

### Phase 3 – Prover Implementation

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 3.1 | Integrate chosen PCS & proof system | Compiles |
| 3.2 | Implement prover for single program execution | CLI tool |
| 3.3 | Bench prover time vs. program length | Benchmark report |
| 3.4 | Optimise (parallelisation, GPU, recursion) | Speed-up metrics |

---

### Phase 4 – Verifier Implementation

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 4.1 | Native verifier (Rust) | `verifier/src/lib.rs` |
| 4.2 | On-chain verifier (Solidity/Move/Go) | Smart contract |
| 4.3 | Gas-cost benchmarking (if EVM) | Report |

---

### Phase 5 – Proof Aggregation & Recursion (Optional)

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 5.1 | Design recursive proof circuit | Spec |
| 5.2 | Implement aggregation of multiple VM proofs | Aggregator crate |
| 5.3 | Bench aggregation scalability | Charts |

---

### Phase 6 – Host Integration & API

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 6.1 | JSON-RPC / gRPC API for proof generation | Service |
| 6.2 | WASM bindings for browser usage | `pkg/` artefacts |
| 6.3 | SDK examples (TypeScript, Python) | Example apps |

---

### Phase 7 – Security Audit & Optimisation

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 7.1 | Static analysis, fuzzing, property tests | Reports |
| 7.2 | External audit (Halborn, Zellic, etc.) | Audit report |
| 7.3 | Performance profiling & refactoring | ≥ 2× speed-up |

---

### Phase 8 – Documentation & Release

| Sub-task | Description | Deliverable |
| --- | --- | --- |
| 8.1 | Developer docs, API reference | `docs.rs` |
| 8.2 | Tutorial: "Prove a Fibonacci program" | MD guide |
| 8.3 | Versioned release v1.0.0 | Tag + changelog |
| 8.4 | Publish blog-post & announcement | Medium/GitHub |

---

## 2. Cross-Cutting Concerns

* **Testing Strategy** – unit, integration, circuit equivalence, adversarial inputs.
* **CI/CD** – GitHub Actions: lint, test, prove sample program, run verifier.
* **Licensing** – MIT/Apache 2.0 dual-license.
* **Governance** – RFC process for future extensions.

---

## 3. Required Skill Sets

| Discipline | Responsibilities |
| --- | --- |
| Cryptography | Circuit design, proof system integration |
| Systems | RISC-V ISA, emulator, memory model |
| DevOps | CI/CD, benchmarking |
| Smart-contract | On-chain verifier |
| Technical writing | Docs, tutorials |

---

## 4. Risk Register

| Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- |
| Under-estimating circuit complexity | Medium | High | Prototype small subset first |
| Prover too slow | Medium | High | GPU acceleration, recursion |
| RISC-V compliance gaps | Low | Medium | Use official test-suite |
| Cryptographic bugs | Low | High | External audits |

---

## 5. Next Steps

1. Approve or amend ISA subset choice.
2. Finalise proof system selection.
3. Start Phase 0 tasks and spin up repository structure.

> _"The journey of a thousand miles begins with a single `cargo init`."_ 