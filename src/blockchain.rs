use crate::crypto::{Hash, ZERO_HASH, calculate_hash, hash_matches_difficulty};
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: Reimplement the Transaction struct later
type Transaction = String;

pub struct Header {
    timestamp: u64,
    previous_hash: Hash,
    height: u64,
    nonce: u64
}

pub struct Block {
    header: Header,
    data: Transaction,
    hash: Hash
}

pub struct Blockchain {
    chain: Vec<Block>,
    // pending: Vec<Transaction>
}

impl Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(56);

        // Same order as the struct for simplicity
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.previous_hash);
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());

        return bytes;
    }
}

impl Block {
    fn new(data: Transaction, previous_hash: Hash, height: u64) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let header: Header = Header {
            timestamp: timestamp,
            previous_hash: previous_hash,
            height: height,
            nonce: 0
        };

        Self {
            header: header,
            data: data,
            hash: ZERO_HASH
        }
    }

    fn mine(mut self, difficulty: u32) -> Self {
        loop {
            self.hash = calculate_hash(&self.header.to_bytes());

            if hash_matches_difficulty(&self.hash, difficulty) {
                return self;
            }

            self.header.nonce += 1;
        }
    }
}

impl Blockchain {
    fn new() -> Self {
        let genesis_data = "Genesis Block".to_string();
        let genesis_block = Block::new(genesis_data, ZERO_HASH, 0).mine(3);

        Self {
            chain: vec![genesis_block]
        }
    }
}
