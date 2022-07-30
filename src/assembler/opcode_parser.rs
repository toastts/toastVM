use nom::*;

use crate::assembler::Token;
use crate::instruction::Opcode;

named!(pub opcode_load<types::CompleteStr, Token>,
  do_parse!(
      tag!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);

named!(pub opcode<types::CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        Token::Op{code: Opcode::from(opcode)}
      )
  )
);

