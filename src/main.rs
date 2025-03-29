mod decoder;
mod instruction;
mod cpu;
use cpu::Cpu;
use decoder::decode;
use std::env;
use std::fs::File;
use std::io::{ BufReader, Read };
use std::process::exit;

fn main() {
    let env: Vec<String> = env::args().collect();
    if env.len() != 2 {
        println!("Please provide the binary files to simulate");
        exit(64);
    }
    let mut cpu = Cpu::new();
    let my_buf = read_binary_file(String::from(&env[1]));

    let instructions = decode(&my_buf);

    for inst in &instructions {
        println!("{:?}", inst);
        cpu.execute(inst);
    }
    println!("{:?}", cpu)
}

fn read_binary_file(filepath: String) -> Vec<u8> {
    let my_buf = BufReader::new(File::open(filepath).unwrap());

    let mut buf: Vec<u8> = vec![];
    for i in my_buf.bytes() {
        buf.push(i.unwrap());
    }
    return buf;
}
