mod transaction;
mod blockchain;
mod wallet;
mod crypto;

fn main() {
    let mut chain = blockchain::Blockchain::new();

    let alice_wallet = wallet::Wallet::new();
    let bob_wallet = wallet::Wallet::new();
    let charlie_wallet = wallet::Wallet::new();

    chain.add_transaction(transaction::Transaction::new(
        &alice_wallet,
        bob_wallet.public_key_bytes(),
        50
    ));

    chain.add_transaction(transaction::Transaction::new(
        &bob_wallet,
        charlie_wallet.public_key_bytes(),
        20
    ));

    chain.mine_pending_transactions();
    chain.dump();
}
