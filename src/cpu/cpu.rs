use crate::{bus::Bus, rom::Rom};

use super::{opscodes::OPS_CODES, memory::Mem};

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
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }
 
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
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
            status: 0,
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
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU)
    {
        loop {
            let opscode = self.mem_read(self.program_counter);
            if opscode == 0 {return;}

            self.program_counter += 1;

            let pc = self.program_counter;

            let ops = OPS_CODES.get(&opscode).expect(&format!("Invalid operation: {opscode:x}"));

            (ops.call)(self, &ops.mode);

            if pc == self.program_counter {
                self.program_counter += (ops.len - 1) as u16;
            }

            callback(self);
        }
    }
}
