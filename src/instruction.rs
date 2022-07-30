#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    ILGL,
    ADD,
    DIV,
    LOAD,
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
            0 => return Opcode::HLT,
            _ => return Opcode::ILGL,
        }
    }
}

