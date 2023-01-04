use crate::cpu::cpu::CPU;

const CARRY: u8                = 0b0000_0001;
const ZERO: u8                 = 0b0000_0010;
const INTERRUPT_DISABLE: u8    = 0b0000_0100;
const DECIMAL_MODE: u8         = 0b0000_1000;
const BREAK: u8                = 0b0001_0000;
const BREAK2: u8               = 0b0010_0000;
const OVERFLOW: u8             = 0b0100_0000;
const NEGATIVE: u8             = 0b1000_0000;

impl CPU {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.set_zero_flag(result == 0);
        self.set_negative_flag(result & 0b1000_0000 != 0);
    }

    pub fn set_negative_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | NEGATIVE
        } else {
            self.status & !NEGATIVE
        };
    }

    pub fn set_overflow_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | OVERFLOW
        } else {
            self.status & !OVERFLOW
        };
    }

    pub fn set_break2_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | BREAK2
        } else {
            self.status & !BREAK2
        };
    }

    pub fn set_break_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | BREAK
        } else {
            self.status & !BREAK
        };
    }

    pub fn set_decimal_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | DECIMAL_MODE
        } else {
            self.status & !DECIMAL_MODE
        };
    }

    pub fn set_interrupt_disable_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | INTERRUPT_DISABLE
        } else {
            self.status & !INTERRUPT_DISABLE
        };
    }

    pub fn set_zero_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | ZERO
        } else {
            self.status & !ZERO
        };
    }

    pub fn set_carry_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | CARRY
        } else {
            self.status & !CARRY
        };
    }

    pub fn is_negative_set(&self) -> bool {
        self.status & NEGATIVE != 0
    }
    pub fn is_overflow_set(&self) -> bool {
        self.status & OVERFLOW != 0
    }
    pub fn is_break2_set(&self) -> bool {
        self.status & BREAK2 != 0
    }
    pub fn is_break_set(&self) -> bool {
        self.status & BREAK != 0
    }
    pub fn is_decimal_mode_set(&self) -> bool {
        self.status & DECIMAL_MODE != 0
    }
    pub fn is_interrupt_disable_set(&self) -> bool {
        self.status & INTERRUPT_DISABLE != 0
    }
    pub fn is_zero_set(&self) -> bool {
        self.status & ZERO != 0
    }
    pub fn is_carry_set(&self) -> bool {
        self.status & CARRY != 0
    }
}
