use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

mod execute;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Execute Hash");
    execute::calc_hash::execute_hash();
    info!("Execute Hashchain");
    execute::calc_hashchain::execute_hashchain();
    info!("Execute Merkle tree");
    execute::calc_merkle_root::execute_merkletree();
    info!("Create Bitcoin Wallet Address");
    execute::calc_wallet::create_bitcoin_address();
    info!("Create Ethereum Wallet Address");
    execute::calc_wallet::create_ethereum_address();
    info!("Execute PoW");
    execute::calc_pow::execute_pow();
    info!("Execute PoW v2");
    execute::calc_pow::execute_pow_v2();
    info!("Execute PoW Multithread");
    execute::calc_pow::execute_pow_multithread();
    info!("Compare single thread pow and multithread pow");
    execute::calc_pow::compare_outputs();
}
