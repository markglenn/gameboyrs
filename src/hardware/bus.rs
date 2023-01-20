use super::{cartridge::Cartridge, MemoryBlock};

pub struct Bus {
    cartridge: Cartridge,
}

impl MemoryBlock for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read(address),
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write(address, value),
            _ => unreachable!(),
        }
    }
}
