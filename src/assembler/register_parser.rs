use nom::digit;
use nom::types::CompleteStr;

use assembler::Token;

named!(pub register <CompleteStr, Token>,
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
