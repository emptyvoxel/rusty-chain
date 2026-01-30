use k256::ecdsa::{
    Signature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier}
};
use rand_core::OsRng;

pub struct Wallet {
    private_key: SigningKey,
    pub public_key: VerifyingKey
}

impl Wallet {
    pub fn new() -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&private_key);

        Self {
            private_key,
            public_key
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.private_key.sign(message)
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.to_encoded_point(false).as_bytes().to_vec()
    }
}

pub fn verify_signature(
    public_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature
) -> bool {
    public_key.verify(message, signature).is_ok()
}

pub fn public_key_string(public_key: &[u8]) -> String {
    if public_key.is_empty() {
        return "GENESIS".to_string();
    }

    let hex: String = public_key.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    format!("{}...{}", &hex[..8], &hex[hex.len()-8..])
}
