mod transaction;
mod blockchain;
mod wallet;
mod crypto;

fn main() {
    let mut chain = blockchain::Blockchain::new();

    chain.add_transaction(transaction::Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        50
    ));

    chain.add_transaction(transaction::Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        20
    ));

    chain.mine_pending_transactions();
    chain.dump();
}
