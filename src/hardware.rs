pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod mbc;
pub mod mmu;
pub mod registers;

pub trait MemoryBlock {
    // Read a byte
    fn read(&self, address: u16) -> u8;

    // Write a byte
    fn write(&mut self, address: u16, value: u8);

    // Read a word (2 bytes)
    fn read_word(&self, address: u16) -> u16 {
        (self.read(address) as u16) | ((self.read(address + 1) as u16) << 8)
    }

    // Write a word (2 bytes)
    fn write_word(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xFF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }
}
