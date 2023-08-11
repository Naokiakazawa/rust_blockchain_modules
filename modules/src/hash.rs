use sha2::Digest;

pub fn get_hash_256<D: Digest>(input: &[u8], output: &mut [u8; 32]) -> () {
    let mut hasher = D::new();
    hasher.update(input);
    let hash = hasher.finalize();

    output.copy_from_slice(&hash);
}
