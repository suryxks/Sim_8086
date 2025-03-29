use crate::instruction::{ Instruction, Operand, Operation, Register };
#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    pub registers: CPURegisters,
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
        }
    }
    pub fn get(&self, reg: Register) -> u16 {
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
    pub fn set(&mut self, reg: Register, val: u16) {
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
        Cpu { registers: CPURegisters::new() }
    }
    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Operation::Mov => {
                match (&instruction.source, &instruction.destination) {
                    (Some(source), destination) => {
                        match (source, destination) {
                            (Operand::Register(reg1), Operand::Register(reg2)) => {
                                self.registers.set(reg2.clone(), self.registers.get(reg1.clone()));
                            }
                            (Operand::Immediate(val), Operand::Register(reg)) => {
                                self.registers.set(reg.clone(), *val as u16);
                            }
                            (_, _) => { println!("Not supported") }
                        }
                    }
                    (_, _) => { println!("Not supported") }
                }
            }
            _ => {
                println!("Not supported as of now");
            }
        }
    }
}
