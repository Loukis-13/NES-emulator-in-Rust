use crate::cpu::cpu::CPU;

pub const CARRY: u8                = 0b0000_0001;
pub const ZERO: u8                 = 0b0000_0010;
pub const INTERRUPT_DISABLE: u8    = 0b0000_0100;
pub const DECIMAL_MODE: u8         = 0b0000_1000;
pub const BREAK: u8                = 0b0001_0000;
pub const BREAK2: u8               = 0b0010_0000;
pub const OVERFLOW: u8             = 0b0100_0000;
pub const NEGATIVE: u8             = 0b1000_0000;

impl<'a> CPU<'a> {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.set_zero_flag(result == 0);
        self.set_negative_flag(result & 0b1000_0000 != 0);
    }

    // SETTERS

    fn set_flag(&mut self, flag: u8, set: bool) {
        self.status = if set { self.status | flag } else { self.status & !flag };
    }

    pub fn set_negative_flag(&mut self, result: bool) {
        self.set_flag(NEGATIVE, result);
    }

    pub fn set_overflow_flag(&mut self, result: bool) {
        self.set_flag(OVERFLOW, result);
    }

    pub fn set_break2_flag(&mut self, result: bool) {
        self.set_flag(BREAK2, result);
    }

    pub fn set_break_flag(&mut self, result: bool) {
        self.set_flag(BREAK, result);
    }

    pub fn set_decimal_flag(&mut self, result: bool) {
        self.set_flag(DECIMAL_MODE, result);
    }

    pub fn set_interrupt_disable_flag(&mut self, result: bool) {
        self.set_flag(INTERRUPT_DISABLE, result);
    }

    pub fn set_zero_flag(&mut self, result: bool) {
        self.set_flag(ZERO, result);
    }

    pub fn set_carry_flag(&mut self, result: bool) {
        self.set_flag(CARRY, result);
    }

    // GETTERS

    pub fn is_negative_set(&self) -> bool {
        self.status & NEGATIVE != 0
    }
    pub fn is_overflow_set(&self) -> bool {
        self.status & OVERFLOW != 0
    }
    // pub fn is_break2_set(&self) -> bool {
    //     self.status & BREAK2 != 0
    // }
    // pub fn is_break_set(&self) -> bool {
    //     self.status & BREAK != 0
    // }
    // pub fn is_decimal_mode_set(&self) -> bool {
    //     self.status & DECIMAL_MODE != 0
    // }
    // pub fn is_interrupt_disable_set(&self) -> bool {
    //     self.status & INTERRUPT_DISABLE != 0
    // }
    pub fn is_zero_set(&self) -> bool {
        self.status & ZERO != 0
    }
    pub fn is_carry_set(&self) -> bool {
        self.status & CARRY != 0
    }
}
