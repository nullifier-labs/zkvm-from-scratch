//! # zkvm-from-scratch
//!
//! A zero-knowledge virtual machine implementation from scratch.
//!
//! This library provides the core components for building and running
//! zero-knowledge proofs for arbitrary computations.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::mixed_attributes_style)]

// WebAssembly-specific setup
#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Core virtual machine components
pub mod vm;

/// Zero-knowledge proof components
pub mod zkp;

/// Cryptographic utilities
pub mod crypto;

/// Utility functions and helpers
pub mod utils;

// Re-export key items from modules
pub use crypto::{HashValue, MerkleTree};
pub use utils::{decode_hex, encode_hex};
pub use vm::{ExecutionStep, Instruction, Opcode, VmState};
pub use zkp::{
    ConstraintSystem, ExecutionTrace, Proof, Prover, StarkProver, StarkVerifier, Verifier,
};

// WebAssembly bindings (optional). Enable via `wasm-bindings` feature if needed.
#[cfg(all(target_arch = "wasm32", feature = "wasm-bindings"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindings"))]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindings"))]
#[wasm_bindgen]
pub fn create_vm() -> wasm_bindgen::JsValue {
    let vm = VmState::new(1024 * 1024); // 1MB memory
    wasm_bindgen::JsValue::NULL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        let vm = VmState::new(1024);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.registers[0], 0);
    }

    #[test]
    fn test_merkle_tree() {
        let data: Vec<&[u8]> = vec![b"leaf1", b"leaf2"];
        let tree = MerkleTree::new(data);
        assert_ne!(tree.root(), &[0u8; 32]);
    }

    #[test]
    fn test_hex_conversion() {
        let bytes = vec![0x12, 0x34, 0xab, 0xcd];
        let hex = encode_hex(&bytes);
        assert_eq!(hex, "1234abcd");

        let back_to_bytes = decode_hex(&hex).unwrap();
        assert_eq!(bytes, back_to_bytes);
    }
}
