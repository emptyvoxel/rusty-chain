use sha2::{Sha256, Digest};

type Hash = [u8; 32];
const ZERO_HASH = [0u8; 32];

struct Transaction {
    // from: Hash,
    // to: Hash,
    value: u64
}

struct Header {
    height: u64,
    previous_hash: Hash,
    nonce: u64,
    timestamp: u64
}

impl Header {
    fn digest_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(56);

        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.previous_hash);
        bytes.extend_from_slice(&self.nonce.to_be_bytes());

        bytes
    }
}

fn check_hash(hash: &Hash, difficulty: u32) -> bool {
    let mut remaining = difficulty;

    for byte in hash.iter() {
        if remaining == 0 {
            return true;
        }

        let zeros = byte.leading_zeros();

        if zeros >= remaining {
            return true;
        }

        if *byte != 0 {
            return false;
        }

        remaining -= 8;
    }

    true
}

struct Block {
    header: Header,
    hash: Hash,
    data: Transaction
}

impl Block {
    fn genesis() -> Self {
        Self {
            data: Transaction {
                value: 69420
            },
            header: Header {
                timestamp: 0,
                nonce: 0,
                height: 0,
                previous_hash: ZERO_HASH
            },
            hash: ZERO_HASH
        }
    }

    fn calculate_hash(&self) -> Hash {
        let header = self.header.digest_bytes();
        
        let mut hasher = Sha256::new();
        hasher.update(&header);
        hasher.finalize().into()
    }

    fn mine(&mut self, difficulty: u32) -> Hash {
        loop {
            let hash = self.calculate_hash();

            if check_hash(&hash, difficulty) {
                return hash;
            }

            self.header.nonce += 1;
        }
    }

    fn prepare_block(&mut self, previous_hash: Hash, height: u64) {
        self.header.previous_hash = previous_hash;
        self.header.height = height;
        self.mine(3);
    }
}

struct Blockchain {
    chain: Vec<Block>,
    pending: Vec<Transaction>
}

fn main() {
    println!("I'll finish this tomorrow!");
}
