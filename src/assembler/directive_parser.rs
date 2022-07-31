use nom::alpha;
use nom::types::CompleteStr;
use nom::*;

use crate::assembler::Token;

named!(pub directive <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!(".") >>
            d: alpha >>
            (
                Token::Directive{
                  name: d.to_string(),
                }
            )
        )
    )
);

