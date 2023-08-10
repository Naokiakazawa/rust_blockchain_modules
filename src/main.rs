use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

mod execute;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    execute::calc_hash::execute_hash();
}
