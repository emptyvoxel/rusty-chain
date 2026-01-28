use crate::{
    crypto::{
        Hash,
        ZERO_HASH,
        calculate_hash,
        hash_matches_difficulty,
        hex_digest
    },
    transaction::{
        Transaction,
        calculate_merkle_root
    }
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Header {
    timestamp: u64,
    previous_hash: Hash,
    height: u64,
    nonce: u64,
    merkle_root: Hash
}

pub struct Block {
    header: Header,
    data: Vec<Transaction>,
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
    pub fn new(data: Vec<Transaction>, previous_hash: Hash, height: u64) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let header: Header = Header {
            timestamp: timestamp,
            previous_hash: previous_hash,
            height: height,
            nonce: 0,
            merkle_root: calculate_merkle_root(&data)
        };

        Self {
            header: header,
            data: data,
            hash: ZERO_HASH
        }
    }

    pub fn mine(mut self, difficulty: u32) -> Self {
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
    pub fn new() -> Self {
        let transaction = vec![Transaction::genesis()];
        let genesis_block = Block::new(
            transaction,
            ZERO_HASH,
            0
        ).mine(3);

        Self {
            chain: vec![genesis_block]
        }
    }

    pub fn dump(self) {
        for block in self.chain.iter() {
            println!("{}", hex_digest(&block.hash));
        }
    }
}
