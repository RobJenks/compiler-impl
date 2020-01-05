use crate::input::{InputStream, ValidToken};
use crate::tokens::token::Token;
use crate::rules::general::single_string_token;

pub fn read_keyword_let(stream: InputStream) -> Option<ValidToken> {
    single_string_token(stream, "let".to_string(), Token::Let)
}