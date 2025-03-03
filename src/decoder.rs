pub struct DoubleByteInstruction {
    pub opcode: u8,
    pub dw: u8,
    pub _mod: u8,
    pub reg: u8,
    pub rm: u8,
}
pub struct instruction {}
const REGISTER_TABLE: [(u8, [&str; 2]); 8] = [
    (0b000, ["al", "ax"]),
    (0b001, ["cl", "cx"]),
    (0b010, ["dl", "dx"]),
    (0b011, ["bl", "bx"]),
    (0b100, ["ah", "sp"]),
    (0b101, ["ch", "bp"]),
    (0b110, ["dh", "si"]),
    (0b111, ["bh", "di"]),
];
fn get_source_destination(dw: u8, reg: u8, rm: u8) -> [String; 2] {
    let d = dw >> 1;
    let w = dw & 0b00000001;
    match d {
        0 => [get_register(reg, w), get_register(rm, w)],
        1 => [get_register(rm, w), get_register(reg, w)],
        _ => [String::from("invalid source"), String::from("Invalid source")],
    }
}
fn get_register(idx: u8, w: u8) -> String {
    let reg = REGISTER_TABLE.iter()
        .find(|element| {
            return element.0 == idx;
        })
        .unwrap()
        .to_owned().1[w as usize];
    return String::from(reg);
}
const OPCODEMAP: [(u8, &str); 1] = [(0b100010, "mov")];
pub fn decode(byte1: u8, byte2: u8) {
    let instruction = DoubleByteInstruction {
        opcode: (byte1 >> 2) as u8,
        dw: (byte1 & 0b000011) as u8,
        _mod: (byte2 >> 6) as u8,
        reg: ((byte2 & 0b00111000) >> 3) as u8,
        rm: (byte2 << 5) >> 5,
    };
    let operation = OPCODEMAP.iter()
        .find(|element| {
            return element.0 == instruction.opcode;
        })
        .unwrap()
        .clone().1;
    let [source, destination] = get_source_destination(
        instruction.dw,
        instruction.reg,
        instruction.rm
    );
    println!("{operation} {destination} ,{source}")
}
