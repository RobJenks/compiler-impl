use crate::input::{InputStream, ValidToken};
use crate::tokens::token::Token;
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

pub fn single_string_token(input: InputStream, st: String, token: Token) -> Option<ValidToken> {
    let it = input.clone();

    if it.clone().zip(st.chars()).all(|(x,y)| x == y) {
        Some(ValidToken::ok(token, it.dropping(st.len())))
    }
    else { None }
}

pub fn read_valid_identifier(input: InputStream) -> Option<ValidToken> {
    let mut it = input.clone();

    let id = it
        .peeking_take_while(|&c| c.is_alphanumeric() || c == '_')
        .collect::<String>();

    if let Some(c) = id.chars().next() {
        if c.is_numeric() { None }
        else { Some(ValidToken::ok(Token::Identifier(id), it)) }
    }
    else { None }
}