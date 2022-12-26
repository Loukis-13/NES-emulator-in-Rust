use super::{cpu::CPU, addrssing_modes::AddressingMode};

impl CPU {
    pub fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let sum = (self.register_a as u16) + data as u16 + (self.status & 0b1000_0000 != 0) as u16;

        self.set_carry_flag(sum > 0xFF);
        self.set_overflow_flag((self.register_a & 0b1000_0000 == 0) && (sum & 0b1000_0000 != 0));

        self.register_a = sum as u8;

        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a &= data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn asl(&mut self, mode: &AddressingMode) {
        if matches!(mode, AddressingMode::Accumulator) {
            self.set_carry_flag(self.register_a & 0b1000_0000 != 0);

            self.register_a <<= 1;

            self.update_zero_and_negative_flags(self.register_a);
            
            return;
        }

        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        self.set_carry_flag(data & 0b1000_0000 != 0);

        data <<= 1;
        self.mem_write(addr, data);

        self.update_zero_and_negative_flags(data);
    }

    fn branch(&mut self) {
        let mem_read = self.mem_read(self.program_counter) as i8 as u16;
        self.program_counter = self.program_counter.wrapping_add(mem_read);

    }

    pub fn bcs(&mut self) {
        if self.status & 0b0000_0001 != 0 { self.branch(); }
    }

    pub fn bcc(&mut self) {
        if self.status & 0b0000_0001 == 0 { self.branch(); }
    }

    pub fn beq(&mut self) {
        if self.status & 0b1000_0000 != 0 { self.branch(); }
    }

    pub fn bpl(&mut self) {
        if self.status & 0b1000_0000 == 0 { self.branch(); }
    }

    pub fn bmi(&mut self) {
        if self.status & 0b0000_0010 != 0 { self.branch(); }
    }

    pub fn bne(&mut self) {
        if self.status & 0b0000_0010 == 0 { self.branch(); }
    }

    pub fn bvs(&mut self) {
        if self.status & 0b0100_0000 != 0 { self.branch(); }
    }

    pub fn bvc(&mut self) {
        if self.status & 0b0100_0000 == 0 { self.branch(); }
    }

    pub fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let result = data & self.register_a;

        self.set_zero_flag(result == 0);
        self.set_overflow_flag(result & 0b0100_0000 != 0);
        self.set_negative_flag(result & 0b1000_0000 != 0);
    }

    pub fn clc(&mut self) {
        self.set_carry_flag(false)  ;
    }

    pub fn cld(&mut self) {
        self.set_decimal_flag(false)  ;
    }

    pub fn cli(&mut self) {
        self.set_interrupt_disable_flag(false)  ;
    }

    pub fn clv(&mut self) {
        self.set_overflow_flag(false)  ;
    }

    // pub fn brk(&mut self) {
    //     self.status |= 0b0010_0000;
    // }

    pub fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr).wrapping_add(1);

        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
    }

    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    pub fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    pub fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }
}
