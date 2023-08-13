use crate::hash;
use crate::utils;
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Block {
    pub block_hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub nonce: u32,
}

impl Block {
    fn new() -> Self {
        Block {
            block_hash: [0; 32],
            previous_hash: [0; 32],
            nonce: 0,
        }
    }
}

pub fn proof_of_work(height: usize) -> Vec<Block> {
    const TARGET: [u8; 32] = [
        0, 0, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let mut output_hash_buffer: [u8; 32] = [0; 32];
    let mut block_chain: Vec<Block> = Vec::new();

    let mut block: Block = Block::new();

    info!("Start mining");

    loop {
        let input: Vec<u8> = utils::bundle_bytes(vec![
            block.previous_hash.to_vec(),
            utils::convert_u32_vec_u8(block.nonce),
        ]);
        hash::get_hash_256::<sha2::Sha256>(&input, &mut output_hash_buffer);
        let block_hash: [u8; 32] = output_hash_buffer;

        if block.nonce % 10000 == 0 {
            let current_hash: String = utils::hex_to_string(&block_hash);
            let current_nonce: u32 = block.nonce;
            trace!(current_nonce, current_hash, "Mining",);
        }

        if block_hash < TARGET {
            info!("Success mining");
            block.block_hash = block_hash;
            block_chain.push(block);
            // Initialize next block.
            block.block_hash = [0; 32];
            block.previous_hash = block_hash;
            block.nonce = 0;
        } else {
            block.nonce += 1;
        }

        if height == block_chain.len() {
            break block_chain;
        }
    }
}
