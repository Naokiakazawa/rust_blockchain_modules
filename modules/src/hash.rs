use anyhow::Result;
use sha2::Digest;

use tracing::info;

pub fn get_hash<D: Digest>(inputs: Vec<String>) -> Result<Vec<u8>> {
    info!("Calcurate single hash");
    
    let mut hasher = D::new();
    for input in inputs.iter() {
        hasher.update(input.as_bytes());
    }

    let hash = hasher.finalize();

    let sz = <D as Digest>::output_size();
    let mut output = vec![0; sz];
    output.copy_from_slice(&hash);

    Ok(output)
}
