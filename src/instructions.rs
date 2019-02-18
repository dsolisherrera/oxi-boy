pub enum Instruction {
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

pub enum ArithmeticRegisters {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
