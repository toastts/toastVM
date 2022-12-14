use std;

use nom::types::CompleteStr;
use nom::*;

use crate::assembler::label_parser::label_declaration;
use crate::assembler::opcode_parser::*;
use crate::assembler::operand_parser::operand;
use crate::assembler::{SymbolTable, Token};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(ref token) => match token {
                Token::Op { code } => match code {
                    _ => {
                        results.push(*code as u8);
                    }
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                    std::process::exit(1);
                }
            },
            None => {}
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols)
            }
        }
        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            Token::LabelUsage { name } => match symbols.symbol_value(name) {
                Some(value) => {
                    let byte1 = value;
                    let byte2 = value >> 8;
                    results.push(byte2 as u8);
                    results.push(byte1 as u8);
                }
                None => {}
            },
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined
        ) >>
        (
            ins
        )
    )
);
