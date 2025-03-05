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
const EFFECTIVE_ADDRESS_CALCULATION: [(u8, [&str; 3]); 8] = [
    (0b000, ["bx+si", "bx+si", "bx+si"]),
    (0b001, ["bx+di", "bx+di", "bx+di"]),
    (0b010, ["bp+si", "bp+si", "bp+si"]),
    (0b011, ["bp+di", "bp+di", "bp+di"]),
    (0b100, ["si", "si", "si"]),
    (0b101, ["di", "di", "di"]),
    (0b110, ["DIRECT ADDRESS", "bp", "bp"]),
    (0b111, ["bx", "bx", "bx"]),
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
const OPCODEMAP: [(u8, &str); 5] = [
    (0b100010, "mov"),
    (0b1100011, "mov"),
    (0b1011, "mov"),
    (0b1010000, "mov"),
    (0b1010001, "mov"),
];
pub fn decode(bytes: Vec<u8>) {
    let mut idx = 0;
    let len = bytes.len();
    loop {
        if idx < len {
            match bytes[idx] {
                v if (v >> 2) == 0b100010 => {
                    let operation = OPCODEMAP.iter()
                        .find(|element| {
                            return element.0 == (v >> 2);
                        })
                        .unwrap()
                        .clone().1;
                    let dw = (bytes[idx] & 0b000011) as u8;
                    let _mod = (bytes[idx + 1] >> 6) as u8;
                    let reg = ((bytes[idx + 1] & 0b00111000) >> 3) as u8;
                    let rm = (bytes[idx + 1] << 5) >> 5;
                    let d = dw >> 1;
                    let [source, destination] = get_source_destination(dw, reg, rm);
                    match _mod {
                        0b11 => {
                            println!("{operation} {destination} ,{source}");
                            idx += 2;
                        }
                        0b00 => {
                            match rm {
                                0b110 => {
                                    let effective_address =
                                        (bytes[idx + 2] as u16) << (8 + (bytes[idx + 3] as u16));
                                    if d == 1 {
                                        println!(
                                            "{operation} {destination} ,[{effective_address}]"
                                        );
                                    } else {
                                        println!("{operation} [{effective_address}] ,{source}");
                                    }
                                    idx += 4;
                                }
                                _ => {
                                    let effective_address = EFFECTIVE_ADDRESS_CALCULATION.iter()
                                        .find(|element| { element.0 == rm })
                                        .unwrap()
                                        .to_owned().1[_mod as usize];
                                    if d == 1 {
                                        println!(
                                            "{operation} {destination} ,[{effective_address}]"
                                        );
                                    } else {
                                        println!("{operation} [{effective_address}] ,{source}");
                                    }
                                    idx += 2;
                                }
                            }
                        }
                        0b01 => {
                            let effective_address = EFFECTIVE_ADDRESS_CALCULATION.iter()
                                .find(|element| { element.0 == rm })
                                .unwrap()
                                .to_owned().1[_mod as usize];
                            let dlow = bytes[idx + 2];
                            if d == 1 {
                                println!("{operation} {destination} ,[{effective_address}+{dlow}]");
                            } else {
                                println!(
                                    "{operation} [{effective_address}+{dlow}] ,{effective_address}"
                                );
                            }
                            idx += 3;
                        }
                        0b10 => {
                            let effective_address = EFFECTIVE_ADDRESS_CALCULATION.iter()
                                .find(|element| { element.0 == rm })
                                .unwrap()
                                .to_owned().1[_mod as usize];
                            let dlow = bytes[idx + 2] as u16;
                            let dhigh = (bytes[idx + 3] as u16) << 8;
                            let displacement = dhigh + dlow;
                            if d == 1 {
                                println!(
                                    "{operation} {destination} ,[{effective_address}+{displacement}]"
                                );
                            } else {
                                println!(
                                    "{operation} [{effective_address}+{displacement}] ,{source}"
                                );
                            }
                            idx += 4;
                        }
                        _ => {
                            println!("invalid");
                            break;
                        }
                    }
                }
                v if (v >> 4) == 0b1011 => {
                    let w = (v << 4) >> 7;
                    let reg = (v << 5) >> 5;
                    let destination = REGISTER_TABLE.iter()
                        .find(|element| {
                            return element.0 == reg;
                        })
                        .unwrap()
                        .to_owned().1[w as usize];
                    let operation = OPCODEMAP.iter()
                        .find(|element| {
                            return element.0 == (v >> 4);
                        })
                        .unwrap()
                        .to_owned().1;
                    if w == 1 {
                        let data = ((bytes[idx + 2] as u16) << 8) + (bytes[idx + 1] as u16);
                        println!("{operation} {destination}, {data}");
                        idx += 3;
                    } else {
                        let data = bytes[idx + 1];
                        println!("{operation} {destination}, {data}");
                        idx += 2;
                    }
                }
                _ => {
                    println!("not supproted now");
                    break;
                }
            }
        } else {
            break;
        }
    }
}
