use k256::ecdsa::{Signature, VerifyingKey};

use crate::crypto::{Hash, Bytes, ZERO_HASH, calculate_hash, hex_digest};
use crate::wallet::{Wallet, public_key_string, verify_signature};
use std::time::{SystemTime, UNIX_EPOCH};

pub const GENESIS_TRANSACTION_HASH: Hash = [55, 71, 8, 255, 247, 113, 157, 213, 151, 158, 200, 117, 213, 108, 210, 40, 111, 109, 60, 247, 236, 49, 122, 59, 37, 99, 42, 171, 40, 236, 55, 187];
pub const GENESIS_MESSAGE: &str = "In Rust We Trust";

// Making everything public makes it easier to test tampering with the chain.
#[derive(Clone)]
pub struct Transaction {
    pub sender: Bytes,
    pub receiver: Bytes,
    pub value: u64,
    pub timestamp: u64,
    pub signature: Bytes,
    pub hash: Hash
}

impl Transaction {
    pub fn new(sender_wallet: &Wallet, receiver_pubkey: Bytes, value: u64) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sender = sender_wallet.public_key_bytes();

        let mut transaction = Self {
            sender,
            value,
            timestamp,
            receiver: receiver_pubkey,
            signature: Vec::new(), // placeholder
            hash: ZERO_HASH // placeholder
        };

        let signature = sender_wallet.sign(&transaction.to_bytes());

        transaction.signature = signature
            .to_der()
            .to_bytes()
            .to_vec();

        transaction.hash = transaction.calculate_hash();
        return transaction;
    }

    pub fn genesis() -> Self {
        Self {
            sender: Vec::new(),
            receiver: Vec::new(),
            value: 0,
            timestamp: 0,
            signature: Vec::new(),
            hash: GENESIS_TRANSACTION_HASH
        }
    }

    fn to_bytes(&self) -> Bytes {
        let mut bytes: Bytes = Vec::new();
        bytes.extend_from_slice(&self.sender);
        bytes.extend_from_slice(&self.receiver);
        bytes.extend_from_slice(&self.value.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());

        return bytes;
    }

    pub fn calculate_hash(&self) -> Hash {
        let mut bytes = self.to_bytes();
        bytes.extend_from_slice(&self.signature);

        return calculate_hash(&bytes);
    }

    pub fn verify(&self) -> bool {
        if self.signature.is_empty() {
            return self.sender.is_empty() && self.receiver.is_empty();
        }

        let signature = match Signature::from_der(self.signature.as_ref()) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let public_key = match VerifyingKey::from_sec1_bytes(&self.sender) {
            Ok(key) => key,
            Err(_) => return false
        };

        let message = self.to_bytes();
        verify_signature(&public_key, &message, &signature)
    }

    pub fn dump(&self) {
        if self.sender.is_empty() {
            println!("\t\t{}", GENESIS_MESSAGE);
            return;
        }

        let sender = public_key_string(&self.sender);
        let receiver = public_key_string(&self.receiver);

        println!("\t\tTransaction {}", hex_digest(&self.hash));
        println!(
            "\t\t-> Content: {} sent {} to {} at {}",
            sender, self.value, receiver, self.timestamp
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
