use crate::hash;
use crate::utils;

use chrono::Utc;
use tracing::{info, trace};

pub fn create_hashchain(genesis: String, height: usize) -> Vec<String> {
    let mut hashchain: Vec<String> = vec![genesis];
    while hashchain.len() < height {
        let last: &String = hashchain.last().unwrap();
        let timestamp: String = Utc::now().format("%s").to_string();
        let inputs: Vec<String> = vec![last.to_owned(), timestamp];
        let best_hash: Vec<u8> = hash::get_hash::<sha2::Sha256>(inputs).unwrap();
        let best_hash_string: String = utils::hex_to_string(&best_hash);
        trace!(best_hash_string, "Pushed new hash");
        hashchain.push(best_hash_string);
    }
    info!("Finished");
    hashchain
}
