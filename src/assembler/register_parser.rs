use nom::digit;
use nom::*;

use crate::assembler::Token;

named!(pub register <types::CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                  reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

