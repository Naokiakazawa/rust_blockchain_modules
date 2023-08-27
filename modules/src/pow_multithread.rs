use crate::hash;
use crate::utils;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::{Duration, Instant};
use rayon::prelude::*;
use tracing::{info, trace};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Block {
    pub block_hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub timestamp: u32,
    pub nonce: u32,
    // pub elapsed_time: u128,
}

impl Block {
    fn new(initial_nonce: u32) -> Self {
        Block {
            block_hash: [0; 32],
            previous_hash: [0; 32],
            timestamp: Utc::now().timestamp() as u32,
            nonce: initial_nonce,
            // elapsed_time: 0,
        }
    }
}

pub fn proof_of_work(height: usize, threads: u32) -> Vec<Block> {
    let target: Arc<[u8; 32]> = Arc::new([
        0, 0, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let found: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let block_chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));

    info!("Start mining");

    (0..threads).into_par_iter().for_each_with(
        (found.clone(), target.clone(), block_chain.clone()),
        |(found, target, block_chain), i| {
            let mut output_hash_buffer: [u8; 32] = [0; 32];
            let mut block: Block = Block::new(i);
            loop {
                let mut found_guard = found.lock().unwrap();
                if block_chain.lock().unwrap().len() == height {
                    break;
                }

                if *found_guard == true {
                    block.block_hash = [0; 32];
                    block.previous_hash = block_chain.lock().unwrap().last().unwrap().block_hash;
                    block.nonce = i;
                    block.timestamp = Utc::now().timestamp() as u32;
                }

                let input: Vec<u8> = utils::bundle_bytes(vec![
                    block.previous_hash.to_vec(),
                    utils::convert_u32_vec_u8(block.nonce),
                ]);
                hash::get_hash_256::<sha2::Sha256>(&input, &mut output_hash_buffer);
                let block_hash: [u8; 32] = output_hash_buffer;

                if block.nonce % 10000 == 0 {
                    let current_hash: String = utils::hex_to_string(&block_hash);
                    let current_nonce: u32 = block.nonce;
                    trace!(i, current_nonce, current_hash, "Mining",);
                }

                if block_hash < **target && *found_guard == false {
                    *found_guard = true;
                    block.block_hash = block_hash;
                    block_chain.lock().unwrap().push(block);
                    info!(i, "Success mining");
                    *found_guard = false;
                    block.block_hash = [0; 32];
                    block.previous_hash = block_hash;
                    block.nonce = i;
                    block.timestamp = Utc::now().timestamp() as u32;    
                } else {
                    block.nonce += threads;
                }
            }
        },
    );
    let result: Vec<Block> = block_chain.lock().unwrap().clone();
    result
}
