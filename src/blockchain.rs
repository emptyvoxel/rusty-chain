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
use std::{time::{SystemTime, UNIX_EPOCH}};

// Making everything public makes it easier to test tampering with the chain.
pub struct Header {
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub height: u64,
    pub nonce: u64,
    pub merkle_root: Hash
}

// Making everything public makes it easier to test tampering with the chain.
pub struct Block {
    pub header: Header,
    pub data: Vec<Transaction>,
    pub hash: Hash
}

// Making everything public makes it easier to test tampering with the chain.
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: u32
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

    pub fn genesis() -> Self {
        let transactions = vec![Transaction::genesis()];

        let header = Header {
            timestamp: 0,
            previous_hash: ZERO_HASH,
            height: 0,
            nonce: 0,
            merkle_root: calculate_merkle_root(&transactions)
        };

        Self {
            header: header,
            data: transactions,
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
        println!("BEGIN BLOCK N. {}: ", self.header.height);
        println!("\tTimestamp: {}", self.header.timestamp);
        println!("\tPrevious Block: {}", hex_digest(&self.header.previous_hash));

        println!("\tBEGIN TRANSACTIONS");
        for transaction in &self.data {
            transaction.dump();
        }
        println!("\tEND TRANSACTIONS");

        println!("\tMerkle root: {}", hex_digest(&self.header.merkle_root));
        println!("\tProof-of-work: {}", self.header.nonce);
        println!("\tHash: {}", hex_digest(&self.hash));
        println!("END BLOCK N. {}", self.header.height);
    }
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let genesis_block = Block::genesis().mine(difficulty);

        Self {
            chain: vec![genesis_block],
            pending_transactions: Vec::new(),
            difficulty
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        assert!(transaction.verify());
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
        ).mine(self.difficulty);

        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn dump(self) {
        if let Err(message) = self.is_valid() {
            println!("Error: {}", message);
            return;
        }

        for block in self.chain.iter() {
            block.dump();
        }
    }

    pub fn is_valid(&self) -> Result<(), String> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i-1];

            if current.header.previous_hash != previous.hash {
                return Err(format!("Invalid hash chain at block {}...", i));
            }

            if !hash_matches_difficulty(&current.hash, self.difficulty) {
                return Err(format!("Insufficient proof of work at block {}...", i));
            }

            if current.hash != calculate_hash(&current.header.to_bytes()) {
                return Err(format!("Invalid block hash at block {}...", i));
            }

            for transaction in &current.data {
                if !transaction.verify() {
                    return Err(format!("Invalid transaction at block {}...", i));
                }
            }
        }

        Ok(())
    }
}
