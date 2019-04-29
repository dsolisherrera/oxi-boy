pub mod flagsregister;
pub mod instructions;
pub mod registers;

use self::flagsregister::{
    FlagsRegister, CARRY_FLAG_POSITION, HALF_CARRY_FLAG_POSITION, OPERATION_FLAG_POSITION,
    ZERO_FLAG_POSITION,
};
use self::instructions::{ArithmeticRegisters, Instruction};
use self::registers::Registers;

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
        let (new_value, overflow) = self.registers.a.overflowing_add(immediate);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substraction = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (immediate & 0xF) > 0xF;
        self.registers.f.carry = overflow;

        self.registers.a = new_value;
    }

    fn execute_add_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_adc_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_adc_immediate(source_value);
    }

    fn execute_adc_immediate(&mut self, immediate: u8) {
        let (new_value, overflow_1) = self.registers.a.overflowing_add(immediate);
        let (new_value, overflow_2) =
            new_value.overflowing_add(if self.registers.f.carry { 1 } else { 0 });

        self.registers.f.zero = new_value == 0;
        self.registers.f.substraction = false;
        self.registers.f.half_carry = (self.registers.a & 0xF)
            + (immediate & 0xF)
            + (if self.registers.f.carry { 1 } else { 0 })
            > 0xF;
        self.registers.f.carry = overflow_1 || overflow_2;

        self.registers.a = new_value;
    }

    fn execute_adc_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_sub_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_sub_immediate(source_value);
    }

    fn execute_sub_immediate(&mut self, immediate: u8) {
        let (new_value, overflow) = self.registers.a.overflowing_sub(immediate);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substraction = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (immediate & 0xF) > 0xF;
        self.registers.f.carry = overflow;

        self.registers.a = new_value;
    }

    fn execute_sub_relative(&mut self, source: ArithmeticRegisters) {}

    fn execute_sbc_reg(&mut self, source: ArithmeticRegisters) {
        let source_value = self.registers.load(source);
        self.execute_sbc_immediate(source_value);
    }

    fn execute_sbc_immediate(&mut self, immediate: u8) {
        let (new_value, overflow_1) = self.registers.a.overflowing_sub(immediate);
        let (new_value, overflow_2) =
            new_value.overflowing_sub(if self.registers.f.carry { 1 } else { 0 });

        self.registers.f.zero = new_value == 0;
        self.registers.f.substraction = true;
        self.registers.f.half_carry = (self.registers.a & 0xF)
            + (immediate & 0xF)
            + (if self.registers.f.carry { 1 } else { 0 })
            > 0xF;
        self.registers.f.carry = overflow_1 || overflow_2;

        self.registers.a = new_value;
    }

    fn execute_scb_relative(&mut self, source: ArithmeticRegisters) {}
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn test_execute_add_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: FlagsRegister::new(),
            h: 6,
            l: 7,
        };

        cpu.execute(Instruction::ADD(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.a, 2);
        assert_eq!(cpu.registers.f.substraction, false);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, 4);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.a, 7);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.a, 11);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.a, 16);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.a, 22);

        cpu.execute(Instruction::ADD(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.a, 29);
    }

    #[test]
    fn test_execute_add_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        cpu.registers.b = 230;

        cpu.execute(Instruction::ADD(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, (244 + 230) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_execute_add_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 0;

        cpu.execute(Instruction::ADDI(immediate));
        assert_eq!(cpu.registers.a, 244);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.substraction, false);
    }

    #[test]
    fn test_execute_add_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 230;

        cpu.execute(Instruction::ADDI(immediate));
        assert_eq!(cpu.registers.a, (244 + 230) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_execute_adc_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: FlagsRegister::new(),
            h: 6,
            l: 7,
        };

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.a, 3);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, 6);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.a, 10);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.a, 15);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.a, 21);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.a, 28);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.a, 36);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_execute_adc_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        cpu.registers.b = 229;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, (244 + 229 + 1) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_execute_adc_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 0;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADCI(immediate));
        assert_eq!(cpu.registers.a, 245);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_execute_adc_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 229;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADCI(immediate));
        assert_eq!(cpu.registers.a, (244 + 229 + 1) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_execute_sub_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            a: 29,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: FlagsRegister::new(),
            h: 6,
            l: 7,
        };

        cpu.execute(Instruction::SUB(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, 27);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.a, 24);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.a, 20);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.a, 15);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.a, 9);

        cpu.execute(Instruction::SUB(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.a, 2);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.substraction, true);
    }

    #[test]
    fn test_execute_sub_reg_from_itself_gives_zero() {
        let mut cpu = CPU::new();

        cpu.registers.a = 255;

        cpu.execute(Instruction::SUB(ArithmeticRegisters::A));
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.substraction, true);
    }

    #[test]
    fn test_execute_sub_reg_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 2;
        cpu.registers.b = 230;

        cpu.execute(Instruction::SUB(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, (2 - 230) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_execute_sub_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 1;

        cpu.execute(Instruction::SUBI(immediate));
        assert_eq!(cpu.registers.a, 243);
        assert_eq!(cpu.registers.f.substraction, true);
    }

    #[test]
    fn test_execute_sbc_reg_for_all_registers() {
        let mut cpu = CPU::new();

        cpu.registers = Registers {
            a: 36,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: FlagsRegister::new(),
            h: 6,
            l: 7,
        };

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.a, 33);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::C));
        assert_eq!(cpu.registers.a, 29);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::D));
        assert_eq!(cpu.registers.a, 24);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::E));
        assert_eq!(cpu.registers.a, 18);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::H));
        assert_eq!(cpu.registers.a, 11);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticRegisters::L));
        assert_eq!(cpu.registers.a, 3);
        assert_eq!(cpu.registers.f.carry, false);

        assert_eq!(cpu.registers.f.substraction, true);
    }

    #[test]
    fn test_execute_sbc_immediate() {
        let mut cpu = CPU::new();

        cpu.registers.a = 244;
        let immediate: u8 = 100;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::SBCI(immediate));
        assert_eq!(cpu.registers.a, 143);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_execute_sbc_immediate_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.a = 10;
        let immediate: u8 = 10;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::SBCI(immediate));
        assert_eq!(cpu.registers.a, (0 - 1) as u8);
        assert_eq!(cpu.registers.f.carry, true);
    }

}
