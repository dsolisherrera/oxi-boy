const ZERO_FLAG_POSITION: u8 = 7;
const OPERATION_FLAG_POSITION: u8 = 6;
const HALF_CARRY_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

struct FlagsRegister {
    zero: bool,
    substraction: bool,
    half_carry: bool,
    carry: bool,
}

impl FlagsRegister {
    fn new() -> Self {
        FlagsRegister {
            zero: false,
            substraction: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_POSITION
            | (if flag.substraction { 1 } else { 0 }) << OPERATION_FLAG_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        FlagsRegister {
            zero: (byte >> ZERO_FLAG_POSITION) & 1 != 0,
            substraction: (byte >> OPERATION_FLAG_POSITION) & 1 != 0,
            half_carry: (byte >> HALF_CARRY_FLAG_POSITION) & 1 != 0,
            carry: (byte >> CARRY_FLAG_POSITION) & 1 != 0,
        }
    }
}

struct Registers {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: FlagsRegister,
    H: u8,
    L: u8,
}

impl Registers {
    fn new() -> Self {
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

    fn load(&self, reg: ArithmeticRegisters) -> u8 {
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

enum Instruction {
    ADD(ArithmeticRegisters),
    ADDI(u8),
    ADDR(),
    ADC(ArithmeticRegisters),
    ADCI(u8),
    ADCR(),
    SUB(ArithmeticRegisters),
    SUBI(u8),
    SUBR(),
    SBC(ArithmeticRegisters),
    SBCI(u8),
    SBCR(),
}

enum ArithmeticRegisters {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct CPU {
    registers: Registers,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(reg) => {
                self.execute_add_reg(reg);
            }
            Instruction::ADDI(immediate) => {
                self.execute_add_immediate(immediate);
            }
            Instruction::ADC(reg) => {
                self.execute_adc_reg(reg);
            }
            Instruction::ADCI(immediate) => {
                self.execute_adc_immediate(immediate);
            }
            Instruction::SUB(reg) => {
                self.execute_sub_reg(reg);
            }
            Instruction::SUBI(immediate) => {
                self.execute_sub_immediate(immediate);
            }
            Instruction::SBC(reg) => {
                self.execute_sbc_reg(reg);
            }
            Instruction::SBCI(immediate) => {
                self.execute_sbc_immediate(immediate);
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn execute_add_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_add_immediate(source_value);
    }

    fn execute_add_immediate(&mut self, immediate: u8) {
        let (new_value, overflow) = self.registers.A.overflowing_add(immediate);

        self.registers.F.zero = new_value == 0;
        self.registers.F.substraction = false;
        self.registers.F.half_carry = (self.registers.A & 0xF) + (immediate & 0xF) > 0xF;
        self.registers.F.carry = overflow;

        self.registers.A = new_value;
    }

    fn execute_add_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_adc_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_adc_immediate(source_value);
    }

    fn execute_adc_immediate(&mut self, immediate: u8) {
        let (new_value, overflow_1) = self.registers.A.overflowing_add(immediate);
        let (new_value, overflow_2) =
            new_value.overflowing_add(if self.registers.F.carry { 1 } else { 0 });

        self.registers.F.zero = new_value == 0;
        self.registers.F.substraction = false;
        self.registers.F.half_carry = (self.registers.A & 0xF)
            + (immediate & 0xF)
            + (if self.registers.F.carry { 1 } else { 0 })
            > 0xF;
        self.registers.F.carry = overflow_1 || overflow_2;

        self.registers.A = new_value;
    }

    fn execute_adc_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_sub_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_sub_immediate(source_value);
    }

    fn execute_sub_immediate(&mut self, immediate: u8) {
        let (new_value, overflow) = self.registers.A.overflowing_sub(immediate);

        self.registers.F.zero = new_value == 0;
        self.registers.F.substraction = true;
        self.registers.F.half_carry = (self.registers.A & 0xF) + (immediate & 0xF) > 0xF;
        self.registers.F.carry = overflow;

        self.registers.A = new_value;
    }

    fn execute_sub_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_sbc_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_sbc_immediate(source_value);
    }

    fn execute_sbc_immediate(&mut self, immediate: u8) {
        let (new_value, overflow_1) = self.registers.A.overflowing_sub(immediate);
        let (new_value, overflow_2) =
            new_value.overflowing_sub(if self.registers.F.carry { 1 } else { 0 });

        self.registers.F.zero = new_value == 0;
        self.registers.F.substraction = true;
        self.registers.F.half_carry = (self.registers.A & 0xF)
            + (immediate & 0xF)
            + (if self.registers.F.carry { 1 } else { 0 })
            > 0xF;
        self.registers.F.carry = overflow_1 || overflow_2;

        self.registers.A = new_value;
    }

    fn execute_scb_relative(&mut self, source: ArithmeticRegisters) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_u8_to_flagsregister_with_all_flags_set() {
        let byte: u8 = (1 << ZERO_FLAG_POSITION)
            | (1 << OPERATION_FLAG_POSITION)
            | (1 << HALF_CARRY_FLAG_POSITION)
            | (1 << CARRY_FLAG_POSITION);
        let flag_reg = FlagsRegister::from(byte);

        assert_eq!(flag_reg.zero, true);
        assert_eq!(flag_reg.substraction, true);
        assert_eq!(flag_reg.half_carry, true);
        assert_eq!(flag_reg.carry, true);
    }

    #[test]
    fn test_convert_u8_to_flagsregister_with_no_flags_set() {
        let byte: u8 = 0;
        let flag_reg = FlagsRegister::from(byte);

        assert_eq!(flag_reg.zero, false);
        assert_eq!(flag_reg.substraction, false);
        assert_eq!(flag_reg.half_carry, false);
        assert_eq!(flag_reg.carry, false);
    }

    #[test]
    fn test_convert_flagsregister_to_u8_with_no_flags_set() {
        let flag_reg = FlagsRegister::new();

        let byte = u8::from(flag_reg);

        assert_eq!(byte, 0);
    }

    #[test]
    fn test_convert_flagsregister_to_u8_with_all_flags_set() {
        let flag_reg = FlagsRegister {
            zero: true,
            substraction: true,
            half_carry: true,
            carry: true,
        };

        let byte = u8::from(flag_reg);

        let expected_byte: u8 = (1 << ZERO_FLAG_POSITION)
            | (1 << OPERATION_FLAG_POSITION)
            | (1 << HALF_CARRY_FLAG_POSITION)
            | (1 << CARRY_FLAG_POSITION);

        assert_eq!(byte, expected_byte);
    }

    #[test]
    fn test_execute_add_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            A: 1,
            B: 2,
            C: 3,
            D: 4,
            E: 5,
            F: FlagsRegister::new(),
            H: 6,
            L: 7,
        };

        cpu.execute(Instruction::ADD(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.A, 2);
        assert_eq!(cpu.registers.F.substraction, false);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, 4);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.A, 7);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.A, 11);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.A, 16);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.A, 22);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.A, 29);
    }

    #[test]
    fn test_execute_add_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        cpu.registers.B = 230;

        cpu.execute(Instruction::ADD(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, (244 + 230) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

    #[test]
    fn test_execute_add_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 0;

        cpu.execute(Instruction::ADDI(immediate));
        assert_eq!(cpu.registers.A, 244);
        assert_eq!(cpu.registers.F.carry, false);
        assert_eq!(cpu.registers.F.substraction, false);
    }

    #[test]
    fn test_execute_add_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 230;

        cpu.execute(Instruction::ADDI(immediate));
        assert_eq!(cpu.registers.A, (244 + 230) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

    #[test]
    fn test_execute_adc_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            A: 1,
            B: 2,
            C: 3,
            D: 4,
            E: 5,
            F: FlagsRegister::new(),
            H: 6,
            L: 7,
        };

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.A, 3);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, 6);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.A, 10);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.A, 15);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.A, 21);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.A, 28);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.A, 36);
        assert_eq!(cpu.registers.F.carry, false);
    }

    #[test]
    fn test_execute_adc_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        cpu.registers.B = 229;
        cpu.registers.F.carry = true;

        cpu.execute(Instruction::ADC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, (244 + 229 + 1) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

    #[test]
    fn test_execute_adc_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 0;
        cpu.registers.F.carry = true;

        cpu.execute(Instruction::ADCI(immediate));
        assert_eq!(cpu.registers.A, 245);
        assert_eq!(cpu.registers.F.carry, false);
    }

    #[test]
    fn test_execute_adc_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 229;
        cpu.registers.F.carry = true;

        cpu.execute(Instruction::ADCI(immediate));
        assert_eq!(cpu.registers.A, (244 + 229 + 1) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

    #[test]
    fn test_execute_sub_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            A: 29,
            B: 2,
            C: 3,
            D: 4,
            E: 5,
            F: FlagsRegister::new(),
            H: 6,
            L: 7,
        };

        cpu.execute(Instruction::SUB(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, 27);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.A, 24);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.A, 20);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.A, 15);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.A, 9);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.A, 2);
        assert_eq!(cpu.registers.F.carry, false);
        assert_eq!(cpu.registers.F.substraction, true);
    }

    #[test]
    fn test_execute_sub_reg_from_itself_gives_zero() {
        let mut cpu = CPU::new();

        cpu.registers.A = 255;

        cpu.execute(Instruction::SUB(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.A, 0);
        assert_eq!(cpu.registers.F.zero, true);
        assert_eq!(cpu.registers.F.substraction, true);
    }

    #[test]
    fn test_execute_sub_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 2;
        cpu.registers.B = 230;

        cpu.execute(Instruction::SUB(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, (2 - 230) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

    #[test]
    fn test_execute_sub_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 1;

        cpu.execute(Instruction::SUBI(immediate));
        assert_eq!(cpu.registers.A, 243);
        assert_eq!(cpu.registers.F.substraction, true);
    }

    #[test]
    fn test_execute_sbc_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            A: 36,
            B: 2,
            C: 3,
            D: 4,
            E: 5,
            F: FlagsRegister::new(),
            H: 6,
            L: 7,
        };

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, 33);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.A, 29);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.A, 24);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.A, 18);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.A, 11);
        assert_eq!(cpu.registers.F.carry, false);

        cpu.registers.F.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.A, 3);
        assert_eq!(cpu.registers.F.carry, false);

        assert_eq!(cpu.registers.F.substraction, true);
    }

    #[test]
    fn test_execute_sbc_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        let immediate: u8 = 100;
        cpu.registers.F.carry = true;

        cpu.execute(Instruction::SBCI(immediate));
        assert_eq!(cpu.registers.A, 143);
        assert_eq!(cpu.registers.F.carry, false);
    }

    #[test]
    fn test_execute_sbc_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 10;
        let immediate: u8 = 10;
        cpu.registers.F.carry = true;

        cpu.execute(Instruction::SBCI(immediate));
        assert_eq!(cpu.registers.A, (0 - 1) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

}
