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

}
