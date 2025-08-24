use crate::cpu::{CPUState, RegisterPair};
use std::slice::Iter;

fn handle_lxi(cpu_state: &mut CPUState, operands: &mut Iter<u8>, reg: RegisterPair) {}

pub fn handle_lxi_b(cpu_state: &mut CPUState, operands: &mut Iter<u8>) {
    handle_lxi(cpu_state, operands, RegisterPair::BC);
}

pub fn handle_lxi_d(cpu_state: &mut CPUState, operands: &mut Iter<u8>) {
    handle_lxi(cpu_state, operands, RegisterPair::DE);
}

pub fn handle_lxi_h(cpu_state: &mut CPUState, operands: &mut Iter<u8>) {
    handle_lxi(cpu_state, operands, RegisterPair::HL);
}

pub fn handle_lxi_sp(cpu_state: &mut CPUState, operands: &mut Iter<u8>) {
    handle_lxi(cpu_state, operands, RegisterPair::SP);
}
