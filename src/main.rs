mod decoder;
mod instruction;

use decoder::decode;
use std::fs::File;
use std::io::{ BufReader, Read };

fn main() {
    let my_buf = read_binary_file(String::from("./assets/listing_0041_add_sub_cmp_jnz"));

    let instructions = decode(&my_buf);

    for inst in &instructions {
        println!("{:?}", inst);
    }
}

fn read_binary_file(filepath: String) -> Vec<u8> {
    let my_buf = BufReader::new(File::open(filepath).unwrap());

    let mut buf: Vec<u8> = vec![];
    for i in my_buf.bytes() {
        buf.push(i.unwrap());
    }
    return buf;
}
