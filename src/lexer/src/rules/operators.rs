use crate::input::{InputStream, ValidToken};
use common::tokens::token::Token;
use crate::rules::general::single_char_token;

pub fn read_plus(stream: InputStream) -> Option<ValidToken> {
    single_char_token(stream, '+', Token::Plus)
}

pub fn read_minus(stream: InputStream) -> Option<ValidToken> {
    single_char_token(stream, '-', Token::Minus)
}

pub fn read_asterisk(stream: InputStream) -> Option<ValidToken> {
    single_char_token(stream, '*', Token::Asterisk)
}

pub fn read_fslash(stream: InputStream) -> Option<ValidToken> {
    single_char_token(stream, '/', Token::FSlash)
}

pub fn read_equals(stream: InputStream) -> Option<ValidToken> {
    single_char_token(stream, '=', Token::Equals)
}
