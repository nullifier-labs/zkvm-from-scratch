use super::{HashValue, hash_bytes, hash_pair};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaves: Vec<HashValue>,
    nodes: Vec<Vec<HashValue>>,
    root: HashValue,
}

impl MerkleTree {
    pub fn new(data: Vec<&[u8]>) -> Self {
        if data.is_empty() {
            return Self {
                leaves: vec![],
                nodes: vec![],
                root: [0u8; 32],
            };
        }

        let mut leaves: Vec<HashValue> = data.iter().map(|d| hash_bytes(d)).collect();
        
        if leaves.len() % 2 != 0 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let mut nodes = vec![leaves.clone()];
        let mut current_level = leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() > 1 { &chunk[1] } else { &chunk[0] };
                next_level.push(hash_pair(left, right));
            }
            
            nodes.push(next_level.clone());
            current_level = next_level;
        }

        let root = current_level[0];

        Self {
            leaves,
            nodes,
            root,
        }
    }

    pub fn root(&self) -> &HashValue {
        &self.root
    }

    pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() {
            return None;
        }

        let mut proof = Vec::new();
        let mut current_index = index;

        for level in &self.nodes[..self.nodes.len() - 1] {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < level.len() {
                proof.push(level[sibling_index]);
            }

            current_index /= 2;
        }

        Some(MerkleProof {
            leaf: self.leaves[index],
            proof,
            index,
        })
    }

    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        let mut current_hash = proof.leaf;
        let mut current_index = proof.index;

        for sibling in &proof.proof {
            if current_index % 2 == 0 {
                current_hash = hash_pair(&current_hash, sibling);
            } else {
                current_hash = hash_pair(sibling, &current_hash);
            }
            current_index /= 2;
        }

        current_hash == self.root
    }
}

#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf: HashValue,
    pub proof: Vec<HashValue>,
    pub index: usize,
}