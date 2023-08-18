use crate::{hash, utils};
use hex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tracing::{debug, info};

pub fn create_address_from_secret(secret: &str) -> String {
    let decorded_secret: Vec<u8> = hex::decode(secret).unwrap();
    let s: Secp256k1<secp256k1::All> = Secp256k1::new();

    // generate public key from secret.
    let secret_key: SecretKey =
        SecretKey::from_slice(&decorded_secret).expect("Private key must be 32 bytes");
    let public_key: PublicKey = PublicKey::from_secret_key(&s, &secret_key);

    // prune first 1byte.
    let mut pruned_public_key: [u8; 64] = [0; 64];
    pruned_public_key.copy_from_slice(&public_key.serialize_uncompressed()[1..]);

    // hash public key
    let h: [u8; 32] = hash::get_keccak_256(&pruned_public_key);
    let address_bytes: [u8; 20] = utils::get_last_160bits(h);
    let address: String = utils::hex_to_string(&address_bytes).to_lowercase();
    debug!(address, "address without checksum");

    // checksum
    let address_str: &str = std::str::from_utf8(&address.as_bytes()).expect("Invalid UTF-8 data");
    let r: [u8; 32] = hash::get_keccak_256(address_str.as_bytes());
    let binding: String = utils::hex_to_string(&r).to_lowercase();
    let reference: &str = binding.as_str();
    debug!(reference, "keccak256 hash");

    let mut address_checksum: String = String::from("0x");
    for (i, char) in address.as_str().to_lowercase().chars().enumerate() {
        if u32::from_str_radix(&reference[i..=i], 16).unwrap() >= 8 {
            address_checksum.push(char.to_ascii_uppercase());
        } else {
            address_checksum.push(char);
        }
    }
    info!(address_checksum, "Checksum address");
    address_checksum
}
