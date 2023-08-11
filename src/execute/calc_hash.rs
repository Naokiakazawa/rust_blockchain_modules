use modules::{hash, utils};
use tracing::trace;

pub fn execute_hash() {
    let mut output_hash_buffer: [u8; 32] = [0; 32];
    let input_data: Vec<u8> = String::from("hello").into_bytes();
    hash::get_hash_256::<sha2::Sha256>(&input_data, &mut output_hash_buffer);
    let hash_string: String = utils::hex_to_string(&output_hash_buffer);

    trace!(hash_string, "Hash result 0");

    let input_data_1: Vec<u8> = String::from("HELLO").into_bytes();
    hash::get_hash_256::<sha2::Sha256>(&input_data_1, &mut output_hash_buffer);
    let hash_string_1: String = utils::hex_to_string(&output_hash_buffer);

    trace!(hash_string_1, "Hash result 1");
}
