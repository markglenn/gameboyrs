use std::{fs, io, str};

use super::{mbc::MBC1, MemoryBlock};

pub struct Cartridge {
    mbc: MBC1,
    title: String,
}

impl Cartridge {
    pub fn load(rom_filename: &str) -> io::Result<Cartridge> {
        let rom = fs::read(rom_filename)?;

        let title = str::from_utf8(&rom[0x0134..0x0143]).unwrap().into();
        let mbc = MBC1::new(rom);

        Ok(Cartridge { mbc, title })
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

impl MemoryBlock for Cartridge {
    // Delegate all read and write calls to the Memory Bank Controller (MBC)

    fn read(&self, address: u16) -> u8 {
        self.mbc.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mbc.write(address, value)
    }
}
