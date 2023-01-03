use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::cpu::CPU;

use super::addrssing_modes::AddressingMode::{self, *};


pub struct OpCode<'a> {
    pub name: &'a str,
    pub call: fn(&mut CPU, &AddressingMode) -> (),
    pub mode: AddressingMode,
    pub len: u8,
    pub cycles: u8,
}

impl<'a> OpCode<'a> {
    pub fn new(name: &'a str, call: fn(&mut CPU, &AddressingMode) -> (), mode: AddressingMode, len: u8, cycles: u8) -> Self { Self { name, call, mode, len, cycles } }
}

lazy_static!(
    pub static ref OPS_CODES: HashMap<u8, OpCode<'static>> = HashMap::from([
        (0x00, OpCode::new("BRK", CPU::nop, NoneAddressing, 0, 0)),

        (0x69, OpCode::new("ADC", CPU::adc, Immediate,  2, 2)),
        (0x65, OpCode::new("ADC", CPU::adc, ZeroPage,   2, 3)),
        (0x75, OpCode::new("ADC", CPU::adc, ZeroPage_X, 2, 4)),
        (0x6D, OpCode::new("ADC", CPU::adc, Absolute,   3, 4)),
        (0x7D, OpCode::new("ADC", CPU::adc, Absolute_X, 3, 4)),
        (0x79, OpCode::new("ADC", CPU::adc, Absolute_Y, 3, 4)),
        (0x61, OpCode::new("ADC", CPU::adc, Indirect_X, 2, 5)),
        (0x71, OpCode::new("ADC", CPU::adc, Indirect_Y, 2, 6)),

        (0x29, OpCode::new("AND", CPU::and, Immediate,  2, 2)),
        (0x25, OpCode::new("AND", CPU::and, ZeroPage,   2, 3)),
        (0x35, OpCode::new("AND", CPU::and, ZeroPage_X, 2, 4)),
        (0x2D, OpCode::new("AND", CPU::and, Absolute,   3, 4)),
        (0x3D, OpCode::new("AND", CPU::and, Absolute_X, 3, 4)),
        (0x39, OpCode::new("AND", CPU::and, Absolute_Y, 3, 4)),
        (0x21, OpCode::new("AND", CPU::and, Indirect_X, 2, 5)),
        (0x31, OpCode::new("AND", CPU::and, Indirect_Y, 2, 6)),

        (0x0A, OpCode::new("ASL", CPU::asl, Accumulator, 1, 2)),
        (0x06, OpCode::new("ASL", CPU::asl, ZeroPage,    2, 5)),
        (0x16, OpCode::new("ASL", CPU::asl, ZeroPage_X,  2, 6)),
        (0x0E, OpCode::new("ASL", CPU::asl, Absolute,    3, 6)),
        (0x1E, OpCode::new("ASL", CPU::asl, Absolute_X,  3, 7)),

        (0x90, OpCode::new("BCC", CPU::bcc, NoneAddressing, 2, 2)),

        (0xB0, OpCode::new("BCS", CPU::bcs, NoneAddressing, 2, 2)),

        (0xF0, OpCode::new("BEQ", CPU::beq, NoneAddressing, 2, 2)),

        (0x30, OpCode::new("BMI", CPU::bmi, NoneAddressing, 2, 2)),

        (0xD0, OpCode::new("BNE", CPU::bne, NoneAddressing, 2, 2)),

        (0x10, OpCode::new("BPL", CPU::bpl, NoneAddressing, 2, 2)),

        (0x50, OpCode::new("BVC", CPU::bvc, NoneAddressing, 2, 2)),

        (0x70, OpCode::new("BVS", CPU::bvs, NoneAddressing, 2, 2)),

        (0x24, OpCode::new("BIT", CPU::bit, ZeroPage, 2, 3)),
        (0x2C, OpCode::new("BIT", CPU::bit, Absolute, 3, 4)),

        (0x18, OpCode::new("CLC", CPU::clc, NoneAddressing, 1, 2)),

        (0xD8, OpCode::new("CLD", CPU::cld, NoneAddressing, 1, 2)),

        (0x58, OpCode::new("CLI", CPU::cli, NoneAddressing, 1, 2)),

        (0xB8, OpCode::new("CLV", CPU::clv, NoneAddressing, 1, 2)),

        (0xC9, OpCode::new("CMP", CPU::cmp, Immediate,  2, 2)),
        (0xC5, OpCode::new("CMP", CPU::cmp, ZeroPage,   2, 3)),
        (0xD5, OpCode::new("CMP", CPU::cmp, ZeroPage_X, 2, 4)),
        (0xCD, OpCode::new("CMP", CPU::cmp, Absolute,   3, 4)),
        (0xDD, OpCode::new("CMP", CPU::cmp, Absolute_X, 3, 4)),
        (0xD9, OpCode::new("CMP", CPU::cmp, Absolute_Y, 3, 4)),
        (0xC1, OpCode::new("CMP", CPU::cmp, Indirect_X, 2, 6)),
        (0xD1, OpCode::new("CMP", CPU::cmp, Indirect_Y, 2, 5)),

        (0xE0, OpCode::new("CPX", CPU::cpx, Immediate, 2, 2)),
        (0xE4, OpCode::new("CPX", CPU::cpx, ZeroPage,  2, 3)),
        (0xEC, OpCode::new("CPX", CPU::cpx, Absolute,  3, 4)),

        (0xC0, OpCode::new("CPY", CPU::cpy, Immediate, 2, 2)),
        (0xC4, OpCode::new("CPY", CPU::cpy, ZeroPage,  2, 3)),
        (0xCC, OpCode::new("CPY", CPU::cpy, Absolute,  3, 4)),

        (0xC6, OpCode::new("DEC", CPU::dec, ZeroPage,   2, 5)),
        (0xD6, OpCode::new("DEC", CPU::dec, ZeroPage_X, 2, 6)),
        (0xCE, OpCode::new("DEC", CPU::dec, Absolute,   3, 6)),
        (0xDE, OpCode::new("DEC", CPU::dec, Absolute_X, 3, 7)),

        (0xCA, OpCode::new("DEX", CPU::dex, NoneAddressing, 1, 2)),

        (0x88, OpCode::new("DEY", CPU::dey, NoneAddressing, 1, 2)),

        (0x49, OpCode::new("EOR", CPU::eor, Immediate,  2, 2)),
        (0x45, OpCode::new("EOR", CPU::eor, ZeroPage,   2, 3)),
        (0x55, OpCode::new("EOR", CPU::eor, ZeroPage_X, 2, 4)),
        (0x4D, OpCode::new("EOR", CPU::eor, Absolute,   3, 4)),
        (0x5D, OpCode::new("EOR", CPU::eor, Absolute_X, 3, 4)),
        (0x59, OpCode::new("EOR", CPU::eor, Absolute_Y, 3, 4)),
        (0x41, OpCode::new("EOR", CPU::eor, Indirect_X, 2, 5)),
        (0x51, OpCode::new("EOR", CPU::eor, Indirect_Y, 2, 6)),

        (0xE6, OpCode::new("INC", CPU::inc, ZeroPage,   2, 5)),
        (0xF6, OpCode::new("INC", CPU::inc, ZeroPage_X, 2, 6)),
        (0xEE, OpCode::new("INC", CPU::inc, Absolute,   3, 6)),
        (0xFE, OpCode::new("INC", CPU::inc, Absolute_X, 3, 7)),

        (0xE8, OpCode::new("INX", CPU::inx, NoneAddressing, 1, 2)),

        (0xC8, OpCode::new("INY", CPU::iny, NoneAddressing, 1, 2)),

        (0x4C, OpCode::new("JMP", CPU::jmpa, NoneAddressing, 3, 3)),
        (0x6C, OpCode::new("JMP", CPU::jmpi, NoneAddressing, 3, 5)),

        (0x20, OpCode::new("JSR", CPU::jsr, NoneAddressing, 3, 6)),

        (0xA9, OpCode::new("LDA", CPU::lda, Immediate,  2, 2)),
        (0xA5, OpCode::new("LDA", CPU::lda, ZeroPage,   2, 3)),
        (0xB5, OpCode::new("LDA", CPU::lda, ZeroPage_X, 2, 4)),
        (0xAD, OpCode::new("LDA", CPU::lda, Absolute,   3, 4)),
        (0xBD, OpCode::new("LDA", CPU::lda, Absolute_X, 3, 4)),
        (0xB9, OpCode::new("LDA", CPU::lda, Absolute_Y, 3, 4)),
        (0xA1, OpCode::new("LDA", CPU::lda, Indirect_X, 2, 6)),
        (0xB1, OpCode::new("LDA", CPU::lda, Indirect_Y, 2, 5)),

        (0xA2, OpCode::new("LDX", CPU::ldx, Immediate,  2, 2)),
        (0xA6, OpCode::new("LDX", CPU::ldx, ZeroPage,   2, 3)),
        (0xB6, OpCode::new("LDX", CPU::ldx, ZeroPage_Y, 2, 4)),
        (0xAE, OpCode::new("LDX", CPU::ldx, Absolute,   3, 4)),
        (0xBE, OpCode::new("LDX", CPU::ldx, Absolute_Y, 3, 4)),

        (0xA0, OpCode::new("LDY", CPU::ldy, Immediate,  2, 2)),
        (0xA4, OpCode::new("LDY", CPU::ldy, ZeroPage,   2, 3)),
        (0xB4, OpCode::new("LDY", CPU::ldy, ZeroPage_X, 2, 4)),
        (0xAC, OpCode::new("LDY", CPU::ldy, Absolute,   3, 4)),
        (0xBC, OpCode::new("LDY", CPU::ldy, Absolute_X, 3, 4)),

        (0x4A, OpCode::new("LSR", CPU::lsr, Accumulator, 1, 2)),
        (0x46, OpCode::new("LSR", CPU::lsr, ZeroPage,    2, 5)),
        (0x56, OpCode::new("LSR", CPU::lsr, ZeroPage_X,  2, 6)),
        (0x4E, OpCode::new("LSR", CPU::lsr, Absolute,    3, 6)),
        (0x5E, OpCode::new("LSR", CPU::lsr, Absolute_X,  3, 7)),

        (0xEA, OpCode::new("NOP", CPU::nop, NoneAddressing, 1, 2)),

        (0x09, OpCode::new("ORA", CPU::ora, Immediate,  2, 2)),
        (0x05, OpCode::new("ORA", CPU::ora, ZeroPage,   2, 3)),
        (0x15, OpCode::new("ORA", CPU::ora, ZeroPage_X, 2, 4)),
        (0x0D, OpCode::new("ORA", CPU::ora, Absolute,   3, 4)),
        (0x1D, OpCode::new("ORA", CPU::ora, Absolute_X, 3, 4)),
        (0x19, OpCode::new("ORA", CPU::ora, Absolute_Y, 3, 4)),
        (0x01, OpCode::new("ORA", CPU::ora, Indirect_X, 2, 6)),
        (0x11, OpCode::new("ORA", CPU::ora, Indirect_Y, 2, 5)),

        (0x48, OpCode::new("PHA", CPU::pha, NoneAddressing, 1, 3)),

        (0x08, OpCode::new("PHP", CPU::php, NoneAddressing, 1, 3)),

        (0x68, OpCode::new("PLA", CPU::pla, NoneAddressing, 1, 3)),

        (0x28, OpCode::new("PLP", CPU::plp, NoneAddressing, 1, 3)),

        (0x2A, OpCode::new("ROL", CPU::rol, Accumulator, 1, 2)),
        (0x26, OpCode::new("ROL", CPU::rol, ZeroPage,    2, 5)),
        (0x36, OpCode::new("ROL", CPU::rol, ZeroPage_X,  2, 6)),
        (0x2E, OpCode::new("ROL", CPU::rol, Absolute,    3, 6)),
        (0x3E, OpCode::new("ROL", CPU::rol, Absolute_X,  3, 7)),

        (0x6A, OpCode::new("ROR", CPU::ror, Accumulator, 1, 2)),
        (0x66, OpCode::new("ROR", CPU::ror, ZeroPage,    2, 5)),
        (0x76, OpCode::new("ROR", CPU::ror, ZeroPage_X,  2, 6)),
        (0x6E, OpCode::new("ROR", CPU::ror, Absolute,    3, 6)),
        (0x7E, OpCode::new("ROR", CPU::ror, Absolute_X,  3, 7)),

        (0x40, OpCode::new("RTI", CPU::rti, NoneAddressing, 1, 6)),

        (0x60, OpCode::new("RTS", CPU::rts, NoneAddressing, 1, 6)),

        (0xE9, OpCode::new("SBC", CPU::sbc, Immediate,  2, 2)),
        (0xE5, OpCode::new("SBC", CPU::sbc, ZeroPage,   2, 3)),
        (0xF5, OpCode::new("SBC", CPU::sbc, ZeroPage_X, 2, 4)),
        (0xED, OpCode::new("SBC", CPU::sbc, Absolute,   3, 4)),
        (0xFD, OpCode::new("SBC", CPU::sbc, Absolute_X, 3, 4)),
        (0xF9, OpCode::new("SBC", CPU::sbc, Absolute_Y, 3, 4)),
        (0xE1, OpCode::new("SBC", CPU::sbc, Indirect_X, 2, 6)),
        (0xF1, OpCode::new("SBC", CPU::sbc, Indirect_Y, 2, 5)),

        (0x38, OpCode::new("SEC", CPU::sec, NoneAddressing, 1, 2)),

        (0xF8, OpCode::new("SED", CPU::sed, NoneAddressing, 1, 2)),

        (0x78, OpCode::new("SEI", CPU::sei, NoneAddressing, 1, 2)),

        (0x85, OpCode::new("STA", CPU::sta, ZeroPage,   2, 3)),
        (0x95, OpCode::new("STA", CPU::sta, ZeroPage_X, 2, 4)),
        (0x8D, OpCode::new("STA", CPU::sta, Absolute,   3, 4)),
        (0x9D, OpCode::new("STA", CPU::sta, Absolute_X, 3, 5)),
        (0x99, OpCode::new("STA", CPU::sta, Absolute_Y, 3, 5)),
        (0x81, OpCode::new("STA", CPU::sta, Indirect_X, 2, 6)),
        (0x91, OpCode::new("STA", CPU::sta, Indirect_Y, 2, 6)),

        (0x86, OpCode::new("STX", CPU::stx, ZeroPage,   2, 3)),
        (0x96, OpCode::new("STX", CPU::stx, ZeroPage_Y, 2, 4)),
        (0x8E, OpCode::new("STX", CPU::stx, Absolute,   3, 4)),

        (0x84, OpCode::new("STY", CPU::sty, ZeroPage,   2, 3)),
        (0x94, OpCode::new("STY", CPU::sty, ZeroPage_X, 2, 4)),
        (0x8C, OpCode::new("STY", CPU::sty, Absolute,   3, 4)),

        (0xAA, OpCode::new("TAX", CPU::tax, NoneAddressing, 1, 2)),

        (0xA8, OpCode::new("TAY", CPU::tay, NoneAddressing, 1, 2)),

        (0xBA, OpCode::new("TSX", CPU::tsx, NoneAddressing, 1, 2)),

        (0x8A, OpCode::new("TXA", CPU::txa, NoneAddressing, 1, 2)),

        (0x9A, OpCode::new("TXS", CPU::txs, NoneAddressing, 1, 2)),

        (0x98, OpCode::new("TYA", CPU::tya, NoneAddressing, 1, 2)),
    ]);
);
