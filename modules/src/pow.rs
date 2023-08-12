use crate::hash;
use crate::utils;
use tracing::{info, trace};

pub struct Block {
    previous_hash: [u8; 32],
    nonce: u32,
}

impl Block {
    fn new() -> Self {
        Block {
            previous_hash: [0; 32],
            nonce: 0,
        }
    }
}

pub fn proof_of_work(height: usize) -> Vec<[u8; 32]> {
    const TARGET: [u8; 32] = [
        0, 0, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let mut output_hash_buffer: [u8; 32] = [0; 32];
    let mut hash_chain: Vec<[u8; 32]> = Vec::new();

    let mut block: Block = Block::new();

    info!("Start mining");

    loop {
        let input: Vec<u8> = utils::bundle_bytes(vec![
            block.previous_hash.to_vec(),
            utils::convert_u32_vec_u8(block.nonce),
        ]);
        hash::get_hash_256::<sha2::Sha256>(&input, &mut output_hash_buffer);
        let block_hash: [u8; 32] = output_hash_buffer;

        if block.nonce % 1000 == 0 {
            let current_hash = utils::hex_to_string(&block_hash);
            trace!(block.nonce, current_hash, "Mining",);
        }

        if block_hash < TARGET {
            info!("Success mining");
            hash_chain.push(block_hash);
            block.previous_hash = block_hash;
            block.nonce = 0;
        } else {
            block.nonce += 1;
        }

        if height == hash_chain.len() {
            break hash_chain;
        }
    }
}
