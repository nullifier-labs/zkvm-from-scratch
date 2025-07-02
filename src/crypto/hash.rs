use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type HashValue = [u8; 32];

pub trait HashFunction {
    fn hash(&self, data: &[u8]) -> HashValue;
}

#[derive(Debug, Clone)]
pub struct SimpleHash;

impl HashFunction for SimpleHash {
    fn hash(&self, data: &[u8]) -> HashValue {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash_u64 = hasher.finish();

        let mut result = [0u8; 32];
        result[0..8].copy_from_slice(&hash_u64.to_le_bytes());
        result
    }
}

pub fn hash_bytes(data: &[u8]) -> HashValue {
    let hasher = SimpleHash;
    hasher.hash(data)
}

pub fn hash_u32(value: u32) -> HashValue {
    hash_bytes(&value.to_le_bytes())
}

pub fn hash_pair(left: &HashValue, right: &HashValue) -> HashValue {
    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    hash_bytes(&combined)
}
