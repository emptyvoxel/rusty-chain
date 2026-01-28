use crate::crypto::{Hash, ZERO_HASH, calculate_hash, hex_digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    value: u64,
    timestamp: u64,
    hash: Hash
}

impl Transaction {
    pub fn new(sender: String, receiver: String, value: u64) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut transaction = Self {
            sender: sender,
            receiver: receiver,
            value: value,
            timestamp: timestamp,
            hash: ZERO_HASH
        };

        transaction.hash = transaction.calculate_hash();
        return transaction;
    }

    pub fn genesis() -> Self {
        Transaction::new(
            "God".to_string(),
            "Adam".to_string(),
            0
        )
    }

    pub fn calculate_hash(&self) -> Hash {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(self.sender.as_bytes());
        bytes.extend_from_slice(self.receiver.as_bytes());
        bytes.extend_from_slice(&self.value.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());

        return calculate_hash(&bytes);
    }

    pub fn dump(self) {
        println!("Transaction {}", hex_digest(&self.hash));
        println!(
            ">>> Content: {} sent {} to {} at {}",
            self.sender, self.value, self.receiver, self.timestamp
        );
    }
}

pub fn calculate_merkle_root(data: &Vec<Transaction>) -> Hash {
    if data.is_empty() {
        return ZERO_HASH;
    }

    if data.len() == 1 {
        return data[0].calculate_hash();
    }

    let mut hashes: Vec<Hash> = data
        .iter()
        .map(|transaction| transaction.calculate_hash())
        .collect();

    if hashes.len() % 2 == 1 {
        hashes.push(
            *hashes
            .last()
            .unwrap()
        );
    }

    while hashes.len() > 1 {
        let mut next_level = Vec::new();

        for pair in hashes.chunks(2) {
            let combined = [pair[0], pair[1]].concat();
            next_level.push(calculate_hash(&combined));
        }

        if next_level.len() % 2 == 1 && next_level.len() > 1 {
            next_level.push(
                *next_level
                .last()
                .unwrap()
            );
        }

        hashes = next_level;
    }

    return hashes[0];
}
