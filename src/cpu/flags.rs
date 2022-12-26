use crate::cpu::cpu::CPU;

// FLAGS 
// CARRY                 0b0000_0001
// ZERO                  0b0000_0010
// INTERRUPT DISABLE     0b0000_0100
// DECIMAL MODE          0b0000_1000
// BREAK                 0b0001_0000
//                       0b0010_0000
// OVERFLOW              0b0100_0000
// NEGATIVE              0b1000_0000

impl CPU {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.set_zero_flag(result == 0);
        self.set_negative_flag(result & 0b1000_0000 != 0);
    }

    pub fn set_negative_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b1000_0000
        } else {
            self.status & 0b0111_1111
        };
    }

    pub fn set_overflow_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0100_0000
        } else {
            self.status & 0b1011_1111
        };
    }

    pub fn set_break_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0001_0000
        } else {
            self.status & 0b1110_1111
        };
    }

    pub fn set_decimal_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0000_1000
        } else {
            self.status & 0b1111_0111
        };
    }

    pub fn set_interrupt_disable_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0000_0100
        } else {
            self.status & 0b1111_1011
        };
    }

    pub fn set_zero_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0000_0010
        } else {
            self.status & 0b1111_1101
        };
    }

    pub fn set_carry_flag(&mut self, result: bool) {
        self.status = if result {
            self.status | 0b0000_0001
        } else {
            self.status & 0b1111_1110
        };
    }
}