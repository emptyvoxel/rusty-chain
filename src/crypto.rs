use sha2::{Sha256, Digest};

pub type Hash = [u8; 32];
pub type Bytes = Vec<u8>;
pub const ZERO_HASH: Hash = [0u8; 32];

pub fn hash_matches_difficulty(hash: &Hash, difficulty: u32) -> bool {
    let mut count: u32 = 0;

    for byte in hash.iter() {
        let zeros = byte.leading_zeros();
        count += zeros;

        if count >= difficulty {
            return true;
        }

        if zeros < 8 {
            return false;
        }
    }

    return false;
}

pub fn calculate_hash(data: &Vec<u8>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);

    return hasher.finalize().into();
}

pub fn hex_digest(hash: &Hash) -> String {
    hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}
