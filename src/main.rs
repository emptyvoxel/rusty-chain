type Hash = [u8; 32];

struct Transaction {
    from: Hash,
    to: Hash,
    value: u64
}

struct Block {
    hash: Hash,
    previous_hash: Hash,
    height: u64,
    nonce: u64,
    timestamp: u64,
    data: Transaction
}

struct Blockchain {
    chain: Vec<Block>,
    pending: Vec<Transaction>
}

fn main() {
    println!("I'll finish this tomorrow!");
}
