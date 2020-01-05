pub mod token;

use crate::tokens::token::Token;


pub struct Tokenized {
    data : Vec<Token>
}

impl Tokenized {
    pub fn new() -> Self {
        Self { data : vec![] }
    }

    pub fn add(&mut self, token: Token) {
        self.data.push(token);
    }

    pub fn data(&self) -> &Vec<Token> {
        &self.data
    }

    pub fn filter(&mut self, pred: fn(&Token) -> bool) {
        self.data = self.data
            .iter()
            .filter(|&t| pred(t))
            .map(|t| *t)
            .collect();
    }
}


