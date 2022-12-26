use super::opscodes::OPS_CODES;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0u8; 0xFFFF],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&mut self, pos: u16) -> u16 {
        (self.mem_read(pos + 1) as u16).swap_bytes() | (self.mem_read(pos) as u16)
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.mem_write(pos, (data & 0xFF) as u8);
        self.mem_write(pos + 1, (data >> 8) as u8);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let ops = OPS_CODES.get(&opscode).expect(&format!("Invalid operation: {opscode:x}"));

            match ops.name {
                "BRK" => return,
                "ADC" => self.adc(&ops.mode),
                "AND" => self.and(&ops.mode),
                "ASL" => self.asl(&ops.mode),
                "BCC" => self.bcc(),
                "BCS" => self.bcs(),
                "BEQ" => self.beq(),
                "BMI" => self.bmi(),
                "BNE" => self.bne(),
                "BPL" => self.bpl(),
                "BVC" => self.bvc(),
                "BVS" => self.bvs(),
                "BIT" => self.bit(&ops.mode),
                "CLC" => self.clc(),
                "CLD" => self.cld(),
                "CLI" => self.cli(),
                "CLV" => self.clv(),
                "TAX" => self.tax(),
                "TAy" => self.tay(),
                "INC" => self.inc(&ops.mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "LDA" => self.lda(&ops.mode),
                "STA" => self.sta(&ops.mode),
                "STX" => self.stx(&ops.mode),
                "STY" => self.sty(&ops.mode),
                _ => unimplemented!(),
            }

            self.program_counter += (ops.len - 1) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0A,0xAA, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0xC1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
}
