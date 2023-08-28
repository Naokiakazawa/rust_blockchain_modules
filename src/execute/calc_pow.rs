use modules::{pow, pow_v2};
use modules::{pow_multithread, utils};
use tracing::info;

pub fn execute_pow() {
    let blocks: Vec<pow::Block> = pow::proof_of_work(5);
    for block in blocks.into_iter() {
        let hash: String = utils::hex_to_string(&block.block_hash);
        let nonce: u32 = block.nonce;
        let elapsed_time: u128 = block.elapsed_time;
        info!(hash, nonce, elapsed_time, "Mining Result");
    }
}

pub fn execute_pow_v2() {
    let blocks: Vec<pow_v2::Block> = pow_v2::proof_of_work(5);
    for block in blocks.into_iter() {
        let hash: String = utils::hex_to_string(&block.block_hash);
        let nonce: u32 = block.nonce;
        let elapsed_time: u128 = block.elapsed_time;
        info!(hash, nonce, elapsed_time, "Mining Result");
    }
}

pub fn execute_pow_multithread() {
    let blocks: Vec<pow_multithread::Block> = pow_multithread::proof_of_work(5, 4);
    for block in blocks.into_iter() {
        let hash: String = utils::hex_to_string(&block.block_hash);
        let nonce: u32 = block.nonce;
        let elapsed_time: u128 = block.elapsed_time;
        info!(hash, nonce, elapsed_time, "Mining Result");
    }
}
