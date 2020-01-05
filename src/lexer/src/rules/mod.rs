mod general;
mod integral;
mod operators;
use crate::input::{InputStream, ValidToken};

const TOKENIZER_RULES : [fn(InputStream) -> Option<ValidToken>; 6] = [
    general::read_whitespace,
    integral::read_integral,
    operators::read_plus,
    operators::read_minus,
    operators::read_asterisk,
    operators::read_fslash
];

pub fn match_token(input: InputStream) -> Vec<ValidToken> {
    TOKENIZER_RULES.iter()
        .filter_map(|f| f(input.clone()))
        .collect()
}

trait LexerRule {
    fn evaluate(&self, stream: InputStream) -> Option<ValidToken>;
}