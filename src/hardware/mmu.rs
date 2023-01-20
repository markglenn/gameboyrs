use super::MemoryBlock;

const WRAM_SIZE: usize = 0x1000;

pub struct MMU {
    // Work RAM
    pub wram: [u8; WRAM_SIZE],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            wram: [0; WRAM_SIZE],
        }
    }
}

impl MemoryBlock for MMU {
    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xC000..=0xCFFF => self.wram[(address as usize) & 0x0FFF] = byte,
            _ => unimplemented!(),
        }
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            // Work RAM
            0xC000..=0xCFFF => self.wram[(address as usize) & 0x0FFF],

            // CGB Bankable Work RAM
            0xD000..=0xDFFF => unimplemented!(),

            // Echo RAM
            0xE000..=0xFDFF => self.read(address - 0xE000 + 0xC000),

            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::hardware::MemoryBlock;

    use super::MMU;

    #[test]
    fn write_wram() {
        let mut mmu = MMU::new();

        mmu.write(0xC000, 0x01);
        mmu.write(0xC001, 0x02);

        assert_eq!(mmu.read(0xC000), 0x01);
        assert_eq!(mmu.read(0xC001), 0x02);

        assert_eq!(mmu.wram[0], 0x01);
        assert_eq!(mmu.wram[1], 0x02);
    }
}
