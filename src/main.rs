mod decoder;
mod instruction;
mod cpu;
use cpu::Cpu;
use decoder::decode;
use std::env;
use std::fs::File;
use std::io::{ Error, Read };
use std::process::exit;

fn main() {
    let env: Vec<String> = env::args().collect();
    if env.len() != 2 {
        println!("Please provide the binary files to simulate");
        exit(64);
    }
    let mut cpu = Cpu::new();
    let size = read_binary_file(String::from(&env[1]), &mut cpu).unwrap();
    while cpu.registers.ip < (size as u16) {
        let instruction = decode(&mut cpu);
        println!("{:?} ,IP:{}", instruction, cpu.registers.ip);
        cpu.execute(&instruction);
    }
    println!("{:?}", cpu.registers)
}

fn read_binary_file(filepath: String, cpu: &mut Cpu) -> Result<usize, Error> {
    let mut file = File::open(filepath).unwrap();

    file.read(&mut cpu.memory)
}
