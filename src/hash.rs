use anyhow::Result;
use sha2::Digest;

pub fn get_hash<D: Digest>(inputs: Vec<String>) -> Result<Vec<u8>> {
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

pub fn hex_to_string(data: &[u8]) -> String {
    let mut ret = String::new();

    for d in data {
        let x = format!("{:02x}", d);
        ret.push_str(&x);
    }

    ret
}
