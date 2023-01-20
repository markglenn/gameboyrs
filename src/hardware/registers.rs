pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

#[repr(u8)]
/// CPU Flags (stored in register f)
pub enum Flags {
    Zero = 0b1000_0000,
    Subtraction = 0b0100_0000,
    HalfCarry = 0b0010_0000,
    Carry = 0b0001_0000,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn is_flag_set(&self, flag: Flags) -> bool {
        self.f & (flag as u8) != 0
    }

    pub fn is_flag_clear(&self, flag: Flags) -> bool {
        self.f & (flag as u8) == 0
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.f = self.f | flag as u8;
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        self.f &= !(flag as u8);
    }
}

#[cfg(test)]
mod test {
    use crate::hardware::registers::Flags;

    use super::Registers;

    #[test]
    fn wide_registers() {
        let mut reg = Registers::new();
        reg.a = 0x12;
        reg.f = 0x23;
        reg.b = 0x34;
        reg.c = 0x45;
        reg.d = 0x56;
        reg.e = 0x67;
        reg.h = 0x78;
        reg.l = 0x89;

        assert_eq!(reg.af(), 0x1220);
        assert_eq!(reg.bc(), 0x3445);
        assert_eq!(reg.de(), 0x5667);
        assert_eq!(reg.hl(), 0x7889);
    }

    #[test]
    fn flags() {
        let mut reg = Registers::new();

        reg.f = 0b0000_1111;

        assert_eq!(reg.is_flag_set(Flags::Zero), false);
        assert_eq!(reg.is_flag_clear(Flags::Zero), true);
        reg.set_flag(Flags::Zero);
        assert_eq!(reg.f, 0b1000_1111);

        assert_eq!(reg.is_flag_set(Flags::Subtraction), false);
        assert_eq!(reg.is_flag_clear(Flags::Subtraction), true);
        reg.set_flag(Flags::Subtraction);
        assert_eq!(reg.f, 0b1100_1111);

        assert_eq!(reg.is_flag_set(Flags::HalfCarry), false);
        assert_eq!(reg.is_flag_clear(Flags::HalfCarry), true);
        reg.set_flag(Flags::HalfCarry);
        assert_eq!(reg.f, 0b1110_1111);

        assert_eq!(reg.is_flag_set(Flags::Carry), false);
        assert_eq!(reg.is_flag_clear(Flags::Carry), true);
        reg.set_flag(Flags::Carry);
        assert_eq!(reg.f, 0b1111_1111);

        assert_eq!(reg.is_flag_set(Flags::Zero), true);
        assert_eq!(reg.is_flag_set(Flags::Subtraction), true);
        assert_eq!(reg.is_flag_set(Flags::HalfCarry), true);
        assert_eq!(reg.is_flag_set(Flags::Carry), true);

        assert_eq!(reg.is_flag_clear(Flags::Zero), false);
        assert_eq!(reg.is_flag_clear(Flags::Subtraction), false);
        assert_eq!(reg.is_flag_clear(Flags::HalfCarry), false);
        assert_eq!(reg.is_flag_clear(Flags::Carry), false);

        reg.clear_flag(Flags::Carry);
        assert_eq!(reg.f, 0b1110_1111);

        reg.clear_flag(Flags::HalfCarry);
        assert_eq!(reg.f, 0b1100_1111);

        reg.clear_flag(Flags::Subtraction);
        assert_eq!(reg.f, 0b1000_1111);

        reg.clear_flag(Flags::Zero);
        assert_eq!(reg.f, 0b0000_1111);
    }
}
