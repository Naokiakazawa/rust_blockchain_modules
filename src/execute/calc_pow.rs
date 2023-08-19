use modules::pow;
use modules::utils;
use tracing::info;

pub fn execute_pow() {
    let blocks: Vec<pow::Block> = pow::proof_of_work(5);
    for block in blocks.into_iter() {
        let hash: String = utils::hex_to_string(&block.block_hash);
        let nonce: u32 = block.nonce;
        let timestamp: u32 = block.timestamp;
        let elapsed_time: u128 = block.elapsed_time;
        info!(hash, timestamp, elapsed_time, nonce, "Mining Result");
    }
}
