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
            _ => Opcode::IGL,
        }
    }
}
