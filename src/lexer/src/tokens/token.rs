#[derive(Clone, Copy, Debug)]
pub enum Token {
    SOF,
    EOF,
    Whitespace,
    Integral (i64),
    Plus,
    Minus,
    Asterisk,
    FSlash
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}