use modules::{pow, pow_v2};
use modules::{pow_multithread, utils};
use std::thread;
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
    let num = thread::available_parallelism().unwrap().get();
    let blocks: Vec<pow_multithread::Block> = pow_multithread::proof_of_work(5, num as u32);
    for block in blocks.into_iter() {
        let hash: String = utils::hex_to_string(&block.block_hash);
        let nonce: u32 = block.nonce;
        let elapsed_time: u128 = block.elapsed_time;
        info!(hash, nonce, elapsed_time, "Mining Result");
    }
}

pub fn compare_outputs() {
    const HEIGHT: usize = 5;
    let num = thread::available_parallelism().unwrap().get();

    let blocks_singlethread: Vec<pow::Block> = pow::proof_of_work(HEIGHT);
    let blocks_multithread: Vec<pow_multithread::Block> =
        pow_multithread::proof_of_work(HEIGHT, num as u32);
    let mut single_time_sum: u128 = 0;
    let mut multi_time_sum: u128 = 0;

    for i in 0..HEIGHT {
        assert!(
            blocks_singlethread[i].block_hash == blocks_multithread[i].block_hash,
            "single_output {:?}, multi_output{:?}",
            utils::hex_to_string(&blocks_singlethread[i].block_hash),
            utils::hex_to_string(&blocks_multithread[i].block_hash)
        );
        assert!(
            blocks_singlethread[i].nonce == blocks_multithread[i].nonce,
            "single_output {}, multi_output{}",
            blocks_singlethread[i].nonce,
            blocks_multithread[i].nonce
        );
        single_time_sum += blocks_singlethread[i].elapsed_time;
        multi_time_sum += blocks_multithread[i].elapsed_time;
    }
    info!(single_time_sum, multi_time_sum, "Elapsed time detail");
}
