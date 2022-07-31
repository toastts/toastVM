use nom::digit;
use nom::types::CompleteStr;
use nom::*;

use crate::assembler::Token;
use crate::assembler::label_parser::label_usage;

named!(integer_operand<CompleteStr, Token>,
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

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage
    )
);

