#[derive(Debug, PartialEq)]
pub enum Opcode {
    ADD,
    DIV,
    HLT,
    IGL,
    JMP,
    JMPB,
    JMPF,
    LOAD,
    MUL,
    SUB,
    EQ,
    NEQ,
    GTE,
    LTE,
    LT,
    GT,
    JMPE,
    NOP,
}
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            6 => Opcode::HLT,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::NEQ,
            12 => Opcode::GTE,
            13 => Opcode::GT,
            14 => Opcode::LTE,
            15 => Opcode::LT,
            16 => Opcode::JMPE,
            17 => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}
