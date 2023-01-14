use super::{addrssing_modes::AddressingMode, cpu::CPU, Mem};

impl<'a> CPU<'a> {
    pub fn add_to_a(&mut self, data: u8) {
        let sum = (self.register_a as u16) + self.is_carry_set() as u16 + data as u16;

        self.set_carry_flag(sum > 0xFF);

        let sum = sum as u8;
        self.set_overflow_flag((data ^ sum) & (self.register_a ^ sum) & 0x80 != 0);

        self.register_a = sum;

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn branch(&mut self) {
        self.bus.tick(1);

        let mem_read = self.mem_read(self.program_counter) as i8 as u16;
        let jump = self.program_counter.wrapping_add(1).wrapping_add(mem_read);

        if self.program_counter.wrapping_add(1) & 0xFF00 != jump & 0xFF00 {
            self.bus.tick(1);
        }

        self.program_counter = jump;
    }

    pub fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.add_to_a(data);
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

    pub fn bcs(&mut self) {
        if self.is_carry_set() {
            self.branch();
        }
    }

    pub fn bcc(&mut self) {
        if !self.is_carry_set() {
            self.branch();
        }
    }

    pub fn beq(&mut self) {
        if self.is_zero_set() {
            self.branch();
        }
    }

    pub fn bne(&mut self) {
        if !self.is_zero_set() {
            self.branch();
        }
    }

    pub fn bmi(&mut self) {
        if self.is_negative_set() {
            self.branch();
        }
    }

    pub fn bpl(&mut self) {
        if !self.is_negative_set() {
            self.branch();
        }
    }

    pub fn bvs(&mut self) {
        if self.is_overflow_set() {
            self.branch();
        }
    }

    pub fn bvc(&mut self) {
        if !self.is_overflow_set() {
            self.branch();
        }
    }

    pub fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.set_zero_flag(data & self.register_a == 0);
        self.set_overflow_flag(data & 0b0100_0000 != 0);
        self.set_negative_flag(data & 0b1000_0000 != 0);
    }

    // pub fn brk(&mut self) {
    //     self.status |= 0b0010_0000;
    // }

    pub fn clc(&mut self) {
        self.set_carry_flag(false);
    }

    pub fn cld(&mut self) {
        self.set_decimal_flag(false);
    }

    pub fn cli(&mut self) {
        self.set_interrupt_disable_flag(false);
    }

    pub fn clv(&mut self) {
        self.set_overflow_flag(false);
    }

    pub fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.set_carry_flag(self.register_a >= data);
        self.update_zero_and_negative_flags(self.register_a.wrapping_sub(data));
    }

    pub fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.set_carry_flag(self.register_x >= data);
        self.update_zero_and_negative_flags(self.register_x.wrapping_sub(data));
    }

    pub fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.set_carry_flag(self.register_y >= data);
        self.update_zero_and_negative_flags(self.register_y.wrapping_sub(data));
    }

    pub fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr).wrapping_sub(1);

        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
    }

    pub fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a ^= data;
        self.update_zero_and_negative_flags(self.register_a);
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

    pub fn jmp(&mut self, mode: &AddressingMode) {
        let mut addr = self.mem_read_u16(self.program_counter);

        if matches!(mode, AddressingMode::Indirect) {
            addr = if addr & 0x00FF == 0x00FF {
                let lo = self.mem_read(addr);
                let hi = self.mem_read(addr & 0xFF00);
                (hi as u16) << 8 | (lo as u16)
            } else {
                self.mem_read_u16(addr)
            };
        }

        self.program_counter = addr;
    }

    pub fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 1);
        self.program_counter = self.mem_read_u16(self.program_counter);
    }

    pub fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = data;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_x = data;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_y = data;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn lsr(&mut self, mode: &AddressingMode) {
        if matches!(mode, AddressingMode::Accumulator) {
            self.set_carry_flag(self.register_a & 0b0000_0001 != 0);

            self.register_a >>= 1;

            self.update_zero_and_negative_flags(self.register_a);

            return;
        }

        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        self.set_carry_flag(data & 0b0000_0001 != 0);

        data >>= 1;
        self.mem_write(addr, data);

        self.update_zero_and_negative_flags(data);
    }

    pub fn nop(&mut self) {
        /* Problem? */
    }

    pub fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a |= data as u8;

        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    pub fn php(&mut self) {
        self.stack_push(self.status | 0b0011_0000);
    }

    pub fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn plp(&mut self) {
        self.status = self.stack_pop();
        self.set_break_flag(false);
        self.set_break2_flag(true);
    }

    pub fn rol(&mut self, mode: &AddressingMode) {
        let m = self.is_carry_set() as u8;

        if matches!(mode, AddressingMode::Accumulator) {
            self.set_carry_flag(self.register_a & 0b1000_0000 != 0);

            self.register_a <<= 1;
            self.register_a |= m;

            self.update_zero_and_negative_flags(self.register_a);

            return;
        }

        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        self.set_carry_flag(data & 0b1000_0000 != 0);

        data <<= 1;
        data |= m;
        self.mem_write(addr, data);

        self.update_zero_and_negative_flags(data);
    }

    pub fn ror(&mut self, mode: &AddressingMode) {
        let m = if self.is_carry_set() { 0b1000_0000 } else { 0 };

        if matches!(mode, AddressingMode::Accumulator) {
            self.set_carry_flag(self.register_a & 0b0000_0001 != 0);

            self.register_a >>= 1;
            self.register_a |= m;

            self.update_zero_and_negative_flags(self.register_a);

            return;
        }

        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        self.set_carry_flag(data & 0b0000_0001 != 0);

        data >>= 1;
        data |= m;
        self.mem_write(addr, data);

        self.update_zero_and_negative_flags(data);
    }

    pub fn rti(&mut self) {
        self.status = self.stack_pop();
        self.set_break_flag(false);
        self.set_break2_flag(true);
        self.program_counter = self.stack_pop_u16();
    }

    pub fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    pub fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.add_to_a(!data);
    }

    pub fn sec(&mut self) {
        self.set_carry_flag(true);
    }

    pub fn sed(&mut self) {
        self.set_decimal_flag(true);
    }

    pub fn sei(&mut self) {
        self.set_interrupt_disable_flag(true);
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

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn tsx(&mut self) {
        self.register_x = self.stack_counter;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn txs(&mut self) {
        self.stack_counter = self.register_x;
    }

    pub fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
}
