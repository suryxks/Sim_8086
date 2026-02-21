# Sim_8086

A Rust implementation of an Intel 8086 CPU simulator with instruction decoding and execution capabilities.

## Overview

Sim_8086 is an educational project that simulates the behavior of the Intel 8086 processor, one of the foundational x86 architecture processors. This simulator can decode and execute 8086 assembly instructions, manage CPU registers, flags, and memory.

## Features

- **CPU Architecture Simulation**
  - 1MB of simulated memory
  - Complete register set (8-bit and 16-bit general purpose registers)
  - CPU flags (Carry, Parity, Auxiliary Carry, Zero, Sign, Overflow)
  - Instruction pointer (IP) management

- **Instruction Support**
  - **Data Movement**: `MOV` - Move data between registers, memory, and immediate values
  - **Arithmetic**: `ADD`, `SUB` - Addition and subtraction with overflow detection
  - **Logic**: `CMP` - Compare values and set flags accordingly
  - **Control Flow**: Conditional jumps (`JNZ`, `JZ`, `JL`, `JLE`, `JB`, `JBE`, `JP`, `JO`, `JS`, `JNE`, `JNBŁ`, `JG`, `JA`, `JNP`, `JNO`, `JNS`)
  - **Loop Instructions**: `LOOP`, `LOOPZ`, `LOOPNZ`, `JCXZ`

- **Addressing Modes**
  - Register addressing
  - Direct memory addressing
  - Complex memory addressing (base + index + displacement)

- **Register Set**
  - General Purpose: AX, BX, CX, DX (16-bit, split into 8-bit halves: AH/AL, BH/BL, etc.)
  - Pointer/Index: SP, BP, SI, DI

##  Project Structure

```
Sim_8086/
├── src/
│   ├── main.rs          # Entry point
│   ├── cpu.rs           # CPU emulation engine
│   ├── decoder.rs       # Instruction decoder
│   └── instruction.rs   # Instruction definitions and data structures
├── Cargo.toml           # Rust project manifest
└── README.md            # This file
```

## 🔧 Project Components

### `src/instruction.rs`
Defines the core instruction set architecture:
- **Register enum**: All 8086 registers (AL, AH, AX, BL, BH, BX, etc.)
- **Operation enum**: All supported instruction types
- **Operand enum**: Register, Memory, or Immediate values
- **AddressingMode enum**: Direct, Register, and Complex memory addressing
- **Instruction struct**: Represents a decoded instruction

### `src/cpu.rs`
Implements the CPU simulation:
- **Cpu struct**: Main emulation engine with memory, registers, and flags
- **CPURegisters struct**: Manages all register values
- **Flags struct**: Manages CPU condition flags
- **execute() method**: Executes decoded instructions
- Helper functions for address calculation and flag manipulation

### `src/decoder.rs`
Handles machine code decoding and instruction parsing (in development)

### `src/main.rs`
Application entry point and initialization

##  Getting Started

### Prerequisites
- Rust 1.70+ 

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

## 📝 Usage Example

```rust
use sim_8086::cpu::Cpu;
use sim_8086::instruction::{Instruction, Operation, Operand, Register};

fn main() {
    let mut cpu = Cpu::new();
    
    // Create a MOV instruction: MOV AX, 0x1234
    let instruction = Instruction {
        operation: Operation::Mov,
        destination: Operand::Register(Register::AX),
        source: Some(Operand::Immediate(0x1234)),
        bytes: vec![],
    };
    
    // Execute the instruction
    cpu.execute(&instruction);
    
    // Check the result
    println!("AX = {:#x}", cpu.registers.get(&Register::AX));
}
```

## 🔄 Instruction Execution Details

### MOV (Move)
Moves data between:
- Register to Register
- Immediate to Register
- Register to Memory
- Memory to Register
- Immediate to Memory

### ADD/SUB
- Performs arithmetic operations
- Sets the Overflow flag (OF) on result overflow
- Maintains register values

### CMP (Compare)
- Performs comparison without modifying operands
- Sets Zero Flag (ZF) if values are equal
- Sets Sign Flag (SF) if result is negative

### Conditional Jumps
- Examines CPU flags
- Sets Instruction Pointer (IP) to target address if condition is true

##  Planned Features

- [ ] Complete instruction decoder from machine code
- [ ] More arithmetic instructions (MUL, DIV, INC, DEC)
- [ ] Shift and rotate operations
- [ ] String operations
- [ ] Interrupt handling
- [ ] Debugger interface
- [ ] Test suite and examples

##  Learning Resources

This project is useful for understanding:
- CPU architecture fundamentals
- x86 instruction set basics
- Memory management in processors
- Flag-based conditional logic
- Rust systems programming

## 📖 References

- [Intel 8086 Instruction Set](https://en.wikipedia.org/wiki/x86)
- [Real Mode Assembly Language Programming](https://en.wikipedia.org/wiki/Real_mode)
- [Rust Programming Language Book](https://doc.rust-lang.org/book/)

---

**Note**: This is an educational simulator and may not perfectly match real 8086 hardware behavior in all edge cases.
