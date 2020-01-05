mod general;
mod integral;
mod operators;
mod keywords;
use crate::input::{InputStream, ValidToken};

const TOKENIZER_RULES : [fn(InputStream) -> Option<ValidToken>; 9] = [
    general::read_whitespace,
    integral::read_integral,
    operators::read_plus,
    operators::read_minus,
    operators::read_asterisk,
    operators::read_fslash,
    operators::read_equals,
    keywords::read_keyword_let,
    general::read_valid_identifier  // Any valid identifier not matching previous rules
];

pub fn match_token(input: InputStream) -> Option<ValidToken> {
    TOKENIZER_RULES.iter()
        .filter_map(|f| f(input.clone()))
        .take(1)
        .next()
}

trait LexerRule {
    fn evaluate(&self, stream: InputStream) -> Option<ValidToken>;
}