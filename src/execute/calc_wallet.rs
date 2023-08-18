use dotenv::dotenv;
use modules::{wallet_bitcoin, wallet_ethereum};
use std::env;
use tracing::info;

pub fn create_bitcoin_address() {
    let addr: String = wallet_bitcoin::create_address();
    info!(addr, "BTC Address");
}

pub fn create_ethereum_address() {
    dotenv().ok();
    let input_secret: String = env::var("EVM_PRIVATE_KEY").expect("EVM_PRIVATE_KEY must be set");
    wallet_ethereum::create_address_from_secret(&input_secret);
}
