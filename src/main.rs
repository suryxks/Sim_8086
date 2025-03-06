mod decoder;
use decoder::decode;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let my_buf = read_binary_file(String::from("./assets/listing_0037_single_register_mov"));
    decode(my_buf);
    let my_buf = read_binary_file(String::from("./assets/listing_0038_many_register_mov"));
    decode(my_buf);
    let mov = read_binary_file(String::from("./assets/listing_0039_more_movs"));
    decode(mov);
    let challenge_movs = read_binary_file(String::from("./assets/listing_0040_challenge_movs"));
    decode(challenge_movs);
}
fn read_binary_file(filepath: String) -> Vec<u8> {
    let my_buf = BufReader::new(File::open(filepath).unwrap());

    let mut buf: Vec<u8> = vec![];
    for i in my_buf.bytes() {
        buf.push(i.unwrap());
    }
    return buf;
}
