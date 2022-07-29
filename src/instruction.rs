#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcdoe::HLT);
    }

    #[test]
    fn test_make_instruction() {
        let instruction = Instruciton::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
