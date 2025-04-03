use crate::instruction::{ AddressingMode, Instruction, Operand, Operation, Register };
#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    pub memory: [u8; 1024 * 1024], // 1MB memory
    pub registers: CPURegisters,
    pub flags: Flags,
}

#[derive(Debug, Clone, Copy)]
pub struct CPURegisters {
    pub ax: [u8; 2],
    pub bx: [u8; 2],
    pub cx: [u8; 2],
    pub dx: [u8; 2],
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,
    pub ip: u16,
}
pub enum Flag {
    CF,
    PF,
    AF,
    ZF,
    SF,
    OF,
}
#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub cf: bool, //carry
    pub pf: bool, // parity
    pub af: bool, // auxilary carry
    pub zf: bool, //zero
    pub sf: bool, //sign
    pub of: bool, //overflow
}
impl Flags {
    pub fn new() -> Self {
        Flags {
            cf: false,
            pf: false,
            af: false,
            zf: false,
            sf: false,
            of: false,
        }
    }
    pub fn set(&mut self, flag: Flag) {
        match flag {
            Flag::AF => {
                self.af = true;
            }
            Flag::CF => {
                self.cf = true;
            }
            Flag::PF => {
                self.pf = true;
            }
            Flag::ZF => {
                self.zf = true;
            }
            Flag::SF => {
                self.sf = true;
            }
            Flag::OF => {
                self.of = true;
            }
        }
    }
    pub fn unset(&mut self, flag: Flag) {
        match flag {
            Flag::AF => {
                self.af = false;
            }
            Flag::CF => {
                self.cf = false;
            }
            Flag::PF => {
                self.pf = false;
            }
            Flag::ZF => {
                self.zf = false;
            }
            Flag::SF => {
                self.sf = false;
            }
            Flag::OF => {
                self.of = false;
            }
        }
    }

    pub fn reset(&mut self) {
        self.cf = false;
        self.pf = false;
        self.af = false;
        self.zf = false;
        self.sf = false;
        self.of = false;
    }
}
impl CPURegisters {
    pub fn new() -> Self {
        CPURegisters {
            ax: [0, 0],
            bx: [0, 0],
            cx: [0, 0],
            dx: [0, 0],
            sp: 0,
            bp: 0,
            si: 0,
            di: 0,
            ip: 0,
        }
    }
    pub fn get(&self, reg: &Register) -> u16 {
        match reg {
            Register::AH => self.ax[0] as u16,
            Register::AL => self.ax[1] as u16,
            Register::AX => ((self.ax[0] as u16) << 8) + (self.ax[1] as u16),
            Register::BH => self.bx[0] as u16,
            Register::BL => self.bx[1] as u16,
            Register::BX => ((self.bx[0] as u16) << 8) + (self.bx[1] as u16),
            Register::CH => self.cx[0] as u16,
            Register::CL => self.cx[1] as u16,
            Register::CX => ((self.cx[0] as u16) << 8) + (self.cx[1] as u16),
            Register::DH => self.cx[0] as u16,
            Register::DL => self.cx[1] as u16,
            Register::DX => ((self.dx[0] as u16) << 8) + (self.dx[1] as u16),
            Register::SI => self.si,
            Register::DI => self.di,
            Register::BP => self.bp,
            Register::SP => self.sp,
        }
    }
    pub fn set(&mut self, reg: &Register, val: u16) {
        match reg {
            Register::AH => {
                self.ax[0] = val as u8;
            }
            Register::AL => {
                self.ax[1] = val as u8;
            }
            Register::AX => {
                self.ax[0] = (val >> 8) as u8;
                self.ax[1] = ((val << 8) >> 8) as u8;
            }
            Register::BH => {
                self.bx[0] = val as u8;
            }
            Register::BL => {
                self.bx[1] = val as u8;
            }
            Register::BX => {
                self.bx[0] = (val >> 8) as u8;
                self.bx[1] = ((val << 8) >> 8) as u8;
            }
            Register::CH => {
                self.cx[0] = val as u8;
            }
            Register::CL => {
                self.cx[1] = val as u8;
            }
            Register::CX => {
                self.cx[0] = (val >> 8) as u8;
                self.cx[1] = ((val << 8) >> 8) as u8;
            }
            Register::DH => {
                self.dx[0] = val as u8;
            }
            Register::DL => {
                self.dx[1] = val as u8;
            }
            Register::DX => {
                self.dx[0] = (val >> 8) as u8;
                self.dx[1] = ((val << 8) >> 8) as u8;
            }
            Register::SI => {
                self.si = val;
            }
            Register::DI => {
                self.di = val;
            }
            Register::BP => {
                self.bp = val;
            }
            Register::SP => {
                self.sp = val;
            }
        }
    }
}
impl Cpu {
    pub fn new() -> Self {
        Cpu { registers: CPURegisters::new(), flags: Flags::new(), memory: [0; 1024 * 1024] }
    }
    pub fn set_ip(&mut self, address: usize) {
        self.registers.ip = address as u16;
    }
    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Operation::Mov => {
                match (&instruction.source, &instruction.destination) {
                    (Some(source), destination) => {
                        match (source, destination) {
                            (Operand::Register(src), Operand::Register(dest)) => {
                                self.registers.set(&dest, self.registers.get(&src));
                            }
                            (Operand::Immediate(val), Operand::Register(reg)) => {
                                self.registers.set(&reg, *val as u16);
                            }
                            (Operand::Register(src_reg), Operand::Memory(addr)) => {
                                let dest_address = match addr {
                                    AddressingMode::Direct(address) => { *address as u16 }
                                    AddressingMode::Register(reg) => {
                                        let address = self.registers.get(&reg);
                                        address
                                    }
                                    AddressingMode::Memory { base, index, displacement } => {
                                        let addr: u16 = match (base, index, displacement) {
                                            (Some(base_reg), Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg) +
                                                    (*disp as u16);
                                                address
                                            }
                                            (None, Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&index_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), None, Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), Some(index_reg), None) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg);
                                                address
                                            }
                                            (_, _, _) => { 0 }
                                        };
                                        addr
                                    }
                                };
                                match *src_reg {
                                    | Register::AL
                                    | Register::AH
                                    | Register::BL
                                    | Register::BH
                                    | Register::CL
                                    | Register::CH
                                    | Register::DL
                                    | Register::DH => {
                                        let val = self.registers.get(&src_reg);
                                        self.memory[dest_address as usize] = val as u8;
                                    }
                                    _ => {
                                        let val = self.registers.get(&src_reg);
                                        self.memory[dest_address as usize] = val as u8;
                                        self.memory[(dest_address + 1) as usize] = (val >> 8) as u8;
                                    }
                                }
                            }
                            (Operand::Memory(addr), Operand::Register(dest_reg)) => {
                                let src_address = match addr {
                                    AddressingMode::Direct(address) => { *address as u16 }
                                    AddressingMode::Register(reg) => {
                                        let address = self.registers.get(&reg);
                                        address
                                    }
                                    AddressingMode::Memory { base, index, displacement } => {
                                        let addr: u16 = match (base, index, displacement) {
                                            (Some(base_reg), Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg) +
                                                    (*disp as u16);
                                                address
                                            }
                                            (None, Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&index_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), None, Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), Some(index_reg), None) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg);
                                                address
                                            }
                                            (_, _, _) => { 0 }
                                        };
                                        addr
                                    }
                                };
                                match *dest_reg {
                                    | Register::AL
                                    | Register::AH
                                    | Register::BL
                                    | Register::BH
                                    | Register::CL
                                    | Register::CH
                                    | Register::DL
                                    | Register::DH => {
                                        let val = self.memory[src_address as usize];
                                        self.registers.set(&dest_reg, val as u16);
                                    }
                                    _ => {
                                        let low_byte = self.memory[src_address as usize] as u16;
                                        let high_byte =
                                            (self.memory[(src_address + 1) as usize] as u16) << 8;
                                        let val = high_byte + low_byte;
                                        self.registers.set(&dest_reg, val);
                                    }
                                }
                            }
                            (Operand::Immediate(val), Operand::Memory(addr)) => {
                                let dest_address = match addr {
                                    AddressingMode::Direct(address) => { *address as u16 }
                                    AddressingMode::Register(reg) => {
                                        let address = self.registers.get(&reg);
                                        address
                                    }
                                    AddressingMode::Memory { base, index, displacement } => {
                                        let addr: u16 = match (base, index, displacement) {
                                            (Some(base_reg), Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg) +
                                                    (*disp as u16);
                                                address
                                            }
                                            (None, Some(index_reg), Some(disp)) => {
                                                let address =
                                                    self.registers.get(&index_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), None, Some(disp)) => {
                                                let address =
                                                    self.registers.get(&base_reg) + (*disp as u16);
                                                address
                                            }
                                            (Some(base_reg), Some(index_reg), None) => {
                                                let address =
                                                    self.registers.get(&base_reg) +
                                                    self.registers.get(&index_reg);
                                                address
                                            }
                                            (_, _, _) => { 0 }
                                        };
                                        addr
                                    }
                                };
                                if ((*val as u16) >> 8) == 0 {
                                    self.memory[dest_address as usize] = *val as u8;
                                } else {
                                    let low_byte = *val as u8;
                                    let high_byte = (*val << 8) as u8;
                                    self.memory[dest_address as usize] = low_byte;
                                    self.memory[(dest_address + 1) as usize] = high_byte;
                                }
                            }
                            (_, _) => { println!("Not supported") }
                        }
                    }
                    (_, _) => { println!("Not supported") }
                }
            }
            Operation::Add => {
                match (&instruction.source, &instruction.destination) {
                    (Some(source), destination) => {
                        match (source, destination) {
                            (Operand::Register(src), Operand::Register(dest)) => {
                                let src_val = self.registers.get(&src);
                                let dest_val = self.registers.get(&dest);
                                let (result, overflowed) = dest_val.overflowing_add(src_val);

                                self.registers.set(&dest, result);
                                if overflowed {
                                    self.flags.set(Flag::OF);
                                }
                            }
                            (Operand::Immediate(val), Operand::Register(reg)) => {
                                let reg_val = self.registers.get(&reg);

                                let (result, overflowed) = reg_val.overflowing_add(*val as u16);

                                self.registers.set(&reg, result);
                                if overflowed {
                                    self.flags.set(Flag::OF);
                                } else {
                                    self.flags.unset(Flag::OF);
                                }
                            }

                            (_, _) => { println!("Not supported") }
                        }
                    }
                    (_, _) => { println!("Not supported") }
                }
            }
            Operation::Sub => {
                match (&instruction.source, &instruction.destination) {
                    (Some(source), destination) => {
                        match (source, destination) {
                            (Operand::Register(src), Operand::Register(dest)) => {
                                let src_val = self.registers.get(&src);
                                let dest_val = self.registers.get(&dest);
                                let (result, is_negative) = dest_val.overflowing_sub(src_val);
                                match (result, is_negative) {
                                    (0, _) => {
                                        self.flags.set(Flag::ZF);
                                        self.flags.unset(Flag::SF);
                                    }
                                    (_, true) => {
                                        self.flags.unset(Flag::ZF);
                                        self.flags.set(Flag::SF);
                                    }
                                    (_, _) => {}
                                }

                                self.registers.set(&dest, result);
                            }
                            (Operand::Immediate(val), Operand::Register(reg)) => {
                                let reg_val = self.registers.get(&reg);

                                let (result, is_negative) = reg_val.overflowing_sub(*val as u16);
                                match (result, is_negative) {
                                    (0, _) => {
                                        self.flags.set(Flag::ZF);
                                        self.flags.unset(Flag::SF);
                                    }
                                    (_, true) => {
                                        self.flags.set(Flag::SF);
                                        self.flags.unset(Flag::ZF);
                                    }
                                    (_, _) => {}
                                }
                                self.registers.set(&reg, result);
                            }

                            (_, _) => { println!("Not supported") }
                        }
                    }
                    (_, _) => { println!("Not supported") }
                }
            }
            Operation::Cmp => {
                match (&instruction.source, &instruction.destination) {
                    (Some(source), destination) => {
                        match (source, destination) {
                            (Operand::Register(src), Operand::Register(dest)) => {
                                let src_val = self.registers.get(&src);
                                let dest_val = self.registers.get(&dest);
                                let (result, is_negative) = dest_val.overflowing_sub(src_val);
                                match (result, is_negative) {
                                    (0, _) => {
                                        self.flags.set(Flag::ZF);
                                        self.flags.unset(Flag::SF);
                                    }
                                    (_, true) => {
                                        self.flags.set(Flag::SF);
                                        self.flags.unset(Flag::ZF);
                                    }
                                    (_, _) => {}
                                }
                            }
                            (Operand::Immediate(val), Operand::Register(reg)) => {
                                let reg_val = self.registers.get(&reg);

                                let (result, is_negative) = reg_val.overflowing_sub(*val as u16);
                                match (result, is_negative) {
                                    (0, _) => {
                                        self.flags.set(Flag::ZF);
                                    }
                                    (_, true) => {
                                        self.flags.set(Flag::SF);
                                    }
                                    (_, _) => {}
                                }
                            }

                            (_, _) => { println!("Not supported") }
                        }
                    }
                    (_, _) => { println!("Not supported") }
                }
            }
            Operation::Jnz => {
                match instruction.destination {
                    Operand::Immediate(val) => {
                        if !self.flags.zf {
                            self.set_ip(val as usize);
                        }
                    }
                    _ => {
                        println!("not supproted as of now");
                    }
                }
            }
            _ => {
                println!("Not supported as of now");
            }
        }
    }
}
