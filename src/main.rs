mod decoder;
use decoder::decode;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let mut my_buf = BufReader::new(
        File::open("./assets/listing_0037_single_register_mov").unwrap()
    );

    let mut buf: [u8; 2] = [0; 2];
    let res = my_buf.read(&mut buf);
    match res {
        Ok(n) => {
            println!("single instruction:");
            decode(buf[0], buf[1]);
        }

        Err(s) => {
            println!("{s}");
        }
    }
    let my_buf = BufReader::new(File::open("./assets/listing_0038_many_register_mov").unwrap());

    let mut buf: Vec<u8> = vec![];
    for i in my_buf.bytes() {
        buf.push(i.unwrap());
    }
    let mut count = 0;
    let buf_len = buf.len();
    println!("");
    println!("Multiple instructions:");
    loop {
        if count < buf_len {
            decode(buf[count], buf[count + 1]);
            count += 2;
        } else {
            break;
        }
    }
}
