//! # zkvm-from-scratch
//!
//! A zero-knowledge virtual machine implementation from scratch.
//!
//! This library provides the core components for building and running
//! zero-knowledge proofs for arbitrary computations.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]

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

/// Core virtual machine components
pub mod vm {
    //! Virtual machine implementation

    /// A basic virtual machine structure
    #[derive(Debug, Clone)]
    pub struct VirtualMachine {
        /// Program counter
        pub pc: usize,
        /// Memory
        pub memory: Vec<u8>,
        /// Stack
        pub stack: Vec<u64>,
    }

    impl VirtualMachine {
        /// Create a new virtual machine
        pub fn new() -> Self {
            Self {
                pc: 0,
                memory: Vec::new(),
                stack: Vec::new(),
            }
        }

        /// Reset the virtual machine state
        pub fn reset(&mut self) {
            self.pc = 0;
            self.memory.clear();
            self.stack.clear();
        }
    }

    impl Default for VirtualMachine {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Zero-knowledge proof components
pub mod zkp {
    //! Zero-knowledge proof implementations

    /// Trait for zero-knowledge proof systems
    pub trait ProofSystem {
        /// Proof type
        type Proof;
        /// Public parameters
        type PublicParams;
        /// Error type
        type Error;

        /// Generate a proof
        fn prove(&self, params: &Self::PublicParams) -> Result<Self::Proof, Self::Error>;

        /// Verify a proof
        fn verify(
            &self,
            proof: &Self::Proof,
            params: &Self::PublicParams,
        ) -> Result<bool, Self::Error>;
    }
}

/// Cryptographic utilities
pub mod crypto {
    //! Cryptographic primitives and utilities

    /// Hash function trait
    pub trait Hash {
        /// Output type
        type Output;

        /// Hash input data
        fn hash(&self, data: &[u8]) -> Self::Output;
    }

    /// Merkle tree implementation
    pub struct MerkleTree {
        /// Tree leaves
        pub leaves: Vec<[u8; 32]>,
        /// Tree root
        pub root: [u8; 32],
    }

    impl MerkleTree {
        /// Create a new Merkle tree
        pub fn new(leaves: Vec<[u8; 32]>) -> Self {
            // Simple placeholder implementation
            let root = [0u8; 32];
            Self { leaves, root }
        }

        /// Get the root hash
        pub fn root(&self) -> [u8; 32] {
            self.root
        }
    }
}

/// Utility functions and helpers
pub mod utils {
    //! Utility functions

    /// Convert bytes to hex string
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Convert hex string to bytes
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
        if hex.len() % 2 != 0 {
            return Err("Hex string must have even length");
        }

        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| "Invalid hex character"))
            .collect()
    }
}

// WebAssembly bindings
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn create_vm() -> vm::VirtualMachine {
    vm::VirtualMachine::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        let vm = vm::VirtualMachine::new();
        assert_eq!(vm.pc, 0);
        assert!(vm.memory.is_empty());
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_merkle_tree() {
        let leaves = vec![[1u8; 32], [2u8; 32]];
        let tree = crypto::MerkleTree::new(leaves);
        assert_eq!(tree.leaves.len(), 2);
    }

    #[test]
    fn test_hex_conversion() {
        let bytes = vec![0x12, 0x34, 0xab, 0xcd];
        let hex = utils::bytes_to_hex(&bytes);
        assert_eq!(hex, "1234abcd");

        let back_to_bytes = utils::hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, back_to_bytes);
    }
}
