use lazy_static::lazy_static;
use std::collections::HashMap;

use super::addrssing_modes::AddressingMode::{self, *};

pub struct OpCode {
    pub name: &'static str,
    pub mode: AddressingMode,
    pub len: u8,
    pub cycles: u8,
}

impl OpCode {
    pub fn new(name: &'static str, mode: AddressingMode, len: u8, cycles: u8) -> Self {
        Self {
            name,
            mode,
            len,
            cycles,
        }
    }
}

lazy_static!(
    pub static ref OPS_CODES: HashMap<u8, OpCode> = HashMap::from([
        (0x00, OpCode::new("BRK", NoneAddressing, 1, 7)),

        (0x69, OpCode::new("ADC", Immediate,  2, 2)),
        (0x65, OpCode::new("ADC", ZeroPage,   2, 3)),
        (0x75, OpCode::new("ADC", ZeroPage_X, 2, 4)),
        (0x6D, OpCode::new("ADC", Absolute,   3, 4)),
        (0x7D, OpCode::new("ADC", Absolute_X, 3, 4)), // +1 if page crossed
        (0x79, OpCode::new("ADC", Absolute_Y, 3, 4)), // +1 if page crossed
        (0x61, OpCode::new("ADC", Indirect_X, 2, 5)),
        (0x71, OpCode::new("ADC", Indirect_Y, 2, 6)), // +1 if page crossed

        (0x29, OpCode::new("AND", Immediate,  2, 2)),
        (0x25, OpCode::new("AND", ZeroPage,   2, 3)),
        (0x35, OpCode::new("AND", ZeroPage_X, 2, 4)),
        (0x2D, OpCode::new("AND", Absolute,   3, 4)),
        (0x3D, OpCode::new("AND", Absolute_X, 3, 4)), // +1 if page crossed
        (0x39, OpCode::new("AND", Absolute_Y, 3, 4)), // +1 if page crossed
        (0x21, OpCode::new("AND", Indirect_X, 2, 5)),
        (0x31, OpCode::new("AND", Indirect_Y, 2, 6)), // +1 if page crossed

        (0x0A, OpCode::new("ASL", Accumulator, 1, 2)),
        (0x06, OpCode::new("ASL", ZeroPage,    2, 5)),
        (0x16, OpCode::new("ASL", ZeroPage_X,  2, 6)),
        (0x0E, OpCode::new("ASL", Absolute,    3, 6)),
        (0x1E, OpCode::new("ASL", Absolute_X,  3, 7)),

        (0x90, OpCode::new("BCC", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0xB0, OpCode::new("BCS", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0xF0, OpCode::new("BEQ", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0x30, OpCode::new("BMI", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0xD0, OpCode::new("BNE", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0x10, OpCode::new("BPL", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0x50, OpCode::new("BVC", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0x70, OpCode::new("BVS", NoneAddressing, 2, 2)), // +1 if branch succeeds, +2 if to a new page

        (0x24, OpCode::new("BIT", ZeroPage, 2, 3)),
        (0x2C, OpCode::new("BIT", Absolute, 3, 4)),

        (0x18, OpCode::new("CLC", NoneAddressing, 1, 2)),

        (0xD8, OpCode::new("CLD", NoneAddressing, 1, 2)),

        (0x58, OpCode::new("CLI", NoneAddressing, 1, 2)),

        (0xB8, OpCode::new("CLV", NoneAddressing, 1, 2)),

        (0xC9, OpCode::new("CMP", Immediate,  2, 2)),
        (0xC5, OpCode::new("CMP", ZeroPage,   2, 3)),
        (0xD5, OpCode::new("CMP", ZeroPage_X, 2, 4)),
        (0xCD, OpCode::new("CMP", Absolute,   3, 4)),
        (0xDD, OpCode::new("CMP", Absolute_X, 3, 4)), // +1 if page crossed
        (0xD9, OpCode::new("CMP", Absolute_Y, 3, 4)), // +1 if page crossed
        (0xC1, OpCode::new("CMP", Indirect_X, 2, 6)),
        (0xD1, OpCode::new("CMP", Indirect_Y, 2, 5)), // +1 if page crossed

        (0xE0, OpCode::new("CPX", Immediate, 2, 2)),
        (0xE4, OpCode::new("CPX", ZeroPage,  2, 3)),
        (0xEC, OpCode::new("CPX", Absolute,  3, 4)),

        (0xC0, OpCode::new("CPY", Immediate, 2, 2)),
        (0xC4, OpCode::new("CPY", ZeroPage,  2, 3)),
        (0xCC, OpCode::new("CPY", Absolute,  3, 4)),

        (0xC6, OpCode::new("DEC", ZeroPage,   2, 5)),
        (0xD6, OpCode::new("DEC", ZeroPage_X, 2, 6)),
        (0xCE, OpCode::new("DEC", Absolute,   3, 6)),
        (0xDE, OpCode::new("DEC", Absolute_X, 3, 7)),

        (0xCA, OpCode::new("DEX", NoneAddressing, 1, 2)),

        (0x88, OpCode::new("DEY", NoneAddressing, 1, 2)),

        (0x49, OpCode::new("EOR", Immediate,  2, 2)),
        (0x45, OpCode::new("EOR", ZeroPage,   2, 3)),
        (0x55, OpCode::new("EOR", ZeroPage_X, 2, 4)),
        (0x4D, OpCode::new("EOR", Absolute,   3, 4)),
        (0x5D, OpCode::new("EOR", Absolute_X, 3, 4)), // +1 if page crossed
        (0x59, OpCode::new("EOR", Absolute_Y, 3, 4)), // +1 if page crossed
        (0x41, OpCode::new("EOR", Indirect_X, 2, 5)),
        (0x51, OpCode::new("EOR", Indirect_Y, 2, 6)), // +1 if page crossed

        (0xE6, OpCode::new("INC", ZeroPage,   2, 5)),
        (0xF6, OpCode::new("INC", ZeroPage_X, 2, 6)),
        (0xEE, OpCode::new("INC", Absolute,   3, 6)),
        (0xFE, OpCode::new("INC", Absolute_X, 3, 7)),

        (0xE8, OpCode::new("INX", NoneAddressing, 1, 2)),

        (0xC8, OpCode::new("INY", NoneAddressing, 1, 2)),

        (0x4C, OpCode::new("JMP", Absolute, 3, 3)),
        (0x6C, OpCode::new("JMP", Indirect, 3, 5)),

        (0x20, OpCode::new("JSR", NoneAddressing, 3, 6)),

        (0xA9, OpCode::new("LDA", Immediate,  2, 2)),
        (0xA5, OpCode::new("LDA", ZeroPage,   2, 3)),
        (0xB5, OpCode::new("LDA", ZeroPage_X, 2, 4)),
        (0xAD, OpCode::new("LDA", Absolute,   3, 4)),
        (0xBD, OpCode::new("LDA", Absolute_X, 3, 4)), // +1 if page crossed
        (0xB9, OpCode::new("LDA", Absolute_Y, 3, 4)), // +1 if page crossed
        (0xA1, OpCode::new("LDA", Indirect_X, 2, 6)),
        (0xB1, OpCode::new("LDA", Indirect_Y, 2, 5)), // +1 if page crossed

        (0xA2, OpCode::new("LDX", Immediate,  2, 2)),
        (0xA6, OpCode::new("LDX", ZeroPage,   2, 3)),
        (0xB6, OpCode::new("LDX", ZeroPage_Y, 2, 4)),
        (0xAE, OpCode::new("LDX", Absolute,   3, 4)),
        (0xBE, OpCode::new("LDX", Absolute_Y, 3, 4)), // +1 if page crossed

        (0xA0, OpCode::new("LDY", Immediate,  2, 2)),
        (0xA4, OpCode::new("LDY", ZeroPage,   2, 3)),
        (0xB4, OpCode::new("LDY", ZeroPage_X, 2, 4)),
        (0xAC, OpCode::new("LDY", Absolute,   3, 4)),
        (0xBC, OpCode::new("LDY", Absolute_X, 3, 4)), // +1 if page crossed

        (0x4A, OpCode::new("LSR", Accumulator, 1, 2)),
        (0x46, OpCode::new("LSR", ZeroPage,    2, 5)),
        (0x56, OpCode::new("LSR", ZeroPage_X,  2, 6)),
        (0x4E, OpCode::new("LSR", Absolute,    3, 6)),
        (0x5E, OpCode::new("LSR", Absolute_X,  3, 7)),

        (0xEA, OpCode::new("NOP", NoneAddressing, 1, 2)),

        (0x09, OpCode::new("ORA", Immediate,  2, 2)),
        (0x05, OpCode::new("ORA", ZeroPage,   2, 3)),
        (0x15, OpCode::new("ORA", ZeroPage_X, 2, 4)),
        (0x0D, OpCode::new("ORA", Absolute,   3, 4)),
        (0x1D, OpCode::new("ORA", Absolute_X, 3, 4)), // +1 if page crossed
        (0x19, OpCode::new("ORA", Absolute_Y, 3, 4)), // +1 if page crossed
        (0x01, OpCode::new("ORA", Indirect_X, 2, 6)),
        (0x11, OpCode::new("ORA", Indirect_Y, 2, 5)), // +1 if page crossed

        (0x48, OpCode::new("PHA", NoneAddressing, 1, 3)),

        (0x08, OpCode::new("PHP", NoneAddressing, 1, 3)),

        (0x68, OpCode::new("PLA", NoneAddressing, 1, 4)),

        (0x28, OpCode::new("PLP", NoneAddressing, 1, 4)),

        (0x2A, OpCode::new("ROL", Accumulator, 1, 2)),
        (0x26, OpCode::new("ROL", ZeroPage,    2, 5)),
        (0x36, OpCode::new("ROL", ZeroPage_X,  2, 6)),
        (0x2E, OpCode::new("ROL", Absolute,    3, 6)),
        (0x3E, OpCode::new("ROL", Absolute_X,  3, 7)),

        (0x6A, OpCode::new("ROR", Accumulator, 1, 2)),
        (0x66, OpCode::new("ROR", ZeroPage,    2, 5)),
        (0x76, OpCode::new("ROR", ZeroPage_X,  2, 6)),
        (0x6E, OpCode::new("ROR", Absolute,    3, 6)),
        (0x7E, OpCode::new("ROR", Absolute_X,  3, 7)),

        (0x40, OpCode::new("RTI", NoneAddressing, 1, 6)),

        (0x60, OpCode::new("RTS", NoneAddressing, 1, 6)),

        (0xE9, OpCode::new("SBC", Immediate,  2, 2)),
        (0xE5, OpCode::new("SBC", ZeroPage,   2, 3)),
        (0xF5, OpCode::new("SBC", ZeroPage_X, 2, 4)),
        (0xED, OpCode::new("SBC", Absolute,   3, 4)),
        (0xFD, OpCode::new("SBC", Absolute_X, 3, 4)), // +1 if page crossed
        (0xF9, OpCode::new("SBC", Absolute_Y, 3, 4)), // +1 if page crossed
        (0xE1, OpCode::new("SBC", Indirect_X, 2, 6)),
        (0xF1, OpCode::new("SBC", Indirect_Y, 2, 5)), // +1 if page crossed

        (0x38, OpCode::new("SEC", NoneAddressing, 1, 2)),

        (0xF8, OpCode::new("SED", NoneAddressing, 1, 2)),

        (0x78, OpCode::new("SEI", NoneAddressing, 1, 2)),

        (0x85, OpCode::new("STA", ZeroPage,   2, 3)),
        (0x95, OpCode::new("STA", ZeroPage_X, 2, 4)),
        (0x8D, OpCode::new("STA", Absolute,   3, 4)),
        (0x9D, OpCode::new("STA", Absolute_X, 3, 5)),
        (0x99, OpCode::new("STA", Absolute_Y, 3, 5)),
        (0x81, OpCode::new("STA", Indirect_X, 2, 6)),
        (0x91, OpCode::new("STA", Indirect_Y, 2, 6)),

        (0x86, OpCode::new("STX", ZeroPage,   2, 3)),
        (0x96, OpCode::new("STX", ZeroPage_Y, 2, 4)),
        (0x8E, OpCode::new("STX", Absolute,   3, 4)),

        (0x84, OpCode::new("STY", ZeroPage,   2, 3)),
        (0x94, OpCode::new("STY", ZeroPage_X, 2, 4)),
        (0x8C, OpCode::new("STY", Absolute,   3, 4)),

        (0xAA, OpCode::new("TAX", NoneAddressing, 1, 2)),

        (0xA8, OpCode::new("TAY", NoneAddressing, 1, 2)),

        (0xBA, OpCode::new("TSX", NoneAddressing, 1, 2)),

        (0x8A, OpCode::new("TXA", NoneAddressing, 1, 2)),

        (0x9A, OpCode::new("TXS", NoneAddressing, 1, 2)),

        (0x98, OpCode::new("TYA", NoneAddressing, 1, 2)),
    ]);
);
