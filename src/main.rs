mod blockchain;
mod crypto;

fn main() {
    let chain = blockchain::Blockchain::new();

    chain.dump();
}
