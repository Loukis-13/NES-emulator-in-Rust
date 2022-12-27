use lazy_static::lazy_static;
use std::collections::HashMap;

use super::cpu::CPU;
use super::addrssing_modes::AddressingMode::{self, *};

pub struct OpCode {
    pub call: fn(&mut CPU, & AddressingMode) -> (),
    pub mode: AddressingMode,
    pub len: u8,
    pub cycles: u8,
}

impl OpCode {
    pub fn new(call: fn(&mut CPU, & AddressingMode) -> (), mode: AddressingMode, len: u8, cycles: u8) -> Self { Self { call, mode, len, cycles } }
}

lazy_static!(
    pub static ref OPS_CODES: HashMap<u8, OpCode> = HashMap::from([
        // (0x00, OpCode::new(CPU::brk, NoneAddressing, 0, 0)),

        (0x69, OpCode::new(CPU::adc, Immediate,  2, 2)),
        (0x65, OpCode::new(CPU::adc, ZeroPage,   2, 3)),
        (0x75, OpCode::new(CPU::adc, ZeroPage_X, 2, 4)),
        (0x6D, OpCode::new(CPU::adc, Absolute,   3, 4)),
        (0x7D, OpCode::new(CPU::adc, Absolute_X, 3, 4)),
        (0x79, OpCode::new(CPU::adc, Absolute_Y, 3, 4)),
        (0x61, OpCode::new(CPU::adc, Indirect_X, 2, 5)),
        (0x71, OpCode::new(CPU::adc, Indirect_Y, 2, 6)),

        (0x29, OpCode::new(CPU::and, Immediate,  2, 2)),
        (0x25, OpCode::new(CPU::and, ZeroPage,   2, 3)),
        (0x35, OpCode::new(CPU::and, ZeroPage_X, 2, 4)),
        (0x2D, OpCode::new(CPU::and, Absolute,   3, 4)),
        (0x3D, OpCode::new(CPU::and, Absolute_X, 3, 4)),
        (0x39, OpCode::new(CPU::and, Absolute_Y, 3, 4)),
        (0x21, OpCode::new(CPU::and, Indirect_X, 2, 5)),
        (0x31, OpCode::new(CPU::and, Indirect_Y, 2, 6)),

        (0x0A, OpCode::new(CPU::asl, Accumulator, 1, 2)),
        (0x06, OpCode::new(CPU::asl, ZeroPage,    2, 5)),
        (0x16, OpCode::new(CPU::asl, ZeroPage_X,  2, 6)),
        (0x0E, OpCode::new(CPU::asl, Absolute,    3, 6)),
        (0x1E, OpCode::new(CPU::asl, Absolute_X,  3, 7)),

        (0x90, OpCode::new(CPU::bcc, NoneAddressing, 2, 2)),

        (0xB0, OpCode::new(CPU::bcs, NoneAddressing, 2, 2)),

        (0xF0, OpCode::new(CPU::beq, NoneAddressing, 2, 2)),

        (0x30, OpCode::new(CPU::bmi, NoneAddressing, 2, 2)),

        (0xD0, OpCode::new(CPU::bne, NoneAddressing, 2, 2)),

        (0x10, OpCode::new(CPU::bpl, NoneAddressing, 2, 2)),

        (0x50, OpCode::new(CPU::bvc, NoneAddressing, 2, 2)),

        (0x70, OpCode::new(CPU::bvs, NoneAddressing, 2, 2)),

        (0x24, OpCode::new(CPU::bit, ZeroPage, 2, 3)),
        (0x2C, OpCode::new(CPU::bit, Absolute, 3, 4)),

        (0x18, OpCode::new(CPU::clc, NoneAddressing, 1, 2)),

        (0xD8, OpCode::new(CPU::cld, NoneAddressing, 1, 2)),

        (0x58, OpCode::new(CPU::cli, NoneAddressing, 1, 2)),

        (0xB8, OpCode::new(CPU::clv, NoneAddressing, 1, 2)),

        (0xC9, OpCode::new(CPU::cmp, Immediate,  2, 2)),
        (0xC5, OpCode::new(CPU::cmp, ZeroPage,   2, 3)),
        (0xD5, OpCode::new(CPU::cmp, ZeroPage_X, 2, 4)),
        (0xCD, OpCode::new(CPU::cmp, Absolute,   3, 4)),
        (0xDD, OpCode::new(CPU::cmp, Absolute_X, 3, 4)),
        (0xD9, OpCode::new(CPU::cmp, Absolute_Y, 3, 4)),
        (0xC1, OpCode::new(CPU::cmp, Indirect_X, 2, 6)),
        (0xD1, OpCode::new(CPU::cmp, Indirect_Y, 2, 5)),

        (0xE0, OpCode::new(CPU::cpx, Immediate, 2, 2)),
        (0xE4, OpCode::new(CPU::cpx, ZeroPage,  2, 3)),
        (0xEC, OpCode::new(CPU::cpx, Absolute,  3, 4)),

        (0xC0, OpCode::new(CPU::cpy, Immediate, 2, 2)),
        (0xC4, OpCode::new(CPU::cpy, ZeroPage,  2, 3)),
        (0xCC, OpCode::new(CPU::cpy, Absolute,  3, 4)),

        (0xC6, OpCode::new(CPU::dec, ZeroPage,   2, 5)),
        (0xD6, OpCode::new(CPU::dec, ZeroPage_X, 2, 6)),
        (0xCE, OpCode::new(CPU::dec, Absolute,   3, 6)),
        (0xDE, OpCode::new(CPU::dec, Absolute_X, 3, 7)),

        (0xCA, OpCode::new(CPU::dex, NoneAddressing, 1, 2)),

        (0x88, OpCode::new(CPU::dey, NoneAddressing, 1, 2)),

        (0x49, OpCode::new(CPU::eor, Immediate,  2, 2)),
        (0x45, OpCode::new(CPU::eor, ZeroPage,   2, 3)),
        (0x55, OpCode::new(CPU::eor, ZeroPage_X, 2, 4)),
        (0x4D, OpCode::new(CPU::eor, Absolute,   3, 4)),
        (0x5D, OpCode::new(CPU::eor, Absolute_X, 3, 4)),
        (0x59, OpCode::new(CPU::eor, Absolute_Y, 3, 4)),
        (0x41, OpCode::new(CPU::eor, Indirect_X, 2, 5)),
        (0x51, OpCode::new(CPU::eor, Indirect_Y, 2, 6)),

        (0xE6, OpCode::new(CPU::inc, ZeroPage,   2, 5)),
        (0xF6, OpCode::new(CPU::inc, ZeroPage_X, 2, 6)),
        (0xEE, OpCode::new(CPU::inc, Absolute,   3, 6)),
        (0xFE, OpCode::new(CPU::inc, Absolute_X, 3, 7)),

        (0xE8, OpCode::new(CPU::inx, NoneAddressing, 1, 2)),

        (0xC8, OpCode::new(CPU::iny, NoneAddressing, 1, 2)),

        (0x4C, OpCode::new(CPU::jmp, Absolute, 3, 3)),
        (0x6C, OpCode::new(CPU::jmp, Indirect, 3, 5)),

        (0x20, OpCode::new(CPU::jsr, Absolute, 3, 6)),

        (0xA9, OpCode::new(CPU::lda, Immediate,  2, 2)),
        (0xA5, OpCode::new(CPU::lda, ZeroPage,   2, 3)),
        (0xB5, OpCode::new(CPU::lda, ZeroPage_X, 2, 4)),
        (0xAD, OpCode::new(CPU::lda, Absolute,   3, 4)),
        (0xBD, OpCode::new(CPU::lda, Absolute_X, 3, 4)),
        (0xB9, OpCode::new(CPU::lda, Absolute_Y, 3, 4)),
        (0xA1, OpCode::new(CPU::lda, Indirect_X, 2, 6)),  
        (0xB1, OpCode::new(CPU::lda, Indirect_Y, 2, 5)),

        (0xA2, OpCode::new(CPU::ldx, Immediate,  2, 2)),
        (0xA6, OpCode::new(CPU::ldx, ZeroPage,   2, 3)),
        (0xB6, OpCode::new(CPU::ldx, ZeroPage_Y, 2, 4)),
        (0xAE, OpCode::new(CPU::ldx, Absolute,   3, 4)),
        (0xBE, OpCode::new(CPU::ldx, Absolute_Y, 3, 4)),

        (0xA0, OpCode::new(CPU::ldy, Immediate,  2, 2)),
        (0xA4, OpCode::new(CPU::ldy, ZeroPage,   2, 3)),
        (0xB4, OpCode::new(CPU::ldy, ZeroPage_X, 2, 4)),
        (0xAC, OpCode::new(CPU::ldy, Absolute,   3, 4)),
        (0xBC, OpCode::new(CPU::ldy, Absolute_X, 3, 4)),

        (0x4A, OpCode::new(CPU::lsr, Accumulator, 1, 2)),
        (0x46, OpCode::new(CPU::lsr, ZeroPage,    2, 5)),
        (0x56, OpCode::new(CPU::lsr, ZeroPage_X,  2, 6)),
        (0x4E, OpCode::new(CPU::lsr, Absolute,    3, 6)),
        (0x5E, OpCode::new(CPU::lsr, Absolute_X,  3, 7)),

        (0xEA, OpCode::new(CPU::nop, NoneAddressing, 1, 2)),

        (0x09, OpCode::new(CPU::ora, Immediate,  2, 2)),
        (0x05, OpCode::new(CPU::ora, ZeroPage,   2, 3)),
        (0x15, OpCode::new(CPU::ora, ZeroPage_X, 2, 4)),
        (0x0D, OpCode::new(CPU::ora, Absolute,   3, 4)),
        (0x1D, OpCode::new(CPU::ora, Absolute_X, 3, 4)),
        (0x19, OpCode::new(CPU::ora, Absolute_Y, 3, 4)),
        (0x01, OpCode::new(CPU::ora, Indirect_X, 2, 6)),  
        (0x11, OpCode::new(CPU::ora, Indirect_Y, 2, 5)),

        (0x48, OpCode::new(CPU::pha, NoneAddressing, 1, 3)),

        (0x08, OpCode::new(CPU::php, NoneAddressing, 1, 3)),

        (0x68, OpCode::new(CPU::pla, NoneAddressing, 1, 3)),

        (0x28, OpCode::new(CPU::plp, NoneAddressing, 1, 3)),

        (0x2A, OpCode::new(CPU::rol, Accumulator, 1, 2)),
        (0x26, OpCode::new(CPU::rol, ZeroPage,    2, 5)),
        (0x36, OpCode::new(CPU::rol, ZeroPage_X,  2, 6)),
        (0x2E, OpCode::new(CPU::rol, Absolute,    3, 6)),
        (0x3E, OpCode::new(CPU::rol, Absolute_X,  3, 7)),

        (0x6A, OpCode::new(CPU::ror, Accumulator, 1, 2)),
        (0x66, OpCode::new(CPU::ror, ZeroPage,    2, 5)),
        (0x76, OpCode::new(CPU::ror, ZeroPage_X,  2, 6)),
        (0x6E, OpCode::new(CPU::ror, Absolute,    3, 6)),
        (0x7E, OpCode::new(CPU::ror, Absolute_X,  3, 7)),

        (0x40, OpCode::new(CPU::rti, NoneAddressing, 1, 6)),

        (0x60, OpCode::new(CPU::rts, NoneAddressing, 1, 6)),

        (0xE9, OpCode::new(CPU::sbc, Immediate,  2, 2)),
        (0xE5, OpCode::new(CPU::sbc, ZeroPage,   2, 3)),
        (0xF5, OpCode::new(CPU::sbc, ZeroPage_X, 2, 4)),
        (0xED, OpCode::new(CPU::sbc, Absolute,   3, 4)),
        (0xFD, OpCode::new(CPU::sbc, Absolute_X, 3, 4)),
        (0xF9, OpCode::new(CPU::sbc, Absolute_Y, 3, 4)),
        (0xE1, OpCode::new(CPU::sbc, Indirect_X, 2, 6)),  
        (0xF1, OpCode::new(CPU::sbc, Indirect_Y, 2, 5)),

        (0x38, OpCode::new(CPU::sec, NoneAddressing, 1, 2)),

        (0xF8, OpCode::new(CPU::sed, NoneAddressing, 1, 2)),

        (0x78, OpCode::new(CPU::sei, NoneAddressing, 1, 2)),

        (0x85, OpCode::new(CPU::sta, ZeroPage,   2, 3)),
        (0x95, OpCode::new(CPU::sta, ZeroPage_X, 2, 4)),
        (0x8D, OpCode::new(CPU::sta, Absolute,   3, 4)),
        (0x9D, OpCode::new(CPU::sta, Absolute_X, 3, 5)),
        (0x99, OpCode::new(CPU::sta, Absolute_Y, 3, 5)),
        (0x81, OpCode::new(CPU::sta, Indirect_X, 2, 6)),
        (0x91, OpCode::new(CPU::sta, Indirect_Y, 2, 6)),

        (0x86, OpCode::new(CPU::stx, ZeroPage,   2, 3)),
        (0x96, OpCode::new(CPU::stx, ZeroPage_Y, 2, 4)),
        (0x8E, OpCode::new(CPU::stx, Absolute,   3, 4)),

        (0x84, OpCode::new(CPU::sty, ZeroPage,   2, 3)),
        (0x94, OpCode::new(CPU::sty, ZeroPage_Y, 2, 4)),
        (0x8C, OpCode::new(CPU::sty, Absolute,   3, 4)),

        (0xAA, OpCode::new(CPU::tax, NoneAddressing, 1, 2)),

        (0xA8, OpCode::new(CPU::tay, NoneAddressing, 1, 2)),

        (0xBA, OpCode::new(CPU::tsx, NoneAddressing, 1, 2)),

        (0x8A, OpCode::new(CPU::txa, NoneAddressing, 1, 2)),

        (0x9A, OpCode::new(CPU::txs, NoneAddressing, 1, 2)),

        (0x98, OpCode::new(CPU::tya, NoneAddressing, 1, 2)),
    ]);
);
