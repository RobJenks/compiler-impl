use crate::input::{InputStream, ValidToken};
use common::tokens::token::Token;
use itertools::Itertools;


pub fn read_integral(input: InputStream) -> Option<ValidToken> {
    let mut it = input.clone();

    match it
        .peeking_take_while(|c| c.is_digit(10))
        .collect::<String>() {

        s if s.is_empty() => None,
        s => Some(ValidToken::ok(Token::Integral(
            s.parse()
             .unwrap_or_else(|e| panic!("Failed to parse identified integral token ({})", e))
        ), it))
    }
}