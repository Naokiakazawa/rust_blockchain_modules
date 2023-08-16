use crate::hash;
use crate::utils::hex_to_string;
use bs58;
use rand::{self, Rng};
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub fn create_address() -> String {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let _random_number: [u8; 32] = rng.gen();
    create_address_from_secret(&_random_number)
}

fn create_address_from_secret(secret: &[u8; 32]) -> String {
    let mut output_buffer_sha256: [u8; 32] = [0; 32];
    let mut output_buffer_ripemd: [u8; 20] = [0; 20];
    let s: Secp256k1<secp256k1::All> = Secp256k1::new();

    let secret_key: SecretKey = SecretKey::from_slice(secret).expect("32 bytes");
    let public_key: PublicKey = PublicKey::from_secret_key(&s, &secret_key);
    println!("{:?}", &secret_key.display_secret());
    println!("{:?}", &public_key.serialize());
    println!("{:?}", &public_key.serialize_uncompressed());

    hash::get_hash_256::<sha2::Sha256>(
        &public_key.serialize_uncompressed(),
        &mut output_buffer_sha256,
    );
    hash::get_hash_ripemd(&output_buffer_sha256, &mut output_buffer_ripemd);
    let payload: [u8; 20] = output_buffer_ripemd;
    println!("{:?}", &payload);

    const PREFIX: [u8; 1] = [0];

    let mut prefix_payload: [u8; 21] = [0; 21];
    prefix_payload[0] = PREFIX[0];
    prefix_payload[1..].copy_from_slice(&payload);

    // TODO: double hash
    hash::get_hash_256::<sha2::Sha256>(&prefix_payload, &mut output_buffer_sha256);
    let h: [u8; 32] = output_buffer_sha256;
    let checksum: [u8; 4] = [h[0], h[1], h[2], h[3]];

    let mut prefix_payload_checksum: [u8; 25] = [0; 25];
    prefix_payload_checksum[..21].copy_from_slice(&prefix_payload);
    prefix_payload_checksum[21..].copy_from_slice(&checksum);
    println!("{:?}", prefix_payload_checksum);
    println!("{:?}", hex_to_string(&prefix_payload_checksum));

    bs58::encode(prefix_payload_checksum).with_alphabet(bs58::Alphabet::BITCOIN).into_string()

}
