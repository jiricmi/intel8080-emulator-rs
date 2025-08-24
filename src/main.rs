mod cpu;
mod instructions;
mod one_byte_inst;
mod opcodes;
mod three_byte_inst;

use crate::instructions::build_instruction_table;

fn main() {
    println!("Hello, world!");

    let op_table = build_instruction_table();
}
