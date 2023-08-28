use crate::hash;
use crate::utils;
use crate::utils::hex_to_string;
use std::collections::VecDeque;
use tracing::trace;

pub fn create_merkle(inputs: Vec<String>) -> Vec<u8> {
    let mut output_hash_buffer: [u8; 32] = [0; 32];
    let mut leaf_hashes: VecDeque<Vec<u8>> = inputs
        .iter()
        .cloned()
        .map(|input| {
            let h: Vec<u8> = input.into_bytes();
            hash::get_hash_256::<sha2::Sha256>(&h, &mut output_hash_buffer);
            output_hash_buffer.to_vec()
        })
        .collect();

    while leaf_hashes.len() > 1 {
        if leaf_hashes.len() % 2 != 0 {
            let last: Vec<u8> = leaf_hashes.back().unwrap().clone();
            leaf_hashes.push_back(last);
        }

        let mut inner_hashes: VecDeque<Vec<u8>> = VecDeque::<Vec<u8>>::new();
        while let (Some(left), Some(right)) = (leaf_hashes.pop_front(), leaf_hashes.pop_front()) {
            let input: Vec<u8> = utils::bundle_bytes(vec![left, right]);
            hash::get_hash_256::<sha2::Sha256>(&input, &mut output_hash_buffer);
            inner_hashes.push_back(output_hash_buffer.to_vec());
        }
        leaf_hashes = inner_hashes;
    }
    let root: Vec<u8> = leaf_hashes.pop_front().unwrap_or_default();
    let merkle_root: String = hex_to_string(&root);
    trace!(merkle_root, "Calcurated merkle root");
    root
}
