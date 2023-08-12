use modules::pow;
use modules::utils;
use tracing::info;

pub fn execute_pow() {
    let results: Vec<[u8; 32]> = pow::proof_of_work(5);
    for r in results.into_iter() {
        let hash: String = utils::hex_to_string(&r);
        info!(hash, "Mining Result");
    }
}
