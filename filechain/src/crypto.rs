use blake3;
use ed25519_dalek::{Keypair, Signature, Signer};
use hex;

pub fn hash_str(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}

pub fn sign(data: &str, keypair: &Keypair) -> String {
    let sig: Signature = keypair.sign(data.as_bytes());
    hex::encode(sig.to_bytes())
}

pub fn hash_events(events: &[crate::models::FileEvent]) -> String {
    let mut combined = String::new();
    for e in events {
        combined.push_str(&serde_json::to_string(e).unwrap());
    }
    hash_str(&combined)
}
