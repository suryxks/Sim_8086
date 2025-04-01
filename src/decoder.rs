use std::usize;

use crate::{ cpu::Cpu, instruction::{ AddressingMode, Instruction, Operand, Operation, Register } };

// Original tables for reference
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

fn register_from_index(idx: u8, w: u8) -> Register {
    match (idx, w) {
        (0b000, 0) => Register::AL,
        (0b000, 1) => Register::AX,
        (0b001, 0) => Register::CL,
        (0b001, 1) => Register::CX,
        (0b010, 0) => Register::DL,
        (0b010, 1) => Register::DX,
        (0b011, 0) => Register::BL,
        (0b011, 1) => Register::BX,
        (0b100, 0) => Register::AH,
        (0b100, 1) => Register::SP,
        (0b101, 0) => Register::CH,
        (0b101, 1) => Register::BP,
        (0b110, 0) => Register::DH,
        (0b110, 1) => Register::SI,
        (0b111, 0) => Register::BH,
        (0b111, 1) => Register::DI,
        _ => panic!("Invalid register index: {}, {}", idx, w),
    }
}

fn operation_from_opcode(opcode: u8) -> Operation {
    match opcode {
        0b100010 => Operation::Mov, // Register/memory to/from register
        0b1100011 => Operation::Mov, // Immediate to register/memory
        0b1011 => Operation::Mov, // Immediate to register
        0b1010000 => Operation::Mov, // Memory to accumulator
        0b1010001 => Operation::Mov, // Accumulator to memory
        0b000000 => Operation::Add, // Register/memory with register to either
        0b100000 => Operation::Add, // Immediate to register/memory
        0b0000010 => Operation::Add, // Immediate to accumulator
        0b001010 => Operation::Sub, // Register/memory with register to either
        0b100000 => Operation::Sub, // Immediate to register/memory (same opcode as ADD, but reg field differs)
        0b0010110 => Operation::Sub, // Immediate to accumulator
        0b001110 => Operation::Cmp, // Register/memory with register
        0b100000 => Operation::Cmp, // Immediate with register/memory (same opcode as ADD, but reg field differs)
        0b0011110 => Operation::Cmp, // Immediate with accumulator
        0b01110100 => Operation::Jz, // Jump if zero/equal
        0b01110101 => Operation::Jnz, // Jump not zero/not equal
        0b01111100 => Operation::Jl, // Jump if less
        0b01111110 => Operation::Jle, // Jump if less or equal
        0b01110010 => Operation::Jb, // Jump if below
        0b01110110 => Operation::Jbe, // Jump if below or equal
        0b01111010 => Operation::Jp, // Jump if parity
        0b01110000 => Operation::Jo, // Jump if overflow
        0b01111000 => Operation::Js, // Jump if sign
        0b01110101 => Operation::Jne, // Jump if not equal
        0b01111101 => Operation::Jnl, // Jump if not less
        0b01111111 => Operation::Jg, // Jump if greater
        0b01110011 => Operation::Jnb, // Jump if not below
        0b01110111 => Operation::Ja, // Jump if above
        0b01111011 => Operation::Jnp, // Jump if not parity
        0b01110001 => Operation::Jno, // Jump if not overflow
        0b01111001 => Operation::Jns, // Jump if not sign
        0b11100010 => Operation::Loop, // Loop
        0b11100001 => Operation::Loopz, // Loop if zero
        0b11100000 => Operation::Loopnz, // Loop if not zero
        0b11100011 => Operation::Jcxz, // Jump if CX is zero
        _ => panic!("Unsupported operation code: {:b}", opcode),
    }
}

fn decode_rm_operand(mod_val: u8, rm: u8, w: u8, bytes: &[u8], offset: usize) -> (Operand, usize) {
    match mod_val {
        0b11 => {
            // Register mode
            (Operand::Register(register_from_index(rm, w)), 0)
        }
        0b00 => {
            // Memory mode, no displacement (except direct address)
            if rm == 0b110 {
                // Direct address
                let addr = ((bytes[offset + 1] as u16) << 8) | (bytes[offset] as u16);
                (Operand::Memory(AddressingMode::Direct(addr.try_into().unwrap())), 2)
            } else {
                let memory = match rm {
                    0b000 =>
                        AddressingMode::Memory {
                            base: Some(Register::BX),
                            index: Some(Register::SI),
                            displacement: None,
                        },
                    0b001 =>
                        AddressingMode::Memory {
                            base: Some(Register::BX),
                            index: Some(Register::DI),
                            displacement: None,
                        },
                    0b010 =>
                        AddressingMode::Memory {
                            base: Some(Register::BP),
                            index: Some(Register::SI),
                            displacement: None,
                        },
                    0b011 =>
                        AddressingMode::Memory {
                            base: Some(Register::BP),
                            index: Some(Register::DI),
                            displacement: None,
                        },
                    0b100 =>
                        AddressingMode::Memory {
                            base: None,
                            index: Some(Register::SI),
                            displacement: None,
                        },
                    0b101 =>
                        AddressingMode::Memory {
                            base: None,
                            index: Some(Register::DI),
                            displacement: None,
                        },
                    0b111 =>
                        AddressingMode::Memory {
                            base: Some(Register::BX),
                            index: None,
                            displacement: None,
                        },
                    _ => unreachable!(),
                };
                (Operand::Memory(memory), 0)
            }
        }
        0b01 => {
            // Memory mode, 8-bit displacement
            let displacement = bytes[offset] as i8 as i16; // Sign extend to 16 bits
            let memory = match rm {
                0b000 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b001 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b010 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b011 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b100 =>
                    AddressingMode::Memory {
                        base: None,
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b101 =>
                    AddressingMode::Memory {
                        base: None,
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b110 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: None,
                        displacement: Some(displacement),
                    },
                0b111 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: None,
                        displacement: Some(displacement),
                    },
                _ => unreachable!(),
            };
            (Operand::Memory(memory), 1)
        }
        0b10 => {
            // Memory mode, 16-bit displacement
            let displacement = (((bytes[offset + 1] as u16) << 8) | (bytes[offset] as u16)) as i16;
            let memory = match rm {
                0b000 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b001 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b010 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b011 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b100 =>
                    AddressingMode::Memory {
                        base: None,
                        index: Some(Register::SI),
                        displacement: Some(displacement),
                    },
                0b101 =>
                    AddressingMode::Memory {
                        base: None,
                        index: Some(Register::DI),
                        displacement: Some(displacement),
                    },
                0b110 =>
                    AddressingMode::Memory {
                        base: Some(Register::BP),
                        index: None,
                        displacement: Some(displacement),
                    },
                0b111 =>
                    AddressingMode::Memory {
                        base: Some(Register::BX),
                        index: None,
                        displacement: Some(displacement),
                    },
                _ => unreachable!(),
            };
            (Operand::Memory(memory), 2)
        }
        _ => unreachable!(),
    }
}

pub fn decode(cpu: &mut Cpu) -> Instruction {
    let mut offset = cpu.registers.ip as usize;
    let start_offset = offset;
    let buffer = &cpu.memory;
    let current_byte = buffer[offset];

    // Match different instruction patterns
    let instruction = match current_byte {
        // Reg/Mem with Register to either (MOV, ADD, SUB, CMP, etc.)
        v if
            (v >> 2) == 0b100010 || // MOV r/m, r/r, r/m
            (v >> 2) == 0b000000 || // ADD r/m, r/r, r/m
            (v >> 2) == 0b001010 || // SUB r/m, r/r, r/m
            (v >> 2) == 0b001110 // CMP r/m, r/r, r/m
        => {
            let opcode = v >> 2;
            let d = (v >> 1) & 0b1; // Direction bit
            let w = v & 0b1; // Word/byte bit

            offset += 1;

            let mod_val = (buffer[offset] >> 6) & 0b11;
            let reg = (buffer[offset] >> 3) & 0b111;
            let rm = buffer[offset] & 0b111;

            offset += 1;

            // Get register operand
            let reg_operand = Operand::Register(register_from_index(reg, w));

            // Get r/m operand and update offset
            let (rm_operand, additional_bytes) = decode_rm_operand(
                mod_val,
                rm,
                w,
                &buffer[offset..],
                0
            );
            offset += additional_bytes;

            // Determine source and destination based on direction bit
            let (source, destination) = if d == 0 {
                (reg_operand, rm_operand)
            } else {
                (rm_operand, reg_operand)
            };

            let operation = operation_from_opcode(opcode);
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Immediate to Register (MOV)
        v if (v >> 4) == 0b1011 => {
            let w = (v >> 3) & 0b1; // Word/byte bit
            let reg = v & 0b111; // Register field

            offset += 1;

            let immediate = if w == 1 {
                let value = ((buffer[offset + 1] as u16) << 8) | (buffer[offset] as u16);
                offset += 2;
                value
            } else {
                let value = buffer[offset] as u16;
                offset += 1;
                value
            };

            let operation = Operation::Mov;
            let destination = Operand::Register(register_from_index(reg, w));
            let source = Operand::Immediate(immediate as i16);
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Immediate to Register/Memory (MOV, ADD, SUB, CMP)
        v if (v >> 1) == 0b1100011 || (v & 0b11111100) == 0b10000000 => {
            let op_type = if (v >> 1) == 0b1100011 {
                // C6/C7 - MOV immediate to r/m
                Operation::Mov
            } else {
                // 80/81/83 - need to check reg field for specific operation
                let next_offset = offset + 1;

                let reg_field = (buffer[next_offset] >> 3) & 0b111;
                match reg_field {
                    0b000 => Operation::Add,
                    0b101 => Operation::Sub,
                    0b111 => Operation::Cmp,
                    _ => panic!("Unsupported reg field for immediate group: {}", reg_field),
                }
            };

            let w = v & 0b1; // Word/byte bit
            let s = if (v >> 1) == 0b1100011 { 0 } else { (v >> 1) & 0b1 }; // Sign extend for 80/81/83

            offset += 1;

            let mod_val = (buffer[offset] >> 6) & 0b11;
            let rm = buffer[offset] & 0b111;

            offset += 1;

            // Get r/m operand and update offset
            let (destination, additional_bytes) = decode_rm_operand(
                mod_val,
                rm,
                w,
                &buffer[offset..],
                0
            );
            offset += additional_bytes;

            let immediate = if w == 1 && s == 0 {
                // 16-bit immediate

                let value = ((buffer[offset + 1] as u16) << 8) | (buffer[offset] as u16);
                offset += 2;
                value
            } else {
                // 8-bit immediate (sign-extended if needed)

                let value = if s == 1 && w == 1 {
                    // Sign-extend 8-bit to 16-bit
                    buffer[offset] as i8 as i16 as u16
                } else {
                    buffer[offset] as u16
                };
                offset += 1;
                value
            };

            let source = Operand::Immediate(immediate as i16);
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation: op_type,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Memory to Accumulator (MOV)
        v if (v >> 1) == 0b1010000 => {
            let w = v & 0b1;

            offset += 1;

            let addr = if w == 1 {
                let value = ((buffer[offset + 1] as u16) << 8) | (buffer[offset] as u16);
                offset += 2;
                value
            } else {
                let value = buffer[offset] as u16;
                offset += 1;
                value
            };

            let operation = Operation::Mov;
            let destination = Operand::Register(if w == 1 { Register::AX } else { Register::AL });
            let source = Operand::Memory(AddressingMode::Direct(addr as i16));
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Accumulator to Memory (MOV)
        v if (v >> 1) == 0b1010001 => {
            let w = v & 0b1;

            offset += 1;

            let addr = if w == 1 {
                let value = ((buffer[offset + 1] as u16) << 8) | (buffer[offset] as u16);
                offset += 2;
                value
            } else {
                let value = buffer[offset] as u16;
                offset += 1;
                value
            };

            let operation = Operation::Mov;
            let source = Operand::Register(if w == 1 { Register::AX } else { Register::AL });
            let destination = Operand::Memory(AddressingMode::Direct(addr as i16));
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Immediate to Accumulator (ADD, SUB, CMP)
        v if
            v == 0b00000100 ||
            v == 0b00000101 || // ADD AL, imm8 / ADD AX, imm16
            v == 0b00101100 ||
            v == 0b00101101 || // SUB AL, imm8 / SUB AX, imm16
            v == 0b00111100 ||
            v == 0b00111101 // CMP AL, imm8 / CMP AX, imm16
        => {
            let w = v & 0b1; // Word/byte bit
            let operation = match v & 0b11111110 {
                0b00000100 => Operation::Add,
                0b00101100 => Operation::Sub,
                0b00111100 => Operation::Cmp,
                _ => unreachable!(),
            };

            offset += 1;

            let immediate = if w == 1 {
                let value = ((buffer[offset + 1] as u16) << 8) | (buffer[offset] as u16);
                offset += 2;
                value as i16
            } else {
                let value = buffer[offset] as i8 as i16; // Sign extend
                offset += 1;
                value
            };

            let destination = Operand::Register(if w == 1 { Register::AX } else { Register::AL });
            let source = Operand::Immediate(immediate);
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: Some(source),
                bytes: instruction_bytes,
            }
        }

        // Conditional Jumps
        v if (v >= 0b01110000 && v <= 0b01111111) || (v >= 0b11100000 && v <= 0b11100011) => {
            let operation = match v {
                0b01110100 => Operation::Jz, // JE
                0b01110101 => Operation::Jnz, // JNE
                0b01111100 => Operation::Jl,
                0b01111110 => Operation::Jle,
                0b01110010 => Operation::Jb,
                0b01110110 => Operation::Jbe,
                0b01111010 => Operation::Jp,
                0b01110000 => Operation::Jo,
                0b01111000 => Operation::Js,
                0b01111101 => Operation::Jnl,
                0b01111111 => Operation::Jg,
                0b01110011 => Operation::Jnb,
                0b01110111 => Operation::Ja,
                0b01111011 => Operation::Jnp,
                0b01110001 => Operation::Jno,
                0b01111001 => Operation::Jns,
                0b11100010 => Operation::Loop,
                0b11100001 => Operation::Loopz,
                0b11100000 => Operation::Loopnz,
                0b11100011 => Operation::Jcxz,
                _ => panic!("Unsupported jump opcode: {:08b}", v),
            };

            offset += 1;

            // Get offset for jump target (signed 8-bit displacement from next instruction)
            let jump_offset = buffer[offset] as i8;
            let target_offset = (offset as i16) + 1 + (jump_offset as i16);

            offset += 1;

            // Create the instruction
            let destination = Operand::Immediate(target_offset); // Relative target address
            let instruction_bytes = buffer[start_offset..offset].to_vec();

            Instruction {
                operation,
                destination,
                source: None, // No source operand for jumps
                bytes: instruction_bytes,
            }
        }

        _ => {
            // Unsupported instruction
            panic!("Unsupported instruction at index {}: {:08b}", offset, current_byte);
        }
    };

    cpu.set_ip(offset);
    instruction
}
