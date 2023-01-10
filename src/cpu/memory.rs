pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        (self.mem_read(pos + 1) as u16).swap_bytes() | (self.mem_read(pos) as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.mem_write(pos, (data & 0xFF) as u8);
        self.mem_write(pos + 1, (data >> 8) as u8);
    }
}
