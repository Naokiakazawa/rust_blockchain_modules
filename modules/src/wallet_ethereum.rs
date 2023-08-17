use crate::{hash, utils};
use hex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub fn create_address_from_secret(secret: &str) {
    let decorded_secret: Vec<u8> = hex::decode(secret).unwrap();
    let mut output_buffer_256: [u8; 32] = [0; 32];
    let s: Secp256k1<secp256k1::All> = Secp256k1::new();

    // generate public key from secret.
    let secret_key: SecretKey = SecretKey::from_slice(&decorded_secret).expect("32 bytes");
    let public_key: PublicKey = PublicKey::from_secret_key(&s, &secret_key);

    // prune first 1byte.
    let mut pruned_public_key: [u8; 64] = [0; 64];
    pruned_public_key.copy_from_slice(&public_key.serialize_uncompressed()[1..]);

    // hash public key
    hash::get_keccak_256(&pruned_public_key, &mut output_buffer_256);
    println!(
        "{:?}",
        utils::hex_to_string(&utils::get_last_160bits(output_buffer_256))
    );
}
