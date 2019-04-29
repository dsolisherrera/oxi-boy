use super::flagsregister::FlagsRegister;
use super::instructions::ArithmeticRegisters;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

    pub fn load(&self, reg: ArithmeticRegisters) -> u8 {
        match reg {
            ArithmeticRegisters::A => self.a,
            ArithmeticRegisters::B => self.b,
            ArithmeticRegisters::C => self.c,
            ArithmeticRegisters::D => self.d,
            ArithmeticRegisters::E => self.e,
            ArithmeticRegisters::H => self.h,
            ArithmeticRegisters::L => self.l,
        }
    }
}

#[cfg(test)]
mod registers_tests {
    use super::*;

    #[test]
    fn test_load_works_for_reg_a() {
        let mut registers: Registers = Default::default();

        registers.a = 1;
        assert_eq!(registers.load(ArithmeticRegisters::A), registers.a);
    }

    #[test]
    fn test_load_works_for_reg_b() {
        let mut registers: Registers = Default::default();

        registers.b = 1;
        assert_eq!(registers.load(ArithmeticRegisters::B), registers.b);
    }

    #[test]
    fn test_load_works_for_reg_c() {
        let mut registers: Registers = Default::default();

        registers.c = 1;
        assert_eq!(registers.load(ArithmeticRegisters::C), registers.c);
    }

    #[test]
    fn test_load_works_for_reg_d() {
        let mut registers: Registers = Default::default();

        registers.d = 1;
        assert_eq!(registers.load(ArithmeticRegisters::D), registers.d);
    }

    #[test]
    fn test_load_works_for_reg_e() {
        let mut registers: Registers = Default::default();

        registers.e = 1;
        assert_eq!(registers.load(ArithmeticRegisters::E), registers.e);
    }

    #[test]
    fn test_load_works_for_reg_h() {
        let mut registers: Registers = Default::default();

        registers.h = 1;
        assert_eq!(registers.load(ArithmeticRegisters::H), registers.h);
    }

    #[test]
    fn test_load_works_for_reg_l() {
        let mut registers: Registers = Default::default();

        registers.l = 1;
        assert_eq!(registers.load(ArithmeticRegisters::L), registers.l);
    }

}
