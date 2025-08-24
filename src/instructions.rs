use crate::cpu::{CPUState, OPCODE_COUNT};
use crate::one_byte_inst::*;
use crate::opcodes::Opcodes;
use crate::three_byte_inst::*;
use std::slice::Iter;

pub struct InstructionInfo {
    pub name: &'static str,
    pub len: u8,
    pub handler: fn(&mut CPUState, &mut Iter<u8>),
}

const UNKNOWN_INSTRUCTION: InstructionInfo = InstructionInfo {
    name: "???",
    len: 1,
    handler: handle_nop,
};

pub const fn build_instruction_table() -> [InstructionInfo; OPCODE_COUNT] {
    let mut table = [UNKNOWN_INSTRUCTION; OPCODE_COUNT];

    table[Opcodes::NOP as usize] = InstructionInfo {
        name: "NOP",
        len: 1,
        handler: handle_nop,
    };

    table[Opcodes::HLT as usize] = InstructionInfo {
        name: "HLT",
        len: 1,
        handler: handle_halt,
    };

    // LXI
    let lxi_handlers = [handle_lxi_b, handle_lxi_d, handle_lxi_h, handle_lxi_sp];

    return table;
}
