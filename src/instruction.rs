#[derive(Debug, Clone)]
pub enum Register {
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
    DH,
    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,
}

#[derive(Debug)]
pub enum AddressingMode {
    Direct(i16),
    Register(Register),
    Memory {
        base: Option<Register>,
        index: Option<Register>,
        displacement: Option<i16>,
    },
}

#[derive(Debug)]
pub enum Operation {
    Mov,
    Add,
    Sub,
    Cmp,
    Jnz, // Jump not zero/not equal
    Jz, // Jump if zero/equal
    Jl, // Jump if less
    Jle, // Jump if less or equal
    Jb, // Jump if below
    Jbe, // Jump if below or equal
    Jp, // Jump if parity
    Jo, // Jump if overflow
    Js, // Jump if sign
    Jne, // Jump if not equal (synonym for Jnz)
    Jnl, // Jump if not less
    Jg, // Jump if greater
    Jnb, // Jump if not below
    Ja, // Jump if above
    Jnp, // Jump if not parity
    Jno, // Jump if not overflow
    Jns, // Jump if not sign
    Loop, // Loop
    Loopz, // Loop if zero
    Loopnz, // Loop if not zero
    Jcxz, // Jump if CX is zero
}

#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Memory(AddressingMode),
    Immediate(i16),
}

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub destination: Operand,
    pub source: Option<Operand>,
    pub bytes: Vec<u8>,
}
