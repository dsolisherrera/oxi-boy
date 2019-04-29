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

    AND(ArithmeticRegisters),
    ANDI(u8),
    ANDR(),

    OR(ArithmeticRegisters),
    ORI(u8),
    ORR(),

    XOR(ArithmeticRegisters),
    XORI(u8),
    XORR(),
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
