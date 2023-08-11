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
}
