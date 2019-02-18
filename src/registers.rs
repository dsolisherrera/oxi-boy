use super::flagsregister::FlagsRegister;
use super::instructions::ArithmeticRegisters;

#[derive(Default)]
pub struct Registers {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub F: FlagsRegister,
    pub H: u8,
    pub L: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: FlagsRegister::new(),
            H: 0,
            L: 0,
        }
    }

    pub fn load(&self, reg: ArithmeticRegisters) -> u8 {
        match reg {
            ArithmeticRegisters::A => self.A,
            ArithmeticRegisters::B => self.B,
            ArithmeticRegisters::C => self.C,
            ArithmeticRegisters::D => self.D,
            ArithmeticRegisters::E => self.E,
            ArithmeticRegisters::H => self.H,
            ArithmeticRegisters::L => self.L,
        }
    }
}

#[cfg(test)]
mod registers_tests {
    use super::*;

    #[test]
    fn test_load_works_for_reg_a() {
        let mut registers: Registers = Default::default();

        registers.A = 1;
        assert_eq!(registers.load(ArithmeticRegisters::A), registers.A);
    }

    #[test]
    fn test_load_works_for_reg_b() {
        let mut registers: Registers = Default::default();

        registers.B = 1;
        assert_eq!(registers.load(ArithmeticRegisters::B), registers.B);
    }

    #[test]
    fn test_load_works_for_reg_c() {
        let mut registers: Registers = Default::default();

        registers.C = 1;
        assert_eq!(registers.load(ArithmeticRegisters::C), registers.C);
    }

    #[test]
    fn test_load_works_for_reg_d() {
        let mut registers: Registers = Default::default();

        registers.D = 1;
        assert_eq!(registers.load(ArithmeticRegisters::D), registers.D);
    }

    #[test]
    fn test_load_works_for_reg_e() {
        let mut registers: Registers = Default::default();

        registers.E = 1;
        assert_eq!(registers.load(ArithmeticRegisters::E), registers.E);
    }

    #[test]
    fn test_load_works_for_reg_h() {
        let mut registers: Registers = Default::default();

        registers.H = 1;
        assert_eq!(registers.load(ArithmeticRegisters::H), registers.H);
    }

    #[test]
    fn test_load_works_for_reg_l() {
        let mut registers: Registers = Default::default();

        registers.L = 1;
        assert_eq!(registers.load(ArithmeticRegisters::L), registers.L);
    }

}
