use sha2::Sha256;

mod hash;

fn main() {
    let input_data: String = String::from("hello");
    let inputs_data: Vec<String> = vec![input_data];

    let output_hash: Vec<u8> = hash::get_hash::<Sha256>(inputs_data).unwrap();
    let hash_string: String = hash::hex_to_string(&output_hash);

    println!("{:?}", output_hash);
    println!("{}", hash_string);
}
