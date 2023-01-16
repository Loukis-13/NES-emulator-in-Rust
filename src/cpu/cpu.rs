use crate::bus::Bus;

use super::{flags, memory::Mem, opscodes::OPS_CODES};

// static DEBUG: bool = true;
const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

pub struct CPU<'a> {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_counter: u8,
    pub bus: Bus<'a>,
}

impl<'a> Mem for CPU<'a> {
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

impl<'a> CPU<'a> {
    pub fn new<'b>(bus: Bus<'b>) -> CPU<'b> {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0100,
            program_counter: 0,
            stack_counter: STACK_RESET,
            bus,
        }
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK + self.stack_counter as u16, data);
        self.stack_counter -= 1;
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_counter += 1;
        self.mem_read(STACK + self.stack_counter as u16)
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

    fn interrupt_nmi(&mut self) {
        self.stack_push_u16(self.program_counter);
        let mut flag = self.status.clone();
        flag &= !flags::BREAK;
        flag |= flags::BREAK2;

        self.stack_push(flag);
        self.set_interrupt_disable_flag(true);

        self.bus.tick(2);
        self.program_counter = self.mem_read_u16(0xfffA);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.program_counter = 0x0600;
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            self.mem_write(0x0600 + i, program[i as usize]);
        }
        // self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_counter = STACK_RESET;
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
            if let Some(_nmi) = self.bus.poll_nmi_status() {
                self.interrupt_nmi();
            }

            callback(self);

            // debug
            // println!("{}", trace(self));

            let opscode = self.mem_read(self.program_counter);

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

            // (ops.call)(self, &ops.mode);

            match ops.name {
                "BRK" => return,
                "ADC" => self.adc(&ops.mode),
                "AND" => self.and(&ops.mode),
                "ASL" => self.asl(&ops.mode),
                "BCC" => self.bcc(),
                "BCS" => self.bcs(),
                "BEQ" => self.beq(),
                "BIT" => self.bit(&ops.mode),
                "BMI" => self.bmi(),
                "BNE" => self.bne(),
                "BPL" => self.bpl(),
                "BVC" => self.bvc(),
                "BVS" => self.bvs(),
                "CLC" => self.clc(),
                "CLD" => self.cld(),
                "CLI" => self.cli(),
                "CLV" => self.clv(),
                "CMP" => self.cmp(&ops.mode),
                "CPX" => self.cpx(&ops.mode),
                "CPY" => self.cpy(&ops.mode),
                "DEC" => self.dec(&ops.mode),
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "EOR" => self.eor(&ops.mode),
                "INC" => self.inc(&ops.mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "JMP" => self.jmp(&ops.mode),
                "JSR" => self.jsr(),
                "LDA" => self.lda(&ops.mode),
                "LDX" => self.ldx(&ops.mode),
                "LDY" => self.ldy(&ops.mode),
                "LSR" => self.lsr(&ops.mode),
                "NOP" => self.nop(),
                "ORA" => self.ora(&ops.mode),
                "PHA" => self.pha(),
                "PHP" => self.php(),
                "PLA" => self.pla(),
                "PLP" => self.plp(),
                "ROL" => self.rol(&ops.mode),
                "ROR" => self.ror(&ops.mode),
                "RTI" => self.rti(),
                "RTS" => self.rts(),
                "SBC" => self.sbc(&ops.mode),
                "SEC" => self.sec(),
                "SED" => self.sed(),
                "SEI" => self.sei(),
                "STA" => self.sta(&ops.mode),
                "STX" => self.stx(&ops.mode),
                "STY" => self.sty(&ops.mode),
                "TAX" => self.tax(),
                "TAY" => self.tay(),
                "TSX" => self.tsx(),
                "TXA" => self.txa(),
                "TXS" => self.txs(),
                "TYA" => self.tya(),
                x => panic!("Invalid Code: {x}"),
            }

            self.bus.tick(ops.cycles);

            if pc == self.program_counter {
                self.program_counter += (ops.len - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::controller::Joypad;
    use crate::ppu::NesPPU;
    use crate::rom::test;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let bus = Bus::new(test::test_rom(), |_ppu: &NesPPU, _joypad: &mut Joypad| {});
        let mut cpu = CPU::new(bus);
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let bus = Bus::new(test::test_rom(), |_ppu: &NesPPU, _joypad: &mut Joypad| {});
        let mut cpu = CPU::new(bus);
        cpu.register_a = 10;
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let bus = Bus::new(test::test_rom(), |_ppu: &NesPPU, _joypad: &mut Joypad| {});
        let mut cpu = CPU::new(bus);
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let bus = Bus::new(test::test_rom(), |_ppu: &NesPPU, _joypad: &mut Joypad| {});
        let mut cpu = CPU::new(bus);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.program_counter = 0x0600;
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let bus = Bus::new(test::test_rom(), |_ppu: &NesPPU, _joypad: &mut Joypad| {});
        let mut cpu = CPU::new(bus);
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
}
