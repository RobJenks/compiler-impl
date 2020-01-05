#![feature(slice_patterns)]
#![feature(bindings_after_at)]

mod input;
mod tokens;
mod rules;
mod config;

use crate::tokens::Tokenized;
use crate::tokens::token::Token;
use crate::input::{InputStream, StreamingInput, ValidToken};
use itertools::Itertools;

pub struct Lexer {
    strip_whitespace: bool
}

impl Lexer {
    pub fn new(strip_whitespace: bool) -> Self {
        Self { strip_whitespace }
    }
}

impl Lexer {
    pub fn tokenize(&self, data: &str) -> Tokenized {
        let mut tokenized = Tokenized::new();
        let mut stream: InputStream = InputStream::for_data(data);

        while let Some(_) = stream.peek() {
            let token = rules::match_token(stream.clone());
            match token.as_slice() {
                [] => Lexer::throw_unrecognised_identifier(stream.clone()),
                ts @ [_, _, ..] => Lexer::throw_multiple_tokens(ts, stream.clone()),
                [res] => {
                    if self.should_emit_token(res.get_token()) {
                        tokenized.add(res.get_token());
                    }
                    stream = res.get_next();
                }
            }
        }

        tokenized
    }

    fn should_emit_token(&self, token: Token) -> bool {
        if self.strip_whitespace && token == Token::Whitespace { false }

        else { true }
    }

    fn throw_unrecognised_identifier(stream: InputStream) {
        Lexer::throw_error_with_stream_context("Unrecognised identifier", stream)
    }

    fn throw_multiple_tokens(candidates: &[ValidToken], stream: InputStream) {
        Lexer::throw_error_with_stream_context(format!("Multiple tokens ({}) matching input",
                                                       candidates.iter()
                                                           .map(ValidToken::get_token)
                                                           .join(",")
        ).as_str(), stream)
    }

    fn throw_error_with_stream_context(error_msg: &str, stream: InputStream) {
        let context = stream
            .take(config::constants::LEXER_INPUT_ERROR_STREAM_CONTEXT_MAX_SIZE)
            .collect::<String>();

        panic!("{} '{}' reached at '{}{}'",
               error_msg,
               context.chars().next().map(|c| c.to_string()).unwrap_or("<no-identifier>".to_string()),
               context,
               if context.len() == config::constants::LEXER_INPUT_ERROR_STREAM_CONTEXT_MAX_SIZE { "..." } else { "" }
        );
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
