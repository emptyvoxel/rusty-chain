mod transaction;
mod blockchain;
mod wallet;
mod crypto;

const DIFFICULTY: u32 = 20; // 5 hex zeros

fn main() {
    let mut chain = blockchain::Blockchain::new(DIFFICULTY);

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

    //chain.chain[1].data[0].value = 10; // Invalidates transaction 0 at block 1
    chain.dump();
}
