use nom::digit;
use nom::*;

use crate::assembler::Token;

named!(pub integer_operand<types::CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);
