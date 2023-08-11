use crate::hash;
use crate::utils;

use chrono::Utc;
use tracing::{info, trace};

pub fn create_hashchain(genesis: String, height: usize) -> Vec<String> {
    let mut hashchain: Vec<String> = vec![genesis];
    let mut output_hash_buffer: [u8; 32] = [0; 32];
    while hashchain.len() < height {
        let last: Vec<u8> = hashchain.last().unwrap().clone().into_bytes();
        let timestamp: Vec<u8> = Utc::now().format("%s").to_string().into_bytes();
        let input: Vec<u8> = utils::bundle_bytes(vec![last, timestamp]);
        hash::get_hash_256::<sha2::Sha256>(&input, &mut output_hash_buffer);
        let hash: String = utils::hex_to_string(&output_hash_buffer);
        trace!(hash, "Pushed new hash");
        hashchain.push(hash);
    }
    info!("Finished");
    hashchain
}
