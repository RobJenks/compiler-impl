use crate::input::{InputStream, ValidToken};
use crate::tokens::token::Token;
use std::borrow::BorrowMut;
use itertools::{Itertools, PeekingNext};


pub fn read_whitespace(input: InputStream) -> Option<ValidToken> {
    let mut it = input.clone();

    match it.peeking_take_while(|c| c.is_whitespace()).collect::<Vec<char>>().len() {
        0 => None,
        _ => Some(ValidToken::ok(Token::Whitespace, it))
    }
}

pub fn single_char_token(input: InputStream, ch: char, token: Token) -> Option<ValidToken> {
    let mut it = input.clone();

    if let Some(_) = it.peeking_next(|&c| c == ch) {
        Some(ValidToken::ok(token, it))
    }
    else { None }
}