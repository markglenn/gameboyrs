use super::MemoryBlock;

const BANK_SIZE: usize = 0x4000;

fn ram_size(value: u8) -> usize {
    match value {
        1 => 0x800,
        2 => 0x2000,
        3 => 0x8000,
        4 => 0x20000,
        5 => 0x80000,
        _ => 0,
    }
}

enum BankMode {
    ROM,
    RAM,
}

pub struct MBC1 {
    bank_mode: BankMode, // MBC1 has two different maximum memory modes: 16Mbit ROM/8KByte RAM or 4Mbit ROM/32KByte RAM.
    bank: u8,
    ram_enable: bool,
    pub rom: Vec<u8>,
    ram: Vec<u8>,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> MBC1 {
        let ram = vec![0; ram_size(rom[0x0149])];

        MBC1 {
            bank_mode: BankMode::ROM,
            bank: 1,
            ram_enable: false,
            rom,
            ram,
        }
    }

    fn rom_bank_offset(&self) -> usize {
        let bank = (match self.bank_mode {
            BankMode::ROM => self.bank & 0x7f,
            BankMode::RAM => self.bank & 0x1f,
        }) as usize;

        bank * BANK_SIZE
    }
}

impl MemoryBlock for MBC1 {
    fn read(&self, address: u16) -> u8 {
        match address {
            // ROM Bank 0
            0x0000..=0x3FFF => self.rom[address as usize],

            // ROM Bank 1..n
            0x4000..=0x7FFF => {
                let index = self.rom_bank_offset() + address as usize;
                self.rom[index]
            }
            // Video RAM
            0x8000..=0x9FFF => unimplemented!(),
            0xA000..=0xA1FF => {
                if self.ram_enable {
                    return self.ram[(address - 0xA000) as usize];
                }

                // RAM is disabled, so always return 0
                0x00
            }
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, _address: u16, _value: u8) {
        todo!()
    }
}
