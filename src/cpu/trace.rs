use super::{addrssing_modes::AddressingMode, opscodes::OPS_CODES, Mem, CPU};

impl<'a> CPU<'a> {
    pub fn get_absolute_address(&mut self, mode: &AddressingMode, addr: u16) -> u16 {
        match mode {
            AddressingMode::ZeroPage => self.mem_read(addr) as u16,

            AddressingMode::Absolute => self.mem_read_u16(addr),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(addr);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(addr);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(addr);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(addr);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(addr);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(addr);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
}

pub fn trace(cpu: &mut CPU) -> String {
    let code = cpu.mem_read(cpu.program_counter);
    let ops = OPS_CODES
        .get(&code)
        .expect(format!("Error: {code}; PC: {}", cpu.program_counter).as_str());

    let begin = cpu.program_counter;
    let mut hex_dump = vec![code];

    let (mem_addr, stored_value) = match ops.mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing | AddressingMode::Accumulator => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&ops.mode, begin + 1);
            (addr, cpu.mem_read(addr))
        }
    };

    let tmp = match ops.len {
        1 => match code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.mem_read(begin + 1);
            // let value = cpu.mem_read(address));
            hex_dump.push(address);

            match ops.mode {
                AddressingMode::Immediate => format!("#${:02X}", address),
                AddressingMode::ZeroPage => format!("${:02X} = {:02X}", mem_addr, stored_value),
                AddressingMode::ZeroPage_X => format!("${address:02X},X @ {mem_addr:02X} = {stored_value:02X}"),
                AddressingMode::ZeroPage_Y => format!("${address:02X},Y @ {mem_addr:02X} = {stored_value:02X}"),
                AddressingMode::Indirect_X => format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    address,
                    address.wrapping_add(cpu.register_x),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::Indirect_Y => format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    address,
                    mem_addr.wrapping_sub(cpu.register_y as u16),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::NoneAddressing => {
                    // assuming local jumps: BNE, BVS, etc....
                    format!("${:04X}", (begin as usize + 2).wrapping_add((address as i8) as usize))
                }
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02X}",
                    ops.mode, code
                ),
            }
        }
        3 => {
            let address_lo = cpu.mem_read(begin + 1);
            let address_hi = cpu.mem_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.mem_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::Indirect => format!("${:04X}", address),
                AddressingMode::Absolute => {
                    if code == 0x6c {
                        //jmp indirect
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.mem_read(address);
                            let hi = cpu.mem_read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.mem_read_u16(address)
                        };

                        format!("(${:04X}) = {:04X}", address, jmp_addr)
                    } else {
                        format!("${:04X} = {:02X}", mem_addr, stored_value)
                    }
                }
                AddressingMode::Absolute_X => format!("${:04X},X @ {:04X} = {:02X}", address, mem_addr, stored_value),
                AddressingMode::Absolute_Y => format!("${:04X},Y @ {:04X} = {:02X}", address, mem_addr, stored_value),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02X}",
                    ops.mode, code
                ),
            }
        }
        _ => String::from(""),
    };

    format!(
        "{:04X}  {:8}  {} {:27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.program_counter,
        hex_dump
            .iter()
            .map(|z| format!("{:02X}", z))
            .collect::<Vec<String>>()
            .join(" "),
        ops.name,
        tmp,
        cpu.register_a,
        cpu.register_x,
        cpu.register_y,
        cpu.status,
        cpu.stack_counter
    )
}
