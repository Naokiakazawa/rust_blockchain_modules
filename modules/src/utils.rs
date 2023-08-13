pub fn hex_to_string(data: &[u8]) -> String {
    let mut ret = String::new();

    for d in data {
        let x = format!("{:02x}", d);
        ret.push_str(&x);
    }

    ret
}

pub fn bundle_bytes(inputs: Vec<Vec<u8>>) -> Vec<u8> {
    let length: usize = inputs.iter().map(|v| v.len()).sum();
    let mut result: Vec<u8> = Vec::with_capacity(length);

    for input in inputs {
        result.extend(input);
    }

    result
}

pub fn convert_u32_vec_u8(value: u32) -> Vec<u8> {
    vec![
        ((value >> 24) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ]
}
