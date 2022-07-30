use nom::types::CompleteStr;

use crate::assembler::Token;
use crate::instruction::Opcode;

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);
