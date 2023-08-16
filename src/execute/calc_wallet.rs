use modules::{wallet_bitcoin, wallet_ethereum};
use tracing::info;

pub fn create_bitcoin_address() {
    let addr: String = wallet_bitcoin::create_address();
    info!(addr, "BTC Address");
}

pub fn create_ethereum_address() {}
