use core::iter::*;
use core::str::Chars;
use crate::tokens::token::Token;

pub type InputStream<'a> = Peekable<Chars<'a>>;

pub trait StreamingInput <'a> {
    fn for_data(data: &'a str) -> Self;
}

impl <'a> StreamingInput<'a> for InputStream<'a> {
    fn for_data(input: &'a str) -> Self {
        input.chars().peekable()
    }
}

pub struct ValidToken<'a> {
    token: Token,
    next: InputStream<'a>
}

impl <'a> ValidToken<'a> {
    pub fn ok(token: Token, next: InputStream<'a>) -> Self {
        Self { token, next }
    }

    pub fn get_token(&self) -> Token { self.token.clone() }
    pub fn get_next(&self) -> InputStream<'a> { self.next.clone() }

}
