const ZERO_FLAG_POSITION: u8 = 7;
const OPERATION_FLAG_POSITION: u8 = 6;
const HALF_CARRY_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

struct FlagsRegister {
    zero: bool,
    operation: bool,
    half_carry: bool,
    carry: bool,
}

impl FlagsRegister {
    fn new() -> Self {
        FlagsRegister {
            zero: false,
            operation: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_POSITION
            | (if flag.operation { 1 } else { 0 }) << OPERATION_FLAG_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        FlagsRegister {
            zero: (byte >> ZERO_FLAG_POSITION) & 1 != 0,
            operation: (byte >> OPERATION_FLAG_POSITION) & 1 != 0,
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
            Instruction::ADD(target) => {
                self.execute_add(target);
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn execute_add(&mut self, source: ArithmeticRegisters) {

        let source_value = self.registers.load(source);

        let (new_value, overflow) = self.registers.A.overflowing_add(source_value);

        self.registers.F.zero = new_value == 0;
        self.registers.F.operation = false;
        self.registers.F.half_carry = (self.registers.A & 0xF) + (source_value & 0xF) > 0xF;
        self.registers.F.carry = overflow;

        self.registers.A = new_value;
    }
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
        assert_eq!(flag_reg.operation, true);
        assert_eq!(flag_reg.half_carry, true);
        assert_eq!(flag_reg.carry, true);
    }

    #[test]
    fn test_convert_u8_to_flagsregister_with_no_flags_set() {
        let byte: u8 = 0;
        let flag_reg = FlagsRegister::from(byte);

        assert_eq!(flag_reg.zero, false);
        assert_eq!(flag_reg.operation, false);
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
            operation: true,
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
    fn test_execute_add_instruction_for_all_registers() {
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
    fn test_execute_add_instruction_with_overflow() {
        let mut cpu = CPU::new();

        cpu.registers.A = 244;
        cpu.registers.B = 230;

        cpu.execute(Instruction::ADD(ArithmeticRegisters::B));
        assert_eq!(cpu.registers.A, (244 + 230) as u8);
        assert_eq!(cpu.registers.F.carry, true);
    }

}
