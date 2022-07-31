use nom::types::CompleteStr;
use nom::*;

use crate::assembler::instruction_parser::{instruction, AssemblerInstruction};
use crate::assembler::SymbolTable;
use crate::assembler::directive_parser::directive;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes(symbols));
        }
        program
    }
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(alt!(instruction | directive)) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

