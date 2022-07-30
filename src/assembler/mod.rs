pub mod opcode_parser;
pub mod operand_parser;
pub mod register_parser;
pub mod instruction_parser;
pub mod program_parser;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
}

