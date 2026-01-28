use sha2::{Sha256, Digest};

pub type Hash = [u8; 32];
pub const ZERO_HASH: Hash = [0u8; 32];

pub fn hash_matches_difficulty(hash: &Hash, difficulty: u32) -> bool {
    let mut remaining: u32 = difficulty;

    for byte in hash.iter() {
        if remaining == 0 { return true; }

        let zeros: u32 = byte.leading_zeros();

        if zeros >= remaining { return true; }
        if *byte != 0 { return false; }

        remaining -= 8;
    }

    return true;
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
