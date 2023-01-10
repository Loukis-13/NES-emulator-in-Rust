use crate::{bus::Bus, rom::Rom};

use super::{memory::Mem, opscodes::OPS_CODES};

// static DEBUG: bool = true;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_counter: u8,
    pub bus: Bus,
}

impl Mem for CPU {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

impl CPU {
    pub fn new(rom: Rom) -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0100,
            program_counter: 0,
            stack_counter: 0xFD,
            bus: Bus::new(rom),
        }
    }

    pub fn stack_push(&mut self, data: u8) {
        if self.stack_counter == 0 {
            panic!("Stack overflow");
        }

        self.mem_write(0x0100 + self.stack_counter as u16, data);
        self.stack_counter -= 1;
    }

    pub fn stack_pop(&mut self) -> u8 {
        if self.stack_counter == 0xFD {
            panic!("Stack empty");
        }

        self.stack_counter += 1;
        self.mem_read(0x0100 + self.stack_counter as u16)
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        self.stack_push((data >> 8) as u8);
        self.stack_push((data & 0x00FF) as u8);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        for i in 0..(program.len() as u16) {
            self.mem_write(0x0600 + i, program[i as usize]);
        }
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0b0010_0100;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            let opscode = self.mem_read(self.program_counter);
            if opscode == 0 {
                return;
            }

            self.program_counter += 1;

            let pc = self.program_counter;

            let ops = OPS_CODES
                .get(&opscode)
                .expect(&format!("Invalid operation: {opscode:x}"));

            // if DEBUG {
            //     print!("{:4X}  ", self.program_counter);

            //     match ops.len {
            //         1 => print!("{:02X}      ", opscode),
            //         2 => print!("{:02X} {:02X}   ", opscode, self.mem_read(pc)),
            //         3 => print!("{:02X} {:02X} {:02X}", opscode, self.mem_read(pc), self.mem_read(pc+1)),
            //         _ => panic!("More than three bytes taken in a cycle"),
            //     }

            //     print!("  {}", ops.name);

            //     print!(
            //         "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            //         self.register_a,
            //         self.register_x,
            //         self.register_y,
            //         self.status,
            //         self.stack_counter
            //     );

            //     println!();
            // }

            (ops.call)(self, &ops.mode);

            self.bus.tick(ops.cycles);

            if pc == self.program_counter {
                self.program_counter += (ops.len - 1) as u16;
            }

            // callback(self);
        }
    }
}

#[cfg(test)]
mod test_cpu {
    use crate::cpu::trace::trace;

    use super::*;

    #[test]
    fn test_() {
        let game_code = std::fs::read("test/nestest.nes").unwrap();
        let rom = Rom::new(&game_code).unwrap();

        //load the game
        let mut cpu = CPU::new(rom);
        cpu.reset();

        cpu.run_with_callback(move |cpu| {
            println!("{}", trace(cpu));
        });
    }
}
