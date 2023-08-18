use keccak_hash::keccak_256;
use ripemd::Ripemd160;
use sha2::Digest;
use tracing::debug;

pub fn get_hash_256<D: Digest>(input: &[u8], output: &mut [u8; 32]) {
    let mut hasher = D::new();
    hasher.update(input);
    let hash = hasher.finalize();

    output.copy_from_slice(&hash);
}

pub fn get_hash_double_256<D: Digest>(input: &[u8], output: &mut [u8; 32]) {
    let mut intermediate_hash: [u8; 32] = [0; 32];

    let mut hasher = D::new();
    hasher.update(input);
    let hash_1 = hasher.finalize();
    intermediate_hash.copy_from_slice(&hash_1);

    let mut hasher = D::new();
    hasher.update(intermediate_hash);
    let hash_2 = hasher.finalize();
    output.copy_from_slice(&hash_2);
}

pub fn get_hash_ripemd(input: &[u8], output: &mut [u8; 20]) {
    let mut hasher = Ripemd160::new();
    hasher.update(input);
    let hash = hasher.finalize();

    output.copy_from_slice(&hash);
}

pub fn get_keccak_256(input: &[u8]) -> [u8; 32] {
    let mut output_buffer_256: [u8; 32] = [0; 32];
    keccak_256(input, &mut output_buffer_256);
    output_buffer_256
}
