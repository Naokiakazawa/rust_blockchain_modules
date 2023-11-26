use modules::hashchain::create_hashchain;
use tracing::info;

pub fn execute_hashchain() {
    let genesis_phase: String =
        String::from("cd372fb85148700fa88095e3492d3f9f5beb43e555e5ff26d95f5a6adc36f8e6");
    let results: Vec<String> = create_hashchain(genesis_phase, 50);
    let length: usize = results.len();
    info!(length, "Hashchain created");
}
