use crate::cpu::CPUState;
use std::slice::Iter;

pub fn handle_nop(_cpu_state: &mut CPUState, _operands: &mut Iter<u8>) {}

pub fn handle_halt(cpu_state: &mut CPUState, _operands: &mut Iter<u8>) {
    cpu_state.halted = true;
}
