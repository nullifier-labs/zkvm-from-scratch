# Using Multiple Proving Systems in a zkVM

**Can a zkVM use multiple proving systems?**

Yes—there are several architectures that mix proof systems safely and productively. This document outlines the common patterns, their benefits, and the engineering caveats to keep in mind.

---

## 1. Component-level heterogeneity
You split the VM into logical subsystems and give each its own proof system.

| Sub-system | Typical choice | Rationale |
|-----------|----------------|-----------|
| Core execution trace | Halo 2 / Plonky2 | Fast recursion, small proofs, mature libraries |
| Large data-availability checks (hashing, Merkle proofs, etc.) | STARK | Transparent setup, post-quantum, highly parallel |
| Cryptography gadgets (ECDSA, pairings, etc.) | Halo 2 | Widely audited gadget implementations |

**Pros**
* Lets each subsystem use the proving system it performs best on.  
* You can iterate or swap out a subsystem without touching the rest.

**Cons**
* You must aggregate heterogeneous proofs into **one** statement eventually (see §3).  
* Two (or more) sets of tooling/tests to maintain.

---

## 2. Migration / feature-flag approach
Begin with one system (e.g. Halo 2) and add another (e.g. STARK) behind a protocol-version flag.

Useful when you want to ship today but leave a clear path to post-quantum security or cheaper calldata costs later.

---

## 3. Recursive or "bridge" proofs
A proof in system A *verifies* a proof in system B. Two popular topologies:

1. **SNARK → STARK wrapper**  
   * Generate a small Halo 2/Plonky2 proof.  
   * Embed the SNARK verifier inside a STARK circuit; output a STARK proof that "this SNARK is valid."  
   * Yields transparency and post-quantum security at the outer layer while keeping the inner proof tiny.

2. **STARK → SNARK compression**  
   * Produce a large STARK first.  
   * Generate a small SNARK that verifies the STARK.  
   * Handy when on-chain calldata costs dominate.

Existing projects: zkREC architecture, Arecibo, Succinct Labs' SVT (STARK Verified by a zk-SNARK), RISC-Zero's proof-of-proving.

---

## 4. Unified Rust trait/interface layer
Below is a minimal interface you can implement for each backend.

    ```rust
    pub trait ProofSystem {
        type ProverParams;
        type VerifierParams;
        type Proof;

        fn setup(security_level: usize) -> (Self::ProverParams, Self::VerifierParams);
        fn prove(params: &Self::ProverParams, witness: Witness) -> Self::Proof;
        fn verify(params: &Self::VerifierParams, proof: &Self::Proof, public: PublicInputs) -> bool;
    }
    ```

Then provide `impl ProofSystem for Halo2Backend`, `impl ProofSystem for WinterfellBackend`, etc.  
Your VM emits an *abstract* trace; the build pipeline chooses the backend(s).

---

## 5. Practical caveats
1. **Field compatibility** – Halo 2 uses 255-bit prime fields; many STARK frameworks use 64-bit friendly fields. Bridging may require field emulation or translation gadgets.
2. **Trusted setup** – Mixing systems doesn't remove the trusted setup requirement for the SNARK part; it simply scopes it more narrowly.
3. **Verifier cost explosion** – A naïve composition can multiply gas/CPU costs; use compressed or batched recursion strategies.
4. **Maintenance overhead** – Multiple dependencies → larger audit surface. Make sure your team has bandwidth for that.

---

## 6. When is it worth it?
* Distinct workloads (e.g. small logic + huge hashing) map clearly to different systems.
* You need a staged roll-out: small proofs today, post-quantum tomorrow.
* Your product can absorb the extra engineering and audit cost.

### When to keep it simple
* Early-stage prototype with tight deadlines.
* Team is still building expertise in **one** proof system.
* On-chain constraints point decisively toward a single system.

---

**Bottom-line:** You *can* combine proving systems, and many advanced zkVM designs do, but treat this as an optimization or long-term upgrade path rather than a Phase-0 requirement. 