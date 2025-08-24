mod cpu;
mod instructions;
mod one_byte_inst;
mod opcodes;
mod three_byte_inst;

use crate::opcodes::*;
use crate::{cpu::CPUState, instructions::build_instruction_table};

fn main() {
    println!("Hello, world!");

    let op_table = build_instruction_table();

    let rom = vec![Opcodes::NOP as u8, Opcodes::HLT as u8];
    let mut cpu = CPUState::new(&rom);

    while !cpu.halted {
        let pc = cpu.registers.pc;
        let opcode = match cpu.memory.get(pc as usize) {
            Some(value) => *value,
            None => {
                println!("Error: opcode read out of memory 0x{:04X}", pc);
                cpu.halted = true;
                0
            }
        };

        let instruction_info = &op_table[opcode as usize];

        println!(
            "PC: 0x{:04X} -> Fetching: {} (opcode: 0x{:02X})",
            pc, instruction_info.name, opcode
        );

        cpu.registers.pc += instruction_info.len as u16;

        if let Some(new_pc) = cpu.registers.pc.checked_add(instruction_info.len as u16) {
            cpu.registers.pc = new_pc;
        } else {
            println!("Error: program counter overflow!");
            cpu.halted = true;
        }
    }

    println!("Halt!");
}
