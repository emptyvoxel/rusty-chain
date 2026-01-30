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

const DIFFICULTY: u32 = 20; // 5 hex zeros

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
    pending_transactions: Vec<Transaction>
}

impl Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(56);

        // Same order as the struct for simplicity
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.previous_hash);
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes.extend_from_slice(&self.merkle_root);

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

    pub fn dump(&self) {
        println!("< BEGIN BLOCK N. {}: ", self.header.height);
        println!("Timestamp: {}", self.header.timestamp);
        println!("Previous Block: {}", hex_digest(&self.header.previous_hash));

        println!("<< BEGIN TRANSACTIONS");
        for transaction in &self.data {
            transaction.dump();
        }
        println!(">> END TRANSACTIONS");

        println!("Merkle root: {}", hex_digest(&self.header.merkle_root));
        println!("Proof-of-work: {}", self.header.nonce);
        println!("Hash: {}", hex_digest(&self.hash));
        println!("> END BLOCK N. {}", self.header.height);
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let transaction = vec![Transaction::genesis()];
        let genesis_block = Block::new(
            transaction,
            ZERO_HASH,
            0
        ).mine(DIFFICULTY);

        Self {
            chain: vec![genesis_block],
            pending_transactions: Vec::new()
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) {
        if self.pending_transactions.is_empty() { return; }

        let previous_hash = self
            .chain
            .last()
            .unwrap()
            .hash;

        let height = self.chain.len() as u64;

        let block = Block::new(
            self.pending_transactions.clone(),
            previous_hash,
            height
        ).mine(DIFFICULTY);

        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn dump(self) {
        for block in self.chain.iter() {
            block.dump();
        }
    }
}
