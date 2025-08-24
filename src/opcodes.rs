#[repr(u8)]
pub enum Opcodes {
    NOP = 0x00,
    LXI_B = 0x01,
    LXI_D = 0x11,
    LXI_H = 0x21,
    LXI_SP = 0x31,
    HLT = 0x76,
}
