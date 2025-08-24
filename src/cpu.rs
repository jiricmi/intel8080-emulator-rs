pub const MEMORY_SIZE: usize = 65536;
pub const OPCODE_COUNT: usize = 256;

pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Default)]
pub struct StatusFlags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool, // 3 and 4 bit carry
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub flags: StatusFlags,
}

pub struct CPUState {
    pub interrupt: bool,
    pub halted: bool,
    pub registers: Registers,
    pub memory: [u8; MEMORY_SIZE],
}

impl CPUState {
    fn new(rom: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[..rom.len()].copy_from_slice(rom);
        Self {
            registers: Registers::default(),
            memory,
            halted: false,
            interrupt: false,
        }
    }
}
