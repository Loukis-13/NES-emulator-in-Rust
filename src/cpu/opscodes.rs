use lazy_static::lazy_static;
use std::collections::HashMap;

use super::addrssing_modes::AddressingMode;

pub struct OpCode<'a> {
    pub name: &'a str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl<'a> OpCode<'a> {
    pub fn new(name: &'a str, len: u8, cycles: u8, mode: AddressingMode) -> Self { Self { name, len, cycles, mode } }
}

lazy_static!(
    pub static ref OPS_CODES: HashMap<u8, OpCode<'static>> = HashMap::from([
        (0x00, OpCode::new("BRK", 0, 0, AddressingMode::NoneAddressing)),

        (0x69, OpCode::new("ADC", 2, 2, AddressingMode::Immediate)),
        (0x65, OpCode::new("ADC", 2, 3, AddressingMode::ZeroPage)),
        (0x75, OpCode::new("ADC", 2, 4, AddressingMode::ZeroPage_X)),
        (0x6D, OpCode::new("ADC", 3, 4, AddressingMode::Absolute)),
        (0x7D, OpCode::new("ADC", 3, 4, AddressingMode::Absolute_X)),
        (0x79, OpCode::new("ADC", 3, 4, AddressingMode::Absolute_Y)),
        (0x61, OpCode::new("ADC", 2, 5, AddressingMode::Indirect_X)),
        (0x71, OpCode::new("ADC", 2, 6, AddressingMode::Indirect_Y)),

        (0x29, OpCode::new("AND", 2, 2, AddressingMode::Immediate)),
        (0x25, OpCode::new("AND", 2, 3, AddressingMode::ZeroPage)),
        (0x35, OpCode::new("AND", 2, 4, AddressingMode::ZeroPage_X)),
        (0x2D, OpCode::new("AND", 3, 4, AddressingMode::Absolute)),
        (0x3D, OpCode::new("AND", 3, 4, AddressingMode::Absolute_X)),
        (0x39, OpCode::new("AND", 3, 4, AddressingMode::Absolute_Y)),
        (0x21, OpCode::new("AND", 2, 5, AddressingMode::Indirect_X)),
        (0x31, OpCode::new("AND", 2, 6, AddressingMode::Indirect_Y)),

        (0x0A, OpCode::new("ASL", 1, 2, AddressingMode::Accumulator)),
        (0x06, OpCode::new("ASL", 2, 5, AddressingMode::ZeroPage)),
        (0x16, OpCode::new("ASL", 2, 6, AddressingMode::ZeroPage_X)),
        (0x0E, OpCode::new("ASL", 3, 6, AddressingMode::Absolute)),
        (0x1E, OpCode::new("ASL", 3, 7, AddressingMode::Absolute_X)),

        (0x90, OpCode::new("BCC", 2, 2, AddressingMode::NoneAddressing)),

        (0xB0, OpCode::new("BCS", 2, 2, AddressingMode::NoneAddressing)),

        (0xF0, OpCode::new("BEQ", 2, 2, AddressingMode::NoneAddressing)),

        (0x30, OpCode::new("BMI", 2, 2, AddressingMode::NoneAddressing)),

        (0xD0, OpCode::new("BNE", 2, 2, AddressingMode::NoneAddressing)),

        (0x10, OpCode::new("BPL", 2, 2, AddressingMode::NoneAddressing)),

        (0x50, OpCode::new("BVC", 2, 2, AddressingMode::NoneAddressing)),

        (0x70, OpCode::new("BVS", 2, 2, AddressingMode::NoneAddressing)),

        (0x24, OpCode::new("BIT", 2, 3, AddressingMode::ZeroPage)),
        (0x2C, OpCode::new("BIT", 3, 4, AddressingMode::Absolute)),

        (0x18, OpCode::new("CLC", 1, 2, AddressingMode::NoneAddressing)),

        (0xD8, OpCode::new("CLD", 1, 2, AddressingMode::NoneAddressing)),

        (0x58, OpCode::new("CLI", 1, 2, AddressingMode::NoneAddressing)),

        (0xB8, OpCode::new("CLV", 1, 2, AddressingMode::NoneAddressing)),
        
        (0xAA, OpCode::new("TAX", 1, 2, AddressingMode::NoneAddressing)),

        (0xA8, OpCode::new("TAY", 1, 2, AddressingMode::NoneAddressing)),

        // (0xBA, OpCode::new("TSX", 1, 2, AddressingMode::NoneAddressing)),

        (0xE6, OpCode::new("INC", 2, 5, AddressingMode::ZeroPage)),
        (0xF6, OpCode::new("INC", 2, 6, AddressingMode::ZeroPage_X)),
        (0xEE, OpCode::new("INC", 3, 6, AddressingMode::Absolute)),
        (0xFE, OpCode::new("INC", 3, 7, AddressingMode::Absolute_X)),

        (0xE8, OpCode::new("INX", 1, 2, AddressingMode::NoneAddressing)),

        (0xC8, OpCode::new("INY", 1, 2, AddressingMode::NoneAddressing)),

        (0xA9, OpCode::new("LDA", 2, 2, AddressingMode::Immediate)),
        (0xA5, OpCode::new("LDA", 2, 3, AddressingMode::ZeroPage)),
        (0xB5, OpCode::new("LDA", 2, 4, AddressingMode::ZeroPage_X)),
        (0xAD, OpCode::new("LDA", 3, 4, AddressingMode::Absolute)),
        (0xBD, OpCode::new("LDA", 3, 4, AddressingMode::Absolute_X)),
        (0xB9, OpCode::new("LDA", 3, 4, AddressingMode::Absolute_Y)),
        (0xA1, OpCode::new("LDA", 2, 6, AddressingMode::Indirect_X)),
        (0xB1, OpCode::new("LDA", 2, 5, AddressingMode::Indirect_Y)),

        (0x85, OpCode::new("STA", 2, 3, AddressingMode::ZeroPage)),
        (0x95, OpCode::new("STA", 2, 4, AddressingMode::ZeroPage_X)),
        (0x8D, OpCode::new("STA", 3, 4, AddressingMode::Absolute)),
        (0x9D, OpCode::new("STA", 3, 5, AddressingMode::Absolute_X)),
        (0x99, OpCode::new("STA", 3, 5, AddressingMode::Absolute_Y)),
        (0x81, OpCode::new("STA", 2, 6, AddressingMode::Indirect_X)),
        (0x91, OpCode::new("STA", 2, 6, AddressingMode::Indirect_Y)),

        (0x86, OpCode::new("STX", 2, 3, AddressingMode::ZeroPage)),
        (0x96, OpCode::new("STX", 2, 4, AddressingMode::ZeroPage_Y)),
        (0x8E, OpCode::new("STX", 3, 4, AddressingMode::Absolute)),

        (0x84, OpCode::new("STY", 2, 3, AddressingMode::ZeroPage)),
        (0x94, OpCode::new("STY", 2, 4, AddressingMode::ZeroPage_Y)),
        (0x8C, OpCode::new("STY", 3, 4, AddressingMode::Absolute)),
    ]);
);
